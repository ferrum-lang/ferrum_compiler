use crate::r#type::*;
use crate::syntax::*;

use crate::result::Result;

use crate::token::Token;
use crate::token::TokenType;

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

type SharedDecl = Arc<Mutex<Decl<Option<FeType>>>>;

pub struct FeTypeResolver {
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

            current_return_type: None,
            breakable_count: 0,
            thenable_count: 0,
        };

        while !pkg.try_lock().unwrap().is_resolved() {
            println!("1");
            let changed = match &mut *pkg.try_lock().unwrap() {
                FeSyntaxPackage::File(file) => this.resolve_file(file)?,
                FeSyntaxPackage::Dir(dir) => this.resolve_dir(dir)?,
            };
            dbg!("2");

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
                self.root_pkg_exports.clone(),
                self.current_pkg_exports.clone(),
                scope.clone(),
                pkg.clone(),
            )?;
            is_changed |= changed;

            while changed {
                changed = Self::internal_resolve_package(
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

    fn evaluate_decl(&mut self, decl: Arc<Mutex<Decl<Option<FeType>>>>) -> Result<bool> {
        match &mut *decl.try_lock().unwrap() {
            Decl::Fn(shared_decl) => {
                // dbg!(&self.scope);
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

impl UseVisitor<Option<FeType>, Result<bool>> for FeTypeResolver {
    fn visit_use(&mut self, shared_use_decl: Arc<Mutex<Use<Option<FeType>>>>) -> Result<bool> {
        let use_decl = &mut *shared_use_decl.try_lock().unwrap();

        if use_decl.is_resolved() {
            return Ok(false);
        }

        if use_decl.path.name.lexeme.as_ref() == "fe" {
            if let Either::A(UseStaticPathNext::Single(next)) = &mut use_decl.path.details {
                if next.path.name.lexeme.as_ref() == "print" && next.path.details.is_b() {
                    self.scope.try_lock().unwrap().insert(
                        "print".into(),
                        ScopedType {
                            is_pub: matches!(use_decl.use_mod, Some(UseMod::Pub(_))),

                            typ: FeType::Callable(Callable {
                                special: Some(SpecialCallable::Print),
                                name: "print".into(),
                                params: vec![("text".into(), FeType::String(None))],
                                return_type: None,
                            }),
                        },
                    );
                    next.path.details = Either::B(Some(FeType::Callable(Callable {
                        special: Some(SpecialCallable::Print),
                        name: "print".into(),
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

            let found = match &*exports.try_lock().unwrap() {
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
                    .try_lock()
                    .unwrap()
                    .scope()
                    .try_lock()
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

                    self.scope.try_lock().unwrap().insert(
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

impl StaticVisitor<Option<FeType>, Result<bool>> for FeTypeResolver {
    fn visit_static_type(&mut self, static_type: &mut StaticType<Option<FeType>>) -> Result<bool> {
        if static_type.is_resolved() {
            return Ok(false);
        }

        let mut changed = static_type.static_path.accept(self)?;

        // TODO: Handle references
        match static_type.ref_type {
            Some(RefType::Shared { .. }) => {
                static_type.resolved_type =
                    static_type
                        .static_path
                        .resolved_type
                        .clone()
                        .map(|resolved_type| {
                            FeType::Ref(FeRefOf {
                                ref_type: FeRefType::Const,
                                of: Box::new(resolved_type),
                            })
                        });
            }

            Some(RefType::Mut { .. }) => {
                static_type.resolved_type =
                    static_type
                        .static_path
                        .resolved_type
                        .clone()
                        .map(|resolved_type| {
                            FeType::Ref(FeRefOf {
                                ref_type: FeRefType::Mut,
                                of: Box::new(resolved_type),
                            })
                        });
            }

            None => static_type.resolved_type = static_type.static_path.resolved_type.clone(),
        }

        if !changed && static_type.static_path.is_resolved() {
            changed = true;
        }

        return Ok(changed);
    }

    fn visit_static_path(&mut self, static_path: &mut StaticPath<Option<FeType>>) -> Result<bool> {
        if static_path.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        if let Some(root) = &mut static_path.root {
            changed |= root.accept(self)?;

            // TODO: Handle package types and navigating scope
        } else {
            match static_path.name.lexeme.as_ref() {
                "String" => {
                    static_path.resolved_type = Some(FeType::String(None));
                    changed = true;
                }

                "Bool" => {
                    static_path.resolved_type = Some(FeType::Bool(None));
                    changed = true;
                }

                "Int" => {
                    static_path.resolved_type =
                        Some(FeType::Number(Some(NumberDetails::Integer(None))));
                    changed = true;
                }

                other => todo!("Check scope for imported type: {other}"),
            }
        }

        return Ok(changed);
    }
}

impl DeclVisitor<Option<FeType>, Result<bool>> for FeTypeResolver {
    fn visit_function_decl(
        &mut self,
        shared_decl: Arc<Mutex<FnDecl<Option<FeType>>>>,
    ) -> Result<bool> {
        {
            let decl = &mut *shared_decl.try_lock().unwrap();

            if decl.is_signature_resolved() {
                return Ok(false);
            }
        }

        let mut changed = false;

        let mut params = vec![];
        let mut all_resolved = true;

        {
            let decl = &mut *shared_decl.try_lock().unwrap();

            for param in &mut decl.params {
                if let Some(resolved_type) = &param.resolved_type {
                    params.push((param.name.lexeme.clone(), resolved_type.clone()));
                } else {
                    changed |= param.static_type_ref.accept(self)?;
                    param.resolved_type = param.static_type_ref.resolved_type.clone();

                    if let Some(resolved_type) = &param.resolved_type {
                        params.push((param.name.lexeme.clone(), resolved_type.clone()));
                    } else {
                        all_resolved = false;
                    }
                }
            }
        }

        let mut fn_return_type = None;

        {
            let decl = &mut *shared_decl.try_lock().unwrap();

            if let Some(return_type) = &mut decl.return_type {
                if let Some(resolved_type) = &return_type.resolved_type {
                    fn_return_type = Some(Box::new(resolved_type.clone()));
                } else {
                    changed |= return_type.static_type.accept(self)?;
                    return_type.resolved_type = return_type.static_type.resolved_type.clone();

                    if let Some(resolved_type) = &mut return_type.resolved_type {
                        fn_return_type = Some(Box::new(resolved_type.clone()));
                    } else {
                        all_resolved = false;
                    }
                }
            }
        }

        if all_resolved {
            let decl = &mut *shared_decl.try_lock().unwrap();

            changed = true;
            self.scope.try_lock().unwrap().insert(
                decl.name.lexeme.clone(),
                ScopedType {
                    is_pub: matches!(decl.decl_mod, Some(DeclMod::Pub(_))),
                    typ: FeType::Callable(Callable {
                        special: None,
                        name: decl.name.lexeme.clone(),
                        params,
                        return_type: fn_return_type,
                    }),
                },
            );

            decl.has_resolved_signature = true;
        }

        return Ok(changed);
    }

    fn visit_struct_decl(
        &mut self,
        shared_decl: Arc<Mutex<StructDecl<Option<FeType>>>>,
    ) -> Result<bool> {
        let decl = &mut *shared_decl.try_lock().unwrap();

        if decl.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        // TODO: Generics

        let mut fields = vec![];
        let mut all_done = true;
        for field in &mut decl.fields {
            changed |= field.static_type_ref.accept(self)?;

            if let Some(resolved) = &field.static_type_ref.resolved_type {
                fields.push(FeStructField {
                    is_pub: matches!(field.field_mod, Some(StructFieldMod::Pub(_))),
                    name: field.name.lexeme.clone(),
                    typ: resolved.clone(),
                });
            } else {
                all_done = false;
            }
        }

        if all_done {
            changed = true;
            self.scope.try_lock().unwrap().insert(
                decl.name.lexeme.clone(),
                ScopedType {
                    is_pub: matches!(decl.decl_mod, Some(DeclMod::Pub(_))),
                    typ: FeType::Struct(FeStruct {
                        special: None,
                        name: decl.name.lexeme.clone(),
                        fields,
                    }),
                },
            );
        }

        return Ok(changed);
    }
}

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
            dbg!(&stmt, &self.scope);
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
                dbg!(&stmt, &self.scope);
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

impl ExprVisitor<Option<FeType>, Result<bool>> for FeTypeResolver {
    fn visit_bool_literal_expr(
        &mut self,
        shared_expr: Arc<Mutex<BoolLiteralExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let mut expr = shared_expr.try_lock().unwrap();

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

    fn visit_number_literal_expr(
        &mut self,
        shared_expr: Arc<Mutex<NumberLiteralExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let mut expr = shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        expr.resolved_type = Some(FeType::Number(Some(match expr.details {
            NumberLiteralDetails::Integer(val) => NumberDetails::Integer(Some(val as i64)),
            NumberLiteralDetails::Decimal(val) => NumberDetails::Decimal(Some(val)),
        })));

        return Ok(true);
    }

    fn visit_plain_string_literal_expr(
        &mut self,
        shared_expr: Arc<Mutex<PlainStringLiteralExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let mut expr = shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        expr.resolved_type = Some(FeType::String(Some(StringDetails::PlainLiteral)));

        return Ok(true);
    }

    fn visit_fmt_string_literal_expr(
        &mut self,
        shared_expr: Arc<Mutex<FmtStringLiteralExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let mut expr = shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;
        let mut is_all_checked = true;

        for part in &mut expr.rest {
            changed |= part.expr.0.try_lock().unwrap().accept(self)?;

            if !part.expr.0.try_lock().unwrap().is_resolved() {
                is_all_checked = false;
            }
        }

        if is_all_checked {
            expr.resolved_type = Some(FeType::String(Some(StringDetails::Format)));
            changed = true;
        }

        return Ok(changed);
    }

    fn visit_ident_expr(
        &mut self,
        shared_expr: Arc<Mutex<IdentExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let mut expr = shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let ident = &expr.ident.lexeme;

        if let Some(found) = self.scope.try_lock().unwrap().search(ident) {
            expr.resolved_type = Some(found.typ.clone());
            self.expr_lookup.insert(expr.id, found.typ.clone());
        } else {
            return Ok(false);
        }

        return Ok(true);
    }

    fn visit_call_expr(
        &mut self,
        shared_expr: Arc<Mutex<CallExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let expr = &mut *shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        let callee = &mut *expr.callee.0.try_lock().unwrap();

        if !callee.is_resolved() {
            changed |= callee.accept(self)?;
        }

        let Some(FeType::Callable(callee)) = self.expr_lookup.get(&callee.node_id()).cloned()
        else {
            // todo!("Callee not found: {callee:?}");
            return Ok(false);
        };

        if expr.args.len() > callee.params.len() {
            todo!(
                "too many args!\nExpected: {:#?}\nGot: {:#?}",
                callee.params,
                expr.args
            );
        }

        for i in 0..expr.args.len() {
            let arg = &mut expr.args[i];

            if !arg.is_resolved() {
                let expr = &mut *arg.value.0.try_lock().unwrap();
                let local_changed = expr.accept(self)?;

                if !local_changed {
                    continue;
                }
                changed = true;

                let Some(resolved_type) = expr.resolved_type() else {
                    continue;
                };

                arg.resolved_type = resolved_type.clone();
            }

            let Some(resolved_type) = &arg.resolved_type else {
                todo!("How did this get here??")
            };
            let (_, param) = &callee.params[i];

            if !Self::can_implicit_cast(resolved_type, param) {
                todo!("wrong type!\nCannot implicitly cast {resolved_type:#?}\nto {param:#?}");
            }
        }

        expr.resolved_type = callee.return_type.as_deref().map(|rt| Some(rt.clone()));

        return Ok(changed);
    }

    fn visit_unary_expr(
        &mut self,
        shared_expr: Arc<Mutex<UnaryExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let expr = &mut *shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        changed |= expr.value.0.try_lock().unwrap().accept(self)?;

        if let Some(resolved_type) = expr.value.0.try_lock().unwrap().resolved_type().flatten() {
            changed = true;

            match &expr.op {
                UnaryOp::Ref(RefType::Shared { .. }) => {
                    expr.resolved_type = Some(FeType::Ref(FeRefOf {
                        ref_type: FeRefType::Const,
                        of: Box::new(resolved_type),
                    }));
                }
                UnaryOp::Ref(RefType::Mut { .. }) => {
                    expr.resolved_type = Some(FeType::Ref(FeRefOf {
                        ref_type: FeRefType::Mut,
                        of: Box::new(resolved_type),
                    }));
                }
                UnaryOp::Not(_) => {
                    // Only apply to bool
                    if !Self::can_implicit_cast(&resolved_type, &FeType::Bool(None)) {
                        todo!("Can't cast implicitly to bool");
                    }

                    expr.resolved_type = if let FeType::Bool(details) = &resolved_type {
                        Some(FeType::Bool(details.map(|known_val| !known_val)))
                    } else {
                        Some(FeType::Bool(None))
                    };
                }
            }
        }

        return Ok(changed);
    }

    fn visit_binary_expr(
        &mut self,
        shared_expr: Arc<Mutex<BinaryExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let expr = &mut *shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        changed |= expr.lhs.0.try_lock().unwrap().accept(self)?;
        changed |= expr.rhs.0.try_lock().unwrap().accept(self)?;

        if let (Some(resolved_lhs), Some(resolved_rhs)) = (
            expr.lhs.0.try_lock().unwrap().resolved_type().flatten(),
            expr.rhs.0.try_lock().unwrap().resolved_type().flatten(),
        ) {
            changed = true;

            match &expr.op {
                BinaryOp::Less(_)
                | BinaryOp::LessEq(_)
                | BinaryOp::Greater(_)
                | BinaryOp::GreaterEq(_) => {
                    let resolved_lhs = resolved_lhs.actual_type();
                    let resolved_rhs = resolved_rhs.actual_type();

                    if matches!(
                        (resolved_lhs, resolved_rhs),
                        (FeType::Number(_), FeType::Number(_))
                    ) {
                        expr.resolved_type = Some(FeType::Bool(None));
                    } else {
                        todo!();
                    }
                }

                BinaryOp::Add(_) => {
                    let resolved_lhs = resolved_lhs.actual_type();
                    let resolved_rhs = resolved_rhs.actual_type();

                    match (resolved_lhs, resolved_rhs) {
                        (FeType::Number(lhs), FeType::Number(rhs)) => match (lhs, rhs) {
                            // known values at compile time
                            (
                                Some(NumberDetails::Integer(Some(lhs))),
                                Some(NumberDetails::Integer(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Integer(Some(*lhs + *rhs)),
                                )));
                            }
                            (
                                Some(NumberDetails::Decimal(Some(lhs))),
                                Some(NumberDetails::Integer(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Decimal(Some(*lhs + *rhs as f64)),
                                )));
                            }
                            (
                                Some(NumberDetails::Integer(Some(lhs))),
                                Some(NumberDetails::Decimal(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Decimal(Some(*lhs as f64 + *rhs)),
                                )));
                            }
                            (
                                Some(NumberDetails::Decimal(Some(lhs))),
                                Some(NumberDetails::Decimal(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Decimal(Some(*lhs + *rhs)),
                                )));
                            }

                            // unknown values, known types
                            (
                                Some(NumberDetails::Integer(_)),
                                Some(NumberDetails::Integer(None)),
                            )
                            | (
                                Some(NumberDetails::Integer(None)),
                                Some(NumberDetails::Integer(_)),
                            ) => {
                                expr.resolved_type =
                                    Some(FeType::Number(Some(NumberDetails::Integer(None))));
                            }
                            (Some(NumberDetails::Decimal(_)), _) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    // TODO: we know this is greater-than lhs val
                                    NumberDetails::Decimal(None),
                                )));
                            }
                            (_, Some(NumberDetails::Decimal(_))) => {
                                expr.resolved_type =
                                    Some(FeType::Number(Some(NumberDetails::Decimal(None))))
                            }

                            // other
                            (_, None) | (None, _) => {
                                expr.resolved_type = Some(FeType::Number(None));
                            }
                        },
                        _ => todo!("unsure how to add {resolved_lhs:#?} to {resolved_rhs:#?}"),
                    }
                }

                BinaryOp::Subtract(_) => {
                    let resolved_lhs = resolved_lhs.actual_type();
                    let resolved_rhs = resolved_rhs.actual_type();

                    match (resolved_lhs, resolved_rhs) {
                        (FeType::Number(lhs), FeType::Number(rhs)) => match (lhs, rhs) {
                            // known values at compile time
                            (
                                Some(NumberDetails::Integer(Some(lhs))),
                                Some(NumberDetails::Integer(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Integer(Some(*lhs - *rhs)),
                                )));
                            }
                            (
                                Some(NumberDetails::Decimal(Some(lhs))),
                                Some(NumberDetails::Integer(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Decimal(Some(*lhs - *rhs as f64)),
                                )));
                            }
                            (
                                Some(NumberDetails::Integer(Some(lhs))),
                                Some(NumberDetails::Decimal(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Decimal(Some(*lhs as f64 - *rhs)),
                                )));
                            }
                            (
                                Some(NumberDetails::Decimal(Some(lhs))),
                                Some(NumberDetails::Decimal(Some(rhs))),
                            ) => {
                                expr.resolved_type = Some(FeType::Number(Some(
                                    NumberDetails::Decimal(Some(*lhs - *rhs)),
                                )));
                            }

                            // unknown values, known types
                            (
                                Some(NumberDetails::Integer(_)),
                                Some(NumberDetails::Integer(None)),
                            )
                            | (
                                Some(NumberDetails::Integer(None)),
                                Some(NumberDetails::Integer(_)),
                            ) => {
                                expr.resolved_type =
                                    Some(FeType::Number(Some(NumberDetails::Integer(None))));
                            }
                            (Some(NumberDetails::Decimal(_)), _) => {
                                expr.resolved_type =
                                    Some(FeType::Number(Some(NumberDetails::Decimal(None))));
                            }
                            (_, Some(NumberDetails::Decimal(_))) => {
                                expr.resolved_type =
                                    Some(FeType::Number(Some(NumberDetails::Decimal(None))))
                            }

                            // other
                            (_, None) | (None, _) => {
                                expr.resolved_type = Some(FeType::Number(None));
                            }
                        },
                        _ => todo!("unsure how to add {resolved_lhs:#?} to {resolved_rhs:#?}"),
                    }
                }
            }
        }

        if !changed {
            todo!("determine lhs or rhs error");
        }

        return Ok(changed);
    }

    fn visit_static_ref_expr(
        &mut self,
        shared_expr: Arc<Mutex<StaticRefExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let expr = &mut *shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        // let mut changed = false;
        // return Ok(changed);

        todo!();
    }

    fn visit_construct_expr(
        &mut self,
        shared_expr: Arc<Mutex<ConstructExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        let mut expr = shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;
        let mut target = None;

        match &mut expr.target {
            ConstructTarget::Ident(ident) => {
                changed |= ident.accept(self)?;

                if let Some(resolved) = &ident.try_lock().unwrap().resolved_type {
                    target = Some(resolved.clone());
                }
            }
            ConstructTarget::StaticPath(path) => {
                changed |= path.accept(self)?;

                if let Some(resolved) = &path.resolved_type {
                    target = Some(resolved.clone());
                }
            }
        }

        if let Some(target) = target {
            let FeType::Struct(target) = target else {
                todo!("Can't construct type {target:#?}");
            };

            let fields_map = target
                .fields
                .iter()
                .cloned()
                .map(|f| (f.name.clone(), f))
                .collect::<HashMap<Arc<str>, FeStructField>>();

            let mut seen = HashSet::new();

            for arg in &mut expr.args {
                match arg {
                    ConstructArg::Field(field) => {
                        changed |= field.value.0.try_lock().unwrap().accept(self)?;

                        let Some(struct_field) = fields_map.get(&field.name.lexeme) else {
                            todo!(
                                "No field found with name {:?} for struct {:?}",
                                field.name.lexeme,
                                target.name
                            );
                        };

                        if seen.contains(&field.name.lexeme) {
                            todo!("Duplicate arg! {field:#?}");
                        }

                        seen.insert(field.name.lexeme.clone());

                        if let Some(resolved) =
                            field.value.0.try_lock().unwrap().resolved_type().flatten()
                        {
                            if !Self::can_implicit_cast(&resolved, &struct_field.typ) {
                                todo!("Invalid type! {resolved:#?}");
                            }
                        }
                    }
                }
            }

            let leftover_fields = fields_map.into_iter().filter_map(|(name, field)| {
                if seen.contains(&name) {
                    return None;
                }

                return Some(field);
            });

            for field in leftover_fields {
                // TODO: Check for default or optional

                todo!("Field not instantiated! {field:#?}");
            }

            expr.resolved_type = Some(FeType::Instance(FeInstance {
                special: None,
                name: target.name,
                fields: target
                    .fields
                    .into_iter()
                    .map(|f| (f.name.clone(), f))
                    .collect(),
            }));
        }

        return Ok(changed);
    }

    fn visit_get_expr(&mut self, shared_expr: Arc<Mutex<GetExpr<Option<FeType>>>>) -> Result<bool> {
        let expr = &mut *shared_expr.try_lock().unwrap();

        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        changed |= expr.target.0.try_lock().unwrap().accept(self)?;

        if let Some(resolved) = expr.target.0.try_lock().unwrap().resolved_type().flatten() {
            // TODO: I don't love this, what if theres a shared ref of a mut ref or something weird?
            let Some(instance) = resolved.instance() else {
                todo!("How can you get a property of a non-instance? Maybe the type system needs reworking... {resolved:#?}");
            };

            // TODO: methods?

            let Some(field) = instance.fields.get(&expr.name.lexeme).cloned() else {
                todo!(
                    "Couldn't find property {:#?} on instance {:#?}",
                    expr.name,
                    instance
                );
            };

            let resolved = match resolved {
                FeType::Instance(_) => field.typ,
                FeType::Ref(FeRefOf { ref_type, .. }) => FeType::Ref(FeRefOf {
                    ref_type,
                    of: Box::new(field.typ),
                }),
                FeType::Owned(FeOwnedOf { owned_mut, .. }) => FeType::Owned(FeOwnedOf {
                    owned_mut,
                    of: Box::new(field.typ),
                }),

                _ => todo!(),
            };

            expr.resolved_type = Some(resolved);

            changed = true;
        }

        return Ok(changed);
    }

    fn visit_if_expr(&mut self, shared_expr: Arc<Mutex<IfExpr<Option<FeType>>>>) -> Result<bool> {
        {
            let expr = &mut *shared_expr.try_lock().unwrap();

            if expr.is_resolved() {
                return Ok(false);
            }
        }

        let mut changed = false;

        {
            let condition = {
                let expr = &mut *shared_expr.try_lock().unwrap();
                expr.condition.clone()
            };

            let condition = condition.0.try_lock().unwrap();

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

        let mut typ = None;

        let then = {
            let expr = &mut *shared_expr.try_lock().unwrap();
            expr.then.clone()
        };

        match &then {
            IfExprThen::Ternary(then) => {
                changed |= then.then_expr.0.try_lock().unwrap().accept(self)?;

                typ = then
                    .then_expr
                    .0
                    .try_lock()
                    .unwrap()
                    .resolved_type()
                    .flatten();
            }
            IfExprThen::Block(then) => {
                self.scope
                    .try_lock()
                    .unwrap()
                    .begin_scope(Some(ScopeCreator::IfExpr(
                        IfBlock::Then,
                        shared_expr.clone(),
                    )));

                self.thenable_count += 1;
                let (local_changed, terminal) = self.resolve_stmts(&then.block.stmts)?;
                self.thenable_count -= 1;

                self.scope.try_lock().unwrap().end_scope();

                changed |= local_changed;

                if terminal.is_none() {
                    todo!("TODO: Implicit optional");
                }

                // TODO: Try to determine and check type here if terminal is Then stmt
            }
        }

        let else_ifs = {
            let expr = &mut *shared_expr.try_lock().unwrap();
            expr.else_ifs.clone()
        };

        for (idx, else_if) in else_ifs.iter().enumerate() {
            match else_if {
                IfExprElseIf::Ternary(else_if) => {
                    {
                        let condition = else_if.condition.0.try_lock().unwrap();

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

                    {
                        let expr = else_if.expr.0.try_lock().unwrap();

                        changed |= expr.accept(self)?;

                        let Some(resolved_type) = expr.resolved_type() else {
                            todo!("Can't use no-type as a value");
                        };

                        if let Some(typ) = &typ {
                            if let Some(resolved_type) = resolved_type {
                                if !Self::can_implicit_cast(&resolved_type, typ) {
                                    todo!("Can't cast!");
                                }
                            }
                        } else if let Some(resolved_type) = resolved_type {
                            typ = Some(resolved_type.clone());
                        }
                    }
                }
                IfExprElseIf::Block(else_if) => {
                    self.scope
                        .try_lock()
                        .unwrap()
                        .begin_scope(Some(ScopeCreator::IfExpr(
                            IfBlock::ElseIf(idx),
                            shared_expr.clone(),
                        )));

                    self.thenable_count += 1;
                    let (local_changed, _terminal) = self.resolve_stmts(&else_if.block.stmts)?;
                    self.thenable_count -= 1;

                    self.scope.try_lock().unwrap().end_scope();

                    changed |= local_changed;

                    // TODO: Try to determine and check type here if terminal is Then stmt
                }
            }
        }

        let else_ = {
            let expr = &mut *shared_expr.try_lock().unwrap();
            expr.else_.clone()
        };

        if let Some(else_) = &else_ {
            if !else_.is_resolved() {
                match else_ {
                    IfExprElse::Ternary(else_) => {
                        let else_expr = else_.else_expr.0.try_lock().unwrap();

                        changed |= else_expr.accept(self)?;

                        let Some(resolved_type) = else_expr.resolved_type() else {
                            todo!("Can't use no-type as a value");
                        };

                        if let Some(typ) = &typ {
                            if let Some(resolved_type) = resolved_type {
                                if !Self::can_implicit_cast(&resolved_type, typ) {
                                    todo!("Can't cast!");
                                }
                            }
                        } else if let Some(resolved_type) = resolved_type {
                            typ = Some(resolved_type);
                        }
                    }
                    IfExprElse::Block(else_) => {
                        self.scope
                            .try_lock()
                            .unwrap()
                            .begin_scope(Some(ScopeCreator::IfExpr(
                                IfBlock::Else,
                                shared_expr.clone(),
                            )));

                        self.thenable_count += 1;
                        let (local_changed, _terminal) = self.resolve_stmts(&else_.block.stmts)?;
                        self.thenable_count -= 1;

                        self.scope.try_lock().unwrap().end_scope();

                        changed |= local_changed;

                        // TODO: Try to determine and check type here if terminal is Then stmt
                    }
                }
            }
        } else {
            todo!("TODO: Implicit wrap as optional")
        }

        let expr = &mut *shared_expr.try_lock().unwrap();

        if let Some(Some(already)) = &expr.resolved_type {
            if let Some(typ) = typ {
                if !Self::can_implicit_cast(&typ, already) {
                    todo!();
                }
            }
        } else {
            expr.resolved_type = Some(typ);
        }

        return Ok(changed);
    }

    fn visit_loop_expr(
        &mut self,
        shared_expr: Arc<Mutex<LoopExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        {
            let expr = &mut *shared_expr.try_lock().unwrap();

            // TODO: Think about how looping affects types

            if expr.is_resolved() {
                return Ok(false);
            }
        }

        let mut changed = false;

        let stmts = {
            let expr = &mut *shared_expr.try_lock().unwrap();
            expr.block.stmts.clone()
        };

        self.scope
            .try_lock()
            .unwrap()
            .begin_scope(Some(ScopeCreator::LoopExpr(shared_expr)));

        self.breakable_count += 1;
        let (local_changed, _terminal) = self.resolve_stmts(&stmts)?;
        changed |= local_changed;
        self.breakable_count -= 1;

        self.scope.try_lock().unwrap().end_scope();

        return Ok(changed);
    }

    fn visit_while_expr(
        &mut self,
        shared_expr: Arc<Mutex<WhileExpr<Option<FeType>>>>,
    ) -> Result<bool> {
        {
            let expr = &mut *shared_expr.try_lock().unwrap();

            if expr.is_resolved() {
                return Ok(false);
            }
        }

        let mut changed = false;

        let condition = {
            let expr = &mut *shared_expr.try_lock().unwrap();
            expr.condition.clone()
        };

        changed |= condition.0.try_lock().unwrap().accept(self)?;

        let stmts = {
            let expr = &mut *shared_expr.try_lock().unwrap();
            expr.block.stmts.clone()
        };

        self.scope
            .try_lock()
            .unwrap()
            .begin_scope(Some(ScopeCreator::WhileExpr(shared_expr)));

        self.breakable_count += 1;
        changed |= self.resolve_stmts(&stmts)?.0;
        self.breakable_count -= 1;

        self.scope.try_lock().unwrap().end_scope();

        return Ok(changed);
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
    creator: Option<ScopeCreator>,
    name_lookup: HashMap<Arc<str>, ScopedType>,
}

#[derive(Debug, Clone)]
enum ScopeCreator {
    Fn(Arc<Mutex<FnDecl<Option<FeType>>>>),
    IfStmt(IfBlock, Arc<Mutex<IfStmt<Option<FeType>>>>),
    IfExpr(IfBlock, Arc<Mutex<IfExpr<Option<FeType>>>>),
    WhileStmt(Arc<Mutex<WhileStmt<Option<FeType>>>>),
    WhileExpr(Arc<Mutex<WhileExpr<Option<FeType>>>>),
    LoopStmt(Arc<Mutex<LoopStmt<Option<FeType>>>>),
    LoopExpr(Arc<Mutex<LoopExpr<Option<FeType>>>>),
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
                creator: None,
                name_lookup: HashMap::new(),
            }],
        };
    }

    pub fn begin_scope(&mut self, creator: Option<ScopeCreator>) {
        self.stack.push(FlatScope {
            creator,
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

    // pub fn update(&mut self, name: &str, typ: ScopedType) {
    //     for data in self.stack.iter_mut().rev() {
    //         if let Some(found) = data.name_lookup.get_mut(name) {
    //             *found = typ;
    //             return;
    //         }
    //     }
    // }

    pub fn search(&self, name: &str) -> Option<&ScopedType> {
        for data in self.stack.iter().rev() {
            if let Some(found) = data.name_lookup.get(name) {
                return Some(found);
            }
        }

        return None;
    }

    // pub fn handle_return(&self) -> Option<ReturnHandler> {
    //     for scope in self.stack.iter().rev() {
    //         match &scope.creator {
    //             Some(ScopeCreator::Fn(v)) => {
    //                 return Some(ReturnHandler::Fn(v.clone()));
    //             }

    //             _ => {}
    //         }
    //     }

    //     return None;
    // }

    pub fn handle_then(&self, label: Option<Arc<Token>>) -> Option<ThenHandler> {
        let label = label.as_ref().map(|label| label.lexeme.as_ref());

        for scope in self.stack.iter().rev() {
            match &scope.creator {
                Some(ScopeCreator::IfStmt(block, v)) => {
                    if label.is_none() {
                        return Some(ThenHandler::IfStmt(block.clone(), v.clone()));
                    }
                }

                Some(ScopeCreator::IfExpr(block, v)) => {
                    if let Some(label) = label {
                        let scope_label = match block {
                            IfBlock::Then => match &v.lock().unwrap().then {
                                IfExprThen::Block(then) => then.label.clone(),
                                _ => continue,
                            },

                            IfBlock::ElseIf(idx) => match &v.lock().unwrap().else_ifs.get(*idx) {
                                Some(IfExprElseIf::Block(else_if)) => else_if.label.clone(),
                                _ => continue,
                            },

                            IfBlock::Else => match &v.lock().unwrap().else_ {
                                Some(IfExprElse::Block(else_)) => else_.label.clone(),
                                _ => continue,
                            },
                        };

                        let Some(scope_label) = scope_label else {
                            continue;
                        };

                        if label != scope_label.lexeme.as_ref() {
                            continue;
                        }
                    }

                    return Some(ThenHandler::IfExpr(block.clone(), v.clone()));
                }

                _ => {}
            }
        }

        return None;
    }

    pub fn handle_break(&self, label: Option<Arc<Token>>) -> Option<BreakHandler> {
        let label = label.as_ref().map(|label| label.lexeme.as_ref());

        for scope in self.stack.iter().rev() {
            match &scope.creator {
                Some(ScopeCreator::LoopStmt(v)) => {
                    if label
                        == v.try_lock()
                            .unwrap()
                            .label
                            .as_ref()
                            .map(|l| l.lexeme.as_ref())
                    {
                        return Some(BreakHandler::LoopStmt(v.clone()));
                    }
                }

                Some(ScopeCreator::LoopExpr(v)) => {
                    if label
                        == v.try_lock()
                            .unwrap()
                            .label
                            .as_ref()
                            .map(|l| l.lexeme.as_ref())
                    {
                        return Some(BreakHandler::LoopExpr(v.clone()));
                    }
                }

                Some(ScopeCreator::WhileStmt(v)) => {
                    if label
                        == v.try_lock()
                            .unwrap()
                            .label
                            .as_ref()
                            .map(|l| l.lexeme.as_ref())
                    {
                        return Some(BreakHandler::WhileStmt(v.clone()));
                    }
                }

                Some(ScopeCreator::WhileExpr(v)) => {
                    if label
                        == v.try_lock()
                            .unwrap()
                            .label
                            .as_ref()
                            .map(|l| l.lexeme.as_ref())
                    {
                        return Some(BreakHandler::WhileExpr(v.clone()));
                    }
                }

                _ => {}
            }
        }

        return None;
    }
}
