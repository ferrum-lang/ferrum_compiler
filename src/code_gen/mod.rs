mod rust;
pub use rust::*;

use crate::result::Result;

use std::sync::{Arc, Mutex};

pub trait IRToCode {
    type Code;
}

pub trait CodeGen<IR: IRToCode> {
    fn generate_code(ir: Arc<Mutex<IR>>) -> Result<IR::Code>;
}
