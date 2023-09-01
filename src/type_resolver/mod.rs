use crate::r#type::*;
use crate::syntax::*;

use crate::result::Result;

use crate::token::TokenType;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct FeTypeResolver {
    expr_lookup: HashMap<NodeId<Expr>, FeType>,
    decls_to_eval: HashMap<NodeId<Decl>, Arc<Mutex<Decl<Option<FeType>>>>>,

    scope: Arc<Mutex<Scope>>,

    root_pkg_exports: Arc<Mutex<ExportsPackage>>,
    current_pkg_exports: Arc<Mutex<ExportsPackage>>,
}

impl FeTypeResolver {
    pub fn resolve_package(pkg: FeSyntaxPackage) -> Result<FeSyntaxPackage<FeType>> {
        let exports = Arc::new(Mutex::new(match pkg {
            FeSyntaxPackage::File(_) => ExportsPackage::new_file(),
            FeSyntaxPackage::Dir(_) => ExportsPackage::new_dir(),
        }));

        let pkg: Arc<Mutex<FeSyntaxPackage<Option<FeType>>>> = Arc::new(Mutex::new(pkg.into()));

        let mut this = Self {
            expr_lookup: HashMap::new(),
            decls_to_eval: HashMap::new(),
            scope: Arc::new(Mutex::new(Scope::new())),

            root_pkg_exports: exports.clone(),
            current_pkg_exports: exports,
        };

        while !pkg.lock().unwrap().is_resolved() {
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

    fn internal_resolve_package(
        root_pkg_exports: Arc<Mutex<ExportsPackage>>,
        current_pkg_exports: Arc<Mutex<ExportsPackage>>,
        scope: Arc<Mutex<Scope>>,
        pkg: Arc<Mutex<FeSyntaxPackage<Option<FeType>>>>,
    ) -> Result<bool> {
        let mut this = Self {
            expr_lookup: HashMap::new(),
            decls_to_eval: HashMap::new(),
            scope,

            root_pkg_exports,
            current_pkg_exports,
        };

        match &mut *pkg.lock().unwrap() {
            FeSyntaxPackage::File(file) => return this.resolve_file(file),
            FeSyntaxPackage::Dir(dir) => return this.resolve_dir(dir),
        }
    }

    fn resolve_dir(&mut self, dir: &mut FeSyntaxDir<Option<FeType>>) -> Result<bool> {
        let mut changed = self.resolve_file(&mut dir.entry_file)?;

        for (name, pkg) in &dir.local_packages {
            let scope = {
                let ExportsPackage::Dir(dir) = &mut *self.current_pkg_exports.lock().unwrap() else {
                    todo!("how?")
                };

                let exports =
                    dir.local_packages
                        .entry(name.clone())
                        .or_insert(Arc::new(Mutex::new(match &*pkg.lock().unwrap() {
                            FeSyntaxPackage::File(_) => ExportsPackage::new_file(),
                            FeSyntaxPackage::Dir(_) => ExportsPackage::new_dir(),
                        })));

                let lock = exports.lock().unwrap();

                lock.scope()
            };

            changed = changed
                || Self::internal_resolve_package(
                    self.root_pkg_exports.clone(),
                    self.current_pkg_exports.clone(),
                    scope,
                    pkg.clone(),
                )?;
        }

        return Ok(changed);
    }

    fn resolve_file(&mut self, file: &mut FeSyntaxFile<Option<FeType>>) -> Result<bool> {
        let mut changed = None;

        let syntax = file.syntax.lock().unwrap();

        for u in &syntax.uses {
            let local = u.lock().unwrap().accept(self)?;

            if let Some(changed) = &mut changed {
                *changed = *changed || local;
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
            Decl::Fn(decl) => {
                self.scope.lock().unwrap().begin_scope();
                let res = self.evaluate_fn_decl(decl);
                self.scope.lock().unwrap().end_scope();

                return res;
            }
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
            let exports = match use_decl.path.pre {
                Some(UseStaticPathPre::RootDir(_)) => self.root_pkg_exports.clone(),
                Some(UseStaticPathPre::CurrentDir(_)) => self.current_pkg_exports.clone(),

                None | Some(UseStaticPathPre::DoubleColon(_)) => {
                    todo!("TODO: import dependencies and std lib")
                }
            };

            let found = match &*exports.lock().unwrap() {
                ExportsPackage::File(f) => todo!("{f:#?}"),
                ExportsPackage::Dir(d) => d
                    .local_packages
                    .get(&SyntaxPackageName(use_decl.path.name.lexeme.clone()))
                    .cloned(),
            };

            let Either::A(next) = &mut use_decl.path.details else {
                todo!()
            };

            let UseStaticPathNext::Single(next) = next else {
                todo!()
            };

            if let Some(found) = found {
                let typ = found
                    .lock()
                    .unwrap()
                    .scope()
                    .lock()
                    .unwrap()
                    .search(&next.path.name.lexeme)
                    .cloned();

                if let Some(typ) = typ {
                    if !typ.is_pub {
                        todo!("Not public!");
                    }

                    let Either::B(use_typ) = &mut next.path.details else {
                        todo!()
                    };
                    *use_typ = Some(typ.typ.clone());

                    self.scope.lock().unwrap().insert(
                        next.path.name.lexeme.clone(),
                        ScopedType {
                            is_pub: matches!(use_decl.use_mod, Some(UseMod::Pub(_))),
                            typ: typ.typ,
                        },
                    );
                } else {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
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

    fn visit_var_decl_stmt(&mut self, stmt: &mut VarDeclStmt<Option<FeType>>) -> Result<bool> {
        let mut changed = false;

        let typ = if let Some(value) = &stmt.value {
            let value = &mut *value.value.0.lock().unwrap();

            changed = changed || value.accept(self)?;

            value.resolved_type().cloned().flatten()
        } else {
            None
        };

        // TODO: check explicit types

        if let Some(typ) = typ {
            match &mut stmt.target {
                VarDeclTarget::Ident(ident) => {
                    self.scope.lock().unwrap().insert(
                        ident.ident.lexeme.clone(),
                        ScopedType { is_pub: false, typ },
                    );

                    changed = changed || ident.accept(self)?;
                }
            }
        }

        return Ok(changed);
    }
}

impl ExprVisitor<Option<FeType>, Result<bool>> for FeTypeResolver {
    fn visit_bool_literal_expr(
        &mut self,
        expr: &mut BoolLiteralExpr<Option<FeType>>,
    ) -> Result<bool> {
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

    fn visit_ident_expr(&mut self, expr: &mut IdentExpr<Option<FeType>>) -> Result<bool> {
        if expr.is_resolved() {
            return Ok(false);
        }

        let ident = &expr.ident.lexeme;

        dbg!(&self.scope);

        if let Some(found) = self.scope.lock().unwrap().search(ident) {
            expr.resolved_type = Some(found.typ.clone());
            self.expr_lookup.insert(expr.id, found.typ.clone());
        } else {
            dbg!(&expr);
            return Ok(false);
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
            // todo!("Callee not found: {callee:?}");
            return Ok(false);
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
}

#[derive(Debug, Clone)]
enum ExportsPackage {
    File(ExportsFile),
    Dir(ExportsDir),
}

impl ExportsPackage {
    pub fn new_file() -> Self {
        return Self::File(ExportsFile {
            scope: Arc::new(Mutex::new(Scope::new())),
        });
    }

    pub fn new_dir() -> Self {
        return Self::Dir(ExportsDir {
            entry: ExportsFile {
                scope: Arc::new(Mutex::new(Scope::new())),
            },
            local_packages: HashMap::new(),
        });
    }

    pub fn scope(&self) -> Arc<Mutex<Scope>> {
        match self {
            Self::File(file) => return file.scope.clone(),
            Self::Dir(dir) => return dir.entry.scope.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct ExportsFile {
    scope: Arc<Mutex<Scope>>,
}

#[derive(Debug, Clone)]
struct ExportsDir {
    entry: ExportsFile,
    local_packages: HashMap<SyntaxPackageName, Arc<Mutex<ExportsPackage>>>,
}

#[derive(Debug, Clone)]
struct Scope {
    stack: Vec<FlatScope>,
}

#[derive(Debug, Clone)]
struct FlatScope {
    name_lookup: HashMap<Arc<str>, ScopedType>,
}

#[derive(Debug, Clone)]
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

    pub fn update(&mut self, name: &str, typ: ScopedType) {
        for data in self.stack.iter_mut().rev() {
            if let Some(found) = data.name_lookup.get_mut(name) {
                *found = typ;
                return;
            }
        }
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
