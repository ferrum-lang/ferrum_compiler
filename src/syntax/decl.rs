use super::*;

#[derive(Debug, Clone)]
pub enum Decl {
    Fn(FnDecl),
}

impl Node for Decl {
    fn node_id(&self) -> &NodeId {
        match self {
            Self::Fn(decl) => return decl.node_id(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FnDecl {
    pub id: NodeId,
}

impl Node for FnDecl {
    fn node_id(&self) -> &NodeId {
        return &self.id;
    }
}
