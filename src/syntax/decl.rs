use super::*;

use crate::token::Token;
use crate::utils::{fe_from, fe_try_from, from, invert, try_from};

#[derive(Debug, Clone, PartialEq)]
pub enum Decl<T: ResolvedType = ()> {
    Fn(FnDecl<T>),
}

impl<T: ResolvedType> Node<Decl> for Decl<T> {
    fn node_id(&self) -> &NodeId<Decl> {
        match self {
            Self::Fn(decl) => return decl.node_id(),
        }
    }
}

impl<T: ResolvedType> From<Decl<()>> for Decl<Option<T>> {
    fn from(value: Decl<()>) -> Self {
        match value {
            Decl::Fn(decl) => return Self::Fn(from(decl)),
        }
    }
}

impl<T: ResolvedType> Resolvable for Decl<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            Self::Fn(decl) => return decl.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<Decl<Option<T>>> for Decl<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: Decl<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            Decl::Fn(decl) => return Ok(Self::Fn(try_from(decl)?)),
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
    pub pre_comma_token: Option<Arc<Token>>,
    pub params: Vec<FnDeclParam<T>>,
    pub close_paren_token: Arc<Token>,
    pub return_type: Option<FnDeclReturnType<T>>,
    pub body: FnDeclBody<T>,
}

impl<T: ResolvedType> Node<Decl> for FnDecl<T> {
    fn node_id(&self) -> &NodeId<Decl> {
        return &self.id;
    }
}

impl<T: ResolvedType> From<FnDecl<()>> for FnDecl<Option<T>> {
    fn from(value: FnDecl<()>) -> Self {
        return Self {
            id: value.id,
            decl_mod: value.decl_mod,
            fn_mod: value.fn_mod,
            fn_token: value.fn_token,
            name: value.name,
            generics: value.generics.map(from),
            open_paren_token: value.open_paren_token,
            pre_comma_token: value.pre_comma_token,
            params: value.params.into_iter().map(from).collect(),
            close_paren_token: value.close_paren_token,
            return_type: value.return_type.map(from),
            body: from(value.body),
        };
    }
}

impl<T: ResolvedType> Resolvable for FnDecl<Option<T>> {
    fn is_signature_resolved(&self) -> bool {
        if let Some(generics) = &self.generics {
            if !generics.is_resolved() {
                dbg!("false");
                return false;
            }
        }

        for param in &self.params {
            if !param.is_resolved() {
                dbg!("false");
                return false;
            }
        }

        if let Some(return_type) = &self.return_type {
            if !return_type.is_resolved() {
                dbg!("false");
                return false;
            }
        }

        return true;
    }

    fn is_resolved(&self) -> bool {
        if !self.is_signature_resolved() {
            dbg!("false");
            return false;
        }

        if !self.body.is_resolved() {
            dbg!("false");
            return false;
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<FnDecl<Option<T>>> for FnDecl<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: FnDecl<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            decl_mod: value.decl_mod,
            fn_mod: value.fn_mod,
            fn_token: value.fn_token,
            name: value.name,
            generics: invert(value.generics.map(try_from))?,
            open_paren_token: value.open_paren_token,
            pre_comma_token: value.pre_comma_token,
            params: value
                .params
                .into_iter()
                .map(try_from)
                .collect::<Result<Vec<FnDeclParam<T>>, Self::Error>>()?,
            close_paren_token: value.close_paren_token,
            return_type: invert(value.return_type.map(try_from))?,
            body: try_from(value.body)?,
        });
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

impl<T: ResolvedType> From<FnDeclGenerics<()>> for FnDeclGenerics<Option<T>> {
    fn from(_: FnDeclGenerics<()>) -> Self {
        return Self {
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for FnDeclGenerics<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<FnDeclGenerics<Option<T>>> for FnDeclGenerics<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: FnDeclGenerics<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDeclParam<T: ResolvedType = ()> {
    pub name: Arc<Token>,
    pub colon_token: Arc<Token>,
    pub static_type_ref: StaticType<T>,
    pub comma_token: Option<Arc<Token>>,
    pub resolved_type: T,
}

impl<T: ResolvedType> From<FnDeclParam<()>> for FnDeclParam<Option<T>> {
    fn from(value: FnDeclParam<()>) -> Self {
        return Self {
            name: value.name,
            colon_token: value.colon_token,
            static_type_ref: from(value.static_type_ref),
            comma_token: value.comma_token,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for FnDeclParam<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<FnDeclParam<Option<T>>> for FnDeclParam<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: FnDeclParam<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            name: value.name,
            colon_token: value.colon_token,
            static_type_ref: try_from(value.static_type_ref)?,
            comma_token: value.comma_token,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDeclReturnType<T: ResolvedType = ()> {
    pub colon_token: Arc<Token>,
    pub static_type: StaticType<T>,
    pub resolved_type: T,
}

impl<T: ResolvedType> From<FnDeclReturnType<()>> for FnDeclReturnType<Option<T>> {
    fn from(value: FnDeclReturnType<()>) -> Self {
        return Self {
            colon_token: value.colon_token,
            static_type: from(value.static_type),
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for FnDeclReturnType<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<FnDeclReturnType<Option<T>>> for FnDeclReturnType<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: FnDeclReturnType<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            colon_token: value.colon_token,
            static_type: try_from(value.static_type)?,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FnDeclBody<T: ResolvedType = ()> {
    Short(FnDeclBodyShort<T>),
    Block(CodeBlock<T>),
}

impl<T: ResolvedType> From<FnDeclBody<()>> for FnDeclBody<Option<T>> {
    fn from(value: FnDeclBody<()>) -> Self {
        match value {
            FnDeclBody::Short(body) => return Self::Short(from(body)),
            FnDeclBody::Block(body) => return Self::Block(from(body)),
        }
    }
}

impl<T: ResolvedType> Resolvable for FnDeclBody<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            Self::Short(body) => return body.is_resolved(),
            Self::Block(body) => return body.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<FnDeclBody<Option<T>>> for FnDeclBody<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: FnDeclBody<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            FnDeclBody::Short(body) => return Ok(Self::Short(try_from(body)?)),
            FnDeclBody::Block(body) => return Ok(Self::Block(try_from(body)?)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDeclBodyShort<T: ResolvedType = ()> {
    pub resolved_type: T,
}

impl<T: ResolvedType> From<FnDeclBodyShort<()>> for FnDeclBodyShort<Option<T>> {
    fn from(_: FnDeclBodyShort<()>) -> Self {
        return Self {
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for FnDeclBodyShort<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<FnDeclBodyShort<Option<T>>> for FnDeclBodyShort<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: FnDeclBodyShort<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
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

impl<T: ResolvedType> From<CodeBlock<()>> for CodeBlock<Option<T>> {
    fn from(value: CodeBlock<()>) -> Self {
        return Self {
            stmts: value.stmts.into_iter().map(fe_from).collect(),
            end_semicolon_token: value.end_semicolon_token,
        };
    }
}

impl<T: ResolvedType> Resolvable for CodeBlock<Option<T>> {
    fn is_resolved(&self) -> bool {
        for stmt in &self.stmts {
            if !stmt.lock().unwrap().is_resolved() {
                dbg!("false");
                return false;
            }
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<CodeBlock<Option<T>>> for CodeBlock<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: CodeBlock<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            stmts: value
                .stmts
                .into_iter()
                .map(fe_try_from)
                .collect::<Result<Vec<Arc<Mutex<Stmt<T>>>>, Self::Error>>()?,
            end_semicolon_token: value.end_semicolon_token,
        });
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
