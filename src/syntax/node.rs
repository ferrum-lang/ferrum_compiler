use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;

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

    pub fn into<R>(self) -> NodeId<R> {
        return NodeId(self.0, PhantomData);
    }
}

impl<T> Clone for NodeId<T> {
    fn clone(&self) -> Self {
        *self
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

impl<T> fmt::Display for NodeId<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}

pub trait Node<T> {
    fn node_id(&self) -> NodeId<T>;
}
