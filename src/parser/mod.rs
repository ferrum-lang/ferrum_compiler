use crate::result::Result;
use crate::syntax::{Decl, FeSyntaxPackage, SyntaxTree, Use};
use crate::token::{FeTokenPackage, Token, TokenType};

use std::cell::RefCell;
use std::rc::Rc;

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct FeSyntaxParser {
    token_pkg: Rc<RefCell<FeTokenPackage>>,

    out: FeSyntaxPackage,
}

impl FeSyntaxParser {
    pub fn parse_package(token_pkg: Rc<RefCell<FeTokenPackage>>) -> Result<FeSyntaxPackage> {
        return Self::new(token_pkg).parse();
    }

    pub fn new(token_pkg: Rc<RefCell<FeTokenPackage>>) -> Self {
        let out = token_pkg.borrow().clone().into();

        return Self { token_pkg, out };
    }

    pub fn parse(mut self) -> Result<FeSyntaxPackage> {
        match (&*self.token_pkg.borrow(), &mut self.out) {
            (FeTokenPackage::File(token_file), FeSyntaxPackage::File(syntax_file)) => {
                FeTokenSyntaxParser::parse_syntax(
                    token_file.tokens.borrow().clone(),
                    syntax_file.syntax.clone(),
                )?;
            }
            (FeTokenPackage::Dir(token_dir), FeSyntaxPackage::Dir(syntax_dir)) => todo!(),

            (FeTokenPackage::File(_), _) | (FeTokenPackage::Dir(_), _) => unreachable!(),
        }

        return Ok(self.out);
    }
}

struct FeTokenSyntaxParser {
    tokens: Vec<Rc<Token>>,
    out: Rc<RefCell<SyntaxTree>>,

    current_idx: usize,
}

// TODO: Improve error reporting
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Error: {message}")]
    Error { message: String },
}

#[derive(Clone, Debug, PartialEq)]
pub enum WithNewlines {
    None,
    One,
    Many,
}

impl FeTokenSyntaxParser {
    fn parse_syntax(tokens: Vec<Rc<Token>>, syntax_tree: Rc<RefCell<SyntaxTree>>) -> Result {
        return Self::new(tokens, syntax_tree).parse();
    }

    fn new(tokens: Vec<Rc<Token>>, syntax_tree: Rc<RefCell<SyntaxTree>>) -> Self {
        return Self {
            tokens,
            out: syntax_tree,

            current_idx: 0,
        };
    }

    fn parse(mut self) -> Result {
        while !self.is_at_end() {
            if self.allow_many_newlines() > 0 {
                continue;
            }

            if let Some(use_decl) = self.use_declaration() {
                self.out.borrow_mut().uses.push(use_decl);

                if !self.is_at_end() {
                    let _ = self.consume(&TokenType::Newline, "Expect newline after use");
                }
            }
        }

        while !self.is_at_end() {
            if self.allow_many_newlines() > 0 {
                continue;
            }

            if let Some(decl) = self.declaration() {
                self.out.borrow_mut().decls.push(decl);

                if !self.is_at_end() {
                    let _ = self.consume(&TokenType::Newline, "Expect newline after declaration");
                }
            }
        }

        return Ok(());
    }

    fn use_declaration(&mut self) -> Option<Rc<RefCell<Use>>> {
        self.advance();
        return None;
    }

    fn declaration(&mut self) -> Option<Rc<RefCell<Decl>>> {
        self.advance();
        return None;
    }

    fn consume(&mut self, token_type: &TokenType, err_msg: impl Into<String>) -> Result<Rc<Token>> {
        if self.check(token_type) {
            return self.advance().ok_or_else(|| unreachable!());
        }

        let t = self.peek().ok_or_else(|| self.eof_err())?;

        return Err(self.error(err_msg.into(), t).into());
    }

    fn eof_err(&mut self) -> ParserError {
        let message = format!("[{}:{}] Unexpected end of file.", file!(), line!());
        // self.error_ctx.error(Span::new(), message.clone());

        return ParserError::Error { message };
    }

    fn error(&mut self, message: String, t: Rc<Token>) -> ParserError {
        // self.error_ctx.token_error(t, message.clone());

        return ParserError::Error { message };
    }

    fn allow_many_newlines(&mut self) -> usize {
        let mut any_newlines = 0;

        while self.allow_one_newline() {
            any_newlines += 1;
        }

        return any_newlines;
    }

    fn allow_one_newline(&mut self) -> bool {
        return self.match_any(&[TokenType::Newline], WithNewlines::None);
    }

    fn match_any(&mut self, token_types: &[TokenType], with_newlines: WithNewlines) -> bool {
        let newlines: usize = match with_newlines {
            WithNewlines::None => 0,
            WithNewlines::One => {
                if self.allow_one_newline() {
                    1
                } else {
                    0
                }
            }

            WithNewlines::Many => self.allow_many_newlines(),
        };

        for token_type in token_types {
            if self.check(token_type) {
                self.advance();

                return true;
            }
        }

        for _ in 0..newlines {
            self.backtrack();
        }

        return false;
    }

    fn check(&self, token_type: &TokenType) -> bool {
        return self.check_offset(0, token_type);
    }

    fn check_offset(&self, offset: usize, token_type: &TokenType) -> bool {
        return self
            .tokens
            .get(self.current_idx + offset)
            .map(|peek| peek.token_type == *token_type)
            .unwrap_or(false);
    }

    fn advance(&mut self) -> Option<Rc<Token>> {
        if !self.is_at_end() {
            self.current_idx += 1;
        }

        return self.previous();
    }

    fn backtrack(&mut self) -> Option<Rc<Token>> {
        if self.current_idx == 0 {
            return None;
        }

        self.current_idx -= 1;

        return self.peek();
    }

    fn is_at_end(&self) -> bool {
        return self.current_idx >= self.tokens.len();
    }

    fn peek(&self) -> Option<Rc<Token>> {
        return self.tokens.get(self.current_idx).cloned();
    }

    fn previous(&self) -> Option<Rc<Token>> {
        if self.current_idx == 0 {
            return None;
        }

        return self.tokens.get(self.current_idx - 1).cloned();
    }
}
