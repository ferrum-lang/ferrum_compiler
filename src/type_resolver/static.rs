use super::*;

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

        let scope = if let Some(root) = &mut static_path.root {
            changed |= root.accept(self)?;

            if let Some(root_type) = &root.resolved_type {
                let FeType::Package(pkg) = root_type else {
                    todo!("Cannot path on root type: {root_type:#?}");
                };

                Some(pkg.try_lock().unwrap().scope())
            } else {
                None
            }
        } else {
            Some(self.scope.clone())
        };

        if let Some(scope) = scope {
            let scope = &mut *scope.try_lock().unwrap();

            let name = &static_path.name.lexeme;

            if let Some(typ) = scope.search(name) {
                static_path.resolved_type = Some(typ.typ.clone());
                changed = true;
            } else {
                // crate::log::trace!(&scope);
                // todo!("Check scope for imported type: {name}");
            }
        }

        return Ok(changed);
    }
}
