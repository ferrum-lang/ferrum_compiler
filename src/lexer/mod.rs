use crate::result::Result;
use crate::source::*;
use crate::token::*;
use crate::utils::*;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use lazy_static;

lazy_static::lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut keywords = HashMap::new();
        // keywords.insert("and".to_string(), TokenType::And);
        // keywords.insert("as".to_string(), TokenType::As);
        // keywords.insert("const".to_string(), TokenType::Const);
        // keywords.insert("CRASH!".to_string(), TokenType::Crash);
        // keywords.insert("else".to_string(), TokenType::Else);
        // keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("fn".to_string(), TokenType::Fn);
        // keywords.insert("for".to_string(), TokenType::For);
        // keywords.insert("if".to_string(), TokenType::If);
        // keywords.insert("impl".to_string(), TokenType::Impl);
        // keywords.insert("in".to_string(), TokenType::In);
        // keywords.insert("match".to_string(), TokenType::Match);
        // keywords.insert("mut".to_string(), TokenType::Mut);
        // keywords.insert("norm".to_string(), TokenType::Norm);
        // keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("pub".to_string(), TokenType::Pub);
        // keywords.insert("pure".to_string(), TokenType::Pure);
        // keywords.insert("return".to_string(), TokenType::Return);
        // keywords.insert("risk".to_string(), TokenType::Risk);
        // keywords.insert("safe".to_string(), TokenType::Safe);
        // keywords.insert("self".to_string(), TokenType::SelfVal);
        // keywords.insert("Self".to_string(), TokenType::SelfType);
        // keywords.insert("struct".to_string(), TokenType::Struct);
        // keywords.insert("trait".to_string(), TokenType::Trait);
        // keywords.insert("true".to_string(), TokenType::True);
        // keywords.insert("type".to_string(), TokenType::Type);
        keywords.insert("use".to_string(), TokenType::Use);
        // keywords.insert("while".to_string(), TokenType::While);
        // keywords.insert("yield".to_string(), TokenType::Yield);

        keywords
    };
}

#[derive(Debug, Clone)]
pub struct FeLexer {
    source_pkg: Arc<Mutex<FeSourcePackage>>,

    out: FeTokenPackage,
}

impl FeLexer {
    pub fn scan_package(source_pkg: Arc<Mutex<FeSourcePackage>>) -> Result<FeTokenPackage> {
        return Self::new(source_pkg).scan();
    }

    pub fn new(source_pkg: Arc<Mutex<FeSourcePackage>>) -> Self {
        let out = source_pkg.lock().unwrap().clone().into();

        return Self { source_pkg, out };
    }

    pub fn scan(mut self) -> Result<FeTokenPackage> {
        fn _scan<'a, 'b>(
            src_pkg: &'a FeSourcePackage,
            out: &'b mut FeTokenPackage,
        ) -> Result<&'b mut FeTokenPackage> {
            match (src_pkg, &mut *out) {
                (FeSourcePackage::File(source_file), FeTokenPackage::File(token_file)) => {
                    FeSourceScanner::scan_source(
                        source_file.content.clone(),
                        token_file.tokens.clone(),
                    )?;
                }

                (FeSourcePackage::Dir(source_dir), FeTokenPackage::Dir(token_dir)) => {
                    FeSourceScanner::scan_source(
                        source_dir.entry_file.content.clone(),
                        token_dir.entry_file.tokens.clone(),
                    )?;

                    for (name, source_pkg) in source_dir.local_packages.iter() {
                        let token_pkg = token_dir
                            .local_packages
                            .get(&TokenPackageName::from(name.clone()))
                            .expect("source doesn't match tokens structure");

                        _scan(&source_pkg.lock().unwrap(), &mut token_pkg.lock().unwrap())?;
                    }
                }

                (FeSourcePackage::File(_), _) | (FeSourcePackage::Dir(_), _) => unreachable!(),
            }

            return Ok(out);
        }

        _scan(&*self.source_pkg.lock().unwrap(), &mut self.out)?;

        return Ok(self.out);
    }
}

#[derive(Debug, Clone)]
struct FeSourceScanner {
    source: Arc<str>,

    out: Arc<Mutex<Vec<Arc<Token>>>>,

    cursor: usize,
    span: Span,
    format_string_nest: usize,
}

impl FeSourceScanner {
    fn scan_source(
        source: Arc<str>,
        tokens: Arc<Mutex<Vec<Arc<Token>>>>,
    ) -> Result<Arc<Mutex<Vec<Arc<Token>>>>> {
        return Self::new(source, tokens).scan();
    }

