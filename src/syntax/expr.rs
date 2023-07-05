use super::*;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(IdentExpr),
    Call(CallExpr),
    StringLiteral(StringLiteralExpr),
}

impl Node<Expr> for Expr {
    fn node_id(&self) -> &NodeId<Expr> {
        match self {
            Self::Ident(expr) => return expr.node_id(),
            Self::Call(expr) => return expr.node_id(),
            Self::StringLiteral(expr) => return expr.node_id(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentExpr {
    pub id: NodeId<Expr>,
    pub ident: Token,
}

impl Node<Expr> for IdentExpr {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub id: NodeId<Expr>,
    pub callee: Box<Expr>,
    pub open_paren_token: Token,
    pub args: Vec<CallArg>,
    pub close_paren_token: Token,
}

impl Node<Expr> for CallExpr {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArg {
    pub param_name: Option<CallArgParamName>,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallArgParamName {
    pub name: Token,
    pub eq_token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteralExpr {
    pub id: NodeId<Expr>,
    pub literal: Token,
}

impl Node<Expr> for StringLiteralExpr {
    fn node_id(&self) -> &NodeId<Expr> {
        return &self.id;
    }
}
