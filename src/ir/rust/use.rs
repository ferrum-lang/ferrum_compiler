use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRUse {
    pub use_mod: Option<RustIRUseMod>,
    pub pre_double_colon: bool,
    pub path: RustIRUseStaticPath,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RustIRUseMod {
    Pub,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRUseStaticPath {
    pub name: Arc<str>,
    pub next: Option<RustIRUseStaticPathNext>,
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
