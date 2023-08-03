use super::*;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Decl<T: ResolvedType = ()> {
    Fn(FnDecl<T>),
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
pub struct FnDecl<T: ResolvedType = ()> {
    pub id: NodeId<Decl>,
    pub decl_mod: Option<DeclMod>,
    pub fn_mod: Option<FnMod>,
    pub fn_token: Arc<Token>,
    pub name: Arc<Token>,
    pub generics: Option<FnDeclGenerics<T>>,
    pub open_paren_token: Arc<Token>,
    pub params: Vec<FnDeclParam<T>>,
    pub close_paren_token: Arc<Token>,
    pub return_type: Option<FnDeclReturnType<T>>,
    pub body: FnDeclBody<T>,
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
pub struct FnDeclGenerics<T: ResolvedType = ()> {
    pub resolved_type: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDeclParam<T: ResolvedType = ()> {
    pub resolved_type: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDeclReturnType<T: ResolvedType = ()> {
    pub resolved_type: T,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FnDeclBody<T: ResolvedType = ()> {
    Short(FnDeclBodyShort<T>),
    Block(CodeBlock<T>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDeclBodyShort<T: ResolvedType = ()> {
    pub resolved_type: T,
}

#[derive(Debug, Clone)]
pub struct CodeBlock<T: ResolvedType = ()> {
    pub stmts: Vec<Arc<Mutex<Stmt<T>>>>,
    pub end_semicolon_token: Arc<Token>,
}

impl<T: ResolvedType> PartialEq for CodeBlock<T> {
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
pub trait DeclVisitor<T: ResolvedType, R = ()> {
    fn visit_function_decl(&mut self, decl: &mut FnDecl<T>) -> R;
}

pub trait DeclAccept<T: ResolvedType, R, V: DeclVisitor<T, R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<T: ResolvedType, R, V: DeclVisitor<T, R>> DeclAccept<T, R, V> for Decl<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return match self {
            Self::Fn(decl) => decl.accept(visitor),
        };
    }
}

impl<T: ResolvedType, R, V: DeclVisitor<T, R>> DeclAccept<T, R, V> for FnDecl<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_function_decl(self);
    }
}
