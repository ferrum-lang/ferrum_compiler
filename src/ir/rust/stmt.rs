use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRStmt {
    ImplicitReturn(Box<RustIRStmt>),

    Expr(RustIRExprStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRExprStmt {
    pub expr: RustIRExpr,
}

// Visitor pattern
pub trait RustIRStmtVisitor<R = ()> {
    fn visit_expr_stmt(&mut self, stmt: &mut RustIRExprStmt) -> R;
}

pub trait RustIRStmtAccept<R, V: RustIRStmtVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: RustIRStmtVisitor<R>> RustIRStmtAccept<R, V> for RustIRStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::ImplicitReturn(stmt) => stmt.accept(visitor),
            Self::Expr(stmt) => stmt.accept(visitor),
        };
    }
}

impl<R, V: RustIRStmtVisitor<R>> RustIRStmtAccept<R, V> for RustIRExprStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_expr_stmt(self);
    }
}
