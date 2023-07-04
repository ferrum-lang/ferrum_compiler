use super::*;

use crate::ir;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct RustSyntaxCompiler {}

impl SyntaxCompiler<ir::RustIR> for RustSyntaxCompiler {
    fn compile_package(entry: FePackage) -> ir::RustIR {
        let dir = match entry {
            FePackage::Dir(dir) => dir,
            FePackage::File(file) => FeDir {
                name: file.name.clone(),
                path: Path::parent(&file.path)
                    .map(|p| PathBuf::from(p))
                    .unwrap_or_else(|| PathBuf::from("")),
                entry_file: file,
                local_packages: HashMap::new(),
            },
        };

        dbg!("TODO");

        return ir::RustIR { files: vec![] };
    }
}
