use super::*;

use crate::result::Result;
use crate::token::Token;
use crate::utils::{from, invert, try_from};

use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct StaticType<T: ResolvedType = ()> {
    pub ref_type: Option<RefType>,
    pub static_path: StaticPath<T>,
    pub resolved_type: T,
}

impl<T: ResolvedType> From<StaticType<()>> for StaticType<Option<T>> {
    fn from(value: StaticType<()>) -> Self {
        return Self {
            ref_type: value.ref_type,
            static_path: from(value.static_path),
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for StaticType<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<StaticType<Option<T>>> for StaticType<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: StaticType<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            ref_type: value.ref_type,
            static_path: try_from(value.static_path)?,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RefType {
    Shared {
        ref_token: Arc<Token>,
        const_token: Option<Arc<Token>>,
    },
    Mut {
        ref_token: Arc<Token>,
        mut_token: Arc<Token>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct StaticPath<T: ResolvedType = ()> {
    pub double_colon_token: Option<Arc<Token>>,
    pub root: Option<Box<Self>>,
    pub name: Arc<Token>,
    pub resolved_type: T,
}

impl<T: ResolvedType> From<StaticPath<()>> for StaticPath<Option<T>> {
    fn from(value: StaticPath<()>) -> Self {
        return Self {
            double_colon_token: value.double_colon_token,
            root: value.root.map(|v| Box::new(from(*v))),
            name: value.name,
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for StaticPath<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.resolved_type.is_some();
    }
}

impl<T: ResolvedType> TryFrom<StaticPath<Option<T>>> for StaticPath<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: StaticPath<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            double_colon_token: value.double_colon_token,
            root: invert(value.root.map(|v| Ok(Box::new(try_from(*v)?))))?,
            name: value.name,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError {
                file: file!(),
                line: line!(),
            })?,
        });
    }
}

// Visitor pattern
pub trait StaticVisitor<T: ResolvedType, R = ()> {
    fn visit_static_type(&mut self, static_type: &mut StaticType<T>) -> R;
    fn visit_static_path(&mut self, static_path: &mut StaticPath<T>) -> R;
}

pub trait StaticTypeAccept<T: ResolvedType, R, V: StaticVisitor<T, R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<T: ResolvedType, R, V: StaticVisitor<T, R>> StaticTypeAccept<T, R, V> for StaticType<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_static_type(self);
    }
}

impl<T: ResolvedType, R, V: StaticVisitor<T, R>> StaticTypeAccept<T, R, V> for StaticPath<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_static_path(self);
    }
}
