use crate::r#type::*;
use crate::syntax::*;

use crate::result::Result;

use std::sync::{Arc, Mutex};

pub struct FeTypeResolver {}

impl FeTypeResolver {
    pub fn resolve_package(pkg: FeSyntaxPackage) -> Result<FeSyntaxPackage<FeType>> {
        let mut pkg: FeSyntaxPackage<Option<FeType>> = pkg.into();

        while !pkg.is_resolved() {
            let changed = match &mut pkg {
                FeSyntaxPackage::File(file) => Self::resolve_file(file)?,
                FeSyntaxPackage::Dir(dir) => Self::resolve_dir(dir)?,
            };

            if !changed {
                todo!("Can't resolve!");
            }
        }

        return Ok(pkg.try_into()?);
    }

    fn internal_resolve_package(pkg: Arc<Mutex<FeSyntaxPackage<Option<FeType>>>>) -> Result<bool> {
        match &mut *pkg.lock().unwrap() {
            FeSyntaxPackage::File(file) => return Self::resolve_file(file),
            FeSyntaxPackage::Dir(dir) => return Self::resolve_dir(dir),
        }
    }

    fn resolve_dir(dir: &mut FeSyntaxDir<Option<FeType>>) -> Result<bool> {
        let mut changed = Self::resolve_file(&mut dir.entry_file)?;

        for pkg in dir.local_packages.values_mut() {
            changed = changed && Self::internal_resolve_package(pkg.clone())?;
        }

        return Ok(changed);
    }

    fn resolve_file(file: &mut FeSyntaxFile<Option<FeType>>) -> Result<bool> {
        return Ok(false);
    }
}
