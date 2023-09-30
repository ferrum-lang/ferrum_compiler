use super::*;

impl UseVisitor<FeType, Result> for RustSyntaxCompiler {
    fn visit_use(&mut self, use_decl: Arc<Mutex<Use<FeType>>>) -> Result {
        let mut use_decl = use_decl.try_lock().unwrap();

        let use_mod = use_decl
            .use_mod
            .as_ref()
            .map(|use_mod| self.translate_use_mod(use_mod));

        let path = Self::translate_use_static_path(&mut use_decl.path)?;

        if let Some(path) = path {
            let use_ir = ir::RustIRUse { use_mod, path };

            let file_idx = self.out.files.len() - 1;
            self.out.files[file_idx].uses.push(use_ir);
        }

        return Ok(());
    }
}
