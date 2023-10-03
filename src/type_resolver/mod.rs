mod decl;
mod expr;
mod scope;
mod r#static;
mod stmt;
mod r#use;

use scope::*;

use crate::config::Config;
use crate::r#type::*;
use crate::syntax::*;

use crate::log;
use crate::result::Result;

use crate::token::Token;
use crate::token::TokenType;

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

type SharedDecl = Arc<Mutex<Decl<Option<FeType>>>>;

pub struct FeTypeResolver {
    cfg: Arc<Config>,

    expr_lookup: HashMap<NodeId<Expr>, FeType>,
    decls_to_eval: HashMap<NodeId<Decl>, SharedDecl>,

    scope: Arc<Mutex<Scope>>,

    root_pkg_exports: Arc<Mutex<ExportsPackage>>,
    current_pkg_exports: Arc<Mutex<ExportsPackage>>,

    current_return_type: Option<Option<FeType>>,
    breakable_count: usize,
    thenable_count: usize,
}

impl FeTypeResolver {
    pub fn resolve_package(
        cfg: Arc<Config>,
        pkg: FeSyntaxPackage,
    ) -> Result<FeSyntaxPackage<FeType>> {
        let exports = Arc::new(Mutex::new(match pkg {
            FeSyntaxPackage::File(_) => ExportsPackage::new_file(),
            FeSyntaxPackage::Dir(_) => ExportsPackage::new_dir(),
        }));

        let pkg: Arc<Mutex<FeSyntaxPackage<Option<FeType>>>> = Arc::new(Mutex::new(pkg.into()));

        let mut this = Self {
            cfg,
            expr_lookup: HashMap::new(),
            decls_to_eval: HashMap::new(),
            scope: Arc::new(Mutex::new(Scope::new())),

            root_pkg_exports: exports.clone(),
            current_pkg_exports: exports,

            current_return_type: None,
            breakable_count: 0,
            thenable_count: 0,
        };

        while !pkg.try_lock().unwrap().is_resolved() {
            log::trace!(1);
            let changed = match &mut *pkg.try_lock().unwrap() {
                FeSyntaxPackage::File(file) => this.resolve_file(file)?,
                FeSyntaxPackage::Dir(dir) => this.resolve_dir(dir)?,
            };
            log::trace!(2);

            if !changed {
                todo!("Can't resolve!");
            }
        }

        let pkg: Mutex<FeSyntaxPackage<Option<FeType>>> =
            Arc::try_unwrap(pkg).expect("Why didn't this work?");

        let pkg: FeSyntaxPackage<Option<FeType>> = pkg.into_inner()?;

        return Ok(pkg.try_into()?);
    }

    fn internal_resolve_package(
        cfg: Arc<Config>,
        root_pkg_exports: Arc<Mutex<ExportsPackage>>,
        current_pkg_exports: Arc<Mutex<ExportsPackage>>,
        scope: Arc<Mutex<Scope>>,
        pkg: Arc<Mutex<FeSyntaxPackage<Option<FeType>>>>,
    ) -> Result<bool> {
        let mut this = Self {
            cfg,
            expr_lookup: HashMap::new(),
            decls_to_eval: HashMap::new(),
            scope,

            root_pkg_exports,
            current_pkg_exports,

            current_return_type: None,
            breakable_count: 0,
            thenable_count: 0,
        };

        match &mut *pkg.try_lock().unwrap() {
            FeSyntaxPackage::File(file) => return this.resolve_file(file),
            FeSyntaxPackage::Dir(dir) => return this.resolve_dir(dir),
        }
    }

