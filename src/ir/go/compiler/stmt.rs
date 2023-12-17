use super::*;

impl StmtVisitor<FeType, Result<Vec<ir::GoIRStmt>>> for GoSyntaxCompiler {
    fn visit_expr_stmt(&mut self, stmt: Arc<Mutex<ExprStmt<FeType>>>) -> Result<Vec<ir::GoIRStmt>> {
        let stmt = &mut *stmt.try_lock().unwrap();

        let expr = stmt.expr.try_lock().unwrap().accept(self)?;

        return Ok(vec![ir::GoIRStmt::Expr(ir::GoIRExprStmt { expr })]);
    }

    fn visit_var_decl_stmt(
        &mut self,
        stmt: Arc<Mutex<VarDeclStmt<FeType>>>,
    ) -> Result<Vec<ir::GoIRStmt>> {
        let mut stmt = stmt.try_lock().unwrap();

        let value = invert(stmt.value.as_mut().map(|value| {
            let value = value.value.0.try_lock().unwrap().accept(self);

            // '?' doesn't work here without explicit type annotation
            // I guess rustc can't guarantee Result::Error here without explicit return Err(...)
            let expr = match value {
                Ok(value) => value,
                Err(e) => return Err(e),
            };

            Ok(ir::GoIRVarDeclValue { expr })
        }))?;

        match &stmt.target {
            VarDeclTarget::Ident(ident) => {
                return Ok(vec![ir::GoIRStmt::VarDecl(ir::GoIRVarDeclStmt {
                    name: ident.try_lock().unwrap().ident.lexeme.clone(),
                    explicit_type: None,
                    value,
                })])
            }
        }
    }

    fn visit_assign_stmt(
        &mut self,
        stmt: Arc<Mutex<AssignStmt<FeType>>>,
    ) -> Result<Vec<ir::GoIRStmt>> {
        let stmt = &mut *stmt.try_lock().unwrap();

        let lhs = stmt.target.0.try_lock().unwrap().accept(self)?;
        let rhs = stmt.value.0.try_lock().unwrap().accept(self)?;

        let op = match &stmt.op {
            AssignOp::Eq(_) => ir::GoIRAssignOp::Eq,
            AssignOp::PlusEq(_) => ir::GoIRAssignOp::PlusEq,
            AssignOp::MinusEq(_) => ir::GoIRAssignOp::MinusEq,
        };

        return Ok(vec![ir::GoIRStmt::Assign(ir::GoIRAssignStmt {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        })]);
    }

    fn visit_return_stmt(
        &mut self,
        stmt: Arc<Mutex<ReturnStmt<FeType>>>,
    ) -> Result<Vec<ir::GoIRStmt>> {
        let mut stmt = stmt.try_lock().unwrap();

        let expr = if let Some(value) = &mut stmt.value {
            Some(value.0.try_lock().unwrap().accept(self)?)
        } else {
            None
        };

        return Ok(vec![ir::GoIRStmt::Return(ir::GoIRReturnStmt { expr })]);
    }

    fn visit_if_stmt(&mut self, stmt: Arc<Mutex<IfStmt<FeType>>>) -> Result<Vec<ir::GoIRStmt>> {
        let mut stmt = stmt.try_lock().unwrap();

        let condition = Box::new(stmt.condition.0.try_lock().unwrap().accept(self)?);

        let mut then = vec![];
        for stmt in &mut stmt.then.stmts {
            let stmts = stmt.try_lock().unwrap().accept(self)?;

            then.extend(stmts);
        }

        let mut else_ifs = vec![];
        for else_if in &mut stmt.else_ifs {
            let condition = Box::new(else_if.condition.0.try_lock().unwrap().accept(self)?);

            let mut then = vec![];
            for stmt in &mut else_if.then.stmts {
                let stmts = stmt.try_lock().unwrap().accept(self)?;

                then.extend(stmts);
            }

            let else_if = ir::GoIRElseIf { condition, then };

            else_ifs.push(else_if);
        }

        let else_ = if let Some(else_) = &mut stmt.else_ {
            let mut then = vec![];
            for stmt in &mut else_.then.stmts {
                let stmts = stmt.try_lock().unwrap().accept(self)?;

                then.extend(stmts);
            }

            Some(ir::GoIRElse { then })
        } else {
            None
        };

        return Ok(vec![ir::GoIRStmt::If(ir::GoIRIfStmt {
            condition,
            then,
            else_ifs,
            else_,
        })]);
    }

