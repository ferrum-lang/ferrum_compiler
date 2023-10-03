use super::*;

use crate::log;

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
    let pre_exports = match &path.pre {
        Some(UseStaticPathPre::RootDir(_)) => Some(resolver.root_pkg_exports.clone()),
        Some(UseStaticPathPre::CurrentDir(_)) => Some(resolver.current_pkg_exports.clone()),
        None | Some(UseStaticPathPre::DoubleColon(_)) => None,
    };

    let found = if let Some(pre_exports) = pre_exports {
        match &*pre_exports.try_lock().unwrap() {
            ExportsPackage::Dir(d) => d
                .local_packages
                .get(&SyntaxPackageName(path.name.lexeme.clone()))
                .cloned(),
            ExportsPackage::File(_f) => todo!("How is pre export a file??"),
        }
    } else if let Either::B(_) = &path.details {
        None
    } else {
        let search_scope = search_scope.try_lock().unwrap();

        let found = search_scope.search(&path.name.lexeme);

        match found {
            Some(ScopedType {
                typ: FeType::Package(pkg),
                ..
            }) => Some(pkg.clone()),

            _ => None,
        }
    };

    let search_scope = found
        .map(|found| found.try_lock().unwrap().scope())
        .unwrap_or(search_scope);

    let mut types = vec![];

    match &mut path.details {
        Either::A(UseStaticPathNext::Single(next)) => {
            types.extend(recursive_resolve(resolver, search_scope, &mut next.path)?);
        }
        Either::A(UseStaticPathNext::Many(nexts)) => {
            for next in &mut nexts.nexts {
                types.extend(recursive_resolve(
                    resolver,
                    search_scope.clone(),
                    &mut next.path,
                )?);
            }
        }
        Either::B(typ) => {
            let typ = if let Some(typ) = typ {
                Some(typ.clone())
            } else {
                search_scope
                    .try_lock()
                    .unwrap()
                    .search(&path.name.lexeme)
                    .map(|s| s.typ.clone())
            };

            if let Some(typ) = typ {
                path.details = Either::B(Some(typ.clone()));
                types.push((path.name.lexeme.clone(), typ));
            } else {
                log::trace!(&search_scope);
                todo!("type not found: {:#?}", path.name);
            }
        }
    }

    return Ok(types);
}
