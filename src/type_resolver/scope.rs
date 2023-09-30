use super::*;

#[derive(Debug, Clone)]
pub enum ExportsPackage {
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
pub struct ExportsFile {
    scope: Arc<Mutex<Scope>>,
}

#[derive(Debug, Clone)]
pub struct ExportsDir {
    entry: ExportsFile,
    pub local_packages: HashMap<SyntaxPackageName, Arc<Mutex<ExportsPackage>>>,
}

#[derive(Debug, Clone)]
pub struct Scope {
    stack: Vec<FlatScope>,
}

#[derive(Debug, Clone)]
pub struct FlatScope {
    creator: Option<ScopeCreator>,
    name_lookup: HashMap<Arc<str>, ScopedType>,
}

#[derive(Debug, Clone)]
pub enum ScopeCreator {
    Fn(Arc<Mutex<FnDecl<Option<FeType>>>>),
    IfStmt(IfBlock, Arc<Mutex<IfStmt<Option<FeType>>>>),
    IfExpr(IfBlock, Arc<Mutex<IfExpr<Option<FeType>>>>),
    WhileStmt(Arc<Mutex<WhileStmt<Option<FeType>>>>),
    WhileExpr(Arc<Mutex<WhileExpr<Option<FeType>>>>),
    LoopStmt(Arc<Mutex<LoopStmt<Option<FeType>>>>),
    LoopExpr(Arc<Mutex<LoopExpr<Option<FeType>>>>),
}

#[derive(Debug, Clone)]
pub struct ScopedType {
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