    fn visit_loop_stmt(&mut self, stmt: Arc<Mutex<LoopStmt<FeType>>>) -> Result<Vec<ir::GoIRStmt>> {
        let mut stmt = stmt.try_lock().unwrap();

        let label = self.map_label(stmt.node_id().to_string(), &stmt.label);

        let mut stmts = vec![];
        for stmt in &mut stmt.block.stmts {
            let ir_stmts = stmt.try_lock().unwrap().accept(self)?;
            stmts.extend(ir_stmts);
        }

        todo!();
        // return Ok(vec![ir::GoIRStmt::ImplicitReturn(
        //     ir::GoIRImplicitReturnStmt {
        //         expr: ir::GoIRExpr::Loop(ir::GoIRLoopExpr { label, stmts }),
        //     },
        // )]);
    }

    fn visit_while_stmt(
        &mut self,
        stmt: Arc<Mutex<WhileStmt<FeType>>>,
    ) -> Result<Vec<ir::GoIRStmt>> {
        let mut stmt = stmt.try_lock().unwrap();

        let label = self.map_label(stmt.node_id().to_string(), &stmt.label);

        let condition = stmt.condition.0.try_lock().unwrap().accept(self)?;

        let mut stmts = vec![];
        for stmt in &mut stmt.block.stmts {
            let ir_stmts = stmt.try_lock().unwrap().accept(self)?;
            stmts.extend(ir_stmts);
        }

        if stmt.label.is_some() {
            todo!();
            // return Ok(vec![ir::GoIRStmt::ImplicitReturn(
            //     ir::GoIRImplicitReturnStmt {
            //         expr: ir::GoIRExpr::Loop(ir::GoIRLoopExpr {
            //             label: label.clone(),
            //             stmts: vec![
            //                 ir::GoIRStmt::While(ir::GoIRWhileStmt { condition, stmts }),
            //                 ir::GoIRStmt::Break(ir::GoIRBreakStmt { label, expr: None }),
            //             ],
            //         }),
            //     },
            // )]);
        }

        return Ok(vec![ir::GoIRStmt::While(ir::GoIRWhileStmt {
            condition,
            stmts,
        })]);
    }

    fn visit_break_stmt(
        &mut self,
        stmt: Arc<Mutex<BreakStmt<FeType>>>,
    ) -> Result<Vec<ir::GoIRStmt>> {
        let stmt = &mut *stmt.try_lock().unwrap();

        let label = self.map_label(
            stmt.handler
                .as_ref()
                .map(|h| h.node_id().to_string())
                .unwrap_or(String::new()),
            &stmt.label,
        );

        let expr = if let Some(value) = &mut stmt.value {
            Some(value.0.try_lock().unwrap().accept(self)?)
        } else {
            None
        };

        return Ok(vec![ir::GoIRStmt::Break(ir::GoIRBreakStmt { label, expr })]);
    }

    fn visit_then_stmt(&mut self, stmt: Arc<Mutex<ThenStmt<FeType>>>) -> Result<Vec<ir::GoIRStmt>> {
        let stmt = &mut *stmt.try_lock().unwrap();

        let expr = stmt.value.0.try_lock().unwrap().accept(self)?;

        let label = self.map_label(
            stmt.handler
                .as_ref()
                .map(|h| h.node_id().to_string())
                .unwrap_or(String::new()),
            &stmt.label,
        );

        if label.is_some() {
            return Ok(vec![ir::GoIRStmt::Break(ir::GoIRBreakStmt {
                label,
                expr: Some(expr),
            })]);
        }

        if let Some(ThenHandler::IfExpr(block, if_expr)) = &stmt.handler {
            let if_expr = &mut *if_expr.try_lock().unwrap();
            let node_id = if_expr.node_id().to_string();

            let label = match block {
                IfBlock::Then => match &if_expr.then {
                    IfExprThen::Block(if_expr) => self.map_label(node_id, &if_expr.label),
                    _ => None,
                },
                IfBlock::ElseIf(idx) => match &if_expr.else_ifs.get(*idx) {
                    Some(IfExprElseIf::Block(if_expr)) => self.map_label(node_id, &if_expr.label),
                    _ => None,
                },
                IfBlock::Else => match &if_expr.else_ {
                    Some(IfExprElse::Block(if_expr)) => self.map_label(node_id, &if_expr.label),
                    _ => None,
                },
            };

            if label.is_some() {
                return Ok(vec![ir::GoIRStmt::Break(ir::GoIRBreakStmt {
                    label,
                    expr: Some(expr),
                })]);
            }
        }

        todo!();
        // return Ok(vec![ir::GoIRStmt::ImplicitReturn(
        //     ir::GoIRImplicitReturnStmt { expr },
        // )]);
    }
}
