use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum FeType {
    Callable(Callable),
    Struct(FeStruct),
    String(Option<StringDetails>),
    Bool(Option<bool>),
    Ref(FeRefOf),
    Owned(FeOwnedOf),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Callable {
    pub special: Option<SpecialCallable>,
    pub name: Arc<str>,
    pub params: Vec<(Arc<str>, FeType)>,
    pub return_type: Option<Box<FeType>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecialCallable {
    Print,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeStruct {
    pub special: Option<SpecialStruct>,
    pub name: Arc<str>,
    pub fields: Vec<(Arc<str>, FeType)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecialStruct {}

#[derive(Debug, Clone, PartialEq)]
pub enum StringDetails {
    PlainLiteral,
    Format,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeRefOf {
    pub ref_type: FeRefType,
    pub of: Box<FeType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeRefType {
    Const,
    Mut,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeOwnedOf {
    pub owned_mut: FeOwnedMut,
    pub of: Box<FeType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeOwnedMut {
    Const,
    Mut,
}
