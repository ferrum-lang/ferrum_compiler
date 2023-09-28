use std::{hash::Hash, marker::PhantomData};

#[derive(Debug)]
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

impl<T> Clone for NodeId<T> {
    fn clone(&self) -> Self {
        return *self;
    }
}

impl<T> Copy for NodeId<T> {}

impl<T> PartialEq for NodeId<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}

impl<T> Eq for NodeId<T> {}

impl<T> Hash for NodeId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        return Hash::hash(&self.0, state);
    }
}

pub trait Node<T> {
    fn node_id(&self) -> NodeId<T>;
}
