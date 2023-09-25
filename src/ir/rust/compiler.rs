use super::*;

use crate::r#type::*;
use crate::syntax::*;

use crate::ir;
use crate::utils::invert;

use crate::result::Result;

use std::sync::{Arc, Mutex};

pub struct RustSyntaxCompiler {
    entry: Arc<Mutex<FeSyntaxPackage<FeType>>>,
    out: ir::RustIR,
}

impl SyntaxCompiler<ir::RustIR> for RustSyntaxCompiler {
    fn compile_package(entry: Arc<Mutex<FeSyntaxPackage<FeType>>>) -> Result<ir::RustIR> {
        return Self::new(entry).compile();
    }
}

impl RustSyntaxCompiler {
    fn new(entry: Arc<Mutex<FeSyntaxPackage<FeType>>>) -> Self {
        return Self {
            entry,
            out: ir::RustIR {
                files: vec![ir::RustIRFile {
                    path: "./main.rs".into(), // TODO
                    mods: vec![],
                    uses: vec![],
                    decls: vec![],
                }],
            },
        };
    }

    fn compile(mut self) -> Result<ir::RustIR> {
        self.compile_package(&mut Arc::clone(&self.entry).lock().unwrap())?;

        return Ok(self.out);
    }

    fn compile_package(&mut self, package: &mut FeSyntaxPackage<FeType>) -> Result {
        match package {
            FeSyntaxPackage::File(file) => {
                self.compile_file(file)?;
            }

            FeSyntaxPackage::Dir(dir) => {
                {
                    let mut syntax = dir.entry_file.syntax.lock().unwrap();

                    for name in dir.local_packages.keys() {
                        syntax.mods.push(Mod(name.0.clone()));
                    }
                }

                self.compile_file(&mut dir.entry_file)?;

                for (name, package) in &dir.local_packages {
                    self.out.files.push(ir::RustIRFile {
                        path: format!("./{}.rs", name.0).into(),
                        mods: vec![],
                        uses: vec![],
                        decls: vec![],
                    });
                    self.compile_package(&mut package.lock().unwrap())?;
                }
            }
        };

        return Ok(());
    }

    fn compile_file(&mut self, file: &mut FeSyntaxFile<FeType>) -> Result {
        let mut syntax = file.syntax.lock().unwrap();

        {
            let file_idx = self.out.files.len() - 1;
            let mods = &mut self.out.files[file_idx].mods;

            for mod_decl in &syntax.mods {
                mods.push(mod_decl.0.clone());
            }
        }

        for use_decl in &mut syntax.uses {
            use_decl.accept(self)?;
        }

        for decl in &mut syntax.decls {
            decl.lock().unwrap().accept(self)?;
        }

        return Ok(());
    }

    fn translate_decl_mod(&self, decl_mod: &DeclMod) -> ir::RustIRDeclMod {
        match decl_mod {
            DeclMod::Pub(_) => return ir::RustIRDeclMod::Pub,
        }
    }

    fn translate_fn_param(&self, param: &mut FnDeclParam<FeType>) -> ir::RustIRFnParam {
        return ir::RustIRFnParam {
            name: param.name.lexeme.clone(),
            static_type_ref: self.translate_static_type(&mut param.static_type_ref),
            trailing_comma: param.comma_token.is_some(),
        };
    }

    fn translate_fn_return_type(
        &self,
        return_type: &mut FnDeclReturnType<FeType>,
    ) -> ir::RustIRStaticType {
        return self.translate_static_type(&mut return_type.static_type);
    }

    fn translate_fn_body(&mut self, body: &mut FnDeclBody<FeType>) -> Result<ir::RustIRBlockExpr> {
        let mut block_ir = ir::RustIRBlockExpr { stmts: vec![] };

        match body {
            FnDeclBody::Short(short) => todo!(),
            FnDeclBody::Block(block) => {
                for stmt in &mut block.stmts {
                    let stmt_ir = stmt.lock().unwrap().accept(self)?;

                    block_ir.stmts.extend(stmt_ir);
                }
            }
        }

        return Ok(block_ir);
    }

