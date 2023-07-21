use super::*;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    PlainStringLiteral(PlainStringLiteralExpr),
    Ident(IdentExpr),
    Call(CallExpr),
}

impl Node<Expr> for Expr {
    fn node_id(&self) -> &NodeId<Expr> {
        match self {
            Self::Ident(expr) => return expr.node_id(),
            Self::Call(expr) => return expr.node_id(),
            Self::PlainStringLiteral(expr) => return expr.node_id(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NestedExpr(pub Arc<Mutex<Expr>>);
impl PartialEq for NestedExpr {
    fn eq(&self, other: &Self) -> bool {
        let cloned = {
            let locked = self.0.lock().unwrap();
            locked.clone()
        };

        let other = other.0.lock().unwrap();

        return cloned == *other;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlainStringLiteralExpr {
    pub id: NodeId<Expr>,
    pub literal: Arc<Token>,
}

impl Node<Expr> for PlainStringLiteralExpr {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentExpr {
    pub id: NodeId<Expr>,
    pub ident: Arc<Token>,
}

impl Node<Expr> for IdentExpr {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub id: NodeId<Expr>,
    pub callee: NestedExpr,
    pub open_paren_token: Arc<Token>,
    pub pre_comma_token: Option<Arc<Token>>,
    pub args: Vec<CallArg>,
    pub close_paren_token: Arc<Token>,
}

impl Node<Expr> for CallExpr {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg {
    pub param_name: Option<CallArgParamName>,
    pub value: NestedExpr,
    pub post_comma_token: Option<Arc<Token>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArgParamName {
    pub name: Token,
    pub eq_token: Token,
}

// Visitor pattern
pub trait ExprVisitor<R = ()> {
    fn visit_ident_expr(&mut self, expr: &mut IdentExpr) -> R;
    fn visit_call_expr(&mut self, expr: &mut CallExpr) -> R;
    fn visit_plain_string_literal_expr(&mut self, expr: &mut PlainStringLiteralExpr) -> R;
}

pub trait ExprAccept<R, V: ExprVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: ExprVisitor<R>> ExprAccept<R, V> for Expr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::Ident(expr) => expr.accept(visitor),
            Self::Call(expr) => expr.accept(visitor),
            Self::PlainStringLiteral(expr) => expr.accept(visitor),
        };
    }
}

impl<R, V: ExprVisitor<R>> ExprAccept<R, V> for IdentExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_ident_expr(self);
    }
}

impl<R, V: ExprVisitor<R>> ExprAccept<R, V> for CallExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_call_expr(self);
    }
}

impl<R, V: ExprVisitor<R>> ExprAccept<R, V> for PlainStringLiteralExpr {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_plain_string_literal_expr(self);
    }
}
