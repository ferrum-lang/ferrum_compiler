use super::*;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Use {
    pub id: NodeId<Use>,
    pub use_token: Token,
    pub pre_double_colon_token: Option<Token>,
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
    pub name: Token,
    pub next: Option<UseStaticPathNext>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UseStaticPathNext {
    Single(UseStaticPathNextSingle),
    Many(UseStaticPathNextMany),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPathNextSingle {
    pub double_colon_token: Token,
    pub path: Box<UseStaticPath>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPathNextMany {
    pub double_colon_token: Token,
    pub open_brace: Token,
    pub nexts: Vec<UseStaticPathNextManyItem>,
    pub close_brace: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStaticPathNextManyItem {
    pub path: UseStaticPath,
    pub comma_token: Option<Token>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UseMod {
    Pub(Token),
}
