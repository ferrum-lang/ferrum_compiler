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
            use_decl.lock().unwrap().accept(self)?;
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
    fn visit_use(&mut self, use_decl: &mut Use<FeType>) -> Result {
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
    fn visit_function_decl(&mut self, decl: &mut FnDecl<FeType>) -> Result {
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
}

impl StmtVisitor<FeType, Result<Vec<ir::RustIRStmt>>> for RustSyntaxCompiler {
    fn visit_expr_stmt(&mut self, stmt: &mut ExprStmt<FeType>) -> Result<Vec<ir::RustIRStmt>> {
        let expr = stmt.expr.lock().unwrap().accept(self)?;

        return Ok(vec![ir::RustIRStmt::Expr(ir::RustIRExprStmt { expr })]);
    }

    fn visit_var_decl_stmt(
        &mut self,
        stmt: &mut VarDeclStmt<FeType>,
    ) -> Result<Vec<ir::RustIRStmt>> {
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
                    name: ident.ident.lexeme.clone(),
                    explicit_type: None,
                    value,
                })])
            }
        }
    }

    fn visit_assign_stmt(&mut self, stmt: &mut AssignStmt<FeType>) -> Result<Vec<ir::RustIRStmt>> {
        let lhs = stmt.target.0.lock().unwrap().accept(self)?;
        let rhs = stmt.value.0.lock().unwrap().accept(self)?;

        return Ok(vec![ir::RustIRStmt::Expr(ir::RustIRExprStmt {
            expr: ir::RustIRExpr::Assign(ir::RustIRAssignExpr {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }),
        })]);
    }

    fn visit_return_stmt(&mut self, stmt: &mut ReturnStmt<FeType>) -> Result<Vec<ir::RustIRStmt>> {
        let expr = if let Some(value) = &mut stmt.value {
            Some(value.0.lock().unwrap().accept(self)?)
        } else {
            None
        };

        return Ok(vec![ir::RustIRStmt::Return(ir::RustIRReturnStmt { expr })]);
    }

    fn visit_if_stmt(&mut self, stmt: &mut IfStmt<FeType>) -> Result<Vec<ir::RustIRStmt>> {
        let if_condition = Box::new(stmt.condition.0.lock().unwrap().accept(self)?);

        let mut if_then_block = vec![];
        for stmt in &mut stmt.then_block.stmts {
            let stmts = stmt.lock().unwrap().accept(self)?;
            if_then_block.extend(stmts.into_iter());
        }

        let mut else_ifs = vec![];
        for else_if in &mut stmt.else_ifs {
            let condition = Box::new(else_if.condition.0.lock().unwrap().accept(self)?);

            let mut then_block = vec![];
            for stmt in &mut else_if.then_block.stmts {
                let stmts = stmt.lock().unwrap().accept(self)?;
                then_block.extend(stmts.into_iter());
            }

            else_ifs.push(RustIRElseIf {
                condition,
                then: then_block,
            });
        }

        let else_ = if let Some(else_) = &mut stmt.else_ {
            let mut then_block = vec![];
            for stmt in &mut else_.then_block.stmts {
                let stmts = stmt.lock().unwrap().accept(self)?;
                then_block.extend(stmts.into_iter());
            }

            Some(ir::RustIRElse { then: then_block })
        } else {
            None
        };

        return Ok(vec![ir::RustIRStmt::ImplicitReturn(
            ir::RustIRImplicitReturnStmt {
                expr: ir::RustIRExpr::If(ir::RustIRIfExpr {
                    condition: if_condition,
                    then: if_then_block,
                    else_ifs,
                    else_,
                }),
            },
        )]);
    }

    fn visit_loop_stmt(&mut self, stmt: &mut LoopStmt<FeType>) -> Result<Vec<ir::RustIRStmt>> {
        let mut stmts = vec![];

        for stmt in &mut stmt.block.stmts {
            let ir_stmts = stmt.lock().unwrap().accept(self)?;
            stmts.extend(ir_stmts);
        }

        return Ok(vec![ir::RustIRStmt::Loop(ir::RustIRLoopStmt { stmts })]);
    }

    fn visit_while_stmt(&mut self, stmt: &mut WhileStmt<FeType>) -> Result<Vec<ir::RustIRStmt>> {
        let condition = stmt.condition.0.lock().unwrap().accept(self)?;

        let mut stmts = vec![];

        for stmt in &mut stmt.block.stmts {
            let ir_stmts = stmt.lock().unwrap().accept(self)?;
            stmts.extend(ir_stmts);
        }

        return Ok(vec![ir::RustIRStmt::While(ir::RustIRWhileStmt {
            condition,
            stmts,
        })]);
    }

    fn visit_break_stmt(&mut self, _stmt: &mut BreakStmt<FeType>) -> Result<Vec<ir::RustIRStmt>> {
        return Ok(vec![ir::RustIRStmt::Break(ir::RustIRBreakStmt {})]);
    }
}

