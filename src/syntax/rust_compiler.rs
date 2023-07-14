use super::*;

use crate::ir;

use std::cell::RefCell;
use std::rc::Rc;

pub struct RustSyntaxCompiler {
    entry: Rc<RefCell<FePackage>>,
    out: ir::RustIR,
}

impl SyntaxCompiler<ir::RustIR> for RustSyntaxCompiler {
    fn compile_package(entry: Rc<RefCell<FePackage>>) -> Result<ir::RustIR> {
        return Self::new(entry).compile();
    }
}

impl RustSyntaxCompiler {
    fn new(entry: Rc<RefCell<FePackage>>) -> Self {
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

    fn compile_package(&mut self, package: &mut FePackage) -> Result {
        match package {
            FePackage::File(file) => {
                self.compile_file(file)?;
            }

            FePackage::Dir(dir) => {
                self.compile_file(&mut dir.entry_file)?;

                for (_name, package) in &dir.local_packages {
                    self.compile_package(&mut package.borrow_mut())?;
                }
            }
        };

        return Ok(());
    }

    fn compile_file(&mut self, file: &mut FeFile) -> Result {
        for decl in &mut file.syntax.borrow_mut().decls {
            decl.borrow_mut().accept(self)?;
        }

        return Ok(());
    }

    fn translate_decl_mod(&self, decl_mod: &DeclMod) -> ir::RustIRDeclMod {
        match decl_mod {
            DeclMod::Pub(_) => return ir::RustIRDeclMod::Pub,
        }
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
            params: vec![],                              // TODO
            return_type: None,                           // TODO
            body: ir::RustIRCodeBlock { stmts: vec![] }, // TODO
        };

        self.out.files[0].decls.push(ir::RustIRDecl::Fn(fn_ir));

        return Ok(());
    }
}

impl StmtVisitor<Result> for RustSyntaxCompiler {
    fn visit_expr_stmt(&mut self, stmt: &mut ExprStmt) -> Result {
        todo!();
    }
}

impl ExprVisitor<Result> for RustSyntaxCompiler {
    fn visit_ident_expr(&mut self, expr: &mut IdentExpr) -> Result {
        todo!();
    }

    fn visit_call_expr(&mut self, expr: &mut CallExpr) -> Result {
        todo!();
    }

    fn visit_string_literal_expr(&mut self, expr: &mut StringLiteralExpr) -> Result {
        todo!();
    }
}
