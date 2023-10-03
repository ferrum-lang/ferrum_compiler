use std::{process, sync::Arc};

use crate::{config::Config, result::Result};

#[derive(Debug, Clone)]
pub struct RustExecutor {}

impl RustExecutor {
    pub fn cargo_run(cfg: Arc<Config>) -> Result<process::Output> {
        let out = process::Command::new("cargo")
            .current_dir(&cfg.rust_gen_dir)
            .args(["run", "--release", "-q"])
            .stdin(process::Stdio::inherit())
            .stdout(process::Stdio::piped())
            .output()?;

        return Ok(out);
    }
}
