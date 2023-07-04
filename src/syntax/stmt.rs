use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(ExprStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprStmt {
    pub id: NodeId<Stmt>,
    pub expr: Expr,
}
