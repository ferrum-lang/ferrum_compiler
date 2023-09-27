use std::path::PathBuf;

use super::*;

use crate::ir::{
    self, RustIRDeclAccept, RustIRExprAccept, RustIRRefType, RustIRStaticAccept, RustIRStmtAccept,
    RustIRUseAccept,
};

#[derive(Debug, Clone)]
pub struct RustCodeGen {
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
    fn generate_code(rust_ir: Arc<Mutex<ir::RustIR>>) -> Result<RustCode> {
        return Self::new(rust_ir).generate();
    }
}

impl RustCodeGen {
    fn new(entry: Arc<Mutex<ir::RustIR>>) -> Self {
        return Self {
            entry,
            out: RustCode { files: vec![] },

            indent: 0,
        };
    }

    fn generate(mut self) -> Result<RustCode> {
        let entry = self.entry.clone();

        for file in &mut entry.lock().unwrap().files {
            let mut content = String::new();

            for mod_decl in &mut file.mods {
                let code = format!("mod {};", mod_decl);
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

impl ir::RustIRUseVisitor<Result<Arc<str>>> for RustCodeGen {
    fn visit_use(&mut self, use_decl: &mut ir::RustIRUse) -> Result<Arc<str>> {
        let mut out = String::new();

        if let Some(use_mod) = &use_decl.use_mod {
            match use_mod {
                ir::RustIRUseMod::Pub => out.push_str("pub "),
            }
        }

        out.push_str("use ");

        let use_path_code = Self::gen_use_path(&mut use_decl.path)?;
        out.push_str(&use_path_code);

        out.push(';');

        return Ok(out.into());
    }
}

impl ir::RustIRStaticVisitor<Result<Arc<str>>> for RustCodeGen {
    fn visit_static_type(&mut self, static_type: &mut ir::RustIRStaticType) -> Result<Arc<str>> {
        let mut out = String::new();

        match &static_type.ref_type {
            Some(RustIRRefType::Shared) => out.push('&'),
            Some(RustIRRefType::Mut) => out.push_str("&mut "),

            None => {}
        }

        out.push_str(&static_type.static_path.accept(self)?);

        return Ok(out.into());
    }

    fn visit_static_path(&mut self, static_path: &mut ir::RustIRStaticPath) -> Result<Arc<str>> {
        if let Some(root) = &mut static_path.root {
            let code = root.accept(self)?;

            return Ok(format!("{}::{}", code, static_path.name).into());
        } else {
            return Ok(static_path.name.clone());
        }
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

        for mut param in decl.params.clone() {
            out.push_str(&format!("{}: ", param.name));

            out.push_str(&param.static_type_ref.accept(self)?);

            if param.trailing_comma {
                out.push_str(", ");
            }
        }

        out.push_str(") ");

        if let Some(return_type) = &mut decl.return_type {
            out.push_str("-> ");

            let code = self.translate_static_type(return_type);
            out.push_str(&code);

            out.push(' ');
        }

        let code = decl.body.accept(self)?;
        out.push_str(&code);

        return Ok(out.into());
    }

    fn visit_struct_decl(&mut self, decl: &mut ir::RustIRStructDecl) -> Result<Arc<str>> {
        let mut out = String::new();

        match &decl.decl_mod {
            Some(ir::RustIRDeclMod::Pub) => out.push_str("pub "),

            None => {}
        }

        out.push_str("struct ");

        out.push_str(&decl.name);

        out.push_str(" {");

        self.indent += 1;
        out.push_str(&self.new_line());

        let fields_code = decl
            .fields
            .iter_mut()
            .map(|field| {
                let mut out = String::new();

                match &field.field_mod {
                    Some(ir::RustIRStructFieldMod::Pub) => out.push_str("pub "),

                    None => {}
                }

                out.push_str(&field.name);

                out.push_str(": ");

                out.push_str(&field.static_type_ref.accept(self)?);

                if field.trailing_comma {
                    out.push(',');
                }

                Ok(out.into())
            })
            .collect::<Result<Vec<Arc<str>>>>()?
            .join(&self.new_line());
        out.push_str(&fields_code);

        self.indent -= 1;
        out.push_str(&self.new_line());

        out.push('}');

        return Ok(out.into());
    }
}

impl ir::RustIRStmtVisitor<Result<Arc<str>>> for RustCodeGen {
    fn visit_implicit_return_stmt(
        &mut self,
        stmt: &mut ir::RustIRImplicitReturnStmt,
    ) -> Result<Arc<str>> {
        return stmt.expr.accept(self);
    }

    fn visit_expr_stmt(&mut self, stmt: &mut ir::RustIRExprStmt) -> Result<Arc<str>> {
        let mut out = String::new();

        let code = stmt.expr.accept(self)?;
        out.push_str(&code);

        out.push(';');

        return Ok(out.into());
    }

    fn visit_let_stmt(&mut self, stmt: &mut ir::RustIRLetStmt) -> Result<Arc<str>> {
        let mut out = String::from("let ");

        if stmt.is_mut {
            out.push_str("mut ");
        }

        out.push_str(&stmt.name);
        out.push(' ');

        if stmt.explicit_type.is_some() {
            todo!()
        }

        if let Some(value) = &mut stmt.value {
            out.push_str("= ");

            let code = value.expr.accept(self)?;
            out.push_str(&code);
        }

        out.push(';');

        return Ok(out.into());
    }

    fn visit_return_stmt(&mut self, stmt: &mut ir::RustIRReturnStmt) -> Result<Arc<str>> {
        let mut out = String::from("return ");

        if let Some(expr) = &mut stmt.expr {
            out.push_str(&expr.accept(self)?);
        }

        out.push(';');

        return Ok(out.into());
    }

    fn visit_while_stmt(&mut self, stmt: &mut ir::RustIRWhileStmt) -> Result<Arc<str>> {
        let mut out = String::from("while ");

        out.push_str(&stmt.condition.accept(self)?);

        out.push_str(" {");

        self.indent += 1;
        out.push_str(&self.new_line());

        let stmts_code = stmt
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

    fn visit_break_stmt(&mut self, stmt: &mut ir::RustIRBreakStmt) -> Result<Arc<str>> {
        let mut out = String::from("break");

        if let Some(label) = &stmt.label {
            out.push(' ');
            out.push_str(label);
        }

        if let Some(expr) = &mut stmt.expr {
            out.push(' ');
            out.push_str(&expr.accept(self)?);
        }

        out.push(';');

        return Ok(out.into());
    }
}

impl ir::RustIRExprVisitor<Result<Arc<str>>> for RustCodeGen {
    fn visit_bool_literal_expr(
        &mut self,
        expr: &mut ir::RustIRBoolLiteralExpr,
    ) -> Result<Arc<str>> {
        return Ok(expr.literal.to_string().into());
    }

    fn visit_number_literal_expr(
        &mut self,
        expr: &mut ir::RustIRNumberLiteralExpr,
    ) -> Result<Arc<str>> {
        return Ok(expr.literal.clone());
    }

    fn visit_string_literal_expr(
        &mut self,
        expr: &mut ir::RustIRStringLiteralExpr,
    ) -> Result<Arc<str>> {
        return Ok(expr.literal.clone());
    }

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

    fn visit_macro_fn_call_expr(
        &mut self,
        expr: &mut ir::RustIRMacroFnCallExpr,
    ) -> Result<Arc<str>> {
        let mut out = format!("{}!(", expr.callee);

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

    fn visit_static_ref_expr(&mut self, expr: &mut ir::RustIRStaticRefExpr) -> Result<Arc<str>> {
        return expr.static_ref.accept(self);
    }

    fn visit_unary_expr(&mut self, expr: &mut ir::RustIRUnaryExpr) -> Result<Arc<str>> {
        let mut out = String::new();

        match &expr.op {
            ir::RustIRUnaryOp::Ref(RustIRRefType::Shared) => {
                out.push('&');
            }
            ir::RustIRUnaryOp::Ref(RustIRRefType::Mut) => {
                out.push_str("&mut ");
            }
            ir::RustIRUnaryOp::Not => {
                out.push('!');
            }
        }

        out.push_str(&expr.value.accept(self)?);

        return Ok(out.into());
    }

    fn visit_binary_expr(&mut self, expr: &mut ir::RustIRBinaryExpr) -> Result<Arc<str>> {
        let mut out = String::new();

        out.push_str(&expr.lhs.accept(self)?);
        out.push(' ');

        match &expr.op {
            ir::RustIRBinaryOp::Add => out.push('+'),
            ir::RustIRBinaryOp::Less => out.push('<'),
            ir::RustIRBinaryOp::LessEq => out.push_str("<="),
            ir::RustIRBinaryOp::Greater => out.push('>'),
            ir::RustIRBinaryOp::GreaterEq => out.push_str(">="),
        }

        out.push(' ');
        out.push_str(&expr.rhs.accept(self)?);

        return Ok(out.into());
    }

    fn visit_assign_expr(&mut self, expr: &mut ir::RustIRAssignExpr) -> Result<Arc<str>> {
        let mut out = String::new();

        out.push_str(&expr.lhs.accept(self)?);
        out.push(' ');

        match &expr.op {
            ir::RustIRAssignOp::Eq => out.push('='),
            ir::RustIRAssignOp::PlusEq => out.push_str("+="),
        }

        out.push(' ');
        out.push_str(&expr.rhs.accept(self)?);

        return Ok(out.into());
    }

    fn visit_if_expr(&mut self, expr: &mut ir::RustIRIfExpr) -> Result<Arc<str>> {
        let mut out = String::from("if ");
        out.push_str(&expr.condition.accept(self)?);
        out.push_str(" {");

        self.indent += 1;
        out.push_str(&self.new_line());

        let stmts_code = expr
            .then
            .iter_mut()
            .map(|stmt| stmt.accept(self))
            .collect::<Result<Vec<Arc<str>>>>()?
            .join(&self.new_line());
        out.push_str(&stmts_code);

        self.indent -= 1;
        out.push_str(&self.new_line());
        out.push('}');

        for else_if in &mut expr.else_ifs {
            out.push_str(" else if ");
            out.push_str(&else_if.condition.accept(self)?);
            out.push_str(" {");

            self.indent += 1;
            out.push_str(&self.new_line());

            let stmts_code = else_if
                .then
                .iter_mut()
                .map(|stmt| stmt.accept(self))
                .collect::<Result<Vec<Arc<str>>>>()?
                .join(&self.new_line());
            out.push_str(&stmts_code);

            self.indent -= 1;
            out.push_str(&self.new_line());
            out.push('}');
        }

        if let Some(else_) = &mut expr.else_ {
            out.push_str(" else {");

            self.indent += 1;
            out.push_str(&self.new_line());

            let stmts_code = else_
                .then
                .iter_mut()
                .map(|stmt| stmt.accept(self))
                .collect::<Result<Vec<Arc<str>>>>()?
                .join(&self.new_line());
            out.push_str(&stmts_code);

            self.indent -= 1;
            out.push_str(&self.new_line());
            out.push('}');
        }

        return Ok(out.into());
    }

    fn visit_loop_expr(&mut self, expr: &mut ir::RustIRLoopExpr) -> Result<Arc<str>> {
        let mut out = String::new();

        if let Some(label) = &expr.label {
            out.push_str(label);
            out.push_str(": ");
        }

        out.push_str("loop {");

        self.indent += 1;
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

    fn visit_construct_expr(&mut self, expr: &mut ir::RustIRConstructExpr) -> Result<Arc<str>> {
        let mut out = String::new();

        match &mut expr.target {
            ir::RustIRConstructTarget::Ident(ident) => out.push_str(ident),

            ir::RustIRConstructTarget::StaticPath(path) => {
                let code = Self::translate_static_path(path);
                out.push_str(&code);
            }
        }

        out.push_str(" {");

        self.indent += 1;
        out.push_str(&self.new_line());

        let code = expr
            .args
            .iter_mut()
            .map(|arg| {
                let mut out = String::new();

                out.push_str(&arg.name);
                out.push_str(": ");
                out.push_str(&arg.value.accept(self)?);
                out.push(',');

                return Ok(out.into());
            })
            .collect::<Result<Vec<Arc<str>>>>()?
            .join(&self.new_line());
        out.push_str(&code);

        self.indent -= 1;
        out.push_str(&self.new_line());

        out.push('}');

        return Ok(out.into());
    }

    fn visit_get_expr(&mut self, expr: &mut ir::RustIRGetExpr) -> Result<Arc<str>> {
        let mut out = String::new();

        out.push_str(&expr.target.accept(self)?);
        out.push('.');
        out.push_str(&expr.name);

        return Ok(out.into());
    }
}
