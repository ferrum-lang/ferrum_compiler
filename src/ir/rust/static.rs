use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRStaticType {
    pub ref_type: Option<RustIRRefType>,
    pub static_path: RustIRStaticPath,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRRefType {
    Shared,
    Mut,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRStaticPath {
    pub root: Option<Box<Self>>,
    pub name: Arc<str>,
}

// Visitor pattern
pub trait RustIRStaticVisitor<R = ()> {
    fn visit_static_type(&mut self, static_type: &mut RustIRStaticType) -> R;
    fn visit_static_path(&mut self, static_path: &mut RustIRStaticPath) -> R;
}

pub trait RustIRStaticAccept<R, V: RustIRStaticVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: RustIRStaticVisitor<R>> RustIRStaticAccept<R, V> for RustIRStaticType {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_static_type(self);
    }
}

impl<R, V: RustIRStaticVisitor<R>> RustIRStaticAccept<R, V> for RustIRStaticPath {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_static_path(self);
    }
}
