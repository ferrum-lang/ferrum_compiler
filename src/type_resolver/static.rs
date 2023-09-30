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
