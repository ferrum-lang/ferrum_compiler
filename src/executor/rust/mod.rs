use std::{path, process};

use crate::result::Result;

#[derive(Debug, Clone)]
pub struct RustExecutor {}

impl RustExecutor {
    pub fn cargo_run(project_dir: &path::PathBuf) -> Result<process::Output> {
        let out = process::Command::new("cargo")
            .current_dir(project_dir)
            .args(["run", "--release", "-q"])
            .stdin(process::Stdio::inherit())
            .stdout(process::Stdio::piped())
            .output()?;

        return Ok(out);
    }
}