    fn new(source: Arc<str>, tokens: Arc<Mutex<Vec<Arc<Token>>>>) -> Self {
        let mut span = Span::zero();
        span.start.line = 1;
        span.start.column = 1;
        span.end = span.start.clone();

        return Self {
            source,

            out: tokens,

            cursor: 0,
            span,
            format_string_nest: 0,
        };
    }

    fn scan(mut self) -> Result<Arc<Mutex<Vec<Arc<Token>>>>> {
        while !self.is_end() {
            self.scan_token();
            self.span.start = self.span.end.clone();
        }

        return Ok(self.out);
    }

    fn scan_token(&mut self) {
        let Some(c) = self.current() else { return };

        let token_type = match c {
            ' ' | '\r' | '\t' => None,

            ',' => Some(TokenType::Comma),
            ';' => Some(TokenType::Semicolon),

            '(' => Some(TokenType::OpenParen),
            ')' => Some(TokenType::CloseParen),

            '\n' => Some(TokenType::Newline),

            ':' => {
                if self.peek_next() == Some(':') {
                    self.advance_col();
                    Some(TokenType::DoubleColon)
                } else {
                    Some(TokenType::Comma)
                }
            }

            '.' => {
                if self.peek_next() == Some('/') {
                    self.advance_col();
                    Some(TokenType::DotSlash)
                } else {
                    Some(TokenType::Dot)
                }
            }

            '"' => Some(self.string(false)),

            c if self.is_digit(c) => todo!(),
            c if self.is_alpha(c) => Some(self.identifier()),

            c => todo!("TODO: Support [{c}]\n{}", &self.source[self.cursor..]),
        };

        match token_type {
            Some(token_type @ TokenType::Newline) => {
                self.add_token(token_type);
                self.advance_line();
            }

            Some(token_type) => {
                self.add_token(token_type);
                self.advance_col();
            }

            None => self.advance_col(),
        }
    }

    fn current(&self) -> Option<char> {
        return self.source.char_at(self.cursor);
    }

    fn peek_next(&self) -> Option<char> {
        return self.source.char_at(self.cursor + 1);
    }

    fn string(&mut self, is_continuing_fmt_str: bool) -> TokenType {
        let mut is_starting_fmt_str = false;

        while !self.is_end() {
            match self.peek_next() {
                Some('"') => {
                    self.advance_col();
                    break;
                }
                Some('{') => {
                    is_starting_fmt_str = true;
                    break;
                }
                Some('\\') => {
                    self.advance_col();
                }
                Some('\n') => {
                    self.advance_line();
                }
                _ => {}
            }

            self.advance_col();
        }

        if self.is_end() {
            todo!();
            // self.error_ctx
            //     .error(self.span.clone(), "Unterminated string.");
        }

        match (is_continuing_fmt_str, is_starting_fmt_str) {
            (false, false) => return TokenType::PlainString,
            // (false, true) => {
            //     self.format_string_nest += 1;
            //     return TokenType::FormatStringOpen;
            // }
            // (true, true) => return TokenType::FormatStringMid,
            // (true, false) => {
            //     self.format_string_nest -= 1;
            //     return TokenType::FormatStringClose;
            // }
            _ => todo!(),
        }
    }

    fn identifier(&mut self) -> TokenType {
        while let Some(peek) = self.peek_next() {
            if !self.is_alpha_numeric(peek) {
                break;
            }

            self.advance_col();
        }

        let text = &self.source[self.span.start.index..=self.span.end.index];

        if text == "CRASH" && self.peek_next() == Some('!') {
            self.advance_col();

            todo!();
            // return TokenType::Crash;
        }

        return KEYWORDS.get(text).cloned().unwrap_or(TokenType::Ident);
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.span.start.index..=self.span.end.index];

        self.out.lock().unwrap().push(Arc::new(Token {
            token_type,
            lexeme: text.into(),
            span: self.span.clone(),
        }));
    }

    fn advance_col(&mut self) {
        self.cursor += 1;

        self.span.end.index += 1;
        self.span.end.column += 1;
    }

    fn advance_line(&mut self) {
        self.cursor += 1;

        self.span.end.index += 1;
        self.span.end.line += 1;

        self.span.end.column = 1;
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }

    fn is_end(&self) -> bool {
        return self.cursor >= self.source.len();
    }
}
