use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRExpr {
    BoolLiteral(RustIRBoolLiteralExpr),
    StringLiteral(RustIRStringLiteralExpr),
    Ident(RustIRIdentExpr),
    Call(RustIRCallExpr),
    MacroFnCall(RustIRMacroFnCallExpr),
    Block(RustIRBlockExpr),
    StaticRef(RustIRStaticRefExpr),
    Unary(RustIRUnaryExpr),
    Assign(RustIRAssignExpr),
    If(RustIRIfExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRBoolLiteralExpr {
    pub literal: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRStringLiteralExpr {
    pub literal: Arc<str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRIdentExpr {
    pub ident: Arc<str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRCallExpr {
    pub callee: Box<RustIRExpr>,
    pub args: Vec<RustIRExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRMacroFnCallExpr {
    pub callee: Arc<str>,
    pub args: Vec<RustIRExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRBlockExpr {
    pub stmts: Vec<RustIRStmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRStaticRefExpr {
    pub static_ref: RustIRStaticPath,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRUnaryExpr {
    pub op: RustIRUnaryOp,
    pub value: Box<RustIRExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRUnaryOp {
    Ref(RustIRRefType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRAssignExpr {
    pub lhs: Box<RustIRExpr>,
    pub rhs: Box<RustIRExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRIfExpr {
    pub condition: Box<RustIRExpr>,
    pub then: Vec<RustIRStmt>,
    pub else_ifs: Vec<RustIRElseIf>,
    pub else_: Option<RustIRElse>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRElseIf {
    pub condition: Box<RustIRExpr>,
    pub then: Vec<RustIRStmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRElse {
    pub then: Vec<RustIRStmt>,
}

// Visitor pattern
pub trait RustIRExprVisitor<R = ()> {
    fn visit_bool_literal_expr(&mut self, expr: &mut RustIRBoolLiteralExpr) -> R;
    fn visit_string_literal_expr(&mut self, expr: &mut RustIRStringLiteralExpr) -> R;
    fn visit_ident_expr(&mut self, expr: &mut RustIRIdentExpr) -> R;
    fn visit_call_expr(&mut self, expr: &mut RustIRCallExpr) -> R;
    fn visit_macro_fn_call_expr(&mut self, expr: &mut RustIRMacroFnCallExpr) -> R;
    fn visit_block_expr(&mut self, expr: &mut RustIRBlockExpr) -> R;
    fn visit_static_ref_expr(&mut self, expr: &mut RustIRStaticRefExpr) -> R;
    fn visit_unary_expr(&mut self, expr: &mut RustIRUnaryExpr) -> R;
    fn visit_assign_expr(&mut self, expr: &mut RustIRAssignExpr) -> R;
    fn visit_if_expr(&mut self, expr: &mut RustIRIfExpr) -> R;
}

pub trait RustIRExprAccept<R, V: RustIRExprVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: RustIRExprVisitor<R>> RustIRExprAccept<R, V> for RustIRExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::BoolLiteral(expr) => expr.accept(visitor),
            Self::StringLiteral(expr) => expr.accept(visitor),
            Self::Ident(expr) => expr.accept(visitor),
            Self::Call(expr) => expr.accept(visitor),
            Self::MacroFnCall(expr) => expr.accept(visitor),
            Self::Block(expr) => expr.accept(visitor),
            Self::StaticRef(expr) => expr.accept(visitor),
            Self::Unary(expr) => expr.accept(visitor),
            Self::Assign(expr) => expr.accept(visitor),
            Self::If(expr) => expr.accept(visitor),
        };
    }
}

impl<R, V: RustIRExprVisitor<R>> RustIRExprAccept<R, V> for RustIRBoolLiteralExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_bool_literal_expr(self);
    }
}

impl<R, V: RustIRExprVisitor<R>> RustIRExprAccept<R, V> for RustIRStringLiteralExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_string_literal_expr(self);
    }
}

impl<R, V: RustIRExprVisitor<R>> RustIRExprAccept<R, V> for RustIRIdentExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_ident_expr(self);
    }
}

impl<R, V: RustIRExprVisitor<R>> RustIRExprAccept<R, V> for RustIRCallExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_call_expr(self);
    }
}

impl<R, V: RustIRExprVisitor<R>> RustIRExprAccept<R, V> for RustIRMacroFnCallExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_macro_fn_call_expr(self);
    }
}

impl<R, V: RustIRExprVisitor<R>> RustIRExprAccept<R, V> for RustIRBlockExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_block_expr(self);
    }
}

impl<R, V: RustIRExprVisitor<R>> RustIRExprAccept<R, V> for RustIRStaticRefExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_static_ref_expr(self);
    }
}

impl<R, V: RustIRExprVisitor<R>> RustIRExprAccept<R, V> for RustIRUnaryExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_unary_expr(self);
    }
}

impl<R, V: RustIRExprVisitor<R>> RustIRExprAccept<R, V> for RustIRAssignExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_assign_expr(self);
    }
}

impl<R, V: RustIRExprVisitor<R>> RustIRExprAccept<R, V> for RustIRIfExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_if_expr(self);
    }
}
