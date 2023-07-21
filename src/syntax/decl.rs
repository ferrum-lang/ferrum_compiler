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
    Pub(Arc<Token>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub id: NodeId<Decl>,
    pub decl_mod: Option<DeclMod>,
    pub fn_mod: Option<FnMod>,
    pub fn_token: Arc<Token>,
    pub name: Arc<Token>,
    pub generics: Option<FnDeclGenerics>,
    pub open_paren_token: Arc<Token>,
    pub params: Vec<FnDeclParam>,
    pub close_paren_token: Arc<Token>,
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
    Pure(Arc<Token>),
    Safe(Arc<Token>),
    Norm(Arc<Token>),
    Risk(Arc<Token>),
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

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub stmts: Vec<Arc<Mutex<Stmt>>>,
    pub end_semicolon_token: Arc<Token>,
}

impl PartialEq for CodeBlock {
    fn eq(&self, other: &Self) -> bool {
        if self.end_semicolon_token != other.end_semicolon_token {
            return false;
        }

        if self.stmts.len() != other.stmts.len() {
            return false;
        }

        for i in 0..self.stmts.len() {
            let stmt = {
                let locked = self.stmts[i].lock().unwrap();
                locked.clone()
            };

            let other = other.stmts[i].lock().unwrap();

            if stmt != *other {
                return false;
            }
        }

        return true;
    }
}

// Visitor pattern
pub trait DeclVisitor<R = ()> {
    fn visit_function_decl(&mut self, decl: &mut FnDecl) -> R;
}

pub trait DeclAccept<R, V: DeclVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: DeclVisitor<R>> DeclAccept<R, V> for Decl {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::Fn(decl) => decl.accept(visitor),
        };
    }
}

impl<R, V: DeclVisitor<R>> DeclAccept<R, V> for FnDecl {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_function_decl(self);
    }
}
