use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRStaticType {
    pub is_ptr: bool,
    pub static_path: GoIRStaticPath,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRStaticPath {
    pub root: Option<Box<Self>>,
    pub name: Arc<str>,
}

// Visitor pattern
pub trait GoIRStaticVisitor<R = ()> {
    fn visit_static_type(&mut self, static_type: &mut GoIRStaticType) -> R;
    fn visit_static_path(&mut self, static_path: &mut GoIRStaticPath) -> R;
}

pub trait GoIRStaticAccept<R, V: GoIRStaticVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: GoIRStaticVisitor<R>> GoIRStaticAccept<R, V> for GoIRStaticType {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_static_type(self);
    }
}

impl<R, V: GoIRStaticVisitor<R>> GoIRStaticAccept<R, V> for GoIRStaticPath {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_static_path(self);
    }
}
