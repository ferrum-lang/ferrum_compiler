use super::*;

mod rust;
pub use rust::*;

use crate::result::Result;

use std::cell::RefCell;
use std::rc::Rc;

pub trait IRToCode {
    type Code;
}

pub trait CodeGen<IR: IRToCode> {
    fn generate_code(entry: Rc<RefCell<IR>>) -> Result<IR::Code>;
}
