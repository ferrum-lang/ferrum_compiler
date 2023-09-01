use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum FeType {
    Callable(Callable),
    String(Option<StringDetails>),
    Bool(Option<bool>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Callable {
    pub special: Option<SpecialCallable>,
    pub params: Vec<(Arc<str>, FeType)>,
    pub return_type: Option<Box<FeType>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecialCallable {
    Print,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringDetails {
    PlainLiteral,
    Format,
}