    fn translate_struct_field(&self, field: &mut StructDeclField<FeType>) -> ir::RustIRStructField {
        return ir::RustIRStructField {
            field_mod: field.field_mod.as_ref().map(|field| match field {
                StructFieldMod::Pub(_) => ir::RustIRStructFieldMod::Pub,
            }),
            name: field.name.lexeme.clone(),
            static_type_ref: self.translate_static_type(&mut field.static_type_ref),
            trailing_comma: field.comma_token.is_some(),
        };
    }

    fn translate_static_type(&self, typ: &mut StaticType<FeType>) -> ir::RustIRStaticType {
        let ref_type = typ.ref_type.as_ref().map(|ref_type| match ref_type {
            RefType::Shared { .. } => ir::RustIRRefType::Shared,
            RefType::Mut { .. } => ir::RustIRRefType::Mut,
        });

        return ir::RustIRStaticType {
            ref_type,
            static_path: self.translate_static_path(&mut typ.static_path),
        };
    }

    fn translate_static_path(&self, path: &mut StaticPath<FeType>) -> ir::RustIRStaticPath {
        if path.root.is_none()
            && path.name.lexeme.as_ref() == "Int"
            && matches!(path.resolved_type, FeType::Number(_))
        {
            return ir::RustIRStaticPath {
                root: None,
                name: "i64".into(),
            };
        }

        return ir::RustIRStaticPath {
            root: path
                .root
                .as_mut()
                .map(|root| Box::new(self.translate_static_path(root))),
            name: path.name.lexeme.clone(),
        };
    }

    fn translate_use_mod(&self, use_mod: &UseMod) -> ir::RustIRUseMod {
        match use_mod {
            UseMod::Pub(_) => ir::RustIRUseMod::Pub,
        }
    }

    fn translate_use_static_path(
        &mut self,
        path: &mut UseStaticPath<FeType>,
    ) -> Result<Option<ir::RustIRUseStaticPath>> {
        let next = match &mut path.details {
            Either::B(_) => None,

            Either::A(UseStaticPathNext::Single(ref mut single)) => {
                if let Either::B(FeType::Callable(Callable {
                    special: Some(SpecialCallable::Print),
                    ..
                })) = &single.path.details
                {
                    // No need to import print
                    return Ok(None);
                } else {
                    let Some(next_path) = self.translate_use_static_path(&mut single.path)? else { return Ok(None) };

                    Some(ir::RustIRUseStaticPathNext::Single(
                        ir::RustIRUseStaticPathNextSingle {
                            path: Box::new(next_path),
                        },
                    ))
                }
            }

            Either::A(UseStaticPathNext::Many(many)) => todo!(),
        };

        let path_ir = ir::RustIRUseStaticPath {
            pre: path.pre.as_ref().map(|pre| match pre {
                UseStaticPathPre::DoubleColon(_) => ir::RustIRUseStaticPathPre::DoubleColon,
                UseStaticPathPre::CurrentDir(_) => ir::RustIRUseStaticPathPre::CurrentDir,
                UseStaticPathPre::RootDir(_) => ir::RustIRUseStaticPathPre::RootDir,
            }),
            name: path.name.lexeme.clone(),
            next,
        };

        return Ok(Some(path_ir));
    }
}

impl UseVisitor<FeType, Result> for RustSyntaxCompiler {
    fn visit_use(&mut self, use_decl: Arc<Mutex<Use<FeType>>>) -> Result {
        let mut use_decl = use_decl.lock().unwrap();

        let use_mod = use_decl
            .use_mod
            .as_ref()
            .map(|use_mod| self.translate_use_mod(use_mod));

        let path = self.translate_use_static_path(&mut use_decl.path)?;

        if let Some(path) = path {
            let use_ir = ir::RustIRUse { use_mod, path };

            let file_idx = self.out.files.len() - 1;
            self.out.files[file_idx].uses.push(use_ir);
        }

        return Ok(());
    }
}

