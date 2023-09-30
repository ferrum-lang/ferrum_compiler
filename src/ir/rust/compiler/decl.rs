use super::*;

impl DeclVisitor<FeType, Result> for RustSyntaxCompiler {
    fn visit_function_decl(&mut self, decl: Arc<Mutex<FnDecl<FeType>>>) -> Result {
        let mut decl = decl.try_lock().unwrap();

        let fn_ir = ir::RustIRFnDecl {
            macros: vec![],

            decl_mod: decl
                .decl_mod
                .as_ref()
                .map(|decl_mod| self.translate_decl_mod(decl_mod)),

            is_async: false, // TODO

            generics: None,

            name: decl.name.lexeme.clone(),
            params: decl
                .params
                .iter_mut()
                .map(|param| self.translate_fn_param(param))
                .collect(),

            return_type: decl
                .return_type
                .as_mut()
                .map(|return_type| self.translate_fn_return_type(return_type)),

            body: self.translate_fn_body(&mut decl.body)?,
        };

        let file_idx = self.out.files.len() - 1;
        self.out.files[file_idx]
            .decls
            .push(ir::RustIRDecl::Fn(fn_ir));

        return Ok(());
    }

    fn visit_struct_decl(&mut self, decl: Arc<Mutex<StructDecl<FeType>>>) -> Result {
        let mut decl = decl.try_lock().unwrap();

        let struct_ir = ir::RustIRStructDecl {
            macros: vec![],
            decl_mod: decl
                .decl_mod
                .as_ref()
                .map(|decl_mod| self.translate_decl_mod(decl_mod)),

            name: decl.name.lexeme.clone(),

            generics: None,

            fields: decl
                .fields
                .iter_mut()
                .map(|field| self.translate_struct_field(field))
                .collect(),
        };

        let file_idx = self.out.files.len() - 1;
        self.out.files[file_idx]
            .decls
            .push(ir::RustIRDecl::Struct(struct_ir));

        return Ok(());
    }
}
