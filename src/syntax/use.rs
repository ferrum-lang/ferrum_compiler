use super::*;

use crate::token::Token;

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

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPath<ResolvedType = ()> {
    pub name: Arc<Token>,
    pub next: Option<UseStaticPathNext<ResolvedType>>,
    pub resolved_type: ResolvedType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UseStaticPathNext<ResolvedType = ()> {
    Single(UseStaticPathNextSingle<ResolvedType>),
    Many(UseStaticPathNextMany<ResolvedType>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPathNextSingle<ResolvedType = ()> {
    pub double_colon_token: Arc<Token>,
    pub path: Box<UseStaticPath<ResolvedType>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPathNextMany<ResolvedType = ()> {
    pub double_colon_token: Arc<Token>,
    pub open_brace: Arc<Token>,
    pub nexts: Vec<UseStaticPathNextManyItem<ResolvedType>>,
    pub close_brace: Arc<Token>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPathNextManyItem<ResolvedType = ()> {
    pub path: UseStaticPath<ResolvedType>,
    pub comma_token: Option<Arc<Token>>,
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
