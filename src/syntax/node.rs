use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct NodeId<T>(usize, PhantomData<T>);

impl<T> NodeId<T> {
    pub fn gen() -> Self {
        static mut NEXT_ID: usize = 0;

        let id = unsafe {
            let id = NEXT_ID;
            NEXT_ID += 1;
            id
        };

        return Self(id, PhantomData);
    }
}

pub trait Node<T> {
    fn node_id(&self) -> &NodeId<T>;
}
