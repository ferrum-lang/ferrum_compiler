mod compiler;
pub use compiler::*;

mod syntax;
pub use syntax::*;

use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct RustIR {
    pub files: Vec<RustIRFile>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRFile {
    pub path: PathBuf,
    pub mods: Vec<RustIRMod>,
    pub uses: Vec<RustIRUse>,
    pub decls: Vec<RustIRDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRMacro {}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRMod {
    pub is_pub: bool,
    pub name: Arc<str>,
}
