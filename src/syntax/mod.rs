mod decl;
pub use decl::*;

mod expr;
pub use expr::*;

mod node;
pub use node::*;

mod rust_compiler;
pub use rust_compiler::*;

mod stmt;
pub use stmt::*;

mod r#use;
pub use r#use::*;

use crate::result::Result;

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

pub trait SyntaxCompiler<IR> {
    fn compile_package(entry: Rc<RefCell<FePackage>>) -> Result<IR>;
}

#[derive(Debug, Clone)]
pub enum FePackage {
    File(FeFile),
    Dir(FeDir),
}

#[derive(Debug, Clone)]
pub struct FeFile {
    pub name: PackageName,
    pub path: PathBuf,
    pub syntax: Rc<RefCell<SyntaxTree>>,
}

#[derive(Debug, Clone)]
pub struct FeDir {
    pub name: PackageName,
    pub path: PathBuf,
    pub entry_file: FeFile,
    pub local_packages: HashMap<PackageName, Rc<RefCell<FePackage>>>,
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct PackageName(pub Rc<str>);

#[derive(Debug, Clone)]
pub struct SyntaxTree {
    pub uses: Vec<Rc<RefCell<Use>>>,
    pub decls: Vec<Rc<RefCell<Decl>>>,
}
