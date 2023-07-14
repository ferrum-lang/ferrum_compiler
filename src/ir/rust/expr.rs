use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRExpr {
    Ident(RustIRIdent),
    Call(RustIRCall),
    StringLiteral(RustIRStringLiteral),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRIdent {}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRCall {}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRStringLiteral {}