impl DeclVisitor<FeType, Result> for RustSyntaxCompiler {
    fn visit_function_decl(&mut self, decl: Arc<Mutex<FnDecl<FeType>>>) -> Result {
        let mut decl = decl.lock().unwrap();

        let fn_ir = ir::RustIRFnDecl {
            macros: vec![],

            decl_mod: decl
                .decl_mod
                .as_ref()
                .map(|decl_mod| self.translate_decl_mod(decl_mod)),

            is_async: false, // TODO

            generics: None,

            name: decl.name.lexeme.clone(),
            params: decl
                .params
                .iter_mut()
                .map(|param| self.translate_fn_param(param))
                .collect(),

            return_type: if let Some(return_type) = &mut decl.return_type {
                Some(self.translate_fn_return_type(return_type))
            } else {
                None
            },

            body: self.translate_fn_body(&mut decl.body)?,
        };

        let file_idx = self.out.files.len() - 1;
        self.out.files[file_idx]
            .decls
            .push(ir::RustIRDecl::Fn(fn_ir));

        return Ok(());
    }

    fn visit_struct_decl(&mut self, decl: Arc<Mutex<StructDecl<FeType>>>) -> Result {
        let mut decl = decl.lock().unwrap();

        let struct_ir = ir::RustIRStructDecl {
            macros: vec![],
            decl_mod: decl
                .decl_mod
                .as_ref()
                .map(|decl_mod| self.translate_decl_mod(decl_mod)),

            name: decl.name.lexeme.clone(),

            generics: None,

            fields: decl
                .fields
                .iter_mut()
                .map(|field| self.translate_struct_field(field))
                .collect(),
        };

        let file_idx = self.out.files.len() - 1;
        self.out.files[file_idx]
            .decls
            .push(ir::RustIRDecl::Struct(struct_ir));

        return Ok(());
    }
}

impl StmtVisitor<FeType, Result<Vec<ir::RustIRStmt>>> for RustSyntaxCompiler {
    fn visit_expr_stmt(
        &mut self,
        stmt: Arc<Mutex<ExprStmt<FeType>>>,
    ) -> Result<Vec<ir::RustIRStmt>> {
        let mut stmt = stmt.lock().unwrap();

        let expr = stmt.expr.lock().unwrap().accept(self)?;

        return Ok(vec![ir::RustIRStmt::Expr(ir::RustIRExprStmt { expr })]);
    }

    fn visit_var_decl_stmt(
        &mut self,
        stmt: Arc<Mutex<VarDeclStmt<FeType>>>,
    ) -> Result<Vec<ir::RustIRStmt>> {
        let mut stmt = stmt.lock().unwrap();

        let value = invert(stmt.value.as_mut().map(|value| {
            let value = value.value.0.lock().unwrap().accept(self);

            // '?' doesn't work here without explicit type annotation
            // I guess rustc can't guarantee Result::Error here without explicit return Err(...)
            let expr = match value {
                Ok(value) => value,
                Err(e) => return Err(e),
            };

            Ok(ir::RustIRLetValue { expr })
        }))?;

        match &stmt.target {
            VarDeclTarget::Ident(ident) => {
                return Ok(vec![ir::RustIRStmt::Let(ir::RustIRLetStmt {
                    is_mut: match &stmt.var_mut {
                        VarDeclMut::Const(_) => false,
                        VarDeclMut::Mut(_) => true,
                    },
                    name: ident.lock().unwrap().ident.lexeme.clone(),
                    explicit_type: None,
                    value,
                })])
            }
        }
    }

    fn visit_assign_stmt(
        &mut self,
        stmt: Arc<Mutex<AssignStmt<FeType>>>,
    ) -> Result<Vec<ir::RustIRStmt>> {
        let mut stmt = stmt.lock().unwrap();

        let lhs = stmt.target.0.lock().unwrap().accept(self)?;
        let rhs = stmt.value.0.lock().unwrap().accept(self)?;

        let op = match &stmt.op {
            AssignOp::Eq(_) => ir::RustIRAssignOp::Eq,
            AssignOp::PlusEq(_) => ir::RustIRAssignOp::PlusEq,
        };

        return Ok(vec![ir::RustIRStmt::Expr(ir::RustIRExprStmt {
            expr: ir::RustIRExpr::Assign(ir::RustIRAssignExpr {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }),
        })]);
    }

    fn visit_return_stmt(
        &mut self,
        stmt: Arc<Mutex<ReturnStmt<FeType>>>,
    ) -> Result<Vec<ir::RustIRStmt>> {
        let mut stmt = stmt.lock().unwrap();

        let expr = if let Some(value) = &mut stmt.value {
            Some(value.0.lock().unwrap().accept(self)?)
        } else {
            None
        };

        return Ok(vec![ir::RustIRStmt::Return(ir::RustIRReturnStmt { expr })]);
    }

