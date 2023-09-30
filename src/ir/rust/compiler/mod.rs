mod decl;
mod expr;
mod stmt;
mod r#use;

use super::*;

use crate::r#type::*;
use crate::syntax::*;

use crate::ir;
use crate::token::Token;
use crate::utils::invert;

use crate::result::Result;

use std::sync::{Arc, Mutex};

pub struct RustSyntaxCompiler {
    entry: Arc<Mutex<FeSyntaxPackage<FeType>>>,
    out: ir::RustIR,
}

impl SyntaxCompiler<ir::RustIR> for RustSyntaxCompiler {
    fn compile_package(entry: Arc<Mutex<FeSyntaxPackage<FeType>>>) -> Result<ir::RustIR> {
        return Self::new(entry).compile();
    }
}

impl RustSyntaxCompiler {
    fn new(entry: Arc<Mutex<FeSyntaxPackage<FeType>>>) -> Self {
        return Self {
            entry,
            out: ir::RustIR {
                files: vec![ir::RustIRFile {
                    path: "./main.rs".into(), // TODO
                    mods: vec![],
                    uses: vec![],
                    decls: vec![],
                }],
            },
        };
    }

    fn compile(mut self) -> Result<ir::RustIR> {
        self.internal_compile_package(&mut Arc::clone(&self.entry).try_lock().unwrap())?;

        return Ok(self.out);
    }

    fn internal_compile_package(&mut self, package: &mut FeSyntaxPackage<FeType>) -> Result {
        match package {
            FeSyntaxPackage::File(file) => {
                self.compile_file(file)?;
            }

            FeSyntaxPackage::Dir(dir) => {
                {
                    let mut syntax = dir.entry_file.syntax.try_lock().unwrap();

                    for name in dir.local_packages.keys() {
                        syntax.mods.push(Mod(name.0.clone()));
                    }
                }

                self.compile_file(&mut dir.entry_file)?;

                for (name, package) in &dir.local_packages {
                    self.out.files.push(ir::RustIRFile {
                        path: format!("./{}.rs", name.0).into(),
                        mods: vec![],
                        uses: vec![],
                        decls: vec![],
                    });
                    self.internal_compile_package(&mut package.try_lock().unwrap())?;
                }
            }
        };

        return Ok(());
    }

    fn compile_file(&mut self, file: &mut FeSyntaxFile<FeType>) -> Result {
        let mut syntax = file.syntax.try_lock().unwrap();

        {
            let file_idx = self.out.files.len() - 1;
            let mods = &mut self.out.files[file_idx].mods;

            for mod_decl in &syntax.mods {
                mods.push(mod_decl.0.clone());
            }
        }

        for use_decl in &mut syntax.uses {
            use_decl.accept(self)?;
        }

        for decl in &mut syntax.decls {
            decl.try_lock().unwrap().accept(self)?;
        }

        return Ok(());
    }

    fn translate_decl_mod(&self, decl_mod: &DeclMod) -> ir::RustIRDeclMod {
        match decl_mod {
            DeclMod::Pub(_) => return ir::RustIRDeclMod::Pub,
        }
    }

    fn translate_fn_param(&self, param: &mut FnDeclParam<FeType>) -> ir::RustIRFnParam {
        return ir::RustIRFnParam {
            name: param.name.lexeme.clone(),
            static_type_ref: self.translate_static_type(&mut param.static_type_ref),
            trailing_comma: param.comma_token.is_some(),
        };
    }

    fn translate_fn_return_type(
        &self,
        return_type: &mut FnDeclReturnType<FeType>,
    ) -> ir::RustIRStaticType {
        return self.translate_static_type(&mut return_type.static_type);
    }

    fn translate_fn_body(&mut self, body: &mut FnDeclBody<FeType>) -> Result<ir::RustIRBlockExpr> {
        let mut block_ir = ir::RustIRBlockExpr { stmts: vec![] };

        match body {
            FnDeclBody::Short(_short) => todo!(),
            FnDeclBody::Block(block) => {
                for stmt in &mut block.stmts {
                    let stmt_ir = stmt.try_lock().unwrap().accept(self)?;

                    block_ir.stmts.extend(stmt_ir);
                }
            }
        }

        return Ok(block_ir);
    }

