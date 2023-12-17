use super::*;

impl ExprVisitor<FeType, Result<ir::GoIRExpr>> for GoSyntaxCompiler {
    fn visit_bool_literal_expr(
        &mut self,
        expr: Arc<Mutex<BoolLiteralExpr<FeType>>>,
    ) -> Result<ir::GoIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        return Ok(ir::GoIRExpr::BoolLiteral(ir::GoIRBoolLiteralExpr {
            literal: expr.resolved_type == FeType::Bool(Some(true)),
        }));
    }

    fn visit_number_literal_expr(
        &mut self,
        expr: Arc<Mutex<NumberLiteralExpr<FeType>>>,
    ) -> Result<ir::GoIRExpr> {
        let expr = expr.try_lock().unwrap();

        return Ok(ir::GoIRExpr::NumberLiteral(ir::GoIRNumberLiteralExpr {
            literal: expr.literal.lexeme.clone(),
        }));
    }

    fn visit_plain_string_literal_expr(
        &mut self,
        expr: Arc<Mutex<PlainStringLiteralExpr<FeType>>>,
    ) -> Result<ir::GoIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        return Ok(ir::GoIRExpr::Call(ir::GoIRCallExpr {
            callee: Box::new(ir::GoIRExpr::StaticRef(ir::GoIRStaticRefExpr {
                static_ref: ir::GoIRStaticPath {
                    root: Some(Box::new(ir::GoIRStaticPath {
                        root: None,
                        name: STRING_TYPE_NAME.into(),
                    })),
                    name: "from".into(),
                },
            })),
            args: vec![ir::GoIRExpr::StringLiteral(ir::GoIRStringLiteralExpr {
                literal: expr.literal.lexeme.clone(),
            })],
        }));
    }

    fn visit_fmt_string_literal_expr(
        &mut self,
        expr: Arc<Mutex<FmtStringLiteralExpr<FeType>>>,
    ) -> Result<ir::GoIRExpr> {
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

        let mut args = vec![ir::GoIRExpr::StringLiteral(ir::GoIRStringLiteralExpr {
            literal: fmt_str.into(),
        })];

        for part in &expr.rest {
            args.push(part.expr.0.try_lock().unwrap().accept(self)?);
        }

        todo!();
        // return Ok(ir::GoIRExpr::MacroFnCall(ir::GoIRMacroFnCallExpr {
        //     callee: "format".into(),
        //     args,
        // }));
    }

    fn visit_ident_expr(&mut self, expr: Arc<Mutex<IdentExpr<FeType>>>) -> Result<ir::GoIRExpr> {
        let expr = expr.try_lock().unwrap();

        return Ok(ir::GoIRExpr::Ident(ir::GoIRIdentExpr {
            ident: expr.ident.lexeme.clone(),
        }));
    }

    fn visit_call_expr(&mut self, expr: Arc<Mutex<CallExpr<FeType>>>) -> Result<ir::GoIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        if let Some(FeType::Callable(Callable {
            special: Some(SpecialCallable::Print),
            ..
        })) = expr.callee.0.try_lock().unwrap().resolved_type()
        {
            if expr.args.len() == 1 {
                match &mut *expr.args[0].value.0.try_lock().unwrap() {
                    Expr::PlainStringLiteral(literal) => {
                        todo!();
                        // return Ok(ir::GoIRExpr::MacroFnCall(ir::GoIRMacroFnCallExpr {
                        //     callee: "println".into(),
                        //     args: vec![ir::GoIRExpr::StringLiteral(ir::GoIRStringLiteralExpr {
                        //         literal: literal
                        //             .try_lock()
                        //             .unwrap()
                        //             .literal
                        //             .lexeme
                        //             .clone()
                        //             .replace("\\{", "{{")
                        //             .replace('}', "}}")
                        //             .into(),
                        //     })],
                        // }));
                    }

                    Expr::FmtStringLiteral(fmt_str) => {
                        todo!();
                        // if let GoIRExpr::MacroFnCall(macro_call) = fmt_str.accept(self)? {
                        //     if macro_call.callee.as_ref() == "format" {
                        //         return Ok(ir::GoIRExpr::MacroFnCall(ir::GoIRMacroFnCallExpr {
                        //             callee: "println".into(),
                        //             args: macro_call.args,
                        //         }));
                        //     }
                        // }
                    }

                    _ => {}
                }
            }

            let mut args = vec![ir::GoIRExpr::StringLiteral(ir::GoIRStringLiteralExpr {
                literal: "\"{}\"".into(),
            })];

            for arg in &expr.args {
                let value = arg.value.0.try_lock().unwrap();
                let arg_ir = value.accept(self)?;

                args.push(arg_ir);
            }

            todo!();
            // return Ok(ir::GoIRExpr::MacroFnCall(ir::GoIRMacroFnCallExpr {
            //     callee: "println".into(),
            //     args,
            // }));
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

        return Ok(ir::GoIRExpr::Call(ir::GoIRCallExpr { callee, args }));
    }

    fn visit_unary_expr(&mut self, expr: Arc<Mutex<UnaryExpr<FeType>>>) -> Result<ir::GoIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        match &expr.op {
            UnaryOp::Ref(RefType::Shared { .. }) => {
                return Ok(ir::GoIRExpr::Unary(ir::GoIRUnaryExpr {
                    op: ir::GoIRUnaryOp::Ptr,
                    value: Box::new(expr.value.0.try_lock().unwrap().accept(self)?),
                }))
            }
            UnaryOp::Ref(RefType::Mut { .. }) => {
                return Ok(ir::GoIRExpr::Unary(ir::GoIRUnaryExpr {
                    op: ir::GoIRUnaryOp::Ptr,
                    value: Box::new(expr.value.0.try_lock().unwrap().accept(self)?),
                }))
            }
            UnaryOp::Not(_) => {
                return Ok(ir::GoIRExpr::Unary(ir::GoIRUnaryExpr {
                    op: ir::GoIRUnaryOp::Not,
                    value: Box::new(expr.value.0.try_lock().unwrap().accept(self)?),
                }));
            }
        }
    }

    fn visit_binary_expr(&mut self, expr: Arc<Mutex<BinaryExpr<FeType>>>) -> Result<ir::GoIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        let lhs = Box::new(expr.lhs.0.try_lock().unwrap().accept(self)?);
        let rhs = Box::new(expr.rhs.0.try_lock().unwrap().accept(self)?);

        match &expr.op {
            BinaryOp::Add(_) => {
                return Ok(ir::GoIRExpr::Binary(ir::GoIRBinaryExpr {
                    lhs,
                    op: ir::GoIRBinaryOp::Add,
                    rhs,
                }));
            }
            BinaryOp::Subtract(_) => {
                return Ok(ir::GoIRExpr::Binary(ir::GoIRBinaryExpr {
                    lhs,
                    op: ir::GoIRBinaryOp::Subtract,
                    rhs,
                }));
            }

            BinaryOp::Less(_) => {
                return Ok(ir::GoIRExpr::Binary(ir::GoIRBinaryExpr {
                    lhs,
                    op: ir::GoIRBinaryOp::Less,
                    rhs,
                }));
            }
            BinaryOp::LessEq(_) => {
                return Ok(ir::GoIRExpr::Binary(ir::GoIRBinaryExpr {
                    lhs,
                    op: ir::GoIRBinaryOp::LessEq,
                    rhs,
                }));
            }
            BinaryOp::Greater(_) => {
                return Ok(ir::GoIRExpr::Binary(ir::GoIRBinaryExpr {
                    lhs,
                    op: ir::GoIRBinaryOp::Greater,
                    rhs,
                }));
            }
            BinaryOp::GreaterEq(_) => {
                return Ok(ir::GoIRExpr::Binary(ir::GoIRBinaryExpr {
                    lhs,
                    op: ir::GoIRBinaryOp::GreaterEq,
                    rhs,
                }));
            }
        }
    }

    fn visit_static_ref_expr(
        &mut self,
        expr: Arc<Mutex<StaticRefExpr<FeType>>>,
    ) -> Result<ir::GoIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        let path = Self::translate_static_path(&mut expr.static_path);

        return Ok(ir::GoIRExpr::StaticRef(ir::GoIRStaticRefExpr {
            static_ref: path,
        }));
    }

    fn visit_construct_expr(
        &mut self,
        expr: Arc<Mutex<ConstructExpr<FeType>>>,
    ) -> Result<ir::GoIRExpr> {
        let mut expr = expr.try_lock().unwrap();

        let target = match &mut expr.target {
            ConstructTarget::Ident(ident) => {
                ir::GoIRConstructTarget::Ident(ident.try_lock().unwrap().ident.lexeme.clone())
            }

            ConstructTarget::StaticPath(path) => {
                ir::GoIRConstructTarget::StaticPath(Self::translate_static_path(path))
            }
        };

        let mut args = vec![];
        for arg in &mut expr.args {
            match arg {
                ConstructArg::Field(field) => {
                    args.push(ir::GoIRConstructArg {
                        name: field.name.lexeme.clone(),
                        value: field.value.0.try_lock().unwrap().accept(self)?,
                    });
                }
            }
        }

        // TODO
        let spread = None;

        return Ok(ir::GoIRExpr::Construct(GoIRConstructExpr {
            target,
            args,
            spread,
        }));
    }

    fn visit_get_expr(&mut self, expr: Arc<Mutex<GetExpr<FeType>>>) -> Result<ir::GoIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        let target = Box::new(expr.target.0.try_lock().unwrap().accept(self)?);

        return Ok(ir::GoIRExpr::Get(ir::GoIRGetExpr {
            target,
            name: expr.name.lexeme.clone(),
        }));
    }

    fn visit_while_expr(&mut self, expr: Arc<Mutex<WhileExpr<FeType>>>) -> Result<ir::GoIRExpr> {
        let expr = &mut *expr.try_lock().unwrap();

        let _label = self.map_label(expr.node_id().to_string(), &expr.label);

        todo!()
    }

    fn visit_if_expr(&mut self, expr: Arc<Mutex<IfExpr<FeType>>>) -> Result<ir::GoIRExpr> {
        todo!();
    }

    fn visit_loop_expr(&mut self, expr: Arc<Mutex<LoopExpr<FeType>>>) -> Result<ir::GoIRExpr> {
        todo!();
    }
}
