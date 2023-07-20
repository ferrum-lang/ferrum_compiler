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
use crate::token;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub trait SyntaxCompiler<IR> {
    fn compile_package(entry: Arc<Mutex<FeSyntaxPackage>>) -> Result<IR>;
}

#[derive(Debug, Clone)]
pub enum FeSyntaxPackage {
    File(FeSyntaxFile),
    Dir(FeSyntaxDir),
}

impl From<token::FeTokenPackage> for FeSyntaxPackage {
    fn from(value: token::FeTokenPackage) -> Self {
        match value {
            token::FeTokenPackage::File(file) => return FeSyntaxPackage::File(file.into()),
            token::FeTokenPackage::Dir(dir) => return FeSyntaxPackage::Dir(dir.into()),
        };
    }
}

#[derive(Debug, Clone)]
pub struct FeSyntaxFile {
    pub name: SyntaxPackageName,
    pub path: PathBuf,
    pub syntax: Arc<Mutex<SyntaxTree>>,
}

impl From<token::FeTokenFile> for FeSyntaxFile {
    fn from(value: token::FeTokenFile) -> Self {
        return Self {
            name: value.name.into(),
            path: value.path,
            syntax: Arc::new(Mutex::new(SyntaxTree {
                uses: vec![],
                decls: vec![],
            })),
        };
    }
}

#[derive(Debug, Clone)]
pub struct FeSyntaxDir {
    pub name: SyntaxPackageName,
    pub path: PathBuf,
    pub entry_file: FeSyntaxFile,
    pub local_packages: HashMap<SyntaxPackageName, Arc<Mutex<FeSyntaxPackage>>>,
}

impl From<token::FeTokenDir> for FeSyntaxDir {
    fn from(value: token::FeTokenDir) -> Self {
        return Self {
            name: value.name.into(),
            path: value.path,
            entry_file: value.entry_file.into(),
            local_packages: value
                .local_packages
                .into_iter()
                .map(|(name, pkg)| {
                    let name: SyntaxPackageName = name.into();

                    let pkg: &token::FeTokenPackage = &pkg.lock().unwrap();
                    let pkg: FeSyntaxPackage = pkg.clone().into();
                    let pkg = Arc::new(Mutex::new(pkg));

                    return (name, pkg);
                })
                .collect(),
        };
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SyntaxPackageName(pub Arc<str>);

impl From<token::TokenPackageName> for SyntaxPackageName {
    fn from(value: token::TokenPackageName) -> Self {
        return Self(value.0);
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxTree {
    pub uses: Vec<Arc<Mutex<Use>>>,
    pub decls: Vec<Arc<Mutex<Decl>>>,
}
