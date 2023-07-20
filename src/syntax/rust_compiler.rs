use super::*;

use crate::ir;

use std::cell::RefCell;
use std::rc::Rc;

pub struct RustSyntaxCompiler {
    entry: Rc<RefCell<FeSyntaxPackage>>,
    out: ir::RustIR,
}

impl SyntaxCompiler<ir::RustIR> for RustSyntaxCompiler {
    fn compile_package(entry: Rc<RefCell<FeSyntaxPackage>>) -> Result<ir::RustIR> {
        return Self::new(entry).compile();
    }
}

impl RustSyntaxCompiler {
    fn new(entry: Rc<RefCell<FeSyntaxPackage>>) -> Self {
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
        self.compile_package(&mut Rc::clone(&self.entry).borrow_mut())?;

        return Ok(self.out);
    }

    fn compile_package(&mut self, package: &mut FeSyntaxPackage) -> Result {
        match package {
            FeSyntaxPackage::File(file) => {
                self.compile_file(file)?;
            }

            FeSyntaxPackage::Dir(dir) => {
                self.compile_file(&mut dir.entry_file)?;

                for (_name, package) in &dir.local_packages {
                    self.compile_package(&mut package.borrow_mut())?;
                }
            }
        };

        return Ok(());
    }

    fn compile_file(&mut self, file: &mut FeSyntaxFile) -> Result {
        let mut syntax = file.syntax.borrow_mut();

        for use_decl in &mut syntax.uses {
            use_decl.borrow_mut().accept(self)?;
        }

        for decl in &mut syntax.decls {
            decl.borrow_mut().accept(self)?;
        }

        return Ok(());
    }

    fn translate_decl_mod(&self, decl_mod: &DeclMod) -> ir::RustIRDeclMod {
        match decl_mod {
            DeclMod::Pub(_) => return ir::RustIRDeclMod::Pub,
        }
    }

    fn translate_fn_param(&mut self, param: &mut FnDeclParam) -> ir::RustIRFnParam {
        todo!();
    }

    fn translate_fn_return_type(
        &mut self,
        return_type: &mut FnDeclReturnType,
    ) -> Result<ir::RustIRStaticType> {
        todo!();
    }

    fn translate_fn_body(&mut self, body: &mut FnDeclBody) -> Result<ir::RustIRBlockExpr> {
        let mut block_ir = ir::RustIRBlockExpr { stmts: vec![] };

        match body {
            FnDeclBody::Short(short) => todo!(),
            FnDeclBody::Block(block) => {
                for stmt in &mut block.stmts {
                    let stmt_ir = stmt.accept(self)?;

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
        path: &mut UseStaticPath,
    ) -> Result<ir::RustIRUseStaticPath> {
        let next = match &mut path.next {
            Some(UseStaticPathNext::Single(ref mut single)) => Some(
                ir::RustIRUseStaticPathNext::Single(ir::RustIRUseStaticPathNextSingle {
                    path: Box::new(self.translate_use_static_path(&mut single.path)?),
                }),
            ),

            Some(UseStaticPathNext::Many(many)) => todo!(),

            None => None,
        };

        let path_ir = ir::RustIRUseStaticPath {
            name: path.name.lexeme.clone(),
            next,
        };

        return Ok(path_ir);
    }
}

impl UseVisitor<Result> for RustSyntaxCompiler {
    fn visit_use(&mut self, use_decl: &mut Use) -> Result {
        let use_mod = use_decl
            .use_mod
            .as_ref()
            .map(|use_mod| self.translate_use_mod(use_mod));

        let use_ir = ir::RustIRUse {
            use_mod,
            pre_double_colon: use_decl.pre_double_colon_token.is_some(),
            path: self.translate_use_static_path(&mut use_decl.path)?,
        };

        // Future Snowy's problem :D
        self.out.files[0].uses.push(use_ir);

        return Ok(());
    }
}

impl DeclVisitor<Result> for RustSyntaxCompiler {
    fn visit_function_decl(&mut self, decl: &mut FnDecl) -> Result {
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

        self.out.files[0].decls.push(ir::RustIRDecl::Fn(fn_ir));

        return Ok(());
    }
}

impl StmtVisitor<Result<Vec<ir::RustIRStmt>>> for RustSyntaxCompiler {
    fn visit_expr_stmt(&mut self, stmt: &mut ExprStmt) -> Result<Vec<ir::RustIRStmt>> {
        let expr = stmt.expr.accept(self)?;

        return Ok(vec![ir::RustIRStmt::Expr(ir::RustIRExprStmt { expr })]);
    }
}

impl ExprVisitor<Result<ir::RustIRExpr>> for RustSyntaxCompiler {
    fn visit_ident_expr(&mut self, expr: &mut IdentExpr) -> Result<ir::RustIRExpr> {
        return Ok(ir::RustIRExpr::Ident(ir::RustIRIdentExpr {
            ident: expr.ident.lexeme.clone(),
        }));
    }

    fn visit_call_expr(&mut self, expr: &mut CallExpr) -> Result<ir::RustIRExpr> {
        let callee = Box::new(expr.callee.accept(self)?);

        let mut args = vec![];

        // TODO: Handle named, variadic, optional, etc params
        for arg in &mut expr.args {
            let arg_ir = arg.value.accept(self)?;

            args.push(arg_ir);
        }

        return Ok(ir::RustIRExpr::Call(ir::RustIRCallExpr { callee, args }));
    }

    fn visit_string_literal_expr(
        &mut self,
        expr: &mut StringLiteralExpr,
    ) -> Result<ir::RustIRExpr> {
        return Ok(ir::RustIRExpr::StringLiteral(ir::RustIRStringLiteralExpr {
            literal: expr.literal.lexeme.clone(),
        }));
    }
}