impl ExprVisitor<FeType, Result<ir::RustIRExpr>> for RustSyntaxCompiler {
    fn visit_bool_literal_expr(
        &mut self,
        expr: &mut BoolLiteralExpr<FeType>,
    ) -> Result<ir::RustIRExpr> {
        return Ok(ir::RustIRExpr::BoolLiteral(ir::RustIRBoolLiteralExpr {
            literal: expr.resolved_type == FeType::Bool(Some(true)),
        }));
    }

    fn visit_plain_string_literal_expr(
        &mut self,
        expr: &mut PlainStringLiteralExpr<FeType>,
    ) -> Result<ir::RustIRExpr> {
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
        expr: &mut FmtStringLiteralExpr<FeType>,
    ) -> Result<ir::RustIRExpr> {
        let mut fmt_str = expr.first.lexeme.to_string();
        for part in &expr.rest {
            fmt_str.push_str(&part.string);
        }

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

    fn visit_ident_expr(&mut self, expr: &mut IdentExpr<FeType>) -> Result<ir::RustIRExpr> {
        return Ok(ir::RustIRExpr::Ident(ir::RustIRIdentExpr {
            ident: expr.ident.lexeme.clone(),
        }));
    }

    fn visit_call_expr(&mut self, expr: &mut CallExpr<FeType>) -> Result<ir::RustIRExpr> {
        if let Some(FeType::Callable(Callable {
            special: Some(SpecialCallable::Print),
            ..
        })) = expr.callee.0.lock().unwrap().resolved_type()
        {
            if expr.args.len() == 1 {
                if let Expr::PlainStringLiteral(literal) = &*expr.args[0].value.0.lock().unwrap() {
                    return Ok(ir::RustIRExpr::MacroFnCall(ir::RustIRMacroFnCallExpr {
                        callee: "println".into(),
                        args: vec![ir::RustIRExpr::StringLiteral(ir::RustIRStringLiteralExpr {
                            literal: literal.literal.lexeme.clone(),
                        })],
                    }));
                }
            }

            let mut args = vec![ir::RustIRExpr::StringLiteral(ir::RustIRStringLiteralExpr {
                literal: "\"{}\"".into(),
            })];

            for arg in &mut expr.args {
                let mut value = arg.value.0.lock().unwrap();
                let arg_ir = value.accept(self)?;

                args.push(arg_ir);
            }

            return Ok(ir::RustIRExpr::MacroFnCall(ir::RustIRMacroFnCallExpr {
                callee: "println".into(),
                args,
            }));
        }

        let mut callee = expr.callee.0.lock().unwrap();
        let callee = Box::new(callee.accept(self)?);

        let mut args = vec![];

        // TODO: Handle named, variadic, optional, etc params
        for arg in &mut expr.args {
            let mut value = arg.value.0.lock().unwrap();
            let arg_ir = value.accept(self)?;

            args.push(arg_ir);
        }

        return Ok(ir::RustIRExpr::Call(ir::RustIRCallExpr { callee, args }));
    }

    fn visit_unary_expr(&mut self, expr: &mut UnaryExpr<FeType>) -> Result<ir::RustIRExpr> {
        match expr.op {
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
}
