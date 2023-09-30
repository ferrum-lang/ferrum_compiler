use super::*;

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
