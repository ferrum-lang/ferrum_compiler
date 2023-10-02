use super::*;

use crate::log;

impl StmtVisitor<Option<FeType>, Result<bool>> for FeTypeResolver {
    fn visit_expr_stmt(&mut self, stmt: Arc<Mutex<ExprStmt<Option<FeType>>>>) -> Result<bool> {
        return stmt
            .try_lock()
            .unwrap()
            .expr
            .try_lock()
            .unwrap()
            .accept(self);
    }

    fn visit_var_decl_stmt(
        &mut self,
        shared_stmt: Arc<Mutex<VarDeclStmt<Option<FeType>>>>,
    ) -> Result<bool> {
        let stmt = &mut *shared_stmt.try_lock().unwrap();

        let mut changed = false;

        let typ = if let Some(value) = &stmt.value {
            let value = &mut *value.value.0.try_lock().unwrap();

            changed |= value.accept(self)?;

            value.resolved_type().flatten()
        } else {
            None
        };

        // TODO: check explicit types

        if let Some(typ) = typ {
            match &mut stmt.target {
                VarDeclTarget::Ident(ident) => {
                    self.scope.try_lock().unwrap().insert(
                        ident.try_lock().unwrap().ident.lexeme.clone(),
                        ScopedType {
                            is_pub: false,
                            typ: FeType::Owned(FeOwnedOf {
                                owned_mut: match stmt.var_mut {
                                    VarDeclMut::Const(_) => FeOwnedMut::Const,
                                    VarDeclMut::Mut(_) => FeOwnedMut::Mut,
                                },
                                of: Box::new(typ),
                            }),
                        },
                    );

                    changed |= ident.accept(self)?;
                }
            }
        }

        return Ok(changed);
    }

    fn visit_assign_stmt(
        &mut self,
        shared_stmt: Arc<Mutex<AssignStmt<Option<FeType>>>>,
    ) -> Result<bool> {
        let stmt = &mut *shared_stmt.try_lock().unwrap();

        if stmt.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;
        let mut types = (None, None);

        {
            let target = &mut *stmt.target.0.try_lock().unwrap();
            changed |= target.accept(self)?;

            // TODO: ensure LHS expr is assignable (unassigned const ident || mut ident || instance_ref)

            types.0 = target.resolved_type().flatten();

            if let Some(resolved_type) = &types.0 {
                match resolved_type {
                    FeType::Ref(ref_of) => {
                        if ref_of.ref_type != FeRefType::Mut {
                            // TODO: handle assigning late to non-assigned const ref
                            todo!("Reference is not mutable: {:#?}", target);
                        }
                    }

                    FeType::Owned(owned_of) => {
                        if owned_of.owned_mut != FeOwnedMut::Mut {
                            // TODO: handle assigning late to non-assigned const
                            todo!("Owned type is not mutable: {:#?}", target);
                        }
                    }

                    other => todo!("Cannot assign to {other:?}"),
                }
            }
        }

        {
            let value = &mut *stmt.value.0.try_lock().unwrap();
            changed |= value.accept(self)?;

            types.1 = value.resolved_type().flatten();
        }

        if let (Some(target_type), Some(value_type)) = types {
            if !Self::can_implicit_cast(&value_type, target_type.actual_type()) {
                todo!(
                    "Can't assign types!\nFrom: {:#?}\nTo: {:#?}",
                    value_type,
                    target_type.actual_type()
                );
            }
        }

        if stmt.is_resolved() {
            changed = true;
        }

        return Ok(changed);
    }

    fn visit_return_stmt(
        &mut self,
        shared_stmt: Arc<Mutex<ReturnStmt<Option<FeType>>>>,
    ) -> Result<bool> {
        let stmt = &mut *shared_stmt.try_lock().unwrap();

        let Some(current_return_type) = self.current_return_type.clone() else {
            todo!("Return statements not allowed!");
        };

        if stmt.value.is_none() && current_return_type.is_some() {
            todo!("Can't return without a value!");
        }

        if stmt.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        if let Some(value) = &stmt.value {
            changed |= value.0.try_lock().unwrap().accept(self)?;

            if let Some(resolved_type) = value.0.try_lock().unwrap().resolved_type().flatten() {
                match current_return_type {
                    Some(return_type) => {
                        if !Self::can_implicit_cast(&resolved_type, &return_type) {
                            todo!("Can't cast to return type!")
                        }
                    }

                    None => todo!("Can't return a value! No return type!"),
                }
            }
        }

        return Ok(changed);
    }

