use crate::r#type::*;
use crate::syntax::*;

use crate::result::Result;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct FeTypeResolver {
    expr_lookup: HashMap<NodeId<Expr>, FeType>,
}

impl FeTypeResolver {
    pub fn resolve_package(pkg: FeSyntaxPackage) -> Result<FeSyntaxPackage<FeType>> {
        let mut pkg: FeSyntaxPackage<Option<FeType>> = pkg.into();

        while !pkg.is_resolved() {
            let changed = match &mut pkg {
                FeSyntaxPackage::File(file) => Self::resolve_file(file)?,
                FeSyntaxPackage::Dir(dir) => Self::resolve_dir(dir)?,
            };

            if !changed {
                todo!("Can't resolve! {pkg:#?}");
            }
        }

        return Ok(pkg.try_into()?);
    }

    fn internal_resolve_package(pkg: Arc<Mutex<FeSyntaxPackage<Option<FeType>>>>) -> Result<bool> {
        match &mut *pkg.lock().unwrap() {
            FeSyntaxPackage::File(file) => return Self::resolve_file(file),
            FeSyntaxPackage::Dir(dir) => return Self::resolve_dir(dir),
        }
    }

    fn resolve_dir(dir: &mut FeSyntaxDir<Option<FeType>>) -> Result<bool> {
        let mut changed = Self::resolve_file(&mut dir.entry_file)?;

        for pkg in dir.local_packages.values_mut() {
            changed = changed && Self::internal_resolve_package(pkg.clone())?;
        }

        return Ok(changed);
    }

    fn resolve_file(file: &mut FeSyntaxFile<Option<FeType>>) -> Result<bool> {
        let mut this = Self {
            expr_lookup: HashMap::new(),
        };
        let mut changed = None;

        let syntax = file.syntax.lock().unwrap();

        for u in &syntax.uses {
            let local = u.lock().unwrap().accept(&mut this)?;

            if let Some(changed) = &mut changed {
                *changed = *changed && local;
            } else {
                changed = Some(local);
            }
        }

        for decl in &syntax.decls {
            let local = decl.lock().unwrap().accept(&mut this)?;

            if let Some(changed) = &mut changed {
                *changed = *changed && local;
            } else {
                changed = Some(local);
            }
        }

        return Ok(changed.unwrap_or(false));
    }

    fn can_implicit_cast(from: &FeType, to: &FeType) -> bool {
        match (from, to) {
            (FeType::String(_), FeType::String(_)) => return true,
            (FeType::String(_), FeType::Bool(_)) => return false,

            (FeType::Bool(_), FeType::Bool(_)) => return true,
            (FeType::Bool(_), FeType::String(_)) => return false,

            _ => todo!(),
        }
    }
}

impl UseVisitor<Option<FeType>, Result<bool>> for FeTypeResolver {
    fn visit_use(&mut self, use_decl: &mut Use<Option<FeType>>) -> Result<bool> {
        if use_decl.is_resolved() {
            return Ok(false);
        }

        if use_decl.path.name.lexeme.as_ref() == "fe" {
            if let Either::A(UseStaticPathNext::Single(next)) = &mut use_decl.path.details {
                if next.path.name.lexeme.as_ref() == "print" && next.path.details.is_b() {
                    next.path.details = Either::B(Some(FeType::Callable(Callable {
                        params: vec![("text".into(), FeType::String(None))],
                        return_type: None,
                    })));
                }
            }
        }

        return Ok(true);
    }
}

impl DeclVisitor<Option<FeType>, Result<bool>> for FeTypeResolver {
    fn visit_function_decl(&mut self, decl: &mut FnDecl<Option<FeType>>) -> Result<bool> {
        // TODO: register fn params

        match &mut decl.body {
            FnDeclBody::Short(body) => {
                todo!()
            }

            FnDeclBody::Block(body) => {
                let mut changed = false;

                for stmt in &mut body.stmts {
                    let stmt = &mut *stmt.lock().unwrap();

                    // TODO: Check for return stmt and compare to return type

                    changed = changed || stmt.accept(self)?;
                }

                return Ok(changed);
            }
        }
    }
}

impl StmtVisitor<Option<FeType>, Result<bool>> for FeTypeResolver {
    fn visit_expr_stmt(&mut self, stmt: &mut ExprStmt<Option<FeType>>) -> Result<bool> {
        return stmt.expr.lock().unwrap().accept(self);
    }
}

impl ExprVisitor<Option<FeType>, Result<bool>> for FeTypeResolver {
    fn visit_ident_expr(&mut self, expr: &mut IdentExpr<Option<FeType>>) -> Result<bool> {
        if expr.is_resolved() {
            return Ok(false);
        }

        match expr.ident.lexeme.as_ref() {
            "print" => {
                let resolved_type = FeType::Callable(Callable {
                    params: vec![("text".into(), FeType::String(None))],
                    return_type: None,
                });

                expr.resolved_type = Some(resolved_type.clone());
                self.expr_lookup.insert(expr.id, resolved_type);
            }

            ident => todo!("ident: {ident:?}"),
        }

        return Ok(true);
    }

    fn visit_call_expr(&mut self, expr: &mut CallExpr<Option<FeType>>) -> Result<bool> {
        if expr.is_resolved() {
            return Ok(false);
        }

        let mut callee = expr.callee.0.lock().unwrap();

        if !callee.is_resolved() {
            callee.accept(self)?;
        }

        // TODO: Lookup callee in self's scoped environment
        let callee = Callable {
            params: vec![("text".into(), FeType::String(None))],
            return_type: None,
        };

        if expr.args.len() > callee.params.len() {
            todo!("too many args!");
        }

        for i in 0..expr.args.len() {
            let arg = &mut expr.args[i];

            if !arg.is_resolved() {
                let mut expr = arg.value.0.lock().unwrap();
                let changed = expr.accept(self)?;

                if !changed {
                    todo!("uh oh");
                }

                let Some(resolved_type) = expr.resolved_type() else {
                    todo!("no type!");
                };

                arg.resolved_type = resolved_type.clone();
            }

            let Some(resolved_type) = &arg.resolved_type else {
                todo!("How did this get here??")
            };

            let (name, param) = &callee.params[i];

            if !Self::can_implicit_cast(resolved_type, param) {
                todo!("wrong type!");
            }
        }

        expr.resolved_type = callee.return_type.as_deref().map(|rt| Some(rt.clone()));

        return Ok(true);
    }

    fn visit_plain_string_literal_expr(
        &mut self,
        expr: &mut PlainStringLiteralExpr<Option<FeType>>,
    ) -> Result<bool> {
        if expr.is_resolved() {
            return Ok(false);
        }

        expr.resolved_type = Some(FeType::String(Some(StringDetails::PlainLiteral)));

        return Ok(true);
    }
}
