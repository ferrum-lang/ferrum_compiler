use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRExpr {
    Ident(RustIRIdentExpr),
    Call(RustIRCallExpr),
    StringLiteral(RustIRStringLiteralExpr),
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
pub struct RustIRStringLiteralExpr {
    pub literal: Arc<str>,
}

// Visitor pattern
pub trait RustIRExprVisitor<R = ()> {
    fn visit_ident_expr(&mut self, expr: &mut RustIRIdentExpr) -> R;
    fn visit_call_expr(&mut self, expr: &mut RustIRCallExpr) -> R;
    fn visit_string_literal_expr(&mut self, expr: &mut RustIRStringLiteralExpr) -> R;
}

pub trait RustIRExprAccept<R, V: RustIRExprVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: RustIRExprVisitor<R>> RustIRExprAccept<R, V> for RustIRExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::Ident(stmt) => stmt.accept(visitor),
            Self::Call(stmt) => stmt.accept(visitor),
            Self::StringLiteral(stmt) => stmt.accept(visitor),
        };
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

impl<R, V: RustIRExprVisitor<R>> RustIRExprAccept<R, V> for RustIRStringLiteralExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_string_literal_expr(self);
    }
}
