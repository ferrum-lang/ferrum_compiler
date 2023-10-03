mod rust;
pub use rust::*;

use crate::{config::Config, result::Result};

use std::sync::{Arc, Mutex};

pub trait IRToCode {
    type Code;
}

pub trait CodeGen<IR: IRToCode> {
    fn generate_code(cfg: Arc<Config>, ir: Arc<Mutex<IR>>) -> Result<IR::Code>;
}
