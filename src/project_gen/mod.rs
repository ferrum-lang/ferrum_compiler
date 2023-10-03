use super::*;

mod rust;
pub use rust::*;

use crate::{config::Config, result::Result};

use std::sync::{Arc, Mutex};

pub trait CodeToProjectFiles {
    type ProjectFiles;
}

pub trait ProjectGen<Code: CodeToProjectFiles> {
    fn generate_project_files(
        cfg: Arc<Config>,
        code: Arc<Mutex<Code>>,
    ) -> Result<Code::ProjectFiles>;
}
