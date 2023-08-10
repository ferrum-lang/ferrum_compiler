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

impl Node<Use> for Use {
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
    pub next: Option<UseStaticPathNext<ResolvedType>>,
    pub resolved_type: ResolvedType,
}

impl<T: ResolvedType> From<UseStaticPath<()>> for UseStaticPath<Option<T>> {
    fn from(value: UseStaticPath<()>) -> Self {
        return Self {
            name: value.name,
            next: value.next.map(|next| from(next)),
            resolved_type: None,
        };
    }
}

impl<T: ResolvedType> Resolvable for UseStaticPath<Option<T>> {
    fn is_resolved(&self) -> bool {
        if self.resolved_type.is_none() {
            return false;
        }

        if let Some(next) = &self.next {
            if !next.is_resolved() {
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
            next: invert(value.next.map(try_from))?,
            resolved_type: value.resolved_type.ok_or(FinalizeResolveTypeError)?,
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
