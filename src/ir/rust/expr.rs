use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRExpr {
    Ident(RustIRIdent),
    Call(RustIRCall),
    StringLiteral(RustIRStringLiteral),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRIdent {
    pub ident: Arc<str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRCall {
    pub callee: Box<RustIRExpr>,
    pub args: Vec<RustIRExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRStringLiteral {
    pub literal: Arc<str>,
}
