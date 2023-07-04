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
