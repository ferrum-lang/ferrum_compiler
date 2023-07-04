use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct NodeId<T>(Rc<RefCell<usize>>, PhantomData<T>);

impl<T> NodeId<T> {
    pub fn gen() -> Self {
        static mut NEXT_ID: usize = 0;

        let id = unsafe {
            let id = NEXT_ID;
            NEXT_ID += 1;
            id
        };

        return Self(Rc::new(RefCell::new(id)), PhantomData);
    }
}

pub trait Node<T> {
    fn node_id(&self) -> &NodeId<T>;
}