    fn translate_struct_field(&self, field: &mut StructDeclField<FeType>) -> ir::RustIRStructField {
        return ir::RustIRStructField {
            field_mod: field.field_mod.as_ref().map(|field| match field {
                StructFieldMod::Pub(_) => ir::RustIRStructFieldMod::Pub,
            }),
            name: field.name.lexeme.clone(),
            static_type_ref: self.translate_static_type(&mut field.static_type_ref),
            trailing_comma: field.comma_token.is_some(),
        };
    }

    fn translate_static_type(&self, typ: &mut StaticType<FeType>) -> ir::RustIRStaticType {
        let ref_type = typ.ref_type.as_ref().map(|ref_type| match ref_type {
            RefType::Shared { .. } => ir::RustIRRefType::Shared,
            RefType::Mut { .. } => ir::RustIRRefType::Mut,
        });

        return ir::RustIRStaticType {
            ref_type,
            static_path: Self::translate_static_path(&mut typ.static_path),
        };
    }

    fn translate_static_path(path: &mut StaticPath<FeType>) -> ir::RustIRStaticPath {
        if path.root.is_none()
            && path.name.lexeme.as_ref() == "Int"
            && matches!(path.resolved_type, FeType::Number(_))
        {
            return ir::RustIRStaticPath {
                root: None,
                name: "i64".into(),
            };
        }

        if path.root.is_none()
            && path.name.lexeme.as_ref() == "Bool"
            && matches!(path.resolved_type, FeType::Bool(_))
        {
            return ir::RustIRStaticPath {
                root: None,
                name: "bool".into(),
            };
        }

        return ir::RustIRStaticPath {
            root: path
                .root
                .as_mut()
                .map(|root| Box::new(Self::translate_static_path(root))),
            name: path.name.lexeme.clone(),
        };
    }

    fn translate_use_mod(&self, use_mod: &UseMod) -> ir::RustIRUseMod {
        match use_mod {
            UseMod::Pub(_) => ir::RustIRUseMod::Pub,
        }
    }

    fn translate_use_static_path(
        path: &mut UseStaticPath<FeType>,
    ) -> Result<Option<ir::RustIRUseStaticPath>> {
        let next = match &mut path.details {
            Either::B(_) => None,

            Either::A(UseStaticPathNext::Single(ref mut single)) => {
                if let Either::B(FeType::Callable(Callable {
                    special: Some(SpecialCallable::Print),
                    ..
                })) = &single.path.details
                {
                    // No need to import print
                    return Ok(None);
                } else {
                    let Some(next_path) = Self::translate_use_static_path(&mut single.path)? else {
                        return Ok(None);
                    };

                    Some(ir::RustIRUseStaticPathNext::Single(
                        ir::RustIRUseStaticPathNextSingle {
                            path: Box::new(next_path),
                        },
                    ))
                }
            }

            Either::A(UseStaticPathNext::Many(_many)) => todo!(),
        };

        let path_ir = ir::RustIRUseStaticPath {
            pre: path.pre.as_ref().map(|pre| match pre {
                UseStaticPathPre::DoubleColon(_) => ir::RustIRUseStaticPathPre::DoubleColon,
                UseStaticPathPre::CurrentDir(_) => ir::RustIRUseStaticPathPre::CurrentDir,
                UseStaticPathPre::RootDir(_) => ir::RustIRUseStaticPathPre::RootDir,
            }),
            name: path.name.lexeme.clone(),
            next,
        };

        return Ok(Some(path_ir));
    }

    fn map_label(&self, id: String, label: &Option<Arc<Token>>) -> Option<Arc<str>> {
        return label
            .as_ref()
            .map(|l| format!("'label_{}_{}", id, &l.lexeme[1..]).into());
    }
}