    fn visit_if_stmt(&mut self, stmt: Arc<Mutex<IfStmt<FeType>>>) -> Result<Vec<ir::RustIRStmt>> {
        let mut stmt = stmt.lock().unwrap();

        let condition = Box::new(stmt.condition.0.lock().unwrap().accept(self)?);

        let mut then = vec![];
        for stmt in &mut stmt.then.stmts {
            let stmts = stmt.lock().unwrap().accept(self)?;

            then.extend(stmts);
        }

        let mut else_ifs = vec![];
        for else_if in &mut stmt.else_ifs {
            let condition = Box::new(else_if.condition.0.lock().unwrap().accept(self)?);

            let mut then = vec![];
            for stmt in &mut else_if.then.stmts {
                let stmts = stmt.lock().unwrap().accept(self)?;

                then.extend(stmts);
            }

            let else_if = ir::RustIRElseIf { condition, then };

            else_ifs.push(else_if);
        }

        let else_ = if let Some(else_) = &mut stmt.else_ {
            let mut then = vec![];
            for stmt in &mut else_.then.stmts {
                let stmts = stmt.lock().unwrap().accept(self)?;

                then.extend(stmts);
            }

            Some(ir::RustIRElse { then })
        } else {
            None
        };

        let expr = ir::RustIRExpr::If(ir::RustIRIfExpr {
            condition,
            then,
            else_ifs,
            else_,
        });

        return Ok(vec![ir::RustIRStmt::ImplicitReturn(
            ir::RustIRImplicitReturnStmt { expr },
        )]);
    }

    fn visit_loop_stmt(
        &mut self,
        stmt: Arc<Mutex<LoopStmt<FeType>>>,
    ) -> Result<Vec<ir::RustIRStmt>> {
        let mut stmt = stmt.lock().unwrap();

        todo!();

        // let mut stmts = vec![];

        // for stmt in &mut stmt.inner.block.stmts {
        //     let ir_stmts = stmt.lock().unwrap().accept(self)?;
        //     stmts.extend(ir_stmts);
        // }

        // return Ok(vec![ir::RustIRStmt::Loop(ir::RustIRLoopStmt { stmts })]);
    }

    fn visit_while_stmt(
        &mut self,
        stmt: Arc<Mutex<WhileStmt<FeType>>>,
    ) -> Result<Vec<ir::RustIRStmt>> {
        let mut stmt = stmt.lock().unwrap();

        todo!();

        // let condition = stmt.inner.condition.0.lock().unwrap().accept(self)?;

        // let mut stmts = vec![];

        // for stmt in &mut stmt.inner.block.stmts {
        //     let ir_stmts = stmt.lock().unwrap().accept(self)?;
        //     stmts.extend(ir_stmts);
        // }

        // return Ok(vec![ir::RustIRStmt::While(ir::RustIRWhileStmt {
        //     condition,
        //     stmts,
        // })]);
    }

    fn visit_break_stmt(
        &mut self,
        stmt: Arc<Mutex<BreakStmt<FeType>>>,
    ) -> Result<Vec<ir::RustIRStmt>> {
        let mut stmt = stmt.lock().unwrap();

        let label = stmt.label.as_ref().map(|l| l.lexeme.clone());

        let expr = if let Some(value) = &mut stmt.value {
            Some(value.0.lock().unwrap().accept(self)?)
        } else {
            None
        };

        return Ok(vec![ir::RustIRStmt::Break(ir::RustIRBreakStmt {
            label,
            expr,
        })]);
    }

