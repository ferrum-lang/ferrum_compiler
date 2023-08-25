use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum FeSourcePackage {
    File(FeSourceFile),
    Dir(FeSourceDir),
}

#[derive(Debug, Clone)]
pub struct FeSourceFile {
    pub name: SourcePackageName,
    pub path: PathBuf,
    pub content: Arc<str>,
}

#[derive(Debug, Clone)]
pub struct FeSourceDir {
    pub name: SourcePackageName,
    pub path: PathBuf,
    pub entry_file: FeSourceFile,
    pub local_packages: HashMap<SourcePackageName, Arc<Mutex<FeSourcePackage>>>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SourcePackageName(pub Arc<str>);
