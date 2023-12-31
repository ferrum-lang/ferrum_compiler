mod decl;
mod expr;
mod r#static;
mod stmt;
mod r#use;

use std::path::PathBuf;

use super::*;

use crate::{
    config::Config,
    ir::{
        self, RustIRDeclAccept, RustIRExprAccept, RustIRRefType, RustIRStaticAccept,
        RustIRStmtAccept, RustIRUseAccept,
    },
};

#[derive(Debug, Clone)]
pub struct RustCodeGen {
    #[allow(unused)]
    cfg: Arc<Config>,

    entry: Arc<Mutex<ir::RustIR>>,
    out: RustCode,

    indent: usize,
}

#[derive(Debug, Clone)]
pub struct RustCode {
    pub files: Vec<RustCodeFile>,
}

#[derive(Debug, Clone)]
pub struct RustCodeFile {
    pub path: PathBuf,
    pub content: Arc<str>,
}

impl IRToCode for ir::RustIR {
    type Code = RustCode;
}

impl CodeGen<ir::RustIR> for RustCodeGen {
    fn generate_code(cfg: Arc<Config>, rust_ir: Arc<Mutex<ir::RustIR>>) -> Result<RustCode> {
        return Self::new(cfg, rust_ir).generate();
    }
}

impl RustCodeGen {
    fn new(cfg: Arc<Config>, entry: Arc<Mutex<ir::RustIR>>) -> Self {
        return Self {
            cfg,
            entry,
            out: RustCode { files: vec![] },

            indent: 0,
        };
    }

    fn generate(mut self) -> Result<RustCode> {
        let entry = self.entry.clone();

        for file in &mut entry.lock().unwrap().files {
            let mut content =
                "#![allow(unreachable_code, while_true, unused_labels)]\n\n".to_string();

            for mod_decl in &mut file.mods {
                let mut code = String::new();

                if mod_decl.is_pub {
                    code.push_str("pub ");
                }

                code.push_str(&format!("mod {};", mod_decl.name));
                content.push_str(&code);

                content.push_str(&self.new_line());
                content.push_str(&self.new_line());
            }

            for use_decl in &mut file.uses {
                let code = use_decl.accept(&mut self)?;
                content.push_str(&code);
                content.push_str(&self.new_line());
                content.push_str(&self.new_line());
            }

            for decl in &mut file.decls {
                let code = decl.accept(&mut self)?;
                content.push_str(&code);
                content.push_str(&self.new_line());
                content.push_str(&self.new_line());
            }

            self.out.files.push(RustCodeFile {
                path: file.path.clone(),
                content: content.into(),
            });
        }

        return Ok(self.out);
    }

    fn gen_use_path(use_path: &mut ir::RustIRUseStaticPath) -> Result<Arc<str>> {
        let mut out = String::new();

        match use_path.pre {
            None => {}

            Some(ir::RustIRUseStaticPathPre::DoubleColon) => out.push_str("::"),
            Some(ir::RustIRUseStaticPathPre::CurrentDir) => out.push_str(""),
            Some(ir::RustIRUseStaticPathPre::RootDir) => out.push_str("crate::"),
        }

        out.push_str(&use_path.name);

        match &mut use_path.next {
            Some(ir::RustIRUseStaticPathNext::Single(single)) => {
                out.push_str("::");

                let code = Self::gen_use_path(&mut single.path)?;
                out.push_str(&code);
            }

            Some(ir::RustIRUseStaticPathNext::Many(_many)) => {
                todo!()
            }

            None => {}
        }

        return Ok(out.into());
    }

    fn translate_static_type(&mut self, static_type: &mut ir::RustIRStaticType) -> Arc<str> {
        let mut out = String::new();

        match static_type.ref_type {
            Some(ir::RustIRRefType::Shared) => out.push('&'),
            Some(ir::RustIRRefType::Mut) => out.push_str("&mut "),
            None => {}
        }

        out.push_str(&Self::translate_static_path(&mut static_type.static_path));

        return out.into();
    }

    fn translate_static_path(static_path: &mut ir::RustIRStaticPath) -> Arc<str> {
        if let Some(root) = &mut static_path.root {
            let mut out = Self::translate_static_path(&mut *root).to_string();

            out.push_str("::");

            out.push_str(&static_path.name);

            return out.into();
        }

        return static_path.name.clone();
    }

    fn new_line(&self) -> String {
        let mut out = String::from("\n");

        for _ in 0..self.indent {
            out.push_str("    ");
        }

        return out;
    }
}
