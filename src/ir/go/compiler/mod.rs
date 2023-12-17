mod decl;
mod expr;
mod import;
mod stmt;

use super::*;

use crate::config::Config;
use crate::r#type::*;
use crate::syntax::*;

use crate::ir;
use crate::token::Token;
use crate::utils::invert;

use crate::result::Result;

use std::sync::{Arc, Mutex};

pub struct GoSyntaxCompiler {
    #[allow(unused)]
    cfg: Arc<Config>,

    entry: Arc<Mutex<FeSyntaxPackage<FeType>>>,
    out: ir::GoIR,
}

impl SyntaxCompiler<ir::GoIR> for GoSyntaxCompiler {
    fn compile_package(
        cfg: Arc<Config>,
        entry: Arc<Mutex<FeSyntaxPackage<FeType>>>,
    ) -> Result<ir::GoIR> {
        return Self::new(cfg, entry).compile();
    }
}

impl GoSyntaxCompiler {
    fn new(cfg: Arc<Config>, entry: Arc<Mutex<FeSyntaxPackage<FeType>>>) -> Self {
        return Self {
            cfg,
            entry,
            out: ir::GoIR { files: vec![] },
        };
    }

    fn compile(mut self) -> Result<ir::GoIR> {
        self.internal_compile_package(
            &mut Arc::clone(&self.entry).try_lock().unwrap(),
            "".into(),
            true,
        )?;

        return Ok(self.out);
    }

    fn internal_compile_package(
        &mut self,
        package: &mut FeSyntaxPackage<FeType>,
        parent_dir: Arc<str>,
        is_main: bool,
    ) -> Result {
        match package {
            FeSyntaxPackage::File(file) => {
                self.out.files.push(ir::GoIRFile {
                    path: format!(
                        "{}{}.go",
                        parent_dir,
                        if is_main { "main" } else { &file.name.0 }
                    )
                    .into(),
                    imports: vec![],
                    decls: vec![],
                });

                self.compile_file(file)?;
            }

            FeSyntaxPackage::Dir(dir) => {
                let parent_dir: Arc<str> = format!("{}{}/", parent_dir, dir.name.0).into();

                self.out.files.push(ir::GoIRFile {
                    path: format!("{}{}.go", parent_dir, if is_main { "main" } else { "mod" })
                        .into(),
                    imports: vec![],
                    decls: vec![],
                });

                self.compile_file(&mut dir.entry_file)?;

                for package in dir.local_packages.values() {
                    self.internal_compile_package(
                        &mut package.try_lock().unwrap(),
                        parent_dir.clone(),
                        false,
                    )?;
                }
            }
        };

        return Ok(());
    }

    fn compile_file(&mut self, file: &mut FeSyntaxFile<FeType>) -> Result {
        let mut syntax = file.syntax.try_lock().unwrap();

        for use_decl in &mut syntax.uses {
            use_decl.accept(self)?;
        }

        for decl in &mut syntax.decls {
            decl.try_lock().unwrap().accept(self)?;
        }

        return Ok(());
    }

    fn translate_decl_mod(&self, decl_mod: &DeclMod) -> ir::GoIRDeclMod {
        match decl_mod {
            DeclMod::Pub(_) => return ir::GoIRDeclMod::Pub,
        }
    }

    fn translate_fn_param(&self, param: &mut FnDeclParam<FeType>) -> ir::GoIRFnParam {
        return ir::GoIRFnParam {
            name: param.name.lexeme.clone(),
            static_type_ref: self.translate_static_type(&mut param.static_type_ref),
            trailing_comma: param.comma_token.is_some(),
        };
    }

    fn translate_fn_return_type(
        &self,
        return_type: &mut FnDeclReturnType<FeType>,
    ) -> ir::GoIRStaticType {
        return self.translate_static_type(&mut return_type.static_type);
    }

