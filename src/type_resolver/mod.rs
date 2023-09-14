use crate::r#type::*;
use crate::syntax::*;

use crate::result::Result;

use crate::token::TokenType;

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub struct FeTypeResolver {
    expr_lookup: HashMap<NodeId<Expr>, FeType>,
    decls_to_eval: HashMap<NodeId<Decl>, Arc<Mutex<Decl<Option<FeType>>>>>,

    scope: Arc<Mutex<Scope>>,

    root_pkg_exports: Arc<Mutex<ExportsPackage>>,
    current_pkg_exports: Arc<Mutex<ExportsPackage>>,

    current_return_type: Option<Option<FeType>>,
    breakable_count: usize,
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
        };

        while !pkg.lock().unwrap().is_resolved() {
            println!("1");
            let changed = match &mut *pkg.lock().unwrap() {
                FeSyntaxPackage::File(file) => this.resolve_file(file)?,
                FeSyntaxPackage::Dir(dir) => this.resolve_dir(dir)?,
            };
            println!("2");

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

            current_return_type: None,
            breakable_count: 0,
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
                *changed = *changed | local;
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

            self.decls_to_eval.insert(id, decl.clone());

            if let Some(changed) = &mut changed {
                *changed = *changed | decl_changed;
            } else {
                changed = Some(decl_changed);
            }
        }

        if !changed.unwrap_or(true) {
            while !self.decls_to_eval.is_empty() {
                for (_, decl) in std::mem::take(&mut self.decls_to_eval) {
                    let decl_changed = self.evaluate_decl(decl)?;

                    if let Some(changed) = &mut changed {
                        *changed = *changed | decl_changed;
                    } else {
                        changed = Some(decl_changed);
                    }
                }
            }
        }

        return Ok(changed.unwrap_or(false));
    }

    fn evaluate_decl(&mut self, decl: Arc<Mutex<Decl<Option<FeType>>>>) -> Result<bool> {
        match &mut *decl.lock().unwrap() {
            Decl::Fn(decl) => {
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

                self.scope.lock().unwrap().begin_scope();
                let res = self.evaluate_fn_decl(decl);
                self.scope.lock().unwrap().end_scope();

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
            let mut scope = self.scope.lock().unwrap();

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
            FnDeclBody::Short(body) => {
                todo!()
            }

            FnDeclBody::Block(body) => {
                let mut changed = false;

                changed |= self.resolve_stmts(&body.stmts)?.0;

                return Ok(changed);
            }
        }
    }

    fn resolve_stmts(
        &mut self,
        stmts: &[Arc<Mutex<Stmt<Option<FeType>>>>],
    ) -> Result<(bool, Option<TerminationType>)> {
        let mut changed = false;
        let mut contains = HashSet::new();

        let mut termination = None;
        for stmt in stmts {
            if let Some(term) = &termination {
                todo!("Unreachable code after {term:?}! {stmt:#?}");
            }

            let mut stmt = stmt.lock().unwrap();
            changed = changed | stmt.accept(self)?;

            match stmt.is_terminal() {
                Some(TerminationType::Contains(terms)) => {
                    contains.extend(terms);
                    termination = None;
                }

                term => {
                    termination = term;
                }
            }
        }

        if termination.is_some() {
            return Ok((changed, termination));
        }

        if !contains.is_empty() {
            return Ok((changed, Some(TerminationType::Contains(contains))));
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

            _ => todo!("Can you cast?\nThis: {from:#?}\nTo: {to:#?}"),
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
            changed = changed | root.accept(self)?;

            // TODO: Handle package types and navigating scope
        } else {
            match static_path.name.lexeme.as_ref() {
                "String" => {
                    static_path.resolved_type = Some(FeType::String(None));
                    changed = true;
                }

                other => todo!("Check scope for imported type: {other}"),
            }
        }

        return Ok(changed);
    }
}

