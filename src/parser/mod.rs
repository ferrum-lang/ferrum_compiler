use crate::result::Result;
use crate::syntax::{Decl, FeSyntaxPackage, SyntaxTree, Use, UseMod};
use crate::token::{FeTokenPackage, Token, TokenType};

use std::sync::{Arc, Mutex};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct FeSyntaxParser {
    token_pkg: Arc<Mutex<FeTokenPackage>>,

    out: FeSyntaxPackage,
}

impl FeSyntaxParser {
    pub fn parse_package(token_pkg: Arc<Mutex<FeTokenPackage>>) -> Result<FeSyntaxPackage> {
        return Self::new(token_pkg).parse();
    }

    pub fn new(token_pkg: Arc<Mutex<FeTokenPackage>>) -> Self {
        let out = token_pkg.lock().unwrap().clone().into();

        return Self { token_pkg, out };
    }

    pub fn parse(mut self) -> Result<FeSyntaxPackage> {
        match (&*self.token_pkg.lock().unwrap(), &mut self.out) {
            (FeTokenPackage::File(token_file), FeSyntaxPackage::File(syntax_file)) => {
                FeTokenSyntaxParser::parse_syntax(
                    token_file.tokens.lock().unwrap().clone(),
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
    tokens: Vec<Arc<Token>>,
    out: Arc<Mutex<SyntaxTree>>,

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
    fn parse_syntax(tokens: Vec<Arc<Token>>, syntax_tree: Arc<Mutex<SyntaxTree>>) -> Result {
        return Self::new(tokens, syntax_tree).parse();
    }

    fn new(tokens: Vec<Arc<Token>>, syntax_tree: Arc<Mutex<SyntaxTree>>) -> Self {
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

            match self.use_declaration() {
                Ok(use_decl) => {
                    self.out.lock().unwrap().uses.push(use_decl);

                    if !self.is_at_end() {
                        let _ = self.consume(&TokenType::Newline, "Expect newline after use");
                    }
                }

                // TODO: Improve compiling around errors and error reporting
                Err(e) => return Err(e),
            }
        }

        while !self.is_at_end() {
            if self.allow_many_newlines() > 0 {
                continue;
            }

            match self.declaration() {
                Ok(decl) => {
                    self.out.lock().unwrap().decls.push(decl);

                    if !self.is_at_end() {
                        let _ =
                            self.consume(&TokenType::Newline, "Expect newline after declaration");
                    }
                }

                // TODO: Improve compiling around errors and error reporting
                Err(e) => return Err(e),
            }
        }

        return Ok(());
    }

    fn use_declaration(&mut self) -> Result<Arc<Mutex<Use>>> {
        let use_mod = self.use_mod();

        todo!()
    }

    fn use_mod(&mut self) -> Option<UseMod> {
        if self.match_any(&[TokenType::Pub], WithNewlines::None) {
            let token = self.previous().unwrap();

            return Some(UseMod::Pub(token));
        }

        return None;
    }

    fn declaration(&mut self) -> Result<Arc<Mutex<Decl>>> {
        self.advance();
        todo!()
    }

    fn consume(
        &mut self,
        token_type: &TokenType,
        err_msg: impl Into<String>,
    ) -> Result<Arc<Token>> {
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

    fn error(&mut self, message: String, t: Arc<Token>) -> ParserError {
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

    fn advance(&mut self) -> Option<Arc<Token>> {
        if !self.is_at_end() {
            self.current_idx += 1;
        }

        return self.previous();
    }

    fn backtrack(&mut self) -> Option<Arc<Token>> {
        if self.current_idx == 0 {
            return None;
        }

        self.current_idx -= 1;

        return self.peek();
    }

    fn is_at_end(&self) -> bool {
        return self.current_idx >= self.tokens.len();
    }

    fn peek(&self) -> Option<Arc<Token>> {
        return self.tokens.get(self.current_idx).cloned();
    }

    fn previous(&self) -> Option<Arc<Token>> {
        if self.current_idx == 0 {
            return None;
        }

        return self.tokens.get(self.current_idx - 1).cloned();
    }
}
