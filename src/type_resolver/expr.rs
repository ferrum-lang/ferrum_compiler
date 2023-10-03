use super::*;

impl ExprVisitor<Option<FeType>, Result<bool>> for FeTypeResolver {
    fn visit_bool_literal_expr(
        &mut self,
        shared_expr: Arc<Mutex<BoolLiteralExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let mut expr = shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let details = match &expr.literal.token_type {
            TokenType::True => Some(true),
            TokenType::False => Some(false),
            _ => None,
        };

        expr.resolved_type = Some(FeType::Bool(details));

        return Ok(true);
    }

    fn visit_number_literal_expr(
        &mut self,
        shared_expr: Arc<Mutex<NumberLiteralExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let mut expr = shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        expr.resolved_type = Some(FeType::Number(Some(match expr.details {
            NumberLiteralDetails::Integer(val) => NumberDetails::Integer(Some(val as i64)),
            NumberLiteralDetails::Decimal(val) => NumberDetails::Decimal(Some(val)),
        })));

        return Ok(true);
    }

    fn visit_plain_string_literal_expr(
        &mut self,
        shared_expr: Arc<Mutex<PlainStringLiteralExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let mut expr = shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        expr.resolved_type = Some(FeType::String(Some(StringDetails::PlainLiteral)));

        return Ok(true);
    }

    fn visit_fmt_string_literal_expr(
        &mut self,
        shared_expr: Arc<Mutex<FmtStringLiteralExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let mut expr = shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;
        let mut is_all_checked = true;

        for part in &mut expr.rest {
            changed |= part.expr.0.try_lock().unwrap().accept(self)?;

            if !part.expr.0.try_lock().unwrap().is_resolved() {
                is_all_checked = false;
            }
        }

        if is_all_checked {
            expr.resolved_type = Some(FeType::String(Some(StringDetails::Format)));
            changed = true;
        }

        return Ok(changed);
    }

    fn visit_ident_expr(
        &mut self,
        shared_expr: Arc<Mutex<IdentExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let mut expr = shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let ident = &expr.ident.lexeme;

        if let Some(found) = self.scope.try_lock().unwrap().search(ident) {
            expr.resolved_type = Some(found.typ.clone());
            self.expr_lookup.insert(expr.id, found.typ.clone());
        } else {
            return Ok(false);
        }

        return Ok(true);
    }

    fn visit_call_expr(
        &mut self,
        shared_expr: Arc<Mutex<CallExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let expr = &mut *shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        let callee = &mut *expr.callee.0.try_lock().unwrap();

        if !callee.is_resolved() {
            changed |= callee.accept(self)?;
        }

        let Some(resolved_type) = callee.resolved_type().flatten() else {
            return Ok(false);
        };

        let FeType::Callable(callee) = resolved_type else {
            todo!("How to call on ?? {callee:#?}");
        };

        if expr.args.len() > callee.params.len() {
            todo!(
                "too many args!\nExpected: {:#?}\nGot: {:#?}",
                callee.params,
                expr.args
            );
        }

        for i in 0..expr.args.len() {
            let arg = &mut expr.args[i];

            if !arg.is_resolved() {
                let expr = &mut *arg.value.0.try_lock().unwrap();
                let local_changed = expr.accept(self)?;

                if !local_changed {
                    continue;
                }
                changed = true;

                let Some(resolved_type) = expr.resolved_type() else {
                    continue;
                };

                arg.resolved_type = resolved_type.clone();
            }

            let Some(resolved_type) = &arg.resolved_type else {
                todo!("How did this get here??")
            };
            let (_, param) = &callee.params[i];

            if !Self::can_implicit_cast(resolved_type, param) {
                todo!("wrong type!\nCannot implicitly cast {resolved_type:#?}\nto {param:#?}");
            }
        }

        expr.resolved_type = callee.return_type.as_deref().map(|rt| Some(rt.clone()));

        return Ok(changed);
    }

