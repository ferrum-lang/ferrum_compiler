use super::*;

use crate::token::Token;
use crate::utils::{from, invert, try_from};

#[derive(Debug, Clone, PartialEq)]
pub struct Use<ResolvedType = ()> {
    pub id: NodeId<Use>,
    pub use_token: Arc<Token>,
    pub pre_double_colon_token: Option<Arc<Token>>,
    pub use_mod: Option<UseMod>,
    pub path: UseStaticPath<ResolvedType>,
}

impl<T: ResolvedType> Node<Use> for Use<T> {
    fn node_id(&self) -> &NodeId<Use> {
        return &self.id;
    }
}

impl<T: ResolvedType> From<Use<()>> for Use<Option<T>> {
    fn from(value: Use<()>) -> Self {
        return Self {
            id: value.id,
            use_token: value.use_token,
            pre_double_colon_token: value.pre_double_colon_token,
            use_mod: value.use_mod,
            path: from(value.path),
        };
    }
}

impl<T: ResolvedType> Resolvable for Use<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.path.is_resolved();
    }
}

impl<T: ResolvedType> TryFrom<Use<Option<T>>> for Use<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: Use<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: value.id,
            use_token: value.use_token,
            pre_double_colon_token: value.pre_double_colon_token,
            use_mod: value.use_mod,
            path: try_from(value.path)?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPath<ResolvedType = ()> {
    pub name: Arc<Token>,
    pub details: Either<UseStaticPathNext<ResolvedType>, ResolvedType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Either<A, B> {
    A(A),
    B(B),
}

impl<A, B> Either<A, B> {
    pub fn is_a(&self) -> bool {
        return matches!(self, Either::A(_));
    }

    pub fn is_b(&self) -> bool {
        return matches!(self, Either::B(_));
    }

    pub fn map_a<U, F>(self, f: F) -> Either<U, B>
    where
        F: FnOnce(A) -> U,
    {
        match self {
            Either::A(a) => return Either::A(f(a)),
            Either::B(b) => return Either::B(b),
        }
    }

    pub fn map_b<U, F>(self, f: F) -> Either<A, U>
    where
        F: FnOnce(B) -> U,
    {
        match self {
            Either::A(a) => return Either::A(a),
            Either::B(b) => return Either::B(f(b)),
        }
    }

    pub fn try_map_a<U, E, F>(self, f: F) -> Result<Either<U, B>, E>
    where
        F: FnOnce(A) -> Result<U, E>,
    {
        match self {
            Either::A(a) => return Ok(Either::A(f(a)?)),
            Either::B(b) => return Ok(Either::B(b)),
        }
    }

    pub fn try_map_b<U, E, F>(self, f: F) -> Result<Either<A, U>, E>
    where
        F: FnOnce(B) -> Result<U, E>,
    {
        match self {
            Either::A(a) => return Ok(Either::A(a)),
            Either::B(b) => return Ok(Either::B(f(b)?)),
        }
    }

    pub fn unwrap_a(self) -> A {
        let a = if let Either::A(a) = self {
            Some(a)
        } else {
            None
        };

        return a.unwrap();
    }

    pub fn unwrap_b(self) -> B {
        let b = if let Either::B(b) = self {
            Some(b)
        } else {
            None
        };

        return b.unwrap();
    }
}

impl<T: ResolvedType> From<UseStaticPath<()>> for UseStaticPath<Option<T>> {
    fn from(value: UseStaticPath<()>) -> Self {
        return Self {
            name: value.name,
            details: value.details.map_a(|next| from(next)).map_b(|_| None),
        };
    }
}

impl<T: ResolvedType> Resolvable for UseStaticPath<Option<T>> {
    fn is_resolved(&self) -> bool {
        if let Either::B(b) = &self.details {
            if b.is_none() {
                dbg!("false");
                return false;
            }
        }

        if let Either::A(next) = &self.details {
            if !next.is_resolved() {
                dbg!("false");
                return false;
            }
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<UseStaticPath<Option<T>>> for UseStaticPath<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: UseStaticPath<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            name: value.name,
            details: value.details.try_map_a(|next| try_from(next))?.try_map_b(
                |resolved_type| {
                    resolved_type.ok_or(FinalizeResolveTypeError {
                        file: file!(),
                        line: line!(),
                    })
                },
            )?,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UseStaticPathNext<ResolvedType = ()> {
    Single(UseStaticPathNextSingle<ResolvedType>),
    Many(UseStaticPathNextMany<ResolvedType>),
}

impl<T: ResolvedType> From<UseStaticPathNext<()>> for UseStaticPathNext<Option<T>> {
    fn from(value: UseStaticPathNext<()>) -> Self {
        match value {
            UseStaticPathNext::Single(single) => return Self::Single(from(single)),
            UseStaticPathNext::Many(many) => return Self::Many(from(many)),
        }
    }
}

impl<T: ResolvedType> Resolvable for UseStaticPathNext<Option<T>> {
    fn is_resolved(&self) -> bool {
        match self {
            Self::Single(single) => return single.is_resolved(),
            Self::Many(many) => return many.is_resolved(),
        }
    }
}

impl<T: ResolvedType> TryFrom<UseStaticPathNext<Option<T>>> for UseStaticPathNext<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: UseStaticPathNext<Option<T>>) -> Result<Self, Self::Error> {
        match value {
            UseStaticPathNext::Single(single) => return Ok(Self::Single(try_from(single)?)),
            UseStaticPathNext::Many(many) => return Ok(Self::Many(try_from(many)?)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPathNextSingle<ResolvedType = ()> {
    pub double_colon_token: Arc<Token>,
    pub path: Box<UseStaticPath<ResolvedType>>,
}

impl<T: ResolvedType> From<UseStaticPathNextSingle<()>> for UseStaticPathNextSingle<Option<T>> {
    fn from(value: UseStaticPathNextSingle<()>) -> Self {
        return Self {
            double_colon_token: value.double_colon_token,
            path: Box::new(from(*value.path)),
        };
    }
}

impl<T: ResolvedType> Resolvable for UseStaticPathNextSingle<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.path.is_resolved();
    }
}

impl<T: ResolvedType> TryFrom<UseStaticPathNextSingle<Option<T>>> for UseStaticPathNextSingle<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: UseStaticPathNextSingle<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            double_colon_token: value.double_colon_token,
            path: Box::new(try_from(*value.path)?),
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPathNextMany<ResolvedType = ()> {
    pub double_colon_token: Arc<Token>,
    pub open_brace: Arc<Token>,
    pub nexts: Vec<UseStaticPathNextManyItem<ResolvedType>>,
    pub close_brace: Arc<Token>,
}

impl<T: ResolvedType> From<UseStaticPathNextMany<()>> for UseStaticPathNextMany<Option<T>> {
    fn from(value: UseStaticPathNextMany<()>) -> Self {
        return Self {
            double_colon_token: value.double_colon_token,
            open_brace: value.open_brace,
            nexts: value.nexts.into_iter().map(from).collect(),
            close_brace: value.close_brace,
        };
    }
}

impl<T: ResolvedType> Resolvable for UseStaticPathNextMany<Option<T>> {
    fn is_resolved(&self) -> bool {
        for next in &self.nexts {
            if !next.is_resolved() {
                dbg!("false");
                return false;
            }
        }

        return true;
    }
}

impl<T: ResolvedType> TryFrom<UseStaticPathNextMany<Option<T>>> for UseStaticPathNextMany<T> {
    type Error = FinalizeResolveTypeError;

    fn try_from(value: UseStaticPathNextMany<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            double_colon_token: value.double_colon_token,
            open_brace: value.open_brace,
            nexts: value
                .nexts
                .into_iter()
                .map(|next| try_from(next))
                .collect::<Result<Vec<UseStaticPathNextManyItem<T>>, Self::Error>>()?,
            close_brace: value.close_brace,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPathNextManyItem<ResolvedType = ()> {
    pub path: UseStaticPath<ResolvedType>,
    pub comma_token: Option<Arc<Token>>,
}

impl<T: ResolvedType> From<UseStaticPathNextManyItem<()>> for UseStaticPathNextManyItem<Option<T>> {
    fn from(value: UseStaticPathNextManyItem<()>) -> Self {
        return Self {
            path: from(value.path),
            comma_token: value.comma_token,
        };
    }
}

impl<T: ResolvedType> Resolvable for UseStaticPathNextManyItem<Option<T>> {
    fn is_resolved(&self) -> bool {
        return self.path.is_resolved();
    }
}

impl<T: ResolvedType> TryFrom<UseStaticPathNextManyItem<Option<T>>>
    for UseStaticPathNextManyItem<T>
{
    type Error = FinalizeResolveTypeError;

    fn try_from(value: UseStaticPathNextManyItem<Option<T>>) -> Result<Self, Self::Error> {
        return Ok(Self {
            path: try_from(value.path)?,
            comma_token: value.comma_token,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UseMod {
    Pub(Arc<Token>),
}

// Visitor pattern
pub trait UseVisitor<T: ResolvedType, R = ()> {
    fn visit_use(&mut self, use_decl: &mut Use<T>) -> R;
}

pub trait UseAccept<T: ResolvedType, R, V: UseVisitor<T, R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<T: ResolvedType, R, V: UseVisitor<T, R>> UseAccept<T, R, V> for Use<T> {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_use(self);
    }
}
