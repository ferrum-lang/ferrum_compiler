use super::*;

impl ExprVisitor<FeType, Result<ir::RustIRExpr>> for RustSyntaxCompiler {
    fn visit_bool_literal_expr(
        &mut self,
        expr: Arc<Mutex<BoolLiteralExpr<FeType>>>,
    ) -> Result<ir::RustIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        return Ok(ir::RustIRExpr::BoolLiteral(ir::RustIRBoolLiteralExpr {
            literal: expr.resolved_type == FeType::Bool(Some(true)),
        }));
    }

    fn visit_number_literal_expr(
        &mut self,
        expr: Arc<Mutex<NumberLiteralExpr<FeType>>>,
    ) -> Result<ir::RustIRExpr> {
        let expr = expr.try_lock().unwrap();

        return Ok(ir::RustIRExpr::NumberLiteral(ir::RustIRNumberLiteralExpr {
            literal: expr.literal.lexeme.clone(),
        }));
    }

    fn visit_plain_string_literal_expr(
        &mut self,
        expr: Arc<Mutex<PlainStringLiteralExpr<FeType>>>,
    ) -> Result<ir::RustIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        return Ok(ir::RustIRExpr::Call(ir::RustIRCallExpr {
            callee: Box::new(ir::RustIRExpr::StaticRef(ir::RustIRStaticRefExpr {
                static_ref: ir::RustIRStaticPath {
                    root: Some(Box::new(ir::RustIRStaticPath {
                        root: None,
                        name: STRING_TYPE_NAME.into(),
                    })),
                    name: "from".into(),
                },
            })),
            args: vec![ir::RustIRExpr::StringLiteral(ir::RustIRStringLiteralExpr {
                literal: expr.literal.lexeme.clone(),
            })],
        }));
    }

    fn visit_fmt_string_literal_expr(
        &mut self,
        expr: Arc<Mutex<FmtStringLiteralExpr<FeType>>>,
    ) -> Result<ir::RustIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        let mut fmt_str = String::new();
        fmt_str.push_str(
            &expr.first.lexeme[0..expr.first.lexeme.len() - 1]
                .replace("\\{", "{{")
                .replace('}', "}}"),
        );

        for part in &expr.rest {
            fmt_str.push_str("{}");

            fmt_str.push_str(
                &part.string[1..part.string.len() - 1]
                    .replace("\\{", "{{")
                    .replace('}', "}}"),
            );
        }
        fmt_str.push('"');

        let mut args = vec![ir::RustIRExpr::StringLiteral(ir::RustIRStringLiteralExpr {
            literal: fmt_str.into(),
        })];

        for part in &expr.rest {
            args.push(part.expr.0.try_lock().unwrap().accept(self)?);
        }

        return Ok(ir::RustIRExpr::MacroFnCall(ir::RustIRMacroFnCallExpr {
            callee: "format".into(),
            args,
        }));
    }

    fn visit_ident_expr(&mut self, expr: Arc<Mutex<IdentExpr<FeType>>>) -> Result<ir::RustIRExpr> {
        let expr = expr.try_lock().unwrap();

        return Ok(ir::RustIRExpr::Ident(ir::RustIRIdentExpr {
            ident: expr.ident.lexeme.clone(),
        }));
    }

    fn visit_call_expr(&mut self, expr: Arc<Mutex<CallExpr<FeType>>>) -> Result<ir::RustIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        if let Some(FeType::Callable(Callable {
            special: Some(SpecialCallable::Print),
            ..
        })) = expr.callee.0.try_lock().unwrap().resolved_type()
        {
            if expr.args.len() == 1 {
                match &mut *expr.args[0].value.0.try_lock().unwrap() {
                    Expr::PlainStringLiteral(literal) => {
                        return Ok(ir::RustIRExpr::MacroFnCall(ir::RustIRMacroFnCallExpr {
                            callee: "println".into(),
                            args: vec![ir::RustIRExpr::StringLiteral(
                                ir::RustIRStringLiteralExpr {
                                    literal: literal
                                        .try_lock()
                                        .unwrap()
                                        .literal
                                        .lexeme
                                        .clone()
                                        .replace("\\{", "{{")
                                        .replace('}', "}}")
                                        .into(),
                                },
                            )],
                        }));
                    }

                    Expr::FmtStringLiteral(fmt_str) => {
                        if let RustIRExpr::MacroFnCall(macro_call) = fmt_str.accept(self)? {
                            if macro_call.callee.as_ref() == "format" {
                                return Ok(ir::RustIRExpr::MacroFnCall(
                                    ir::RustIRMacroFnCallExpr {
                                        callee: "println".into(),
                                        args: macro_call.args,
                                    },
                                ));
                            }
                        }
                    }

                    _ => {}
                }
            }

            let mut args = vec![ir::RustIRExpr::StringLiteral(ir::RustIRStringLiteralExpr {
                literal: "\"{}\"".into(),
            })];

            for arg in &expr.args {
                let value = arg.value.0.try_lock().unwrap();
                let arg_ir = value.accept(self)?;

                args.push(arg_ir);
            }

            return Ok(ir::RustIRExpr::MacroFnCall(ir::RustIRMacroFnCallExpr {
                callee: "println".into(),
                args,
            }));
        }

        let callee = expr.callee.0.try_lock().unwrap();
        let callee = Box::new(callee.accept(self)?);

        let mut args = vec![];

        // TODO: Handle named, variadic, optional, etc params
        for arg in &expr.args {
            let value = arg.value.0.try_lock().unwrap();
            let arg_ir = value.accept(self)?;

            args.push(arg_ir);
        }

        return Ok(ir::RustIRExpr::Call(ir::RustIRCallExpr { callee, args }));
    }

    fn visit_unary_expr(&mut self, expr: Arc<Mutex<UnaryExpr<FeType>>>) -> Result<ir::RustIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        match &expr.op {
            UnaryOp::Ref(RefType::Shared { .. }) => {
                return Ok(ir::RustIRExpr::Unary(ir::RustIRUnaryExpr {
                    op: ir::RustIRUnaryOp::Ref(ir::RustIRRefType::Shared),
                    value: Box::new(expr.value.0.try_lock().unwrap().accept(self)?),
                }))
            }
            UnaryOp::Ref(RefType::Mut { .. }) => {
                return Ok(ir::RustIRExpr::Unary(ir::RustIRUnaryExpr {
                    op: ir::RustIRUnaryOp::Ref(ir::RustIRRefType::Mut),
                    value: Box::new(expr.value.0.try_lock().unwrap().accept(self)?),
                }))
            }
            UnaryOp::Not(_) => {
                return Ok(ir::RustIRExpr::Unary(ir::RustIRUnaryExpr {
                    op: ir::RustIRUnaryOp::Not,
                    value: Box::new(expr.value.0.try_lock().unwrap().accept(self)?),
                }));
            }
        }
    }

    fn visit_binary_expr(
        &mut self,
        expr: Arc<Mutex<BinaryExpr<FeType>>>,
    ) -> Result<ir::RustIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        let lhs = Box::new(expr.lhs.0.try_lock().unwrap().accept(self)?);
        let rhs = Box::new(expr.rhs.0.try_lock().unwrap().accept(self)?);

        match &expr.op {
            BinaryOp::Add(_) => {
                return Ok(ir::RustIRExpr::Binary(ir::RustIRBinaryExpr {
                    lhs,
                    op: ir::RustIRBinaryOp::Add,
                    rhs,
                }));
            }
            BinaryOp::Subtract(_) => {
                return Ok(ir::RustIRExpr::Binary(ir::RustIRBinaryExpr {
                    lhs,
                    op: ir::RustIRBinaryOp::Subtract,
                    rhs,
                }));
            }

            BinaryOp::Less(_) => {
                return Ok(ir::RustIRExpr::Binary(ir::RustIRBinaryExpr {
                    lhs,
                    op: ir::RustIRBinaryOp::Less,
                    rhs,
                }));
            }
            BinaryOp::LessEq(_) => {
                return Ok(ir::RustIRExpr::Binary(ir::RustIRBinaryExpr {
                    lhs,
                    op: ir::RustIRBinaryOp::LessEq,
                    rhs,
                }));
            }
            BinaryOp::Greater(_) => {
                return Ok(ir::RustIRExpr::Binary(ir::RustIRBinaryExpr {
                    lhs,
                    op: ir::RustIRBinaryOp::Greater,
                    rhs,
                }));
            }
            BinaryOp::GreaterEq(_) => {
                return Ok(ir::RustIRExpr::Binary(ir::RustIRBinaryExpr {
                    lhs,
                    op: ir::RustIRBinaryOp::GreaterEq,
                    rhs,
                }));
            }
        }
    }

    fn visit_static_ref_expr(
        &mut self,
        expr: Arc<Mutex<StaticRefExpr<FeType>>>,
    ) -> Result<ir::RustIRExpr> {
        let _expr = &mut *expr.try_lock().unwrap();

        todo!()
    }

    fn visit_construct_expr(
        &mut self,
        expr: Arc<Mutex<ConstructExpr<FeType>>>,
    ) -> Result<ir::RustIRExpr> {
        let mut expr = expr.try_lock().unwrap();

        let target = match &mut expr.target {
            ConstructTarget::Ident(ident) => {
                ir::RustIRConstructTarget::Ident(ident.try_lock().unwrap().ident.lexeme.clone())
            }

            ConstructTarget::StaticPath(path) => {
                ir::RustIRConstructTarget::StaticPath(Self::translate_static_path(path))
            }
        };

        let mut args = vec![];
        for arg in &mut expr.args {
            match arg {
                ConstructArg::Field(field) => {
                    args.push(ir::RustIRConstructArg {
                        name: field.name.lexeme.clone(),
                        value: field.value.0.try_lock().unwrap().accept(self)?,
                    });
                }
            }
        }

        // TODO
        let spread = None;

        return Ok(ir::RustIRExpr::Construct(RustIRConstructExpr {
            target,
            args,
            spread,
        }));
    }

    fn visit_get_expr(&mut self, expr: Arc<Mutex<GetExpr<FeType>>>) -> Result<ir::RustIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        let target = Box::new(expr.target.0.try_lock().unwrap().accept(self)?);

        return Ok(ir::RustIRExpr::Get(ir::RustIRGetExpr {
            target,
            name: expr.name.lexeme.clone(),
        }));
    }

    fn visit_if_expr(&mut self, expr: Arc<Mutex<IfExpr<FeType>>>) -> Result<ir::RustIRExpr> {
        let expr = expr.try_lock().unwrap();
        let node_id = expr.node_id().to_string();

        let condition = Box::new(expr.condition.0.try_lock().unwrap().accept(self)?);

        let then = match &expr.then {
            IfExprThen::Ternary(then) => {
                let expr = then.then_expr.0.try_lock().unwrap().accept(self)?;

                let stmt = ir::RustIRStmt::ImplicitReturn(ir::RustIRImplicitReturnStmt { expr });

                vec![stmt]
            }
            IfExprThen::Block(then) => {
                let mut then_stmts = vec![];

                for stmt in &then.block.stmts {
                    let stmts = stmt.try_lock().unwrap().accept(self)?;
                    then_stmts.extend(stmts);
                }

                if then.label.is_some() {
                    let label = self.map_label(node_id.clone(), &then.label);

                    vec![ir::RustIRStmt::ImplicitReturn(
                        ir::RustIRImplicitReturnStmt {
                            expr: ir::RustIRExpr::Loop(ir::RustIRLoopExpr {
                                label,
                                stmts: then_stmts,
                            }),
                        },
                    )]
                } else {
                    then_stmts
                }
            }
        };

        let mut else_ifs = vec![];
        for else_if in &expr.else_ifs {
            match else_if {
                IfExprElseIf::Ternary(else_if) => {
                    let condition = Box::new(else_if.condition.0.try_lock().unwrap().accept(self)?);

                    let expr = else_if.expr.0.try_lock().unwrap().accept(self)?;

                    let then = vec![ir::RustIRStmt::ImplicitReturn(
                        ir::RustIRImplicitReturnStmt { expr },
                    )];

                    else_ifs.push(ir::RustIRElseIf { condition, then });
                }
                IfExprElseIf::Block(else_if) => {
                    let condition = Box::new(else_if.condition.0.try_lock().unwrap().accept(self)?);

                    let mut then = vec![];

                    for stmt in &else_if.block.stmts {
                        let stmts = stmt.try_lock().unwrap().accept(self)?;
                        then.extend(stmts);
                    }

                    then = if else_if.label.is_some() {
                        let label = self.map_label(node_id.clone(), &else_if.label);

                        vec![ir::RustIRStmt::ImplicitReturn(
                            ir::RustIRImplicitReturnStmt {
                                expr: ir::RustIRExpr::Loop(ir::RustIRLoopExpr {
                                    label,
                                    stmts: then,
                                }),
                            },
                        )]
                    } else {
                        then
                    };

                    else_ifs.push(ir::RustIRElseIf { condition, then });
                }
            }
        }

        let else_ = if let Some(else_) = &expr.else_ {
            match else_ {
                IfExprElse::Ternary(else_) => {
                    let expr = else_.else_expr.0.try_lock().unwrap().accept(self)?;

                    let then = vec![ir::RustIRStmt::ImplicitReturn(
                        ir::RustIRImplicitReturnStmt { expr },
                    )];

                    Some(ir::RustIRElse { then })
                }
                IfExprElse::Block(else_) => {
                    let mut then = vec![];

                    for stmt in &else_.block.stmts {
                        let stmts = stmt.try_lock().unwrap().accept(self)?;
                        then.extend(stmts);
                    }

                    then = if else_.label.is_some() {
                        let label = self.map_label(node_id, &else_.label);

                        vec![ir::RustIRStmt::ImplicitReturn(
                            ir::RustIRImplicitReturnStmt {
                                expr: ir::RustIRExpr::Loop(ir::RustIRLoopExpr {
                                    label,
                                    stmts: then,
                                }),
                            },
                        )]
                    } else {
                        then
                    };

                    Some(ir::RustIRElse { then })
                }
            }
        } else {
            None
        };

        return Ok(ir::RustIRExpr::If(ir::RustIRIfExpr {
            condition,
            then,
            else_ifs,
            else_,
        }));
    }

    fn visit_loop_expr(&mut self, expr: Arc<Mutex<LoopExpr<FeType>>>) -> Result<ir::RustIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        let label = self.map_label(expr.node_id().to_string(), &expr.label);

        let mut stmts = vec![];
        for stmt in &expr.block.stmts {
            let stmt = stmt.try_lock().unwrap().accept(self)?;
            stmts.extend(stmt);
        }

        return Ok(ir::RustIRExpr::Loop(ir::RustIRLoopExpr { label, stmts }));
    }

    fn visit_while_expr(&mut self, expr: Arc<Mutex<WhileExpr<FeType>>>) -> Result<ir::RustIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        let _label = self.map_label(expr.node_id().to_string(), &expr.label);

        todo!()
    }
}
