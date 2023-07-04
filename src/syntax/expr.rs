use super::*;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(IdentExpr),
    Call(CallExpr),
    StringLiteral(StringLiteralExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentExpr {
    pub id: NodeId<Expr>,
    pub ident: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub id: NodeId<Expr>,
    pub callee: Box<Expr>,
    pub open_paren_token: Token,
    pub args: Vec<CallArg>,
    pub close_paren_token: Token,
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
