use super::*;

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
