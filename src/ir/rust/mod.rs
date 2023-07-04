use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RustIR {
    pub files: Vec<RustIRFile>,
}

#[derive(Debug, Clone)]
pub struct RustIRFile {
    pub path: PathBuf,
    pub uses: Vec<RustIRUse>,
    pub decls: Vec<RustIRDecl>,
}

#[derive(Debug, Clone)]
pub enum RustIRUse {}

#[derive(Debug, Clone)]
pub enum RustIRDecl {}
