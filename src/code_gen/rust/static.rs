use super::*;

impl ir::RustIRStaticVisitor<Result<Arc<str>>> for RustCodeGen {
    fn visit_static_type(&mut self, static_type: &mut ir::RustIRStaticType) -> Result<Arc<str>> {
        let mut out = String::new();

        match &static_type.ref_type {
            Some(RustIRRefType::Shared) => out.push('&'),
            Some(RustIRRefType::Mut) => out.push_str("&mut "),

            None => {}
        }

        out.push_str(&static_type.static_path.accept(self)?);

        return Ok(out.into());
    }

    fn visit_static_path(&mut self, static_path: &mut ir::RustIRStaticPath) -> Result<Arc<str>> {
        if let Some(root) = &mut static_path.root {
            let code = root.accept(self)?;

            return Ok(format!("{}::{}", code, static_path.name).into());
        } else {
            return Ok(static_path.name.clone());
        }
    }
}
