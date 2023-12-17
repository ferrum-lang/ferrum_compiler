use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum GoIRStmt {
    Expr(GoIRExprStmt),
    VarDecl(GoIRVarDeclStmt),
    Return(GoIRReturnStmt),
    While(GoIRWhileStmt),
    Break(GoIRBreakStmt),
    If(GoIRIfStmt),
    Assign(GoIRAssignStmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRImplicitReturnStmt {
    pub expr: GoIRExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRExprStmt {
    pub expr: GoIRExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRVarDeclStmt {
    pub name: Arc<str>,
    pub explicit_type: Option<GoIRVarDeclExplicitType>,
    pub value: Option<GoIRVarDeclValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRVarDeclExplicitType {
    // TODO
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRVarDeclValue {
    pub expr: GoIRExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRReturnStmt {
    pub expr: Option<GoIRExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRWhileStmt {
    pub condition: GoIRExpr,
    pub stmts: Vec<GoIRStmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRBreakStmt {
    pub label: Option<Arc<str>>,
    pub expr: Option<GoIRExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRAssignStmt {
    pub lhs: Box<GoIRExpr>,
    pub op: GoIRAssignOp,
    pub rhs: Box<GoIRExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GoIRAssignOp {
    Eq,
    PlusEq,
    MinusEq,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRIfStmt {
    pub condition: Box<GoIRExpr>,
    pub then: Vec<GoIRStmt>,
    pub else_ifs: Vec<GoIRElseIf>,
    pub else_: Option<GoIRElse>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRElseIf {
    pub condition: Box<GoIRExpr>,
    pub then: Vec<GoIRStmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRElse {
    pub then: Vec<GoIRStmt>,
}

// Visitor pattern
pub trait GoIRStmtVisitor<R = ()> {
    fn visit_expr_stmt(&mut self, stmt: &mut GoIRExprStmt) -> R;
    fn visit_var_decl_stmt(&mut self, stmt: &mut GoIRVarDeclStmt) -> R;
    fn visit_return_stmt(&mut self, stmt: &mut GoIRReturnStmt) -> R;
    fn visit_while_stmt(&mut self, stmt: &mut GoIRWhileStmt) -> R;
    fn visit_break_stmt(&mut self, stmt: &mut GoIRBreakStmt) -> R;
    fn visit_assign_stmt(&mut self, expr: &mut GoIRAssignStmt) -> R;
    fn visit_if_smtt(&mut self, expr: &mut GoIRIfStmt) -> R;
}

pub trait GoIRStmtAccept<R, V: GoIRStmtVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: GoIRStmtVisitor<R>> GoIRStmtAccept<R, V> for GoIRStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::Expr(stmt) => stmt.accept(visitor),
            Self::VarDecl(stmt) => stmt.accept(visitor),
            Self::Return(stmt) => stmt.accept(visitor),
            Self::While(stmt) => stmt.accept(visitor),
            Self::Break(stmt) => stmt.accept(visitor),
            Self::Assign(stmt) => stmt.accept(visitor),
            Self::If(stmt) => stmt.accept(visitor),
        };
    }
}

impl<R, V: GoIRStmtVisitor<R>> GoIRStmtAccept<R, V> for GoIRExprStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_expr_stmt(self);
    }
}

impl<R, V: GoIRStmtVisitor<R>> GoIRStmtAccept<R, V> for GoIRVarDeclStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_var_decl_stmt(self);
    }
}

impl<R, V: GoIRStmtVisitor<R>> GoIRStmtAccept<R, V> for GoIRWhileStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_while_stmt(self);
    }
}

impl<R, V: GoIRStmtVisitor<R>> GoIRStmtAccept<R, V> for GoIRReturnStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_return_stmt(self);
    }
}

impl<R, V: GoIRStmtVisitor<R>> GoIRStmtAccept<R, V> for GoIRBreakStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_break_stmt(self);
    }
}

impl<R, V: GoIRStmtVisitor<R>> GoIRStmtAccept<R, V> for GoIRAssignStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_assign_stmt(self);
    }
}

impl<R, V: GoIRStmtVisitor<R>> GoIRStmtAccept<R, V> for GoIRIfStmt {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_if_stmt(self);
    }
}
