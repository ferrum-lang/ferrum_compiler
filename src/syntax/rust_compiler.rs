use super::*;

use crate::ir;

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
                    uses: vec![],
                    decls: vec![],
                }],
            },
        };
    }

    fn compile(mut self) -> Result<ir::RustIR> {
        self.compile_package(&mut Arc::clone(&self.entry).lock().unwrap())?;

        return Ok(self.out);
    }

    fn compile_package(&mut self, package: &mut FeSyntaxPackage<FeType>) -> Result {
        match package {
            FeSyntaxPackage::File(file) => {
                self.compile_file(file)?;
            }

            FeSyntaxPackage::Dir(dir) => {
                self.compile_file(&mut dir.entry_file)?;

                for (name, package) in &dir.local_packages {
                    self.out.files.push(ir::RustIRFile {
                        path: format!("./{}.rs", name.0).into(),
                        uses: vec![],
                        decls: vec![],
                    });
                    self.compile_package(&mut package.lock().unwrap())?;
                }
            }
        };

        return Ok(());
    }

    fn compile_file(&mut self, file: &mut FeSyntaxFile<FeType>) -> Result {
        let mut syntax = file.syntax.lock().unwrap();

        for use_decl in &mut syntax.uses {
            use_decl.lock().unwrap().accept(self)?;
        }

        for decl in &mut syntax.decls {
            decl.lock().unwrap().accept(self)?;
        }

        return Ok(());
    }

    fn translate_decl_mod(&self, decl_mod: &DeclMod) -> ir::RustIRDeclMod {
        match decl_mod {
            DeclMod::Pub(_) => return ir::RustIRDeclMod::Pub,
        }
    }

    fn translate_fn_param(&mut self, param: &mut FnDeclParam<FeType>) -> ir::RustIRFnParam {
        todo!();
    }

    fn translate_fn_return_type(
        &mut self,
        return_type: &mut FnDeclReturnType<FeType>,
    ) -> Result<ir::RustIRStaticType> {
        todo!();
    }

    fn translate_fn_body(&mut self, body: &mut FnDeclBody<FeType>) -> Result<ir::RustIRBlockExpr> {
        let mut block_ir = ir::RustIRBlockExpr { stmts: vec![] };

        match body {
            FnDeclBody::Short(short) => todo!(),
            FnDeclBody::Block(block) => {
                for stmt in &mut block.stmts {
                    let stmt_ir = stmt.lock().unwrap().accept(self)?;

                    block_ir.stmts.extend(stmt_ir);
                }
            }
        }

        return Ok(block_ir);
    }

    fn translate_use_mod(&self, use_mod: &UseMod) -> ir::RustIRUseMod {
        match use_mod {
            UseMod::Pub(_) => ir::RustIRUseMod::Pub,
        }
    }

    fn translate_use_static_path(
        &mut self,
        path: &mut UseStaticPath<FeType>,
    ) -> Result<ir::RustIRUseStaticPath> {
        let next = match &mut path.details {
            Either::A(UseStaticPathNext::Single(ref mut single)) => Some(
                ir::RustIRUseStaticPathNext::Single(ir::RustIRUseStaticPathNextSingle {
                    path: Box::new(self.translate_use_static_path(&mut single.path)?),
                }),
            ),

            Either::A(UseStaticPathNext::Many(many)) => todo!(),

            Either::B(_) => None,
        };

        let path_ir = ir::RustIRUseStaticPath {
            pre: path.pre.as_ref().map(|pre| match pre {
                PreUse::DoubleColon(_) => ir::RustIRUseStaticPathPre::DoubleColon,
                PreUse::CurrentDir(_) => ir::RustIRUseStaticPathPre::CurrentDir,
                PreUse::RootDir(_) => ir::RustIRUseStaticPathPre::RootDir,
            }),
            name: path.name.lexeme.clone(),
            next,
        };

        return Ok(path_ir);
    }
}

impl UseVisitor<FeType, Result> for RustSyntaxCompiler {
    fn visit_use(&mut self, use_decl: &mut Use<FeType>) -> Result {
        let use_mod = use_decl
            .use_mod
            .as_ref()
            .map(|use_mod| self.translate_use_mod(use_mod));

        let use_ir = ir::RustIRUse {
            use_mod,
            path: self.translate_use_static_path(&mut use_decl.path)?,
        };

        let file_idx = self.out.files.len() - 1;
        self.out.files[file_idx].uses.push(use_ir);

        return Ok(());
    }
}

impl DeclVisitor<FeType, Result> for RustSyntaxCompiler {
    fn visit_function_decl(&mut self, decl: &mut FnDecl<FeType>) -> Result {
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

            return_type: if let Some(return_type) = &mut decl.return_type {
                Some(self.translate_fn_return_type(return_type)?)
            } else {
                None
            },

            body: self.translate_fn_body(&mut decl.body)?,
        };

        let file_idx = self.out.files.len() - 1;
        self.out.files[file_idx]
            .decls
            .push(ir::RustIRDecl::Fn(fn_ir));

        return Ok(());
    }
}

impl StmtVisitor<FeType, Result<Vec<ir::RustIRStmt>>> for RustSyntaxCompiler {
    fn visit_expr_stmt(&mut self, stmt: &mut ExprStmt<FeType>) -> Result<Vec<ir::RustIRStmt>> {
        let expr = stmt.expr.lock().unwrap().accept(self)?;

        return Ok(vec![ir::RustIRStmt::Expr(ir::RustIRExprStmt { expr })]);
    }
}

impl ExprVisitor<FeType, Result<ir::RustIRExpr>> for RustSyntaxCompiler {
    fn visit_ident_expr(&mut self, expr: &mut IdentExpr<FeType>) -> Result<ir::RustIRExpr> {
        return Ok(ir::RustIRExpr::Ident(ir::RustIRIdentExpr {
            ident: expr.ident.lexeme.clone(),
        }));
    }

    fn visit_call_expr(&mut self, expr: &mut CallExpr<FeType>) -> Result<ir::RustIRExpr> {
        let mut callee = expr.callee.0.lock().unwrap();
        let callee = Box::new(callee.accept(self)?);

        let mut args = vec![];

        // TODO: Handle named, variadic, optional, etc params
        for arg in &mut expr.args {
            let mut value = arg.value.0.lock().unwrap();
            let arg_ir = value.accept(self)?;

            args.push(arg_ir);
        }

        return Ok(ir::RustIRExpr::Call(ir::RustIRCallExpr { callee, args }));
    }

    fn visit_plain_string_literal_expr(
        &mut self,
        expr: &mut PlainStringLiteralExpr<FeType>,
    ) -> Result<ir::RustIRExpr> {
        return Ok(ir::RustIRExpr::StringLiteral(ir::RustIRStringLiteralExpr {
            literal: expr.literal.lexeme.clone(),
        }));
    }
}
