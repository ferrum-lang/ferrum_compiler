use super::*;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<T: ResolvedType = ()> {
    PlainStringLiteral(PlainStringLiteralExpr<T>),
    Ident(IdentExpr<T>),
    Call(CallExpr<T>),
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
pub struct NestedExpr<T: ResolvedType = ()>(pub Arc<Mutex<Expr<T>>>);
impl<T: ResolvedType> PartialEq for NestedExpr<T> {
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
pub struct PlainStringLiteralExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub literal: Arc<Token>,
    pub resolved_type: T,
}

impl Node<Expr> for PlainStringLiteralExpr {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub ident: Arc<Token>,
    pub resolved_type: T,
}

impl Node<Expr> for IdentExpr {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub callee: NestedExpr<T>,
    pub open_paren_token: Arc<Token>,
    pub pre_comma_token: Option<Arc<Token>>,
    pub args: Vec<CallArg<T>>,
    pub close_paren_token: Arc<Token>,
    pub resolved_type: T,
}

impl Node<Expr> for CallExpr {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg<T: ResolvedType = ()> {
    pub param_name: Option<CallArgParamName>,
    pub value: NestedExpr<T>,
    pub post_comma_token: Option<Arc<Token>>,
    pub resolved_type: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArgParamName {
    pub name: Token,
    pub eq_token: Token,
}

// Visitor pattern
pub trait ExprVisitor<T: ResolvedType, R = ()> {
    fn visit_ident_expr(&mut self, expr: &mut IdentExpr<T>) -> R;
    fn visit_call_expr(&mut self, expr: &mut CallExpr<T>) -> R;
    fn visit_plain_string_literal_expr(&mut self, expr: &mut PlainStringLiteralExpr<T>) -> R;
}

pub trait ExprAccept<T: ResolvedType, R, V: ExprVisitor<T, R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for Expr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::Ident(expr) => expr.accept(visitor),
            Self::Call(expr) => expr.accept(visitor),
            Self::PlainStringLiteral(expr) => expr.accept(visitor),
        };
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for IdentExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_ident_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for CallExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_call_expr(self);
    }
}

impl<T: ResolvedType, R, V: ExprVisitor<T, R>> ExprAccept<T, R, V> for PlainStringLiteralExpr<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_plain_string_literal_expr(self);
    }
}
