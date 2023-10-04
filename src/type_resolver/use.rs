use super::*;

impl UseVisitor<Option<FeType>, Result<bool>> for FeTypeResolver {
    fn visit_use(&mut self, shared_use_decl: Arc<Mutex<Use<Option<FeType>>>>) -> Result<bool> {
        let use_decl = &mut *shared_use_decl.try_lock().unwrap();

        if use_decl.is_resolved() {
            return Ok(false);
        }

        let mut changed = false;
        let is_pub = matches!(use_decl.use_mod, Some(UseMod::Pub(_)));

        let types = recursive_resolve(self, self.scope.clone(), &mut use_decl.path)?;

        let scope = &mut *self.scope.try_lock().unwrap();

        for (name, typ) in types {
            scope.insert(name, ScopedType { is_pub, typ });
            changed = true;
        }

        return Ok(changed);
    }
}

fn recursive_resolve(
    resolver: &mut FeTypeResolver,
    search_scope: Arc<Mutex<Scope>>,
    path: &mut UseStaticPath<Option<FeType>>,
) -> Result<Vec<(Arc<str>, FeType)>> {
    let search_scope = match &path.pre {
        Some(UseStaticPathPre::RootDir(_)) => resolver.root_pkg_exports.clone(),
        Some(UseStaticPathPre::CurrentDir(_)) => resolver.current_pkg_exports.clone(),
        None | Some(UseStaticPathPre::DoubleColon(_)) => {
            Arc::new(Mutex::new(ExportsPackage::File(ExportsFile {
                scope: search_scope,
            })))
        }
    };

    let mut types = vec![];

    match &mut path.details {
        Either::A(next) => {
            let search_scope = match &*search_scope.try_lock().unwrap() {
                ExportsPackage::Dir(d) => d
                    .local_packages
                    .get(&SyntaxPackageName(path.name.lexeme.clone()))
                    .cloned(),

                ExportsPackage::File(f) => f
                    .scope
                    .try_lock()
                    .unwrap()
                    .search(&path.name.lexeme)
                    .map(|st| match &st.typ {
                        FeType::Package(pkg) => pkg.clone(),
                        _ => todo!("Can't export!"),
                    }),
            };

            let Some(search_scope) = search_scope else {
                return Ok(vec![]);
                // todo!("Couldn't find! {path:#?}");
            };

            let search_scope = search_scope.try_lock().unwrap().scope();

            match next {
                UseStaticPathNext::Single(next) => {
                    types.extend(recursive_resolve(resolver, search_scope, &mut next.path)?);
                }
                UseStaticPathNext::Many(nexts) => {
                    for next in &mut nexts.nexts {
                        types.extend(recursive_resolve(
                            resolver,
                            search_scope.clone(),
                            &mut next.path,
                        )?);
                    }
                }
            }
        }
        Either::B(typ) => match &*search_scope.try_lock().unwrap() {
            ExportsPackage::Dir(d) => {
                let found = d
                    .local_packages
                    .get(&SyntaxPackageName(path.name.lexeme.clone()))
                    .cloned();

                let Some(found) = found else {
                    // todo!("package not found: {:#?}", path.name);
                    return Ok(vec![]);
                };

                let t = FeType::Package(found);

                // path.details = Either::B(Some(typ.clone()));
                *typ = Some(t.clone());
                types.push((path.name.lexeme.clone(), t));
            }

            ExportsPackage::File(f) => {
                let t = if let Some(typ) = typ {
                    Some(typ.clone())
                } else {
                    f.scope
                        .try_lock()
                        .unwrap()
                        .search(&path.name.lexeme)
                        .map(|st| st.typ.clone())
                };

                if let Some(t) = t {
                    *typ = Some(t.clone());
                    types.push((path.name.lexeme.clone(), t));
                } else {
                    // log::trace!(&search_scope, &path.name);
                    todo!("type not found: {:#?}", path.name);
                }
            }
        },
    }

    return Ok(types);
}