    fn visit_then_stmt(
        &mut self,
        stmt: Arc<Mutex<ThenStmt<FeType>>>,
    ) -> Result<Vec<ir::RustIRStmt>> {
        let mut stmt = stmt.lock().unwrap();

        let expr = stmt.value.0.lock().unwrap().accept(self)?;

        if let Some(handler) = &stmt.handler {
            match handler {
                ThenHandler::IfStmt(_, _) => todo!(),
                ThenHandler::IfExpr(block, handler) => {
                    let label: Option<Arc<str>> = match block {
                        IfBlock::Then => match &handler.try_lock().unwrap().then {
                            IfExprThen::Block(block) => block
                                .label
                                .as_ref()
                                .map(|l| format!("'if_expr_then_{}", &l.lexeme[1..]).into()),
                            _ => None,
                        },

                        IfBlock::ElseIf(idx) => {
                            match &handler.try_lock().unwrap().else_ifs.get(*idx) {
                                Some(IfExprElseIf::Block(block)) => block.label.as_ref().map(|l| {
                                    format!("'if_expr_else_if_{}_{}", idx, &l.lexeme[1..]).into()
                                }),
                                _ => None,
                            }
                        }

                        IfBlock::Else => match &handler.try_lock().unwrap().else_ {
                            Some(IfExprElse::Block(block)) => block
                                .label
                                .as_ref()
                                .map(|l| format!("'if_expr_else_{}", &l.lexeme[1..]).into()),
                            _ => None,
                        },
                    };

                    if label.is_some() {
                        return Ok(vec![ir::RustIRStmt::Break(ir::RustIRBreakStmt {
                            label,
                            expr: Some(expr),
                        })]);
                    }
                }
            }
        }

        return Ok(vec![ir::RustIRStmt::ImplicitReturn(
            ir::RustIRImplicitReturnStmt { expr },
        )]);
    }
}

impl ExprVisitor<FeType, Result<ir::RustIRExpr>> for RustSyntaxCompiler {
    fn visit_bool_literal_expr(
        &mut self,
        expr: Arc<Mutex<BoolLiteralExpr<FeType>>>,
    ) -> Result<ir::RustIRExpr> {
        let mut expr = expr.lock().unwrap();

        return Ok(ir::RustIRExpr::BoolLiteral(ir::RustIRBoolLiteralExpr {
            literal: expr.resolved_type == FeType::Bool(Some(true)),
        }));
    }

    fn visit_number_literal_expr(
        &mut self,
        expr: Arc<Mutex<NumberLiteralExpr<FeType>>>,
    ) -> Result<ir::RustIRExpr> {
        let mut expr = expr.lock().unwrap();

        return Ok(ir::RustIRExpr::NumberLiteral(ir::RustIRNumberLiteralExpr {
            literal: expr.literal.lexeme.clone(),
        }));
    }

