use super::*;

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
            ir::RustIRBinaryOp::Subtract => out.push('-'),
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
            ir::RustIRAssignOp::MinusEq => out.push_str("-="),
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
