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
            out: ir::RustIR { files: vec![] },
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
}

impl DeclVisitor<Result> for RustSyntaxCompiler {
    fn visit_function_decl(&mut self, decl: &mut FnDecl) -> Result {
        todo!();
    }
}