    fn visit_plain_string_literal_expr(
        &mut self,
        expr: Arc<Mutex<PlainStringLiteralExpr<FeType>>>,
    ) -> Result<ir::RustIRExpr> {
        let mut expr = expr.lock().unwrap();

        return Ok(ir::RustIRExpr::Call(ir::RustIRCallExpr {
            callee: Box::new(ir::RustIRExpr::StaticRef(ir::RustIRStaticRefExpr {
                static_ref: ir::RustIRStaticPath {
                    root: Some(Box::new(ir::RustIRStaticPath {
                        root: None,
                        name: "String".into(),
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
        let mut expr = expr.lock().unwrap();

        let mut fmt_str = String::new();
        fmt_str.push_str(
            &expr.first.lexeme[0..expr.first.lexeme.len() - 1]
                .replace("\\{", "{{")
                .replace("}", "}}"),
        );

        for part in &expr.rest {
            fmt_str.push_str("{}");

            fmt_str.push_str(
                &part.string[1..part.string.len() - 1]
                    .replace("\\{", "{{")
                    .replace("}", "}}"),
            );
        }
        fmt_str.push('"');

        let mut args = vec![ir::RustIRExpr::StringLiteral(ir::RustIRStringLiteralExpr {
            literal: fmt_str.into(),
        })];

        for part in &expr.rest {
            args.push(part.expr.0.lock().unwrap().accept(self)?);
        }

        return Ok(ir::RustIRExpr::MacroFnCall(ir::RustIRMacroFnCallExpr {
            callee: "format".into(),
            args,
        }));
    }

    fn visit_ident_expr(&mut self, expr: Arc<Mutex<IdentExpr<FeType>>>) -> Result<ir::RustIRExpr> {
        let mut expr = expr.lock().unwrap();

        return Ok(ir::RustIRExpr::Ident(ir::RustIRIdentExpr {
            ident: expr.ident.lexeme.clone(),
        }));
    }

    fn visit_call_expr(&mut self, expr: Arc<Mutex<CallExpr<FeType>>>) -> Result<ir::RustIRExpr> {
        let mut expr = expr.lock().unwrap();

        if let Some(FeType::Callable(Callable {
            special: Some(SpecialCallable::Print),
            ..
        })) = expr.callee.0.lock().unwrap().resolved_type()
        {
            if expr.args.len() == 1 {
                match &mut *expr.args[0].value.0.lock().unwrap() {
                    Expr::PlainStringLiteral(literal) => {
                        return Ok(ir::RustIRExpr::MacroFnCall(ir::RustIRMacroFnCallExpr {
                            callee: "println".into(),
                            args: vec![ir::RustIRExpr::StringLiteral(
                                ir::RustIRStringLiteralExpr {
                                    literal: literal
                                        .lock()
                                        .unwrap()
                                        .literal
                                        .lexeme
                                        .clone()
                                        .replace("\\{", "{{")
                                        .replace("}", "}}")
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
                let value = arg.value.0.lock().unwrap();
                let arg_ir = value.accept(self)?;

                args.push(arg_ir);
            }

            return Ok(ir::RustIRExpr::MacroFnCall(ir::RustIRMacroFnCallExpr {
                callee: "println".into(),
                args,
            }));
        }

        let callee = expr.callee.0.lock().unwrap();
        let callee = Box::new(callee.accept(self)?);

        let mut args = vec![];

        // TODO: Handle named, variadic, optional, etc params
        for arg in &expr.args {
            let value = arg.value.0.lock().unwrap();
            let arg_ir = value.accept(self)?;

            args.push(arg_ir);
        }

        return Ok(ir::RustIRExpr::Call(ir::RustIRCallExpr { callee, args }));
    }

    fn visit_unary_expr(&mut self, expr: Arc<Mutex<UnaryExpr<FeType>>>) -> Result<ir::RustIRExpr> {
        let mut expr = expr.lock().unwrap();

        match &expr.op {
            UnaryOp::Ref(RefType::Shared { .. }) => {
                return Ok(ir::RustIRExpr::Unary(ir::RustIRUnaryExpr {
                    op: ir::RustIRUnaryOp::Ref(ir::RustIRRefType::Shared),
                    value: Box::new(expr.value.0.lock().unwrap().accept(self)?),
                }))
            }
            UnaryOp::Ref(RefType::Mut { .. }) => {
                return Ok(ir::RustIRExpr::Unary(ir::RustIRUnaryExpr {
                    op: ir::RustIRUnaryOp::Ref(ir::RustIRRefType::Mut),
                    value: Box::new(expr.value.0.lock().unwrap().accept(self)?),
                }))
            }
            UnaryOp::Not(_) => {
                return Ok(ir::RustIRExpr::Unary(ir::RustIRUnaryExpr {
                    op: ir::RustIRUnaryOp::Not,
                    value: Box::new(expr.value.0.lock().unwrap().accept(self)?),
                }));
            }
        }
    }

    fn visit_binary_expr(
        &mut self,
        expr: Arc<Mutex<BinaryExpr<FeType>>>,
    ) -> Result<ir::RustIRExpr> {
        let mut expr = expr.lock().unwrap();

        let lhs = Box::new(expr.lhs.0.lock().unwrap().accept(self)?);
        let rhs = Box::new(expr.rhs.0.lock().unwrap().accept(self)?);

        match &expr.op {
            BinaryOp::Add(_) => {
                return Ok(ir::RustIRExpr::Binary(ir::RustIRBinaryExpr {
                    lhs,
                    op: ir::RustIRBinaryOp::Add,
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
        let mut expr = expr.lock().unwrap();

        todo!()
    }

    fn visit_construct_expr(
        &mut self,
        expr: Arc<Mutex<ConstructExpr<FeType>>>,
    ) -> Result<ir::RustIRExpr> {
        let mut expr = expr.lock().unwrap();

        let target = match &mut expr.target {
            ConstructTarget::Ident(ident) => {
                ir::RustIRConstructTarget::Ident(ident.lock().unwrap().ident.lexeme.clone())
            }

            ConstructTarget::StaticPath(path) => {
                ir::RustIRConstructTarget::StaticPath(self.translate_static_path(path))
            }
        };

        let mut args = vec![];
        for arg in &mut expr.args {
            match arg {
                ConstructArg::Field(field) => {
                    args.push(ir::RustIRConstructArg {
                        name: field.name.lexeme.clone(),
                        value: field.value.0.lock().unwrap().accept(self)?,
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
        let mut expr = expr.lock().unwrap();

        let target = Box::new(expr.target.0.lock().unwrap().accept(self)?);

        return Ok(ir::RustIRExpr::Get(ir::RustIRGetExpr {
            target,
            name: expr.name.lexeme.clone(),
        }));
    }

    fn visit_if_expr(&mut self, expr: Arc<Mutex<IfExpr<FeType>>>) -> Result<ir::RustIRExpr> {
        let mut expr = expr.lock().unwrap();

        let condition = Box::new(expr.condition.0.lock().unwrap().accept(self)?);

        let then = match &expr.then {
            IfExprThen::Ternary(then) => {
                let expr = then.then_expr.0.lock().unwrap().accept(self)?;

                let stmt = ir::RustIRStmt::ImplicitReturn(ir::RustIRImplicitReturnStmt { expr });

                vec![stmt]
            }
            IfExprThen::Block(then) => {
                let mut then_stmts = vec![];

                for stmt in &then.block.stmts {
                    let stmts = stmt.lock().unwrap().accept(self)?;
                    then_stmts.extend(stmts);
                }

                if let Some(label) = &then.label {
                    let label = format!("'if_expr_then_{}", &label.lexeme[1..]).into();

                    vec![ir::RustIRStmt::ImplicitReturn(
                        ir::RustIRImplicitReturnStmt {
                            expr: ir::RustIRExpr::Loop(ir::RustIRLoopExpr {
                                label: Some(label),
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
                    let condition = Box::new(else_if.condition.0.lock().unwrap().accept(self)?);

                    let expr = else_if.expr.0.lock().unwrap().accept(self)?;

                    let then = vec![ir::RustIRStmt::ImplicitReturn(
                        ir::RustIRImplicitReturnStmt { expr },
                    )];

                    else_ifs.push(ir::RustIRElseIf { condition, then });
                }
                IfExprElseIf::Block(else_if) => {
                    let condition = Box::new(else_if.condition.0.lock().unwrap().accept(self)?);

                    let mut then = vec![];

                    for stmt in &else_if.block.stmts {
                        let stmts = stmt.lock().unwrap().accept(self)?;
                        then.extend(stmts);
                    }

                    then = if let Some(label) = &else_if.label {
                        let label =
                            format!("'if_expr_else_if_{}_{}", else_ifs.len(), &label.lexeme[1..])
                                .into();

                        vec![ir::RustIRStmt::ImplicitReturn(
                            ir::RustIRImplicitReturnStmt {
                                expr: ir::RustIRExpr::Loop(ir::RustIRLoopExpr {
                                    label: Some(label),
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
                    let expr = else_.else_expr.0.lock().unwrap().accept(self)?;

                    let then = vec![ir::RustIRStmt::ImplicitReturn(
                        ir::RustIRImplicitReturnStmt { expr },
                    )];

                    Some(ir::RustIRElse { then })
                }
                IfExprElse::Block(else_) => {
                    let mut then = vec![];

                    for stmt in &else_.block.stmts {
                        let stmts = stmt.lock().unwrap().accept(self)?;
                        then.extend(stmts);
                    }

                    then = if let Some(label) = &else_.label {
                        let label = format!("'if_expr_else_{}", &label.lexeme[1..]).into();

                        vec![ir::RustIRStmt::ImplicitReturn(
                            ir::RustIRImplicitReturnStmt {
                                expr: ir::RustIRExpr::Loop(ir::RustIRLoopExpr {
                                    label: Some(label),
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
        let mut expr = expr.lock().unwrap();

        todo!()
    }

    fn visit_while_expr(&mut self, expr: Arc<Mutex<WhileExpr<FeType>>>) -> Result<ir::RustIRExpr> {
        let mut expr = expr.lock().unwrap();

        todo!()
    }
}
