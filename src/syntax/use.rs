use super::*;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Use {
    pub id: NodeId<Use>,
    pub use_token: Arc<Token>,
    pub pre_double_colon_token: Option<Arc<Token>>,
    pub use_mod: Option<UseMod>,
    pub path: UseStaticPath,
}

impl Node<Use> for Use {
    fn node_id(&self) -> &NodeId<Use> {
        return &self.id;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPath {
    pub name: Arc<Token>,
    pub next: Option<UseStaticPathNext>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UseStaticPathNext {
    Single(UseStaticPathNextSingle),
    Many(UseStaticPathNextMany),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPathNextSingle {
    pub double_colon_token: Arc<Token>,
    pub path: Box<UseStaticPath>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPathNextMany {
    pub double_colon_token: Arc<Token>,
    pub open_brace: Arc<Token>,
    pub nexts: Vec<UseStaticPathNextManyItem>,
    pub close_brace: Arc<Token>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPathNextManyItem {
    pub path: UseStaticPath,
    pub comma_token: Option<Arc<Token>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UseMod {
    Pub(Arc<Token>),
}

// Visitor pattern
pub trait UseVisitor<R = ()> {
    fn visit_use(&mut self, use_decl: &mut Use) -> R;
}

pub trait UseAccept<R, V: UseVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: UseVisitor<R>> UseAccept<R, V> for Use {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_use(self);
    }
}
