use std::{fs, path, process};

use super::*;

#[derive(Debug, Clone)]
pub struct RustProjectGen {
    entry: Arc<Mutex<code_gen::RustCode>>,
    out: RustProjectFiles,
}

#[derive(Debug, Clone)]
pub struct RustProjectFiles {
    dir: path::PathBuf,
    files: Vec<path::PathBuf>,
}

impl RustProjectFiles {
    fn new(dst: path::PathBuf) -> Self {
        return Self {
            dir: dst,
            files: vec![],
        };
    }
}

impl CodeToProjectFiles for code_gen::RustCode {
    type ProjectFiles = RustProjectFiles;
}

impl ProjectGen<code_gen::RustCode> for RustProjectGen {
    fn generate_project_files(
        rust_code: Arc<Mutex<code_gen::RustCode>>,
        dst: impl Into<path::PathBuf>,
    ) -> Result<RustProjectFiles> {
        return Self::new(rust_code, dst).generate();
    }
}

impl RustProjectGen {
    fn new(entry: Arc<Mutex<code_gen::RustCode>>, dst: impl Into<path::PathBuf>) -> Self {
        return Self {
            entry,
            out: RustProjectFiles::new(dst.into()),
        };
    }

    fn generate(mut self) -> Result<RustProjectFiles> {
        let dst = &self.out.dir.clone();

        if !dst.exists() {
            fs::create_dir_all(dst)?;
        } else {
            // TODO: after creation, create a file with a hash of files
            // before deletion, check the hash
            // if hash doesn't match, then don't delete because there have been manual changes

            fs::remove_dir_all(dst)?;
            fs::create_dir_all(dst)?;
        }
        let dst = fs::canonicalize(dst)?;

        let _ = process::Command::new("cargo")
            .current_dir(&dst)
            .arg("init")
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .status()?;

        {
            let main = dst.join("src/main.rs");
            fs::remove_file(&main)?;
        }

        let src = dst.join("src");

        for file in &self.entry.lock().unwrap().files {
            let path = src.join(&file.path);

            fs::write(&path, file.content.as_bytes())?;

            self.out.files.push(path);
        }

        return Ok(self.out);
    }
}
