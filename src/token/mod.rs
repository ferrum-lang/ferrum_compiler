use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum FeTokenPackage {
    File(FeTokenFile),
    Dir(FeTokenDir),
}

#[derive(Debug, Clone)]
pub struct FeTokenFile {
    pub name: TokenPackageName,
    pub path: PathBuf,
    pub tokens: Arc<Mutex<Vec<Arc<Token>>>>,
}

#[derive(Debug, Clone)]
pub struct FeTokenDir {
    pub name: TokenPackageName,
    pub path: PathBuf,
    pub entry_file: FeTokenFile,
    pub local_packages: HashMap<TokenPackageName, Arc<Mutex<FeTokenPackage>>>,
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct TokenPackageName(pub Arc<str>);

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Arc<str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Symbols
    Comma,      // ,
    Colon,      // :
    Semicolon,  // ;
    OpenParen,  // (
    CloseParen, // )

    DoubleColon, // ::

    // Keywords
    Fn,
    Norm,
    Pub,
    Pure,
    Risk,
    Safe,
    Use,

    // Literals
    PlainString,

    // Other
    Ident,

    Newline,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn zero() -> Self {
        return Self {
            start: Position::zero(),
            end: Position::zero(),
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn zero() -> Self {
        return Self {
            index: 0,
            line: 0,
            column: 0,
        };
    }
}
