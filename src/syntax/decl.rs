use super::*;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    Fn(FnDecl),
}

impl Node<Decl> for Decl {
    fn node_id(&self) -> &NodeId<Decl> {
        match self {
            Self::Fn(decl) => return decl.node_id(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeclMod {
    Pub(Token),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub id: NodeId<Decl>,
    pub decl_mod: Option<DeclMod>,
    pub fn_mod: Option<FnMod>,
    pub fn_token: Token,
    pub name: Token,
    pub generics: Option<FnDeclGenerics>,
    pub open_paren_token: Token,
    pub params: Vec<FnDeclParam>,
    pub close_paren_token: Token,
    pub return_type: Option<FnDeclReturnType>,
    pub body: FnDeclBody,
}

impl Node<Decl> for FnDecl {
    fn node_id(&self) -> &NodeId<Decl> {
        return &self.id;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FnMod {
    Pure(Token),
    Safe(Token),
    Norm(Token),
    Risk(Token),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDeclGenerics {}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDeclParam {}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDeclReturnType {}

#[derive(Debug, Clone, PartialEq)]
pub enum FnDeclBody {
    Short(FnDeclBodyShort),
    Block(CodeBlock),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDeclBodyShort {}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock {
    pub stmts: Vec<Stmt>,
    pub end_semicolon_token: Token,
}
