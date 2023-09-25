use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRStmt {
    ImplicitReturn(RustIRImplicitReturnStmt),
    Expr(RustIRExprStmt),
    Let(RustIRLetStmt),
    Return(RustIRReturnStmt),
    While(RustIRWhileStmt),
    Break(RustIRBreakStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRImplicitReturnStmt {
    pub expr: RustIRExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRExprStmt {
    pub expr: RustIRExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRLetStmt {
    pub is_mut: bool,
    pub name: Arc<str>,
    pub explicit_type: Option<RustIRLetExplicitType>,
    pub value: Option<RustIRLetValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRLetExplicitType {
    // TODO
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRLetValue {
    pub expr: RustIRExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRReturnStmt {
    pub expr: Option<RustIRExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRWhileStmt {
    pub condition: RustIRExpr,
    pub stmts: Vec<RustIRStmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRBreakStmt {
    pub label: Option<Arc<str>>,
    pub expr: Option<RustIRExpr>,
}

// Visitor pattern
pub trait RustIRStmtVisitor<R = ()> {
    fn visit_implicit_return_stmt(&mut self, stmt: &mut RustIRImplicitReturnStmt) -> R;
    fn visit_expr_stmt(&mut self, stmt: &mut RustIRExprStmt) -> R;
    fn visit_let_stmt(&mut self, stmt: &mut RustIRLetStmt) -> R;
    fn visit_return_stmt(&mut self, stmt: &mut RustIRReturnStmt) -> R;
    fn visit_while_stmt(&mut self, stmt: &mut RustIRWhileStmt) -> R;
    fn visit_break_stmt(&mut self, stmt: &mut RustIRBreakStmt) -> R;
}

pub trait RustIRStmtAccept<R, V: RustIRStmtVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: RustIRStmtVisitor<R>> RustIRStmtAccept<R, V> for RustIRStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::ImplicitReturn(stmt) => stmt.accept(visitor),
            Self::Expr(stmt) => stmt.accept(visitor),
            Self::Let(stmt) => stmt.accept(visitor),
            Self::Return(stmt) => stmt.accept(visitor),
            Self::While(stmt) => stmt.accept(visitor),
            Self::Break(stmt) => stmt.accept(visitor),
        };
    }
}

impl<R, V: RustIRStmtVisitor<R>> RustIRStmtAccept<R, V> for RustIRImplicitReturnStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_implicit_return_stmt(self);
    }
}

impl<R, V: RustIRStmtVisitor<R>> RustIRStmtAccept<R, V> for RustIRExprStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_expr_stmt(self);
    }
}

impl<R, V: RustIRStmtVisitor<R>> RustIRStmtAccept<R, V> for RustIRLetStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_let_stmt(self);
    }
}

impl<R, V: RustIRStmtVisitor<R>> RustIRStmtAccept<R, V> for RustIRWhileStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_while_stmt(self);
    }
}

impl<R, V: RustIRStmtVisitor<R>> RustIRStmtAccept<R, V> for RustIRReturnStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_return_stmt(self);
    }
}

impl<R, V: RustIRStmtVisitor<R>> RustIRStmtAccept<R, V> for RustIRBreakStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_break_stmt(self);
    }
}
