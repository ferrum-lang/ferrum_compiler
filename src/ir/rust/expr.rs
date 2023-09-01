use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRExpr {
    BoolLiteral(RustIRBoolLiteralExpr),
    StringLiteral(RustIRStringLiteralExpr),
    Ident(RustIRIdentExpr),
    Call(RustIRCallExpr),
    Block(RustIRBlockExpr),
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
pub struct RustIRBlockExpr {
    pub stmts: Vec<RustIRStmt>,
}

// Visitor pattern
pub trait RustIRExprVisitor<R = ()> {
    fn visit_bool_literal_expr(&mut self, expr: &mut RustIRBoolLiteralExpr) -> R;
    fn visit_string_literal_expr(&mut self, expr: &mut RustIRStringLiteralExpr) -> R;
    fn visit_ident_expr(&mut self, expr: &mut RustIRIdentExpr) -> R;
    fn visit_call_expr(&mut self, expr: &mut RustIRCallExpr) -> R;
    fn visit_block_expr(&mut self, expr: &mut RustIRBlockExpr) -> R;
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
            Self::Block(expr) => expr.accept(visitor),
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

impl<R, V: RustIRExprVisitor<R>> RustIRExprAccept<R, V> for RustIRBlockExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_block_expr(self);
    }
}
