use crate::result::Result;

use crate::source::*;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct SourceReader {
    src_root_dir: PathBuf,
}

impl SourceReader {
    pub fn read_src_files(src_root_dir: PathBuf) -> Result<FeSourcePackage> {
        return Self::new(src_root_dir).read();
    }

    pub fn new(src_root_dir: PathBuf) -> Self {
        return Self { src_root_dir };
    }

    pub fn read(&self) -> Result<FeSourcePackage> {
        fn build_dir_pkg(dir: PathBuf, name: SourcePackageName) -> Result<FeSourcePackage> {
            let src_dir_entries = dir.read_dir()?;
            let mut local_packages = HashMap::new();

            for pkg in src_dir_entries {
                let pkg = pkg?;

                let name = SourcePackageName(
                    pkg.path()
                        .file_stem()
                        .unwrap()
                        .to_os_string()
                        .into_string()
                        .unwrap()
                        .into(),
                );

                if name.0.as_ref() == "_pkg" {
                    continue;
                }

                if pkg.file_type()?.is_dir() {
                    let pkg = build_dir_pkg(pkg.path(), name.clone())?;

                    local_packages.insert(name, Arc::new(Mutex::new(pkg)));
                } else {
                    let path = pkg.path().into_os_string().into_string().unwrap().into();
                    let content = std::fs::read_to_string(&path)?.into();

                    let pkg = FeSourcePackage::File(FeSourceFile {
                        name: name.clone(),
                        path,
                        content,
                    });

                    local_packages.insert(name, Arc::new(Mutex::new(pkg)));
                }
            }

            let pkg = dir.join("_pkg.fe");
            if !pkg.is_file() {
                panic!("Expected package {:?} to contain a '_pkg.fe' file", dir);
            }
            let pkg_content = std::fs::read_to_string(&pkg)?.into();

            let entry_file = FeSourceFile {
                name: SourcePackageName("_pkg".into()),
                path: pkg,
                content: pkg_content,
            };

            return Ok(FeSourcePackage::Dir(FeSourceDir {
                name,
                path: dir,
                entry_file,
                local_packages,
            }));
        }

        if !self.src_root_dir.is_dir() {
            panic!(
                "Expected the project root to contain a 'src' directory: {:?}",
                self.src_root_dir
            );
        }

        let src_dir_entries = self.src_root_dir.read_dir()?;
        let mut local_packages = HashMap::new();

        for pkg in src_dir_entries {
            let pkg = pkg?;

            let name = SourcePackageName(
                pkg.path()
                    .file_stem()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap()
                    .into(),
            );

            if name.0.as_ref() == "_main" {
                continue;
            }

            if pkg.file_type()?.is_dir() {
                let pkg = build_dir_pkg(pkg.path(), name.clone())?;

                local_packages.insert(name, Arc::new(Mutex::new(pkg)));
            } else {
                let path = pkg.path().into_os_string().into_string().unwrap().into();
                let content = std::fs::read_to_string(&path)?.into();

                let pkg = FeSourcePackage::File(FeSourceFile {
                    name: name.clone(),
                    path,
                    content,
                });

                local_packages.insert(name, Arc::new(Mutex::new(pkg)));
            }
        }

        let main = self.src_root_dir.join("_main.fe");
        if !main.is_file() {
            panic!("Expected project root to contain a '_main.fe' file");
        }
        let main_content = std::fs::read_to_string(main)?;

        let source = FeSourcePackage::Dir(FeSourceDir {
            name: SourcePackageName("src".into()),
            path: "src/".into(),
            entry_file: FeSourceFile {
                name: SourcePackageName("_main".into()),
                path: "src/_main.fe".into(),
                content: main_content.into(),
            },
            local_packages,
        });

        return Ok(source);
    }
}
