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
            let file_idx = self.out.files.len() - 1;
            let file = &mut self.out.files[file_idx];

            /*
                // TODO: this is hacky
                // Can't have both!
                // Only add the mod, don't add the use
                // Also make the mod pub if use is pub
                mod some_pkg;
                use some_pkg;
            */

            let mut should_add = true;

            if path.next.is_none() {
                for idx in 0..file.mods.len() {
                    if &file.mods[idx].name == &path.name {
                        should_add = false;

                        if let Some(ir::RustIRUseMod::Pub) = use_mod {
                            file.mods[idx].is_pub = true;
                        }

                        break;
                    }
                }
            }

            if should_add {
                let use_ir = ir::RustIRUse { use_mod, path };
                file.uses.push(use_ir);
            }
        }

        return Ok(());
    }
}