    fn translate_fn_body(&mut self, body: &mut FnDeclBody<FeType>) -> Result<ir::GoIRBlockExpr> {
        let mut block_ir = ir::GoIRBlockExpr { stmts: vec![] };

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

    fn translate_struct_field(&self, field: &mut StructDeclField<FeType>) -> ir::GoIRStructField {
        return ir::GoIRStructField {
            field_mod: field.field_mod.as_ref().map(|field| match field {
                StructFieldMod::Pub(_) => ir::GoIRStructFieldMod::Pub,
            }),
            name: field.name.lexeme.clone(),
            static_type_ref: self.translate_static_type(&mut field.static_type_ref),
            trailing_comma: field.comma_token.is_some(),
        };
    }

    fn translate_static_type(&self, typ: &mut StaticType<FeType>) -> ir::GoIRStaticType {
        let is_ptr = typ
            .ref_type
            .as_ref()
            .map(|ref_type| match ref_type {
                RefType::Shared { .. } => false,
                RefType::Mut { .. } => true,
            })
            .unwrap_or(false);

        return ir::GoIRStaticType {
            is_ptr,
            static_path: Self::translate_static_path(&mut typ.static_path),
        };
    }

    fn translate_static_path(path: &mut StaticPath<FeType>) -> ir::GoIRStaticPath {
        if path.root.is_none()
            && path.name.lexeme.as_ref() == INT_TYPE_NAME
            && matches!(path.resolved_type, FeType::Number(_))
        {
            return ir::GoIRStaticPath {
                root: None,
                name: "i64".into(),
            };
        }

        if path.root.is_none()
            && path.name.lexeme.as_ref() == BOOL_TYPE_NAME
            && matches!(path.resolved_type, FeType::Bool(_))
        {
            return ir::GoIRStaticPath {
                root: None,
                name: "bool".into(),
            };
        }

        return ir::GoIRStaticPath {
            root: path
                .root
                .as_mut()
                .map(|root| Box::new(Self::translate_static_path(root))),
            name: path.name.lexeme.clone(),
        };
    }

    // fn translate_use_static_path(
    //     path: &mut UseStaticPath<FeType>,
    // ) -> Result<Option<ir::GoIRUseStaticPath>> {
    //     match &path.pre {
    //         None | Some(UseStaticPathPre::DoubleColon(_)) if path.name.lexeme.as_ref() == "fe" => {
    //             return Ok(None);
    //         }

    //         _ => {}
    //     }

    //     fn _translate_use_static_path(
    //         path: &mut UseStaticPath<FeType>,
    //     ) -> Result<Option<ir::GoIRUseStaticPath>> {
    //         let next = match &mut path.details {
    //             Either::B(_) => None,

    //             Either::A(UseStaticPathNext::Single(ref mut single)) => {
    //                 let Some(next_path) = _translate_use_static_path(&mut single.path)? else {
    //                     return Ok(None);
    //                 };

    //                 Some(ir::GoIRUseStaticPathNext::Single(
    //                     ir::GoIRUseStaticPathNextSingle {
    //                         path: Box::new(next_path),
    //                     },
    //                 ))
    //             }

    //             Either::A(UseStaticPathNext::Many(_many)) => todo!(),
    //         };

    //         let path_ir = ir::GoIRUseStaticPath {
    //             pre: path.pre.as_ref().map(|pre| match pre {
    //                 UseStaticPathPre::DoubleColon(_) => ir::GoIRUseStaticPathPre::DoubleColon,
    //                 UseStaticPathPre::CurrentDir(_) => ir::GoIRUseStaticPathPre::CurrentDir,
    //                 UseStaticPathPre::RootDir(_) => ir::GoIRUseStaticPathPre::RootDir,
    //             }),
    //             name: path.name.lexeme.clone(),
    //             next,
    //         };

    //         return Ok(Some(path_ir));
    //     }

    //     return _translate_use_static_path(path);
    // }

    fn map_label(&self, id: String, label: &Option<Arc<Token>>) -> Option<Arc<str>> {
        return label
            .as_ref()
            .map(|l| format!("'label_{}_{}", id, &l.lexeme[1..]).into());
    }
}
