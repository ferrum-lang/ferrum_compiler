use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct NodeId(Rc<RefCell<usize>>);

impl NodeId {
    pub fn gen() -> Self {
        static mut NEXT_ID: usize = 0;

        let id = unsafe {
            let id = NEXT_ID;
            NEXT_ID += 1;
            id
        };

        return Self(Rc::new(RefCell::new(id)));
    }
}

pub trait Node {
    fn node_id(&self) -> &NodeId;
}

#[derive(Debug, Clone)]
pub enum Use {}

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

#[derive(Debug, Clone)]
pub struct SyntaxTree {
    pub uses: Vec<Rc<RefCell<Use>>>,
    pub declarations: Vec<Rc<RefCell<Decl>>>,
}

pub fn compile_syntax_tree(syntax_tree: SyntaxTree) -> IR {
    return IR::Rust(RustIR {
        uses: vec![],
        declarations: vec![],
    });
}

#[derive(Debug, Clone)]
pub enum IR {
    Rust(RustIR),
}

#[derive(Debug, Clone)]
pub struct RustIR {
    pub uses: Vec<RustIRUse>,
    pub declarations: Vec<RustIRDecl>,
}

#[derive(Debug, Clone)]
pub enum RustIRUse {}

#[derive(Debug, Clone)]
pub enum RustIRDecl {}