    fn visit_unary_expr(
        &mut self,
        shared_expr: Arc<Mutex<UnaryExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let expr = &mut *shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        changed |= expr.value.0.try_lock().unwrap().accept(self)?;

        if let Some(resolved_type) = expr.value.0.try_lock().unwrap().resolved_type().flatten() {
            changed = true;

            match &expr.op {
                UnaryOp::Ref(RefType::Shared { .. }) => {
                    expr.resolved_type = Some(FeType::Ref(FeRefOf {
                        ref_type: FeRefType::Const,
                        of: Box::new(resolved_type),
                    }));
                }
                UnaryOp::Ref(RefType::Mut { .. }) => {
                    expr.resolved_type = Some(FeType::Ref(FeRefOf {
                        ref_type: FeRefType::Mut,
                        of: Box::new(resolved_type),
                    }));
                }
                UnaryOp::Not(_) => {
                    // Only apply to bool
                    if !Self::can_implicit_cast(&resolved_type, &FeType::Bool(None)) {
                        todo!("Can't cast implicitly to bool");
                    }

                    expr.resolved_type = if let FeType::Bool(details) = &resolved_type {
                        Some(FeType::Bool(details.map(|known_val| !known_val)))
                    } else {
                        Some(FeType::Bool(None))
                    };
                }
            }
        }

        return Ok(changed);
    }