    fn visit_if_stmt(&mut self, shared_stmt: Arc<Mutex<IfStmt<Option<FeType>>>>) -> Result<bool> {
        {
            let stmt = &mut *shared_stmt.try_lock().unwrap();

            if stmt.is_resolved() {
                return Ok(false);
            }
        }

        let mut changed = false;

        {
            let condition = {
                let stmt = &mut *shared_stmt.try_lock().unwrap();
                stmt.condition.clone()
            };

            let condition = &mut *condition.0.try_lock().unwrap();

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

        let then = {
            let stmt = &mut *shared_stmt.try_lock().unwrap();
            stmt.then.clone()
        };

        if !then.is_resolved() {
            self.scope
                .try_lock()
                .unwrap()
                .begin_scope(Some(ScopeCreator::IfStmt(
                    IfBlock::Then,
                    shared_stmt.clone(),
                )));

            let (local_changed, _terminal) = self.resolve_stmts(&then.stmts)?;
            changed |= local_changed;

            self.scope.try_lock().unwrap().end_scope();
        }

        {
            let stmt = &mut *shared_stmt.try_lock().unwrap();

            for (idx, else_if) in stmt.else_ifs.iter().enumerate() {
                if !else_if.is_resolved() {
                    {
                        let condition = &mut *else_if.condition.0.try_lock().unwrap();

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

                    self.scope
                        .try_lock()
                        .unwrap()
                        .begin_scope(Some(ScopeCreator::IfStmt(
                            IfBlock::ElseIf(idx),
                            shared_stmt.clone(),
                        )));

                    let (local_changed, _terminal) = self.resolve_stmts(&else_if.then.stmts)?;
                    changed |= local_changed;

                    self.scope.try_lock().unwrap().end_scope();
                }
            }
        }

        let else_ = {
            let stmt = &mut *shared_stmt.try_lock().unwrap();
            stmt.else_.clone()
        };

        if let Some(else_) = &else_ {
            if !else_.is_resolved() {
                self.scope
                    .try_lock()
                    .unwrap()
                    .begin_scope(Some(ScopeCreator::IfStmt(IfBlock::Else, shared_stmt)));

                let (local_changed, _terminal) = self.resolve_stmts(&else_.then.stmts)?;
                changed |= local_changed;

                self.scope.try_lock().unwrap().end_scope();
            }
        }

        return Ok(changed);
    }

    fn visit_loop_stmt(
        &mut self,
        shared_stmt: Arc<Mutex<LoopStmt<Option<FeType>>>>,
    ) -> Result<bool> {
        {
            let stmt = &mut *shared_stmt.try_lock().unwrap();

            if stmt.is_resolved() {
                return Ok(false);
            }
        }

        let mut changed = false;

        let stmts = {
            let stmt = &mut *shared_stmt.try_lock().unwrap();
            stmt.block.stmts.clone()
        };

        self.scope
            .try_lock()
            .unwrap()
            .begin_scope(Some(ScopeCreator::LoopStmt(shared_stmt)));

        self.breakable_count += 1;
        let (local_changed, _terminal) = self.resolve_stmts(&stmts)?;
        self.breakable_count -= 1;

        self.scope.try_lock().unwrap().end_scope();

        changed |= local_changed;

        return Ok(changed);
    }

    fn visit_while_stmt(
        &mut self,
        shared_stmt: Arc<Mutex<WhileStmt<Option<FeType>>>>,
    ) -> Result<bool> {
        {
            let stmt = &mut *shared_stmt.try_lock().unwrap();

            if stmt.is_resolved() {
                return Ok(false);
            }
        }

        let mut changed = false;

        let condition = {
            let stmt = &mut *shared_stmt.try_lock().unwrap();
            stmt.condition.clone()
        };

        changed |= condition.0.try_lock().unwrap().accept(self)?;

        let stmts = {
            let stmt = &mut *shared_stmt.try_lock().unwrap();
            stmt.block.stmts.clone()
        };

        self.scope
            .try_lock()
            .unwrap()
            .begin_scope(Some(ScopeCreator::WhileStmt(shared_stmt)));

        self.breakable_count += 1;
        let (local_changed, _terminal) = self.resolve_stmts(&stmts)?;
        self.breakable_count -= 1;

        self.scope.try_lock().unwrap().end_scope();

        changed |= local_changed;

        return Ok(changed);
    }

    fn visit_break_stmt(
        &mut self,
        shared_stmt: Arc<Mutex<BreakStmt<Option<FeType>>>>,
    ) -> Result<bool> {
        let stmt = &mut *shared_stmt.try_lock().unwrap();

        if self.breakable_count == 0 {
            todo!("Can't break here! {stmt:#?}");
        }

        if stmt.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        let resolved_type = if let Some(value) = &stmt.value {
            changed |= value.0.try_lock().unwrap().accept(self)?;

            if let Some(resolved_type) = value.0.try_lock().unwrap().resolved_type() {
                stmt.resolved_type = Some(resolved_type.clone());
                resolved_type
            } else {
                None
            }
        } else {
            None
        };

        let Some(break_handler) = self
            .scope
            .try_lock()
            .unwrap()
            .handle_break(stmt.label.clone())
        else {
            log::error!(&stmt, &self.scope);
            todo!();
        };

        stmt.handler = Some(break_handler.clone());

        match break_handler {
            BreakHandler::LoopStmt(_loop_stmt) => {
                if stmt.value.is_some() {
                    todo!("Can't break a value");
                }
            }
            BreakHandler::WhileStmt(_while_stmt) => {
                if stmt.value.is_some() {
                    todo!("Can't break a value");
                }
            }

            BreakHandler::LoopExpr(loop_expr) => {
                if stmt.value.is_none() {
                    todo!("TODO: ?::None");
                }

                let loop_expr = &mut *loop_expr.try_lock().unwrap();

                if let Some(Some(loop_typ)) = &loop_expr.resolved_type {
                    let Some(typ) = &resolved_type else {
                        todo!();
                    };

                    if !Self::can_implicit_cast(typ, loop_typ) {
                        todo!();
                    }
                }

                loop_expr.resolved_type = Some(resolved_type);
                changed = true;
            }
            BreakHandler::WhileExpr(while_expr) => {
                if stmt.value.is_none() {
                    todo!("TODO: ?::None");
                }

                let while_expr = &mut *while_expr.try_lock().unwrap();

                if let Some(Some(loop_typ)) = &while_expr.resolved_type {
                    let Some(typ) = &resolved_type else {
                        todo!();
                    };

                    if !Self::can_implicit_cast(typ, loop_typ) {
                        todo!();
                    }
                }

                while_expr.resolved_type = Some(resolved_type);
                changed = true;
            }
        }

        return Ok(changed);
    }

    fn visit_then_stmt(
        &mut self,
        shared_stmt: Arc<Mutex<ThenStmt<Option<FeType>>>>,
    ) -> Result<bool> {
        let stmt = &mut *shared_stmt.try_lock().unwrap();

        if self.thenable_count == 0 {
            todo!("Can't then here! {stmt:#?}");
        }

        if stmt.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        changed |= stmt.value.0.try_lock().unwrap().accept(self)?;

        if let Some(resolved_type) = stmt.value.0.try_lock().unwrap().resolved_type() {
            stmt.resolved_type = resolved_type.clone();

            let Some(then_handler) = self
                .scope
                .try_lock()
                .unwrap()
                .handle_then(stmt.label.clone())
            else {
                log::error!(&stmt, &self.scope);
                todo!();
            };

            stmt.handler = Some(then_handler.clone());

            match then_handler {
                ThenHandler::IfStmt(_block, if_stmt) => {
                    todo!("If statement cannot accept a value: {if_stmt:#?}");
                }
                ThenHandler::IfExpr(_block, if_expr) => {
                    let if_expr = &mut *if_expr.try_lock().unwrap();

                    if let Some(Some(if_typ)) = &if_expr.resolved_type {
                        let Some(typ) = &resolved_type else {
                            todo!();
                        };

                        if !Self::can_implicit_cast(typ, if_typ) {
                            todo!();
                        }
                    }

                    if_expr.resolved_type = Some(resolved_type);
                    changed |= true;
                }
            }
        } else {
            todo!();
        }

        return Ok(changed);
    }
}
