use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum GoIRExpr {
    BoolLiteral(GoIRBoolLiteralExpr),
    NumberLiteral(GoIRNumberLiteralExpr),
    StringLiteral(GoIRStringLiteralExpr),
    Ident(GoIRIdentExpr),
    Call(GoIRCallExpr),
    StaticRef(GoIRStaticRefExpr),
    Unary(GoIRUnaryExpr),
    Binary(GoIRBinaryExpr),
    Construct(GoIRConstructExpr),
    Get(GoIRGetExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRBoolLiteralExpr {
    pub literal: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRNumberLiteralExpr {
    pub literal: Arc<str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRStringLiteralExpr {
    pub literal: Arc<str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRIdentExpr {
    pub ident: Arc<str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRCallExpr {
    pub callee: Box<GoIRExpr>,
    pub args: Vec<GoIRExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRMacroFnCallExpr {
    pub callee: Arc<str>,
    pub args: Vec<GoIRExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRBlockExpr {
    pub stmts: Vec<GoIRStmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRStaticRefExpr {
    pub static_ref: GoIRStaticPath,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRUnaryExpr {
    pub op: GoIRUnaryOp,
    pub value: Box<GoIRExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GoIRUnaryOp {
    Ptr,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRBinaryExpr {
    pub lhs: Box<GoIRExpr>,
    pub op: GoIRBinaryOp,
    pub rhs: Box<GoIRExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GoIRBinaryOp {
    Add,
    Subtract,
    Less,
    LessEq,
    Greater,
    GreaterEq,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRConstructExpr {
    pub target: GoIRConstructTarget,
    pub args: Vec<GoIRConstructArg>,
    pub spread: Option<Box<GoIRExpr>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GoIRConstructTarget {
    Ident(Arc<str>),
    StaticPath(GoIRStaticPath),
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRConstructArg {
    pub name: Arc<str>,
    pub value: GoIRExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRGetExpr {
    pub target: Box<GoIRExpr>,
    pub name: Arc<str>,
}

// Visitor pattern
pub trait GoIRExprVisitor<R = ()> {
    fn visit_bool_literal_expr(&mut self, expr: &mut GoIRBoolLiteralExpr) -> R;
    fn visit_number_literal_expr(&mut self, expr: &mut GoIRNumberLiteralExpr) -> R;
    fn visit_string_literal_expr(&mut self, expr: &mut GoIRStringLiteralExpr) -> R;
    fn visit_ident_expr(&mut self, expr: &mut GoIRIdentExpr) -> R;
    fn visit_call_expr(&mut self, expr: &mut GoIRCallExpr) -> R;
    fn visit_static_ref_expr(&mut self, expr: &mut GoIRStaticRefExpr) -> R;
    fn visit_unary_expr(&mut self, expr: &mut GoIRUnaryExpr) -> R;
    fn visit_binary_expr(&mut self, expr: &mut GoIRBinaryExpr) -> R;
    fn visit_construct_expr(&mut self, expr: &mut GoIRConstructExpr) -> R;
    fn visit_get_expr(&mut self, expr: &mut GoIRGetExpr) -> R;
}

pub trait GoIRExprAccept<R, V: GoIRExprVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: GoIRExprVisitor<R>> GoIRExprAccept<R, V> for GoIRExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::BoolLiteral(expr) => expr.accept(visitor),
            Self::NumberLiteral(expr) => expr.accept(visitor),
            Self::StringLiteral(expr) => expr.accept(visitor),
            Self::Ident(expr) => expr.accept(visitor),
            Self::Call(expr) => expr.accept(visitor),
            Self::StaticRef(expr) => expr.accept(visitor),
            Self::Unary(expr) => expr.accept(visitor),
            Self::Binary(expr) => expr.accept(visitor),
            Self::Construct(expr) => expr.accept(visitor),
            Self::Get(expr) => expr.accept(visitor),
        };
    }
}

impl<R, V: GoIRExprVisitor<R>> GoIRExprAccept<R, V> for GoIRBoolLiteralExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_bool_literal_expr(self);
    }
}

impl<R, V: GoIRExprVisitor<R>> GoIRExprAccept<R, V> for GoIRNumberLiteralExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_number_literal_expr(self);
    }
}

impl<R, V: GoIRExprVisitor<R>> GoIRExprAccept<R, V> for GoIRStringLiteralExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_string_literal_expr(self);
    }
}

impl<R, V: GoIRExprVisitor<R>> GoIRExprAccept<R, V> for GoIRIdentExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_ident_expr(self);
    }
}

impl<R, V: GoIRExprVisitor<R>> GoIRExprAccept<R, V> for GoIRCallExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_call_expr(self);
    }
}

impl<R, V: GoIRExprVisitor<R>> GoIRExprAccept<R, V> for GoIRStaticRefExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_static_ref_expr(self);
    }
}

impl<R, V: GoIRExprVisitor<R>> GoIRExprAccept<R, V> for GoIRUnaryExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_unary_expr(self);
    }
}

impl<R, V: GoIRExprVisitor<R>> GoIRExprAccept<R, V> for GoIRBinaryExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_binary_expr(self);
    }
}

impl<R, V: GoIRExprVisitor<R>> GoIRExprAccept<R, V> for GoIRConstructExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_construct_expr(self);
    }
}

impl<R, V: GoIRExprVisitor<R>> GoIRExprAccept<R, V> for GoIRGetExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_get_expr(self);
    }
}
