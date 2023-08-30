use crate::r#type::*;
use crate::syntax::*;

use crate::result::Result;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct FeTypeResolver {
    expr_lookup: HashMap<NodeId<Expr>, FeType>,
    decls_to_eval: HashMap<NodeId<Decl>, Arc<Mutex<Decl<Option<FeType>>>>>,

    scope: Arc<Mutex<Scope>>,
}

impl FeTypeResolver {
    pub fn resolve_package(pkg: FeSyntaxPackage) -> Result<FeSyntaxPackage<FeType>> {
        let pkg: Arc<Mutex<FeSyntaxPackage<Option<FeType>>>> = Arc::new(Mutex::new(pkg.into()));

        while !pkg.lock().unwrap().is_resolved() {
            let mut this = Self {
                expr_lookup: HashMap::new(),
                decls_to_eval: HashMap::new(),
                scope: Arc::new(Mutex::new(Scope::new())),
            };

            let changed = match &mut *pkg.lock().unwrap() {
                FeSyntaxPackage::File(file) => this.resolve_file(file)?,
                FeSyntaxPackage::Dir(dir) => this.resolve_dir(dir)?,
            };

            if !changed {
                todo!("Can't resolve! {pkg:#?}");
            }
        }

        let pkg: Mutex<FeSyntaxPackage<Option<FeType>>> =
            Arc::try_unwrap(pkg).expect("Why didn't this work?");

        let pkg: FeSyntaxPackage<Option<FeType>> = pkg.into_inner()?;

        return Ok(pkg.try_into()?);
    }

    fn internal_resolve_package(pkg: Arc<Mutex<FeSyntaxPackage<Option<FeType>>>>) -> Result<bool> {
        let mut this = Self {
            expr_lookup: HashMap::new(),
            decls_to_eval: HashMap::new(),
            scope: Arc::new(Mutex::new(Scope::new())),
        };

        match &mut *pkg.lock().unwrap() {
            FeSyntaxPackage::File(file) => return this.resolve_file(file),
            FeSyntaxPackage::Dir(dir) => return this.resolve_dir(dir),
        }
    }

    fn resolve_dir(&mut self, dir: &mut FeSyntaxDir<Option<FeType>>) -> Result<bool> {
        let mut changed = self.resolve_file(&mut dir.entry_file)?;

        for pkg in dir.local_packages.values_mut() {
            changed = changed || Self::internal_resolve_package(pkg.clone())?;
        }

        return Ok(changed);
    }

    fn resolve_file(&mut self, file: &mut FeSyntaxFile<Option<FeType>>) -> Result<bool> {
        let mut changed = None;

        let syntax = file.syntax.lock().unwrap();

        for u in &syntax.uses {
            let local = u.lock().unwrap().accept(self)?;

            if let Some(changed) = &mut changed {
                *changed = *changed && local;
            } else {
                changed = Some(local);
            }
        }

        for decl in &syntax.decls {
            let (id, decl_changed) = {
                let mut lock = decl.lock().unwrap();
                let decl = &mut lock;

                let id = *decl.node_id();
                let decl_changed = decl.accept(self)?;

                (id, decl_changed)
            };

            if !decl_changed {
                self.decls_to_eval.insert(id, decl.clone());
            }

            if let Some(changed) = &mut changed {
                *changed = *changed || decl_changed;
            } else {
                changed = Some(decl_changed);
            }
        }

        while !self.decls_to_eval.is_empty() {
            for (_, decl) in std::mem::take(&mut self.decls_to_eval) {
                let decl_changed = self.evaluate_decl(decl)?;

                if let Some(changed) = &mut changed {
                    *changed = *changed || decl_changed;
                } else {
                    changed = Some(decl_changed);
                }
            }
        }

        return Ok(changed.unwrap_or(false));
    }

    fn evaluate_decl(&mut self, decl: Arc<Mutex<Decl<Option<FeType>>>>) -> Result<bool> {
        match &mut *decl.lock().unwrap() {
            Decl::Fn(decl) => return self.evaluate_fn_decl(decl),
        }
    }

    fn evaluate_fn_decl(&mut self, decl: &mut FnDecl<Option<FeType>>) -> Result<bool> {
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
                    self.scope.lock().unwrap().insert(
                        "print".into(),
                        ScopedType {
                            is_pub: matches!(use_decl.use_mod, Some(UseMod::Pub(_))),

                            typ: FeType::Callable(Callable {
                                params: vec![("text".into(), FeType::String(None))],
                                return_type: None,
                            }),
                        },
                    );
                    next.path.details = Either::B(Some(FeType::Callable(Callable {
                        params: vec![("text".into(), FeType::String(None))],
                        return_type: None,
                    })));
                }
            }
        } else {
            todo!("{use_decl:#?}");
        }

        return Ok(true);
    }
}

impl DeclVisitor<Option<FeType>, Result<bool>> for FeTypeResolver {
    fn visit_function_decl(&mut self, decl: &mut FnDecl<Option<FeType>>) -> Result<bool> {
        // TODO: check and register fn params
        // TODO: check return

        self.scope.lock().unwrap().insert(
            decl.name.lexeme.clone(),
            ScopedType {
                is_pub: matches!(decl.decl_mod, Some(DeclMod::Pub(_))),
                typ: FeType::Callable(Callable {
                    params: vec![],
                    return_type: None,
                }),
            },
        );

        return Ok(false);
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

        let ident = &expr.ident.lexeme;

        if let Some(found) = self.scope.lock().unwrap().search(ident) {
            expr.resolved_type = Some(found.typ.clone());
            self.expr_lookup.insert(expr.id, found.typ.clone());
        } else {
            todo!("Can't find ident: {ident:?}");
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

        let Some(FeType::Callable(callee)) = self
            .expr_lookup
            .get(callee.node_id())
            .cloned()
        else {
            todo!("Callee not found: {callee:?}");
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
            let (_, param) = &callee.params[i];

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

struct Scope {
    stack: Vec<FlatScope>,
}

struct FlatScope {
    name_lookup: HashMap<Arc<str>, ScopedType>,
}

struct ScopedType {
    pub is_pub: bool,
    pub typ: FeType,
}

impl Scope {
    pub fn new() -> Self {
        return Self {
            stack: vec![FlatScope {
                name_lookup: HashMap::new(),
            }],
        };
    }

    pub fn begin_scope(&mut self) {
        self.stack.push(FlatScope {
            name_lookup: HashMap::new(),
        });
    }

    pub fn end_scope(&mut self) {
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }

    pub fn insert(&mut self, name: Arc<str>, typ: ScopedType) {
        self.stack.last_mut().unwrap().name_lookup.insert(name, typ);
    }

    pub fn search(&self, name: &str) -> Option<&ScopedType> {
        for data in self.stack.iter().rev() {
            if let Some(found) = data.name_lookup.get(name) {
                return Some(found);
            }
        }

        return None;
    }
}
