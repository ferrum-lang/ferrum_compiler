use super::*;

impl ir::RustIRDeclVisitor<Result<Arc<str>>> for RustCodeGen {
    fn visit_fn_decl(&mut self, decl: &mut ir::RustIRFnDecl) -> Result<Arc<str>> {
        let mut out = String::new();

        // TODO: Handle proc-macros

        match &decl.decl_mod {
            Some(ir::RustIRDeclMod::Pub) => out.push_str("pub "),

            None => {}
        }

        if decl.is_async {
            out.push_str("async ");
        }

        // TODO: Handle generics

        out.push_str(&format!("fn {}(", decl.name));

        for mut param in decl.params.clone() {
            out.push_str(&format!("{}: ", param.name));

            out.push_str(&param.static_type_ref.accept(self)?);

            if param.trailing_comma {
                out.push_str(", ");
            }
        }

        out.push_str(") ");

        if let Some(return_type) = &mut decl.return_type {
            out.push_str("-> ");

            let code = self.translate_static_type(return_type);
            out.push_str(&code);

            out.push(' ');
        }

        let code = decl.body.accept(self)?;
        out.push_str(&code);

        return Ok(out.into());
    }

    fn visit_struct_decl(&mut self, decl: &mut ir::RustIRStructDecl) -> Result<Arc<str>> {
        let mut out = String::new();

        match &decl.decl_mod {
            Some(ir::RustIRDeclMod::Pub) => out.push_str("pub "),

            None => {}
        }

        out.push_str("struct ");

        out.push_str(&decl.name);

        out.push_str(" {");

        self.indent += 1;
        out.push_str(&self.new_line());

        let fields_code = decl
            .fields
            .iter_mut()
            .map(|field| {
                let mut out = String::new();

                match &field.field_mod {
                    Some(ir::RustIRStructFieldMod::Pub) => out.push_str("pub "),

                    None => {}
                }

                out.push_str(&field.name);

                out.push_str(": ");

                out.push_str(&field.static_type_ref.accept(self)?);

                if field.trailing_comma {
                    out.push(',');
                }

                Ok(out.into())
            })
            .collect::<Result<Vec<Arc<str>>>>()?
            .join(&self.new_line());
        out.push_str(&fields_code);

        self.indent -= 1;
        out.push_str(&self.new_line());

        out.push('}');

        return Ok(out.into());
    }
}
