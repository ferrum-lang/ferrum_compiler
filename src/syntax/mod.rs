mod decl;
pub use decl::*;

mod node;
pub use node::*;

mod rust_compiler;
pub use rust_compiler::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct PackageName(pub String);

#[derive(Debug, Clone)]
pub enum Use {}

#[derive(Debug, Clone)]
pub struct SyntaxTree {
    pub uses: Vec<Rc<RefCell<Use>>>,
    pub decls: Vec<Rc<RefCell<Decl>>>,
}

#[derive(Debug, Clone)]
pub struct FeFile {
    pub name: PackageName,
    pub path: PathBuf,
    pub syntax: SyntaxTree,
}

#[derive(Debug, Clone)]
pub struct FeDir {
    pub name: PackageName,
    pub path: PathBuf,
    pub entry_file: FeFile,
    pub local_packages: HashMap<PackageName, FePackage>,
}

#[derive(Debug, Clone)]
pub enum FePackage {
    File(FeFile),
    Dir(FeDir),
}

pub trait SyntaxCompiler<IR> {
    fn compile_package(entry: FePackage) -> IR;
}
