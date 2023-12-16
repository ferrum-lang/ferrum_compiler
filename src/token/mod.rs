use crate::source;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum FeTokenPackage {
    File(FeTokenFile),
    Dir(FeTokenDir),
}

impl From<source::FeSourcePackage> for FeTokenPackage {
    fn from(value: source::FeSourcePackage) -> Self {
        match value {
            source::FeSourcePackage::File(file) => return FeTokenPackage::File(file.into()),
            source::FeSourcePackage::Dir(dir) => return FeTokenPackage::Dir(dir.into()),
        };
    }
}

#[derive(Debug, Clone)]
pub struct FeTokenFile {
    pub name: TokenPackageName,
    pub path: PathBuf,
    pub tokens: Arc<Mutex<Vec<Arc<Token>>>>,
}

impl From<source::FeSourceFile> for FeTokenFile {
    fn from(value: source::FeSourceFile) -> Self {
        return Self {
            name: value.name.into(),
            path: value.path,
            tokens: Arc::new(Mutex::new(vec![])),
        };
    }
}

#[derive(Debug, Clone)]
pub struct FeTokenDir {
    pub name: TokenPackageName,
    pub path: PathBuf,
    pub entry_file: FeTokenFile,
    pub local_packages: HashMap<TokenPackageName, Arc<Mutex<FeTokenPackage>>>,
}

impl From<source::FeSourceDir> for FeTokenDir {
    fn from(value: source::FeSourceDir) -> Self {
        return Self {
            name: value.name.into(),
            path: value.path,
            entry_file: value.entry_file.into(),
            local_packages: value
                .local_packages
                .into_iter()
                .map(|(name, pkg)| {
                    let name: TokenPackageName = name.into();

                    let pkg: &source::FeSourcePackage = &pkg.lock().unwrap();
                    let pkg: FeTokenPackage = pkg.clone().into();
                    let pkg = Arc::new(Mutex::new(pkg));

                    return (name, pkg);
                })
                .collect(),
        };
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TokenPackageName(pub Arc<str>);

impl From<source::SourcePackageName> for TokenPackageName {
    fn from(value: source::SourcePackageName) -> Self {
        return Self(value.0);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Arc<str>,
    pub span: Span,
}

impl Token {
    pub fn zero(token_type: TokenType, lexeme: impl Into<Arc<str>>) -> Arc<Self> {
        return Arc::new(Self {
            token_type,
            lexeme: lexeme.into(),
            span: Span::zero(),
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Symbols
    Comma, // ,

    Semicolon, // ;

    ParenLeft,  // (
    ParenRight, // )

    SquirlyBraceLeft,  // {
    SquirlyBraceClose, // }

    SquareBracketLeft,  // [
    SquareBracketRight, // ]

    Plus,      // +
    PlusEqual, // +=

    Minus,        // -
    MinusEqual,   // -=
    MinusGreater, // ->

    Equal,        // =
    EqualEqual,   // ==
    EqualGreater, // =>

    Asterisk,      // *
    AsteriskEqual, // *=

    Slash,      // /
    SlashEqual, // /=

    BackSlash, // \

    Amp, // &

    At, // @

    Question, // ?

    Exclamation, // !

    Less,      // <
    LessEqual, // <=

    Greater,        // >
    GreaterGreater, // >>
    GreaterEqual,   // >=

    Colon,      // :
    ColonColon, // ::
    ColonEqual, // :=

    Dot,      // .
    DotDot,   // ..
    DotSlash, // ./

    Tilde,      // ~
    TildeSlash, // ~/

    // Keywords
    And,
    As,
    Break,
    Const,
    Do,
    Else,
    Fn,
    For,
    If,
    In,
    Is,
    Impl,
    Loop,
    Match,
    Mut,
    NewType,
    Noop,
    // Norm,
    Not,
    Or,
    Pub,
    // Pure,
    Return,
    Risk,
    // Safe,
    Struct,
    Then,
    Trait,
    Use,
    While,

    // Literals
    True,
    False,

    String,

    StringFmtStart,
    StringFmtMid,
    StringFmtEnd,

    Char,

    NumberInteger,
    NumberDecimal,
    // TODO: other numbers? scientific notation?

    // Other
    Identifier,
    Label,

    SelfVal,
    SelfType,

    Crash,

    Newline,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
