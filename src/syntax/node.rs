use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct NodeId<T>(usize, PhantomData<T>);

impl<T> NodeId<T> {
    pub fn new(value: usize) -> Self {
        return Self(value, PhantomData);
    }

    pub fn zero() -> Self {
        return Self::new(0);
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

#[derive(Debug, Clone)]
pub enum NodeIdGen {
    Default(DefaultNodeIdGen),
    Zero(ZeroNodeIdGen),
}

impl NodeIdGen {
    pub fn next<T>(&self) -> NodeId<T> {
        match self {
            Self::Default(gen) => return gen.next(),
            Self::Zero(gen) => return gen.next(),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct DefaultNodeIdGenInstance {
    next_id: usize,
}

#[derive(Debug, Clone, Default)]
pub struct DefaultNodeIdGen {
    instance: Arc<Mutex<DefaultNodeIdGenInstance>>,
}

impl DefaultNodeIdGen {
    pub fn new() -> Self {
        return Self {
            instance: Arc::new(Mutex::new(DefaultNodeIdGenInstance { next_id: 0 })),
        };
    }

    fn next<T>(&self) -> NodeId<T> {
        let instance = &mut *self.instance.try_lock().unwrap();

        let id = instance.next_id;
        instance.next_id += 1;

        return NodeId::new(id);
    }
}

#[derive(Debug, Clone)]
pub struct ZeroNodeIdGen {}

impl ZeroNodeIdGen {
    fn next<T>(&self) -> NodeId<T> {
        return NodeId::zero();
    }
}

pub trait Node<T> {
    fn node_id(&self) -> NodeId<T>;
    fn set_node_id(&mut self, id: NodeId<T>);
}
