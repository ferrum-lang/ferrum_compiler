use super::*;

mod rust;
pub use rust::*;

use crate::result::Result;

use std::{
    path,
    sync::{Arc, Mutex},
};

pub trait CodeToProjectFiles {
    type ProjectFiles;
}

pub trait ProjectGen<Code: CodeToProjectFiles> {
    fn generate_project_files(
        code: Arc<Mutex<Code>>,
        dst: impl Into<path::PathBuf>,
    ) -> Result<Code::ProjectFiles>;
}
