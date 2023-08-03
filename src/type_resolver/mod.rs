use crate::r#type::*;
use crate::syntax::*;

use crate::result::Result;

use std::sync::{Arc, Mutex};

pub struct FeTypeResolver {}

impl FeTypeResolver {
    pub fn resolve_package(pkg: Arc<Mutex<FeSyntaxPackage>>) -> Result<FeSyntaxPackage<FeType>> {
        todo!()
    }
}
