use super::*;

use crate::token::Token;
use crate::utils::{fe_from, fe_try_from, from, invert, try_from};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<T: ResolvedType = ()> {
    PlainStringLiteral(PlainStringLiteralExpr<T>),
    Ident(IdentExpr<T>),
    Call(CallExpr<T>),
}

impl<T: ResolvedType> Expr<T> {
    pub fn resolved_type(&self) -> Option<&T> {
        match self {
            Self::PlainStringLiteral(v) => return Some(&v.resolved_type),
            Self::Ident(v) => return Some(&v.resolved_type),
            Self::Call(v) => return v.resolved_type.as_ref(),
        }
    }
}

impl<T: ResolvedType> Node<Expr> for Expr<T> {
    fn node_id(&self) -> &NodeId<Expr> {
        match self {
            Self::Ident(expr) => return expr.node_id(),
            Self::Call(expr) => return expr.node_id(),
            Self::PlainStringLiteral(expr) => return expr.node_id(),
        }
    }
}

impl<T: ResolvedType> From<Expr<()>> for Expr<Option<T>> {
    fn from(value: Expr<()>) -> Self {
        match value {
            Expr::PlainStringLiteral(expr) => return Self::PlainStringLiteral(from(expr)),
            Expr::Ident(expr) => return Self::Ident(from(expr)),
            Expr::Call(expr) => return Self::Call(from(expr)),
        }
    }
}

impl<T: ResolvedType> Resolvable for Expr<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            Expr::PlainStringLiteral(expr) => return expr.is_resolved(),
            Expr::Ident(expr) => return expr.is_resolved(),
            Expr::Call(expr) => return expr.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<Expr<Option<T>>> for Expr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: Expr<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            Expr::PlainStringLiteral(expr) => return Ok(Self::PlainStringLiteral(try_from(expr)?)),
            Expr::Ident(expr) => return Ok(Self::Ident(try_from(expr)?)),
            Expr::Call(expr) => return Ok(Self::Call(try_from(expr)?)),
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

impl<T: ResolvedType> From<NestedExpr<()>> for NestedExpr<Option<T>> {
    fn from(value: NestedExpr<()>) -> Self {
        return Self(fe_from(value.0));
    }
}

impl<T: ResolvedType> Resolvable for NestedExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.0.lock().unwrap().is_resolved();
    }
}

impl<T: ResolvedType> TryFrom<NestedExpr<Option<T>>> for NestedExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: NestedExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self(fe_try_from(value.0)?));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlainStringLiteralExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub literal: Arc<Token>,
    pub resolved_type: T,
}

impl<T: ResolvedType> Node<Expr> for PlainStringLiteralExpr<T> {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}

impl<T: ResolvedType> From<PlainStringLiteralExpr<()>> for PlainStringLiteralExpr<Option<T>> {
    fn from(value: PlainStringLiteralExpr<()>) -> Self {
        return Self {
            id: value.id,
            literal: value.literal,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for PlainStringLiteralExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<PlainStringLiteralExpr<Option<T>>> for PlainStringLiteralExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: PlainStringLiteralExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            literal: value.literal,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentExpr<T: ResolvedType = ()> {
    pub id: NodeId<Expr>,
    pub ident: Arc<Token>,
    pub resolved_type: T,
}

impl<T: ResolvedType> Node<Expr> for IdentExpr<T> {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}

impl<T: ResolvedType> From<IdentExpr<()>> for IdentExpr<Option<T>> {
    fn from(value: IdentExpr<()>) -> Self {
        return Self {
            id: value.id,
            ident: value.ident,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for IdentExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<IdentExpr<Option<T>>> for IdentExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: IdentExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            ident: value.ident,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
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
    pub resolved_type: Option<T>,
}

impl<T: ResolvedType> Node<Expr> for CallExpr<T> {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}

impl<T: ResolvedType> From<CallExpr<()>> for CallExpr<Option<T>> {
    fn from(value: CallExpr<()>) -> Self {
        return Self {
            id: value.id,
            callee: from(value.callee),
            open_paren_token: value.open_paren_token,
            pre_comma_token: value.pre_comma_token,
            args: value.args.into_iter().map(from).collect(),
            close_paren_token: value.close_paren_token,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for CallExpr<Option<T>> {
    fn is_resolved(&self) -> bool {
        if let Some(resolved_type) = &self.resolved_type {
            if resolved_type.is_none() {
                dbg!("false");
                return false;
            }
        }

        if !self.callee.is_resolved() {
            dbg!("false");
            return false;
        }

        for arg in &self.args {
            if !arg.is_resolved() {
                dbg!("false");
                return false;
            }
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<CallExpr<Option<T>>> for CallExpr<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: CallExpr<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            callee: try_from(value.callee)?,
            open_paren_token: value.open_paren_token,
            pre_comma_token: value.pre_comma_token,
            args: value
                .args
                .into_iter()
                .map(try_from)
                .collect::<Result<Vec<CallArg<T>>, Self::Error>>()?,
            close_paren_token: value.close_paren_token,
            resolved_type: if let Some(resolved_type) = value.resolved_type {
                Some(resolved_type.ok_or(FinalizeResolveTypeError {
                    file: file!(),
                    line: line!(),
                })?)
            } else {
                None
            },
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg<T: ResolvedType = ()> {
    pub param_name: Option<CallArgParamName>,
    pub value: NestedExpr<T>,
    pub post_comma_token: Option<Arc<Token>>,
    pub resolved_type: T,
}

impl<T: ResolvedType> From<CallArg<()>> for CallArg<Option<T>> {
    fn from(value: CallArg<()>) -> Self {
        return Self {
            param_name: value.param_name,
            value: from(value.value),
            post_comma_token: value.post_comma_token,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for CallArg<Option<T>> {
    fn is_resolved(&self) -> bool {
        if self.resolved_type.is_none() {
            dbg!("false");
            return false;
        }

        if !self.value.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<CallArg<Option<T>>> for CallArg<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: CallArg<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            param_name: value.param_name,
            value: try_from(value.value)?,
            post_comma_token: value.post_comma_token,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
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