    fn resolve_dir(&mut self, dir: &mut FeSyntaxDir<Option<FeType>>) -> Result<bool> {
        let mut changed = self.resolve_file(&mut dir.entry_file)?;
        let mut is_changed = changed;

        while changed {
            changed = self.resolve_file(&mut dir.entry_file)?;
        }

        for (name, pkg) in &dir.local_packages {
            let scope = {
                let ExportsPackage::Dir(dir) = &mut *self.current_pkg_exports.try_lock().unwrap()
                else {
                    todo!("how?")
                };

                let exports =
                    dir.local_packages
                        .entry(name.clone())
                        .or_insert(Arc::new(Mutex::new(match &*pkg.try_lock().unwrap() {
                            FeSyntaxPackage::File(_) => ExportsPackage::new_file(),
                            FeSyntaxPackage::Dir(_) => ExportsPackage::new_dir(),
                        })));

                let lock = exports.try_lock().unwrap();

                lock.scope()
            };

            changed = Self::internal_resolve_package(
                self.cfg.clone(),
                self.root_pkg_exports.clone(),
                self.current_pkg_exports.clone(),
                scope.clone(),
                pkg.clone(),
            )?;
            is_changed |= changed;

            while changed {
                changed = Self::internal_resolve_package(
                    self.cfg.clone(),
                    self.root_pkg_exports.clone(),
                    self.current_pkg_exports.clone(),
                    scope.clone(),
                    pkg.clone(),
                )?;
            }
        }

        let mut changed = self.resolve_file(&mut dir.entry_file)?;
        is_changed |= changed;

        while changed {
            changed = self.resolve_file(&mut dir.entry_file)?;
        }

        return Ok(is_changed);
    }

    fn resolve_file(&mut self, file: &mut FeSyntaxFile<Option<FeType>>) -> Result<bool> {
        self.fill_scope_with_global_imports()?;

        let mut changed = None;

        let syntax = file.syntax.try_lock().unwrap();

        for u in &syntax.uses {
            let local = u.accept(self)?;

            if let Some(changed) = &mut changed {
                *changed |= local;
            } else {
                changed = Some(local);
            }
        }

        for decl in &syntax.decls {
            let (id, decl_changed) = {
                let mut lock = decl.try_lock().unwrap();
                let decl = &mut lock;

                let id = decl.node_id();
                let decl_changed = decl.accept(self)?;

                (id, decl_changed)
            };

            self.decls_to_eval.insert(id, decl.clone());

            if let Some(changed) = &mut changed {
                *changed |= decl_changed;
            } else {
                changed = Some(decl_changed);
            }
        }

        if !changed.unwrap_or(true) {
            while !self.decls_to_eval.is_empty() {
                for (_, decl) in std::mem::take(&mut self.decls_to_eval) {
                    let decl_changed = self.evaluate_decl(decl)?;

                    if let Some(changed) = &mut changed {
                        *changed |= decl_changed;
                    } else {
                        changed = Some(decl_changed);
                    }
                }
            }
        }

        return Ok(changed.unwrap_or(false));
    }

    fn fill_scope_with_global_imports(&mut self) -> Result {
        let scope = &mut *self.scope.try_lock().unwrap();

        scope.insert(
            INT_TYPE_NAME.into(),
            ScopedType {
                is_pub: false,
                typ: FeType::Number(Some(NumberDetails::Integer(None))),
            },
        );

        scope.insert(
            STRING_TYPE_NAME.into(),
            ScopedType {
                is_pub: false,
                typ: FeType::String(None),
            },
        );

        scope.insert(
            BOOL_TYPE_NAME.into(),
            ScopedType {
                is_pub: false,
                typ: FeType::Bool(None),
            },
        );

        return Ok(());
    }

    fn evaluate_decl(&mut self, decl: Arc<Mutex<Decl<Option<FeType>>>>) -> Result<bool> {
        match &mut *decl.try_lock().unwrap() {
            Decl::Fn(shared_decl) => {
                log::trace!(&self.scope);

                let decl = &mut *shared_decl.try_lock().unwrap();

                if let Some(return_type) = &decl.return_type {
                    if let Some(return_type) = &return_type.resolved_type {
                        self.current_return_type = Some(Some(return_type.clone()));
                    } else {
                        // There is a return type, but haven't resolved it yet?
                        todo!("I don't think this should ever happen?");
                    }
                } else {
                    self.current_return_type = Some(None);
                }

                self.scope
                    .try_lock()
                    .unwrap()
                    .begin_scope(Some(ScopeCreator::Fn(shared_decl.clone())));

                let res = self.evaluate_fn_decl(decl);

                self.scope.try_lock().unwrap().end_scope();

                self.current_return_type = None;

                return res;
            }

            Decl::Struct(_) => {
                // TODO: Check struct field defaults? Otherwise not much to do
                return Ok(false);
            }
        }
    }

