mod compiler;
pub use compiler::*;

mod syntax;
pub use syntax::*;

use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct GoIR {
    pub files: Vec<GoIRFile>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoIRFile {
    pub path: PathBuf,
    pub imports: Vec<GoIRImport>,
    pub decls: Vec<GoIRDecl>,
}