impl DeclVisitor<Option<FeType>, Result<bool>> for FeTypeResolver {
    fn visit_function_decl(&mut self, decl: &mut FnDecl<Option<FeType>>) -> Result<bool> {
        if decl.is_signature_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        let mut params = vec![];
        let mut all_resolved = true;

        for param in &mut decl.params {
            if let Some(resolved_type) = &param.resolved_type {
                params.push((param.name.lexeme.clone(), resolved_type.clone()));
            } else {
                changed = changed | param.static_type_ref.accept(self)?;
                param.resolved_type = param.static_type_ref.resolved_type.clone();

                if let Some(resolved_type) = &param.resolved_type {
                    params.push((param.name.lexeme.clone(), resolved_type.clone()));
                } else {
                    all_resolved = false;
                }
            }
        }

        let mut fn_return_type = None;

        if let Some(return_type) = &mut decl.return_type {
            if let Some(resolved_type) = &return_type.resolved_type {
                fn_return_type = Some(Box::new(resolved_type.clone()));
            } else {
                changed = changed | return_type.static_type.accept(self)?;
                return_type.resolved_type = return_type.static_type.resolved_type.clone();

                if let Some(resolved_type) = &return_type.resolved_type {
                    fn_return_type = Some(Box::new(resolved_type.clone()));
                } else {
                    all_resolved = false;
                }
            }
        }

        if all_resolved {
            changed = true;
            self.scope.lock().unwrap().insert(
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
        }

        return Ok(changed);
    }

    fn visit_struct_decl(&mut self, decl: &mut StructDecl<Option<FeType>>) -> Result<bool> {
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
            self.scope.lock().unwrap().insert(
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
    fn visit_expr_stmt(&mut self, stmt: &mut ExprStmt<Option<FeType>>) -> Result<bool> {
        return stmt.expr.lock().unwrap().accept(self);
    }

    fn visit_var_decl_stmt(&mut self, stmt: &mut VarDeclStmt<Option<FeType>>) -> Result<bool> {
        let mut changed = false;

        let typ = if let Some(value) = &stmt.value {
            let value = &mut *value.value.0.lock().unwrap();

            changed = changed | value.accept(self)?;

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

                    changed = changed | ident.accept(self)?;
                }
            }
        }

        return Ok(changed);
    }

    fn visit_assign_stmt(&mut self, stmt: &mut AssignStmt<Option<FeType>>) -> Result<bool> {
        if stmt.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;
        let mut types = (None, None);

        {
            let mut target = stmt.target.0.lock().unwrap();
            changed = changed | target.accept(self)?;

            // TODO: ensure LHS expr is assignable (unassigned const ident || mut ident || instance_ref)

            types.0 = target.resolved_type().cloned().flatten();

            if let Some(resolved_type) = &types.0 {
                match resolved_type {
                    FeType::Ref(ref_of) => {
                        if ref_of.ref_type == FeRefType::Const {
                            // TODO: handle assigning late to non-assigned const ref
                            todo!("Reference is not mutable: {:#?}", target);
                        }
                    }

                    FeType::Owned(owned_of) => {
                        if owned_of.owned_mut == FeOwnedMut::Const {
                            // TODO: handle assigning late to non-assigned const
                            todo!("Owned type is not mutable: {:#?}", target);
                        }
                    }

                    other => todo!("Cannot assign to {other:?}"),
                }
            }
        }

        {
            let mut value = stmt.value.0.lock().unwrap();
            changed = changed | value.accept(self)?;

            types.1 = value.resolved_type().cloned().flatten();
        }

        if let (Some(target_type), Some(value_type)) = types {
            if !Self::can_implicit_cast(&value_type, &target_type) {
                todo!(
                    "Can't assign types!\nFrom: {:#?}\nTo: {:#?}",
                    value_type,
                    target_type
                );
            }
        }

        if stmt.is_resolved() {
            changed = true;
        }

        return Ok(changed);
    }

    fn visit_return_stmt(&mut self, stmt: &mut ReturnStmt<Option<FeType>>) -> Result<bool> {
        let Some(current_return_type) = self.current_return_type.clone() else {
            todo!("Return statements not allowed!");
        };

        if stmt.value.is_none() {
            if let Some(_) = &current_return_type {
                todo!("Can't return without a value!");
            }
        }

        if stmt.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        if let Some(value) = &stmt.value {
            changed = changed | value.0.lock().unwrap().accept(self)?;

            if let Some(resolved_type) = value.0.lock().unwrap().resolved_type().cloned().flatten()
            {
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

    fn visit_if_stmt(&mut self, stmt: &mut IfStmt<Option<FeType>>) -> Result<bool> {
        if stmt.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        {
            let mut condition = stmt.condition.0.lock().unwrap();
            if !condition.is_resolved() {
                changed = changed | condition.accept(self)?;

                let Some(resolved_type) = condition.resolved_type() else {
                    todo!("Can't check if condition on no type!");
                };

                if let Some(resolved_type) = resolved_type {
                    if !Self::can_implicit_cast(resolved_type, &FeType::Bool(None)) {
                        todo!("Can't cast to bool!");
                    }
                }
            }
        }

        changed |= self.resolve_stmts(&stmt.then_block.stmts)?.0;

        for else_if in &mut stmt.else_ifs {
            let mut condition = else_if.condition.0.lock().unwrap();
            if !condition.is_resolved() {
                changed = changed | condition.accept(self)?;

                let Some(resolved_type) = condition.resolved_type() else {
                    todo!("Can't check else-if condition on no type!");
                };

                if let Some(resolved_type) = resolved_type {
                    if !Self::can_implicit_cast(resolved_type, &FeType::Bool(None)) {
                        todo!("Can't cast to bool!");
                    }
                }
            }

            changed |= self.resolve_stmts(&else_if.then_block.stmts)?.0;
        }

        if let Some(else_) = &mut stmt.else_ {
            changed |= self.resolve_stmts(&else_.then_block.stmts)?.0;
        }

        return Ok(changed);
    }

    fn visit_loop_stmt(&mut self, stmt: &mut LoopStmt<Option<FeType>>) -> Result<bool> {
        // TODO: Think about how looping affects types

        if stmt.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        self.breakable_count += 1;
        let (changes, term) = self.resolve_stmts(&stmt.block.stmts)?;
        changed |= changes;
        self.breakable_count -= 1;

        // if let Some(TerminationType::Base(BaseTerminationType::InfiniteLoop)) = term {
        //     todo!("Infinite loop!");
        // }

        return Ok(changed);
    }

    fn visit_while_stmt(&mut self, stmt: &mut WhileStmt<Option<FeType>>) -> Result<bool> {
        if stmt.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        changed |= stmt.condition.0.lock().unwrap().accept(self)?;

        self.breakable_count += 1;
        changed |= self.resolve_stmts(&stmt.block.stmts)?.0;
        self.breakable_count -= 1;

        return Ok(changed);
    }

    fn visit_break_stmt(&mut self, stmt: &mut BreakStmt<Option<FeType>>) -> Result<bool> {
        if self.breakable_count == 0 {
            todo!("Can't break here! {stmt:#?}");
        }

        return Ok(false);
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

    fn visit_number_literal_expr(
        &mut self,
        expr: &mut NumberLiteralExpr<Option<FeType>>,
    ) -> Result<bool> {
        if expr.is_resolved() {
            return Ok(false);
        }

        expr.resolved_type = Some(FeType::Number(match expr.details {
            NumberLiteralDetails::Integer(val) => NumberDetails::Integer(val as i64),
            NumberLiteralDetails::Decimal(val) => NumberDetails::Decimal(val),
        }));

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

    fn visit_fmt_string_literal_expr(
        &mut self,
        expr: &mut FmtStringLiteralExpr<Option<FeType>>,
    ) -> Result<bool> {
        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;
        let mut is_all_checked = true;

        for part in &mut expr.rest {
            changed = changed | part.expr.0.lock().unwrap().accept(self)?;

            if !part.expr.0.lock().unwrap().is_resolved() {
                is_all_checked = false;
            }
        }

        if is_all_checked {
            expr.resolved_type = Some(FeType::String(Some(StringDetails::Format)));
            changed = true;
        }

        return Ok(changed);
    }

    fn visit_ident_expr(&mut self, expr: &mut IdentExpr<Option<FeType>>) -> Result<bool> {
        if expr.is_resolved() {
            return Ok(false);
        }

        let ident = &expr.ident.lexeme;

        if let Some(found) = self.scope.lock().unwrap().search(ident) {
            expr.resolved_type = Some(found.typ.clone());
            self.expr_lookup.insert(expr.id, found.typ.clone());
        } else {
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
            todo!(
                "too many args!\nExpected: {:#?}\nGot: {:#?}",
                callee.params,
                expr.args
            );
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
                todo!("wrong type!\nCannot implicitly cast {resolved_type:#?}\nto {param:#?}");
            }
        }

        expr.resolved_type = callee.return_type.as_deref().map(|rt| Some(rt.clone()));

        return Ok(true);
    }

    fn visit_unary_expr(&mut self, expr: &mut UnaryExpr<Option<FeType>>) -> Result<bool> {
        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        changed = changed | expr.value.0.lock().unwrap().accept(self)?;

        if let Some(resolved_type) = expr
            .value
            .0
            .lock()
            .unwrap()
            .resolved_type()
            .cloned()
            .flatten()
        {
            changed = true;

            match expr.op {
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

    fn visit_static_ref_expr(&mut self, expr: &mut StaticRefExpr<Option<FeType>>) -> Result<bool> {
        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        todo!();

        return Ok(changed);
    }

    fn visit_construct_expr(&mut self, expr: &mut ConstructExpr<Option<FeType>>) -> Result<bool> {
        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;
        let mut target = None;

        match &mut expr.target {
            ConstructTarget::Ident(ident) => {
                changed |= ident.accept(self)?;

                if let Some(resolved) = &ident.resolved_type {
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
                        changed |= field.value.0.lock().unwrap().accept(self)?;

                        let Some(struct_field) = fields_map.get(&field.name.lexeme) else {
                            todo!("No field found with name {:?} for struct {:?}", field.name.lexeme, target.name);
                        };

                        if seen.contains(&field.name.lexeme) {
                            todo!("Duplicate arg! {field:#?}");
                        }

                        seen.insert(field.name.lexeme.clone());

                        if let Some(resolved) = field
                            .value
                            .0
                            .lock()
                            .unwrap()
                            .resolved_type()
                            .cloned()
                            .flatten()
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

    fn visit_get_expr(&mut self, expr: &mut GetExpr<Option<FeType>>) -> Result<bool> {
        if expr.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;

        changed |= expr.target.0.lock().unwrap().accept(self)?;

        if let Some(resolved) = expr
            .target
            .0
            .lock()
            .unwrap()
            .resolved_type()
            .cloned()
            .flatten()
        {
            // TODO: I don't love this, what if theres a shared ref of a mut ref or something weird?
            let Some(instance) = resolved.instance() else {
                todo!("How can you get a property of a non-instance? Maybe the type system needs reworking... {resolved:#?}");
            };

            // TODO: methods?

            let Some(field) = instance.fields.get(&expr.name.lexeme).cloned() else {
                todo!("Couldn't find property {:#?} on instance {:#?}", expr.name, instance);
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
