use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRStmt {
    Expr(RustIRExprStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRExprStmt {
    pub expr: RustIRExpr,
}
