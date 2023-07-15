mod decl;
pub use decl::*;

mod stmt;
pub use stmt::*;

mod expr;
pub use expr::*;

mod r#use;
pub use r#use::*;

use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct RustIR {
    pub files: Vec<RustIRFile>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRFile {
    pub path: PathBuf,
    pub uses: Vec<RustIRUse>,
    pub decls: Vec<RustIRDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRMacro {}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRStaticType {}

#[derive(Debug, Clone, PartialEq)]
pub struct RustIRCodeBlock {
    pub stmts: Vec<RustIRStmt>,
}