    fn evaluate_fn_decl(&mut self, decl: &mut FnDecl<Option<FeType>>) -> Result<bool> {
        // Add fn params to scope
        {
            let mut scope = self.scope.try_lock().unwrap();

            for param in &decl.params {
                if let Some(resolved_type) = &param.resolved_type {
                    scope.insert(
                        param.name.lexeme.clone(),
                        ScopedType {
                            is_pub: false,
                            typ: resolved_type.clone(),
                        },
                    );
                }
            }
        }

        match &mut decl.body {
            FnDeclBody::Short(_body) => {
                todo!()
            }

            FnDeclBody::Block(body) => {
                let mut changed = false;

                changed |= self.resolve_stmts(&body.stmts)?.0;

                return Ok(changed);
            }
        }
    }

    #[allow(clippy::type_complexity)]
    fn resolve_stmts(
        &mut self,
        stmts: &[Arc<Mutex<Stmt<Option<FeType>>>>],
    ) -> Result<(bool, Option<Arc<Mutex<Stmt<Option<FeType>>>>>)> {
        let mut changed = false;

        let mut terminal = None;
        for stmt in stmts {
            if let Some(terminal) = &terminal {
                todo!("Unreachable code after {terminal:#?}!");
            }

            let s = &mut *stmt.try_lock().unwrap();
            changed |= s.accept(self)?;

            if s.is_terminal() {
                terminal = Some(stmt.clone());
            }
        }

        if terminal.is_some() {
            return Ok((changed, terminal));
        }

        return Ok((changed, None));
    }

    fn can_implicit_cast(from: &FeType, to: &FeType) -> bool {
        match (from, to) {
            (FeType::Ref(from), FeType::Ref(to)) => {
                if from.ref_type == FeRefType::Const && to.ref_type == FeRefType::Mut {
                    return false;
                }

                return Self::can_implicit_cast(&from.of, &to.of);
            }

            (owned, FeType::Ref(FeRefOf { of, .. })) => {
                return Self::can_implicit_cast(owned, of);
            }

            (FeType::Ref(_), _owned) => return false,

            (FeType::Owned(from), to) => {
                return Self::can_implicit_cast(&from.of, to);
            }

            (from, FeType::Owned(to)) => {
                return Self::can_implicit_cast(from, &to.of);
            }

            (FeType::String(_), FeType::String(_)) => return true,
            (FeType::String(_), FeType::Bool(_)) => return false,

            (FeType::Bool(_), FeType::Bool(_)) => return true,
            (FeType::Bool(_), FeType::String(_)) => return false,

            (FeType::Number(from_details), FeType::Number(to_details)) => {
                match (from_details, to_details) {
                    (_, None) => return true,

                    (
                        Some(NumberDetails::Decimal(from_val)),
                        Some(NumberDetails::Decimal(to_val)),
                    ) => match (from_val, to_val) {
                        // TODO
                        // (Some(from_val), Some(to_val)) => return *from_val == *to_val,
                        (None, Some(_)) => false,
                        _ => true,
                    },

                    (Some(NumberDetails::Decimal(_)), _) => return false,

                    (
                        Some(NumberDetails::Integer(from_val)),
                        Some(NumberDetails::Integer(to_val)),
                    ) => match (from_val, to_val) {
                        // (Some(from_val), Some(to_val)) => return *from_val == *to_val,
                        (None, Some(_)) => false,
                        _ => true,
                    },
                    (
                        Some(NumberDetails::Integer(from_val)),
                        Some(NumberDetails::Decimal(to_val)),
                    ) => match (from_val, to_val) {
                        // (Some(from_val), Some(to_val)) => return *from_val as f64 == *to_val,
                        (None, Some(_)) => false,
                        _ => true,
                    },

                    (None, Some(NumberDetails::Integer(_))) => return false,
                    (None, Some(NumberDetails::Decimal(_))) => return true,
                }
            }

            _ => todo!("Can you cast?\nThis: {from:#?}\nTo: {to:#?}"),
        }
    }
}
