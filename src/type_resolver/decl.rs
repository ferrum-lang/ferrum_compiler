use super::*;

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