    fn visit_binary_expr(
        &mut self,
        shared_expr: Arc<Mutex<BinaryExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let expr = &mut *shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        changed |= expr.lhs.0.try_lock().unwrap().accept(self)?;
        changed |= expr.rhs.0.try_lock().unwrap().accept(self)?;

        if let (Some(resolved_lhs), Some(resolved_rhs)) = (
            expr.lhs.0.try_lock().unwrap().resolved_type().flatten(),
            expr.rhs.0.try_lock().unwrap().resolved_type().flatten(),
        ) {
            changed = true;

            match &expr.op {
                BinaryOp::Less(_)
                | BinaryOp::LessEq(_)
                | BinaryOp::Greater(_)
                | BinaryOp::GreaterEq(_) => {
                    let resolved_lhs = resolved_lhs.actual_type();
                    let resolved_rhs = resolved_rhs.actual_type();

                    if matches!(
                        (resolved_lhs, resolved_rhs),
                        (FeType::Number(_), FeType::Number(_))
                    ) {
                        expr.resolved_type = Some(FeType::Bool(None));
                    } else {
                        todo!();
                    }
                }

                BinaryOp::Add(_) => {
                    let resolved_lhs = resolved_lhs.actual_type();
                    let resolved_rhs = resolved_rhs.actual_type();

                    match (resolved_lhs, resolved_rhs) {
                        (FeType::Number(lhs), FeType::Number(rhs)) => match (lhs, rhs) {
                            // known values at compile time
                            (
                                Some(NumberDetails::Integer(Some(lhs))),
                                Some(NumberDetails::Integer(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Integer(Some(*lhs + *rhs)),
                                )));
                            }
                            (
                                Some(NumberDetails::Decimal(Some(lhs))),
                                Some(NumberDetails::Integer(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Decimal(Some(*lhs + *rhs as f64)),
                                )));
                            }
                            (
                                Some(NumberDetails::Integer(Some(lhs))),
                                Some(NumberDetails::Decimal(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Decimal(Some(*lhs as f64 + *rhs)),
                                )));
                            }
                            (
                                Some(NumberDetails::Decimal(Some(lhs))),
                                Some(NumberDetails::Decimal(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Decimal(Some(*lhs + *rhs)),
                                )));
                            }

                            // unknown values, known types
                            (
                                Some(NumberDetails::Integer(_)),
                                Some(NumberDetails::Integer(None)),
                            )
                            | (
                                Some(NumberDetails::Integer(None)),
                                Some(NumberDetails::Integer(_)),
                            ) => {
                                expr.resolved_type =
                                    Some(FeType::Number(Some(NumberDetails::Integer(None))));
                            }
                            (Some(NumberDetails::Decimal(_)), _) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    // TODO: we know this is greater-than lhs val
                                    NumberDetails::Decimal(None),
                                )));
                            }
                            (_, Some(NumberDetails::Decimal(_))) => {
                                expr.resolved_type =
                                    Some(FeType::Number(Some(NumberDetails::Decimal(None))))
                            }

                            // other
                            (_, None) | (None, _) => {
                                expr.resolved_type = Some(FeType::Number(None));
                            }
                        },
                        _ => todo!("unsure how to add {resolved_lhs:#?} to {resolved_rhs:#?}"),
                    }
                }

                BinaryOp::Subtract(_) => {
                    let resolved_lhs = resolved_lhs.actual_type();
                    let resolved_rhs = resolved_rhs.actual_type();

                    match (resolved_lhs, resolved_rhs) {
                        (FeType::Number(lhs), FeType::Number(rhs)) => match (lhs, rhs) {
                            // known values at compile time
                            (
                                Some(NumberDetails::Integer(Some(lhs))),
                                Some(NumberDetails::Integer(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Integer(Some(*lhs - *rhs)),
                                )));
                            }
                            (
                                Some(NumberDetails::Decimal(Some(lhs))),
                                Some(NumberDetails::Integer(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Decimal(Some(*lhs - *rhs as f64)),
                                )));
                            }
                            (
                                Some(NumberDetails::Integer(Some(lhs))),
                                Some(NumberDetails::Decimal(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Decimal(Some(*lhs as f64 - *rhs)),
                                )));
                            }
                            (
                                Some(NumberDetails::Decimal(Some(lhs))),
                                Some(NumberDetails::Decimal(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Decimal(Some(*lhs - *rhs)),
                                )));
                            }

                            // unknown values, known types
                            (
                                Some(NumberDetails::Integer(_)),
                                Some(NumberDetails::Integer(None)),
                            )
                            | (
                                Some(NumberDetails::Integer(None)),
                                Some(NumberDetails::Integer(_)),
                            ) => {
                                expr.resolved_type =
                                    Some(FeType::Number(Some(NumberDetails::Integer(None))));
                            }
                            (Some(NumberDetails::Decimal(_)), _) => {
                                expr.resolved_type =
                                    Some(FeType::Number(Some(NumberDetails::Decimal(None))));
                            }
                            (_, Some(NumberDetails::Decimal(_))) => {
                                expr.resolved_type =
                                    Some(FeType::Number(Some(NumberDetails::Decimal(None))))
                            }

                            // other
                            (_, None) | (None, _) => {
                                expr.resolved_type = Some(FeType::Number(None));
                            }
                        },
                        _ => todo!("unsure how to add {resolved_lhs:#?} to {resolved_rhs:#?}"),
                    }
                }
            }
        }

        if !changed {
            todo!("determine lhs or rhs error");
        }

        return Ok(changed);
    }

    fn visit_static_ref_expr(
        &mut self,
        shared_expr: Arc<Mutex<StaticRefExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let expr = &mut *shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        changed |= expr.static_path.accept(self)?;

        if let Some(resolved_type) = &expr.static_path.resolved_type {
            expr.resolved_type = Some(resolved_type.clone());
        }

        return Ok(changed);
    }

    fn visit_construct_expr(
        &mut self,
        shared_expr: Arc<Mutex<ConstructExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let mut expr = shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;
        let mut target = None;

        match &mut expr.target {
            ConstructTarget::Ident(ident) => {
                changed |= ident.accept(self)?;

                if let Some(resolved) = &ident.try_lock().unwrap().resolved_type {
                    target = Some(resolved.clone());
                }
            }
            ConstructTarget::StaticPath(path) => {
                changed |= path.accept(self)?;

                if let Some(resolved) = &path.resolved_type {
                    target = Some(resolved.clone());
                }
            }
        }

        if let Some(target) = target {
            let FeType::Struct(target) = target else {
                todo!("Can't construct type {target:#?}");
            };

            let fields_map = target
                .fields
                .iter()
                .cloned()
                .map(|f| (f.name.clone(), f))
                .collect::<HashMap<Arc<str>, FeStructField>>();

            let mut seen = HashSet::new();

            for arg in &mut expr.args {
                match arg {
                    ConstructArg::Field(field) => {
                        changed |= field.value.0.try_lock().unwrap().accept(self)?;

                        let Some(struct_field) = fields_map.get(&field.name.lexeme) else {
                            todo!(
                                "No field found with name {:?} for struct {:?}",
                                field.name.lexeme,
                                target.name
                            );
                        };

                        if seen.contains(&field.name.lexeme) {
                            todo!("Duplicate arg! {field:#?}");
                        }

                        seen.insert(field.name.lexeme.clone());

                        if let Some(resolved) =
                            field.value.0.try_lock().unwrap().resolved_type().flatten()
                        {
                            if !Self::can_implicit_cast(&resolved, &struct_field.typ) {
                                todo!("Invalid type! {resolved:#?}");
                            }
                        }
                    }
                }
            }

            let leftover_fields = fields_map.into_iter().filter_map(|(name, field)| {
                if seen.contains(&name) {
                    return None;
                }

                return Some(field);
            });

            for field in leftover_fields {
                // TODO: Check for default or optional

                todo!("Field not instantiated! {field:#?}");
            }

            expr.resolved_type = Some(FeType::Instance(FeInstance {
                special: None,
                name: target.name,
                fields: target
                    .fields
                    .into_iter()
                    .map(|f| (f.name.clone(), f))
                    .collect(),
            }));
        }

        return Ok(changed);
    }

    fn visit_get_expr(&mut self, shared_expr: Arc<Mutex<GetExpr<Option<FeType>>>>) -> Result<bool> {
        let expr = &mut *shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        changed |= expr.target.0.try_lock().unwrap().accept(self)?;

        if let Some(resolved) = expr.target.0.try_lock().unwrap().resolved_type().flatten() {
            // TODO: I don't love this, what if theres a shared ref of a mut ref or something weird?
            let Some(instance) = resolved.instance() else {
                todo!("How can you get a property of a non-instance? Maybe the type system needs reworking... {resolved:#?}");
            };

            // TODO: methods?

            let Some(field) = instance.fields.get(&expr.name.lexeme).cloned() else {
                todo!(
                    "Couldn't find property {:#?} on instance {:#?}",
                    expr.name,
                    instance
                );
            };

            let resolved = match resolved {
                FeType::Instance(_) => field.typ,
                FeType::Ref(FeRefOf { ref_type, .. }) => FeType::Ref(FeRefOf {
                    ref_type,
                    of: Box::new(field.typ),
                }),
                FeType::Owned(FeOwnedOf { owned_mut, .. }) => FeType::Owned(FeOwnedOf {
                    owned_mut,
                    of: Box::new(field.typ),
                }),

                _ => todo!(),
            };

            expr.resolved_type = Some(resolved);

            changed = true;
        }

        return Ok(changed);
    }

    fn visit_if_expr(&mut self, shared_expr: Arc<Mutex<IfExpr<Option<FeType>>>>) -> Result<bool> {
        {
            let expr = &mut *shared_expr.try_lock().unwrap();

            if expr.is_resolved() {
                return Ok(false);
            }
        }

        let mut changed = false;

        {
            let condition = {
                let expr = &mut *shared_expr.try_lock().unwrap();
                expr.condition.clone()
            };

            let condition = condition.0.try_lock().unwrap();

            if !condition.is_resolved() {
                changed |= condition.accept(self)?;

                let Some(resolved_type) = condition.resolved_type() else {
                    todo!("Can't check if condition on no type!");
                };

                if let Some(resolved_type) = resolved_type {
                    if !Self::can_implicit_cast(&resolved_type, &FeType::Bool(None)) {
                        todo!("Can't cast to bool!");
                    }
                }
            }
        }

        // TODO: if stmt terminals? what to do here?

        let mut typ = None;

        let then = {
            let expr = &mut *shared_expr.try_lock().unwrap();
            expr.then.clone()
        };

        match &then {
            IfExprThen::Ternary(then) => {
                changed |= then.then_expr.0.try_lock().unwrap().accept(self)?;

                typ = then
                    .then_expr
                    .0
                    .try_lock()
                    .unwrap()
                    .resolved_type()
                    .flatten();
            }
            IfExprThen::Block(then) => {
                self.scope
                    .try_lock()
                    .unwrap()
                    .begin_scope(Some(ScopeCreator::IfExpr(
                        IfBlock::Then,
                        shared_expr.clone(),
                    )));

                self.thenable_count += 1;
                let (local_changed, terminal) = self.resolve_stmts(&then.block.stmts)?;
                self.thenable_count -= 1;

                self.scope.try_lock().unwrap().end_scope();

                changed |= local_changed;

                if terminal.is_none() {
                    todo!("TODO: Implicit optional");
                }

                // TODO: Try to determine and check type here if terminal is Then stmt
            }
        }

        let else_ifs = {
            let expr = &mut *shared_expr.try_lock().unwrap();
            expr.else_ifs.clone()
        };

        for (idx, else_if) in else_ifs.iter().enumerate() {
            match else_if {
                IfExprElseIf::Ternary(else_if) => {
                    {
                        let condition = else_if.condition.0.try_lock().unwrap();

                        changed |= condition.accept(self)?;

                        let Some(resolved_type) = condition.resolved_type() else {
                            todo!("Can't check if condition on no type!");
                        };

                        if let Some(resolved_type) = resolved_type {
                            if !Self::can_implicit_cast(&resolved_type, &FeType::Bool(None)) {
                                todo!("Can't cast to bool!");
                            }
                        }
                    }

                    {
                        let expr = else_if.expr.0.try_lock().unwrap();

                        changed |= expr.accept(self)?;

                        let Some(resolved_type) = expr.resolved_type() else {
                            todo!("Can't use no-type as a value");
                        };

                        if let Some(typ) = &typ {
                            if let Some(resolved_type) = resolved_type {
                                if !Self::can_implicit_cast(&resolved_type, typ) {
                                    todo!("Can't cast!");
                                }
                            }
                        } else if let Some(resolved_type) = resolved_type {
                            typ = Some(resolved_type.clone());
                        }
                    }
                }
                IfExprElseIf::Block(else_if) => {
                    self.scope
                        .try_lock()
                        .unwrap()
                        .begin_scope(Some(ScopeCreator::IfExpr(
                            IfBlock::ElseIf(idx),
                            shared_expr.clone(),
                        )));

                    self.thenable_count += 1;
                    let (local_changed, _terminal) = self.resolve_stmts(&else_if.block.stmts)?;
                    self.thenable_count -= 1;

                    self.scope.try_lock().unwrap().end_scope();

                    changed |= local_changed;

                    // TODO: Try to determine and check type here if terminal is Then stmt
                }
            }
        }

        let else_ = {
            let expr = &mut *shared_expr.try_lock().unwrap();
            expr.else_.clone()
        };

        if let Some(else_) = &else_ {
            if !else_.is_resolved() {
                match else_ {
                    IfExprElse::Ternary(else_) => {
                        let else_expr = else_.else_expr.0.try_lock().unwrap();

                        changed |= else_expr.accept(self)?;

                        let Some(resolved_type) = else_expr.resolved_type() else {
                            todo!("Can't use no-type as a value");
                        };

                        if let Some(typ) = &typ {
                            if let Some(resolved_type) = resolved_type {
                                if !Self::can_implicit_cast(&resolved_type, typ) {
                                    todo!("Can't cast!");
                                }
                            }
                        } else if let Some(resolved_type) = resolved_type {
                            typ = Some(resolved_type);
                        }
                    }
                    IfExprElse::Block(else_) => {
                        self.scope
                            .try_lock()
                            .unwrap()
                            .begin_scope(Some(ScopeCreator::IfExpr(
                                IfBlock::Else,
                                shared_expr.clone(),
                            )));

                        self.thenable_count += 1;
                        let (local_changed, _terminal) = self.resolve_stmts(&else_.block.stmts)?;
                        self.thenable_count -= 1;

                        self.scope.try_lock().unwrap().end_scope();

                        changed |= local_changed;

                        // TODO: Try to determine and check type here if terminal is Then stmt
                    }
                }
            }
        } else {
            todo!("TODO: Implicit wrap as optional")
        }

        let expr = &mut *shared_expr.try_lock().unwrap();

        if let Some(Some(already)) = &expr.resolved_type {
            if let Some(typ) = typ {
                if !Self::can_implicit_cast(&typ, already) {
                    todo!();
                }
            }
        } else {
            expr.resolved_type = Some(typ);
        }

        return Ok(changed);
    }

    fn visit_loop_expr(
        &mut self,
        shared_expr: Arc<Mutex<LoopExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        {
            let expr = &mut *shared_expr.try_lock().unwrap();

            // TODO: Think about how looping affects types

            if expr.is_resolved() {
                return Ok(false);
            }
        }

        let mut changed = false;

        let stmts = {
            let expr = &mut *shared_expr.try_lock().unwrap();
            expr.block.stmts.clone()
        };

        self.scope
            .try_lock()
            .unwrap()
            .begin_scope(Some(ScopeCreator::LoopExpr(shared_expr)));

        self.breakable_count += 1;
        let (local_changed, _terminal) = self.resolve_stmts(&stmts)?;
        changed |= local_changed;
        self.breakable_count -= 1;

        self.scope.try_lock().unwrap().end_scope();

        return Ok(changed);
    }

    fn visit_while_expr(
        &mut self,
        shared_expr: Arc<Mutex<WhileExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        {
            let expr = &mut *shared_expr.try_lock().unwrap();

            if expr.is_resolved() {
                return Ok(false);
            }
        }

        let mut changed = false;

        let condition = {
            let expr = &mut *shared_expr.try_lock().unwrap();
            expr.condition.clone()
        };

        changed |= condition.0.try_lock().unwrap().accept(self)?;

        let stmts = {
            let expr = &mut *shared_expr.try_lock().unwrap();
            expr.block.stmts.clone()
        };

        self.scope
            .try_lock()
            .unwrap()
            .begin_scope(Some(ScopeCreator::WhileExpr(shared_expr)));

        self.breakable_count += 1;
        changed |= self.resolve_stmts(&stmts)?.0;
        self.breakable_count -= 1;

        self.scope.try_lock().unwrap().end_scope();

        return Ok(changed);
    }
}
