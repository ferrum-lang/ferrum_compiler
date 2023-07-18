use super::*;

use crate::ir::{self, RustIRDeclAccept, RustIRExprAccept, RustIRStmtAccept, RustIRUseAccept};

use std::sync::Arc;

pub struct RustCodeGen {
    entry: Rc<RefCell<ir::RustIR>>,
    out: RustCode,

    indent: usize,
}

#[derive(Debug, Clone)]
pub struct RustCode {
    pub files: Vec<RustCodeFile>,
}

#[derive(Debug, Clone)]
pub struct RustCodeFile {
    pub content: Arc<str>,
}

impl IRToCode for ir::RustIR {
    type Code = RustCode;
}

impl CodeGen<ir::RustIR> for RustCodeGen {
    fn generate_code(entry: Rc<RefCell<ir::RustIR>>) -> Result<RustCode> {
        return Self::new(entry).generate();
    }
}

impl RustCodeGen {
    fn new(entry: Rc<RefCell<ir::RustIR>>) -> Self {
        return Self {
            entry,
            out: RustCode { files: vec![] },

            indent: 0,
        };
    }

    fn generate(mut self) -> Result<RustCode> {
        let entry = self.entry.clone();
        let file = &mut entry.borrow_mut().files[0];

        let mut content = String::new();

        for use_decl in &mut file.uses {
            let code = use_decl.accept(&mut self)?;
            content.push_str(&code);
            content.push_str(&self.new_line());
        }

        content.push('\n');

        for decl in &mut file.decls {
            let code = decl.accept(&mut self)?;
            content.push_str(&code);
            content.push_str(&self.new_line());
        }

        self.out.files.push(RustCodeFile {
            content: content.into(),
        });

        return Ok(self.out);
    }

    fn gen_use_path(&mut self, use_path: &mut ir::RustIRUseStaticPath) -> Result<Arc<str>> {
        let mut out = format!("{}", use_path.name);

        match &mut use_path.next {
            Some(ir::RustIRUseStaticPathNext::Single(single)) => {
                out.push_str("::");

                let code = self.gen_use_path(&mut single.path)?;
                out.push_str(&code);
            }

            Some(ir::RustIRUseStaticPathNext::Many(many)) => {
                todo!()
            }

            None => {}
        }

        return Ok(out.into());
    }

    fn translate_static_type(
        &mut self,
        static_type: &mut ir::RustIRStaticType,
    ) -> Result<Arc<str>> {
        // TODO

        return Ok("".into());
    }

    fn new_line(&self) -> String {
        let mut out = format!("\n");

        for _ in 0..self.indent {
            out.push_str("    ");
        }

        return out;
    }
}

impl ir::RustIRUseVisitor<Result<Arc<str>>> for RustCodeGen {
    fn visit_use(&mut self, use_decl: &mut ir::RustIRUse) -> Result<Arc<str>> {
        let mut out = String::new();

        if let Some(use_mod) = &use_decl.use_mod {
            match use_mod {
                ir::RustIRUseMod::Pub => out.push_str("pub "),
            }
        }

        out.push_str("use ");

        if use_decl.pre_double_colon {
            out.push_str("::");
        }

        let use_path_code = self.gen_use_path(&mut use_decl.path)?;
        out.push_str(&use_path_code);

        out.push(';');

        return Ok(out.into());
    }
}

impl ir::RustIRDeclVisitor<Result<Arc<str>>> for RustCodeGen {
    fn visit_fn_decl(&mut self, decl: &mut ir::RustIRFnDecl) -> Result<Arc<str>> {
        let mut out = String::new();

        // TODO: Handle proc-macros

        match &decl.decl_mod {
            Some(ir::RustIRDeclMod::Pub) => out.push_str("pub "),

            None => {}
        }

        if decl.is_async {
            out.push_str("async ");
        }

        // TODO: Handle generics

        out.push_str(&format!("fn {}(", decl.name));

        for param in &decl.params {
            // TODO: Handle params
        }

        out.push_str(") ");

        if let Some(return_type) = &mut decl.return_type {
            out.push_str("-> ");

            let code = self.translate_static_type(return_type)?;
            out.push_str(&code);

            out.push(' ');
        }

        let code = decl.body.accept(self)?;
        out.push_str(&code);

        return Ok(out.into());
    }
}

impl ir::RustIRStmtVisitor<Result<Arc<str>>> for RustCodeGen {
    fn visit_expr_stmt(&mut self, stmt: &mut ir::RustIRExprStmt) -> Result<Arc<str>> {
        let mut out = String::new();

        let code = stmt.expr.accept(self)?;
        out.push_str(&code);

        out.push(';');

        return Ok(out.into());
    }
}

impl ir::RustIRExprVisitor<Result<Arc<str>>> for RustCodeGen {
    fn visit_ident_expr(&mut self, expr: &mut ir::RustIRIdentExpr) -> Result<Arc<str>> {
        return Ok(expr.ident.clone());
    }

    fn visit_call_expr(&mut self, expr: &mut ir::RustIRCallExpr) -> Result<Arc<str>> {
        let mut out = String::new();

        let code = expr.callee.accept(self)?;
        out.push_str(&code);

        out.push('(');

        let args_code = expr
            .args
            .iter_mut()
            .map(|arg| arg.accept(self))
            .collect::<Result<Vec<Arc<str>>>>()?
            .join(", ");

        out.push_str(&args_code);

        out.push(')');

        return Ok(out.into());
    }

    fn visit_block_expr(&mut self, expr: &mut ir::RustIRBlockExpr) -> Result<Arc<str>> {
        let mut out = String::new();

        self.indent += 1;
        out.push('{');
        out.push_str(&self.new_line());

        let stmts_code = expr
            .stmts
            .iter_mut()
            .map(|stmt| stmt.accept(self))
            .collect::<Result<Vec<Arc<str>>>>()?
            .join(&self.new_line());
        out.push_str(&stmts_code);

        self.indent -= 1;
        out.push_str(&self.new_line());
        out.push('}');

        return Ok(out.into());
    }

    fn visit_string_literal_expr(
        &mut self,
        expr: &mut ir::RustIRStringLiteralExpr,
    ) -> Result<Arc<str>> {
        return Ok(expr.literal.clone());
    }
}
