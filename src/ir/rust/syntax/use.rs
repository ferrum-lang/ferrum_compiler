use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRUse {
    pub use_mod: Option<RustIRUseMod>,
    pub path: RustIRUseStaticPath,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRUseMod {
    Pub,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRUseStaticPath {
    pub pre: Option<RustIRUseStaticPathPre>,
    pub name: Arc<str>,
    pub next: Option<RustIRUseStaticPathNext>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRUseStaticPathPre {
    DoubleColon,
    CurrentDir,
    RootDir,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRUseStaticPathNext {
    Single(RustIRUseStaticPathNextSingle),
    Many(RustIRUseStaticPathNextMany),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRUseStaticPathNextSingle {
    pub path: Box<RustIRUseStaticPath>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRUseStaticPathNextMany {
    pub nexts: Vec<RustIRUseStaticPathNextManyItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRUseStaticPathNextManyItem {
    pub path: RustIRUseStaticPath,
}

// Visitor pattern
pub trait RustIRUseVisitor<R = ()> {
    fn visit_use(&mut self, use_decl: &mut RustIRUse) -> R;
}

pub trait RustIRUseAccept<R, V: RustIRUseVisitor<R>> {
    fn accept(&mut self, visitor: &mut V) -> R;
}

impl<R, V: RustIRUseVisitor<R>> RustIRUseAccept<R, V> for RustIRUse {
    fn accept(&mut self, visitor: &mut V) -> R {
        return visitor.visit_use(self);
    }
}
