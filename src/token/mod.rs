use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Arc<str>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Symbols
    Semicolon,  // ;
    OpenParen,  // (
    CloseParen, // )

    DoubleColon, // ::

    // Keywords
    Fn,
    Pub,
    Use,

    // Literals
    StringLiteral,

    // Other
    Ident,
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
