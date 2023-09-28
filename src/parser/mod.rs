use crate::result::Result;
use crate::syntax::*;
use crate::token::*;

use std::sync::{Arc, Mutex};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct FeSyntaxParser {
    token_pkg: Arc<Mutex<FeTokenPackage>>,

    out: FeSyntaxPackage,
}

impl FeSyntaxParser {
    pub fn parse_package(token_pkg: Arc<Mutex<FeTokenPackage>>) -> Result<FeSyntaxPackage> {
        return Self::new(token_pkg).parse();
    }

    pub fn new(token_pkg: Arc<Mutex<FeTokenPackage>>) -> Self {
        let out = token_pkg.try_lock().unwrap().clone().into();

        return Self { token_pkg, out };
    }

    pub fn parse(mut self) -> Result<FeSyntaxPackage> {
        fn _parse<'a>(
            token_pkg: &FeTokenPackage,
            syntax_pkg: &'a mut FeSyntaxPackage,
        ) -> Result<&'a mut FeSyntaxPackage> {
            match (token_pkg, &mut *syntax_pkg) {
                (FeTokenPackage::File(token_file), FeSyntaxPackage::File(syntax_file)) => {
                    FeTokenSyntaxParser::parse_syntax(
                        token_file.tokens.try_lock().unwrap().clone(),
                        syntax_file.syntax.clone(),
                    )?;
                }
                (FeTokenPackage::Dir(token_dir), FeSyntaxPackage::Dir(syntax_dir)) => {
                    FeTokenSyntaxParser::parse_syntax(
                        token_dir.entry_file.tokens.try_lock().unwrap().clone(),
                        syntax_dir.entry_file.syntax.clone(),
                    )?;

                    for (name, token_pkg) in token_dir.local_packages.iter() {
                        let syntax_pkg = syntax_dir
                            .local_packages
                            .get(&SyntaxPackageName::from(name.clone()))
                            .expect("tokens doesn't match syntax structure");

                        _parse(
                            &token_pkg.try_lock().unwrap(),
                            &mut syntax_pkg.try_lock().unwrap(),
                        )?;
                    }
                }

                (FeTokenPackage::File(_), _) | (FeTokenPackage::Dir(_), _) => unreachable!(),
            }

            return Ok(syntax_pkg);
        }
        _parse(&self.token_pkg.try_lock().unwrap(), &mut self.out)?;

        return Ok(self.out);
    }
}

struct FeTokenSyntaxParser {
    tokens: Vec<Arc<Token>>,
    out: Arc<Mutex<SyntaxTree>>,

    current_idx: usize,
}

// TODO: Improve error reporting
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Error: {message}")]
    Error { message: String },
}

#[derive(Clone, Debug, PartialEq)]
pub enum WithNewlines {
    None,
    One,
    Many,
}

impl FeTokenSyntaxParser {
    fn parse_syntax(tokens: Vec<Arc<Token>>, syntax_tree: Arc<Mutex<SyntaxTree>>) -> Result {
        return Self::new(tokens, syntax_tree).parse();
    }

    fn new(tokens: Vec<Arc<Token>>, syntax_tree: Arc<Mutex<SyntaxTree>>) -> Self {
        return Self {
            tokens,
            out: syntax_tree,

            current_idx: 0,
        };
    }

    fn parse(mut self) -> Result {
        while !self.is_at_end() {
            if self.allow_many_newlines() > 0 {
                continue;
            }

            match self.use_declaration() {
                Ok(None) => break,

                Ok(Some(use_decl)) => {
                    self.out.try_lock().unwrap().uses.push(use_decl);

                    if !self.is_at_end() {
                        self.consume(&TokenType::Newline, "Expect newline after use")?;
                    }
                }

                // TODO: Improve compiling around errors and error reporting
                Err(e) => return Err(e),
            }
        }

        while !self.is_at_end() {
            if self.allow_many_newlines() > 0 {
                continue;
            }

            match self.declaration() {
                Ok(decl) => {
                    self.out.try_lock().unwrap().decls.push(decl);

                    if !self.is_at_end() {
                        self.consume(&TokenType::Newline, "Expect newline after declaration")?;
                    }
                }

                // TODO: Improve compiling around errors and error reporting
                Err(e) => return Err(e),
            }
        }

        return Ok(());
    }

    fn use_declaration(&mut self) -> Result<Option<Arc<Mutex<Use>>>> {
        let use_mod = self.use_mod();

        let Some(use_token) = self.match_any(&[TokenType::Use], WithNewlines::None) else {
            if let Some(UseMod::Pub(_)) = use_mod {
                self.backtrack();
            }

            return Ok(None);
        };

        let path = self.use_static_path()?;

        let use_decl = Use {
            id: NodeId::gen(),
            use_mod,
            use_token,
            path,
        };

        return Ok(Some(Arc::new(Mutex::new(use_decl))));
    }

    fn use_mod(&mut self) -> Option<UseMod> {
        if let Some(token) = self.match_any(&[TokenType::Pub], WithNewlines::None) {
            return Some(UseMod::Pub(token));
        }

        return None;
    }

    fn use_static_path(&mut self) -> Result<UseStaticPath> {
        let pre = if let Some(token) = self.match_any(&[TokenType::DoubleColon], WithNewlines::None)
        {
            Some(UseStaticPathPre::DoubleColon(token))
        } else if let Some(token) = self.match_any(&[TokenType::DotSlash], WithNewlines::None) {
            Some(UseStaticPathPre::CurrentDir(token))
        } else {
            self.match_any(&[TokenType::TildeSlash], WithNewlines::None)
                .map(UseStaticPathPre::RootDir)
        };

        let name = self.consume(&TokenType::Ident, "Expect name of import")?;

        let details = if let Some(double_colon_token) =
            self.match_any(&[TokenType::DoubleColon], WithNewlines::None)
        {
            // TODO: Handle case of 'many'

            let path = self.use_static_path()?;

            Either::A(UseStaticPathNext::Single(UseStaticPathNextSingle {
                double_colon_token,
                path: Box::new(path),
            }))
        } else {
            Either::B(())
        };

        return Ok(UseStaticPath { pre, name, details });
    }

    fn declaration(&mut self) -> Result<Arc<Mutex<Decl>>> {
        let mut decl_mod = None;

        if let Some(token) = self.match_any(&[TokenType::Pub], WithNewlines::Many) {
            decl_mod = Some(DeclMod::Pub(token));
        }

        {
            let fn_mod = match self.peek().as_ref().map(|t| (t.clone(), &t.token_type)) {
                Some((token, TokenType::Pure)) => Some(FnMod::Pure(token)),
                Some((token, TokenType::Safe)) => Some(FnMod::Safe(token)),
                Some((token, TokenType::Norm)) => Some(FnMod::Norm(token)),
                Some((token, TokenType::Risk)) => Some(FnMod::Risk(token)),
                _ => None,
            };

            let fn_token = if fn_mod.is_some() {
                let _ = self.advance();

                Some(self.consume(&TokenType::Fn, "Expect 'fn' after fn modifier")?)
            } else {
                self.match_any(&[TokenType::Fn], WithNewlines::Many)
            };

            if let Some(fn_token) = fn_token {
                return Ok(Arc::new(Mutex::new(Decl::Fn(Arc::new(Mutex::new(
                    self.fn_decl(decl_mod, fn_mod, fn_token)?,
                ))))));
            }
        }

        if let Some(token) = self.match_any(&[TokenType::Struct], WithNewlines::Many) {
            return Ok(Arc::new(Mutex::new(Decl::Struct(Arc::new(Mutex::new(
                self.struct_decl(decl_mod, token)?,
            ))))));
        }

        todo!()
    }

    fn fn_decl(
        &mut self,
        decl_mod: Option<DeclMod>,
        fn_mod: Option<FnMod>,
        fn_token: Arc<Token>,
    ) -> Result<FnDecl> {
        // TODO: Generics?

        let name = self.consume(&TokenType::Ident, "Expect function name")?;

        let open_paren_token =
            self.consume(&TokenType::OpenParen, "Expect '(' after function name")?;

        let mut params = vec![];

        let pre_comma_token = self.match_any(&[TokenType::Comma], WithNewlines::Many);

        while self.check(&TokenType::Ident) {
            // TODO: Do I care about this ??
            if params.len() >= 255 {
                let t = self.peek().ok_or_else(|| self.eof_err())?;

                return Err(self
                    .error("Can't have more than 255 parameters".to_string(), t)
                    .into());
            }

            let mut try_parse_field = |params: &mut Vec<FnDeclParam>| {
                let name = self.consume(&TokenType::Ident, "Expect parameter name")?;
                let colon_token = self.consume(&TokenType::Colon, "Expect ':' after param name")?;

                let static_type_ref = self.static_type_ref()?;

                let comma_token = self.match_any(&[TokenType::Comma], WithNewlines::Many);
                let is_done = comma_token.is_none();

                params.push(FnDeclParam {
                    name,
                    colon_token,
                    static_type_ref,
                    comma_token,
                    resolved_type: (),
                });

                return Ok::<bool, anyhow::Error>(is_done);
            };

            match try_parse_field(&mut params) {
                Ok(done) => {
                    if done {
                        break;
                    }
                }

                Err(e) => todo!("{e}"),
                // Err(e) => self.synchronize_field(e)?,
            }
        }

        self.allow_many_newlines();

        self.consume(&TokenType::CloseParen, "Expect ')' after parameters")?;

        let return_type =
            if let Some(colon_token) = self.match_any(&[TokenType::Colon], WithNewlines::One) {
                Some(FnDeclReturnType {
                    colon_token,
                    static_type: self.static_type_ref()?,
                    resolved_type: (),
                })
            } else {
                None
            };

        let close_paren_token = self.consume(
            &TokenType::Newline,
            "Expect newline after function signature",
        )?;

        // TODO: Handle short fn body syntax
        let body = FnDeclBody::Block(self.code_block()?);

        return Ok(FnDecl {
            id: NodeId::gen(),
            decl_mod,
            fn_mod,
            fn_token,
            generics: None,
            name,
            open_paren_token,
            pre_comma_token,
            params,
            close_paren_token,
            return_type,
            body,
        });
    }

    fn struct_decl(
        &mut self,
        decl_mod: Option<DeclMod>,
        struct_token: Arc<Token>,
    ) -> Result<StructDecl> {
        // TODO: generics

        let name = self.consume(&TokenType::Ident, "Expected struct name")?;

        let open_squirly_brace_token = self.consume(
            &TokenType::OpenSquirlyBrace,
            "Expected '{' after struct name",
        )?;

        let pre_comma_token = self.match_any(&[TokenType::Comma], WithNewlines::Many);

        let mut fields = vec![];
        let close_squirly_brace_token = loop {
            if let Some(token) = self.match_any(&[TokenType::CloseSquirlyBrace], WithNewlines::Many)
            {
                break token;
            }

            self.allow_many_newlines();

            let field_mod = self
                .match_any(&[TokenType::Pub], WithNewlines::Many)
                .map(StructFieldMod::Pub);

            let name = self.consume(&TokenType::Ident, "Expected field name")?;

            self.allow_one_newline();
            let colon_token = self.consume(&TokenType::Colon, "Expected ':'")?;

            let static_type_ref = self.static_type_ref()?;

            let comma_token = self.match_any(&[TokenType::Comma], WithNewlines::Many);
            let is_done = comma_token.is_none();

            fields.push(StructDeclField {
                field_mod,
                name,
                colon_token,
                static_type_ref,
                comma_token,
            });

            if is_done {
                break self.consume(&TokenType::CloseSquirlyBrace, "Expected comma or '}'")?;
            }
        };

        return Ok(StructDecl {
            id: NodeId::gen(),
            decl_mod,
            struct_token,
            name,
            generics: None,
            open_squirly_brace_token,
            pre_comma_token,
            fields,
            close_squirly_brace_token,
        });
    }

    fn static_type_ref(&mut self) -> Result<StaticType> {
        let ref_token = self.match_any(&[TokenType::Amp], WithNewlines::None);

        let ref_type = if let Some(ref_token) = ref_token {
            if let Some(mut_token) = self.match_any(&[TokenType::Mut], WithNewlines::None) {
                Some(RefType::Mut {
                    ref_token,
                    mut_token,
                })
            } else {
                let const_token = self.match_any(&[TokenType::Const], WithNewlines::None);

                Some(RefType::Shared {
                    ref_token,
                    const_token,
                })
            }
        } else {
            None
        };

        let type_ref = StaticType {
            ref_type,
            static_path: self.static_path()?,
            resolved_type: (),
        };

        return Ok(type_ref);
    }

    fn static_path(&mut self) -> Result<StaticPath> {
        let double_colon_token = self.match_any(&[TokenType::DoubleColon], WithNewlines::None);

        let mut name = self.consume(&TokenType::Ident, "Expect type reference")?;
        let mut path = StaticPath {
            double_colon_token,
            root: None,
            name,
            resolved_type: (),
        };

        while let Some(double_colon_token) =
            self.match_any(&[TokenType::DoubleColon], WithNewlines::None)
        {
            name = self.consume(&TokenType::Ident, "Expect type reference")?;
            path = StaticPath {
                double_colon_token: Some(double_colon_token),
                root: Some(Box::new(path)),
                name,
                resolved_type: (),
            };
        }

        return Ok(path);
    }

    fn code_block(&mut self) -> Result<CodeBlock> {
        return self
            .code_block_with_any_end(&[TokenType::Semicolon])
            .map(|(stmts, token)| CodeBlock {
                stmts,
                end_semicolon_token: token,
            });
    }

    #[allow(clippy::type_complexity)]
    fn code_block_with_any_end(
        &mut self,
        any_end: &[TokenType],
    ) -> Result<(Vec<Arc<Mutex<Stmt>>>, Arc<Token>)> {
        let mut block = vec![];

        let close = loop {
            if let Some(close) = self.match_any(any_end, WithNewlines::Many) {
                break close;
            }

            if self.is_at_end() {
                todo!("Unexpected end of file! Expected one of: {any_end:?}");
            }

            if self.allow_many_newlines() == 0 {
                block.push(self.statement()?);

                if !self.is_at_end() {
                    self.consume(
                        &TokenType::Newline,
                        &format!(
                            "Expect newline after statement. Found {:#?}",
                            self.tokens[self.current_idx]
                        ),
                    )?;
                }
            }
        };

        return Ok((block, close));
    }

    fn statement(&mut self) -> Result<Arc<Mutex<Stmt>>> {
        /*
        if self.match_any(&[token::TokenType::For], WithNewlines::Many) {
            return Ok(ast::Stmt::For(self.for_statement()?));
        }
        */

        if let Some(token) = self.match_any(&[TokenType::Loop], WithNewlines::Many) {
            return Ok(Arc::new(Mutex::new(Stmt::Loop(Arc::new(Mutex::new(
                self.loop_statement(token)?,
            ))))));
        }

        if let Some(token) = self.match_any(&[TokenType::While], WithNewlines::Many) {
            return Ok(Arc::new(Mutex::new(Stmt::While(Arc::new(Mutex::new(
                self.while_statement(token)?,
            ))))));
        }

        if let Some(token) = self.match_any(&[TokenType::Const], WithNewlines::Many) {
            return Ok(Arc::new(Mutex::new(Stmt::VarDecl(Arc::new(Mutex::new(
                self.var_decl_statement(VarDeclMut::Const(token))?,
            ))))));
        }

        if let Some(token) = self.match_any(&[TokenType::Mut], WithNewlines::Many) {
            return Ok(Arc::new(Mutex::new(Stmt::VarDecl(Arc::new(Mutex::new(
                self.var_decl_statement(VarDeclMut::Mut(token))?,
            ))))));
        }

        /*
        if self.match_any(&[token::TokenType::Mut], WithNewlines::Many) {
            return Ok(ast::Stmt::VarDecl(
                self.var_decl_statement(ast::VarDeclType::Mut)?,
            ));
        }
        */

        if let Some(token) = self.match_any(&[TokenType::If], WithNewlines::Many) {
            return Ok(Arc::new(Mutex::new(Stmt::If(Arc::new(Mutex::new(
                self.if_statement(token)?,
            ))))));
        }

        if let Some(token) = self.match_any(&[TokenType::Return], WithNewlines::Many) {
            return Ok(Arc::new(Mutex::new(Stmt::Return(Arc::new(Mutex::new(
                self.return_statement(token)?,
            ))))));
        }

        if let Some(token) = self.match_any(&[TokenType::Break], WithNewlines::Many) {
            return Ok(Arc::new(Mutex::new(Stmt::Break(Arc::new(Mutex::new(
                self.break_statement(token)?,
            ))))));
        }

        if let Some(token) = self.match_any(&[TokenType::Then], WithNewlines::Many) {
            return Ok(Arc::new(Mutex::new(Stmt::Then(Arc::new(Mutex::new(
                self.then_statement(token)?,
            ))))));
        }

        let stmt = self.expr_or_assign_statement()?;

        return Ok(stmt);
    }

    fn loop_statement(&mut self, loop_token: Arc<Token>) -> Result<LoopStmt> {
        let label = self.match_any(&[TokenType::Label], WithNewlines::None);

        let _ = self.consume(&TokenType::Newline, "Expected newline after 'loop' keyword")?;

        let block = self.code_block()?;

        return Ok(LoopStmt {
            id: NodeId::gen(),
            loop_token,
            label,
            block,
            resolved_terminal: None,
        });
    }

    fn loop_expr(&mut self, loop_token: Arc<Token>) -> Result<LoopExpr> {
        let label = self.match_any(&[TokenType::Label], WithNewlines::None);

        let _ = self.consume(&TokenType::Newline, "Expected newline after 'loop' keyword")?;

        let block = self.code_block()?;

        return Ok(LoopExpr {
            id: NodeId::gen(),
            loop_token,
            label,
            block,
            resolved_terminal: None,
            resolved_type: None,
        });
    }

    fn while_statement(&mut self, while_token: Arc<Token>) -> Result<WhileStmt> {
        let label = self.match_any(&[TokenType::Label], WithNewlines::None);

        let condition = NestedExpr(self.expression()?);

        let _ = self.consume(
            &TokenType::Newline,
            "Expected newline after 'while' condition",
        )?;

        let (stmts, end) =
            self.code_block_with_any_end(&[TokenType::Semicolon, TokenType::Else])?;

        let block = CodeBlock {
            stmts,
            end_semicolon_token: (),
        };

        if end.token_type == TokenType::Semicolon {
            return Ok(WhileStmt {
                id: NodeId::gen(),
                while_token,
                label,
                condition,
                block,
                else_: None,
                semicolon_token: end,
                resolved_terminal: None,
            });
        }

        let else_token = end;

        let _ = self.consume(&TokenType::Newline, "Expected newline after 'else'")?;

        let (else_stmts, semicolon_token) =
            self.code_block_with_any_end(&[TokenType::Semicolon])?;

        let else_block = CodeBlock {
            stmts: else_stmts,
            end_semicolon_token: (),
        };

        let else_ = Some(WhileStmtElse {
            else_token,
            block: else_block,
        });

        return Ok(WhileStmt {
            id: NodeId::gen(),
            while_token,
            label,
            condition,
            block,
            else_,
            semicolon_token,
            resolved_terminal: None,
        });
    }

    fn while_expr(&mut self, while_token: Arc<Token>) -> Result<WhileExpr> {
        let label = self.match_any(&[TokenType::Label], WithNewlines::None);

        let condition = NestedExpr(self.expression()?);

        let _ = self.consume(
            &TokenType::Newline,
            "Expected newline after 'while' condition",
        )?;

        let (stmts, mut end) = self.code_block_with_any_end(&[
            TokenType::Semicolon,
            TokenType::Then,
            TokenType::Else,
        ])?;
        let block = CodeBlock {
            stmts,
            end_semicolon_token: (),
        };

        let mut semicolon_token = Some(end.clone());

        let then = if end.token_type == TokenType::Then {
            if self.allow_one_newline() {
                let (stmts, then_end) =
                    self.code_block_with_any_end(&[TokenType::Semicolon, TokenType::Else])?;

                let then_block = CodeBlock {
                    stmts,
                    end_semicolon_token: (),
                };

                let then_token = end;
                end = then_end;
                semicolon_token = Some(end.clone());

                Some(WhileExprThen::Block(WhileExprThenBlock {
                    then_token,
                    block: then_block,
                }))
            } else {
                let then_expr = NestedExpr(self.expression()?);
                let then_token = end.clone();

                if self.check(&TokenType::Newline) && self.check_offset(1, &TokenType::Else) {
                    let _ = self.consume(
                        &TokenType::Newline,
                        "Expected newline between 'then' expression and 'else'",
                    )?;

                    end = self.consume(&TokenType::Else, "Expected 'else'")?;
                    semicolon_token = Some(end.clone());
                } else {
                    semicolon_token = None;
                }

                Some(WhileExprThen::Ternary(WhileExprThenTernary {
                    then_token,
                    then_expr,
                }))
            }
        } else {
            None
        };

        let else_ = if end.token_type == TokenType::Else {
            if self.allow_one_newline() {
                let (stmts, else_end) = self.code_block_with_any_end(&[TokenType::Semicolon])?;

                let else_block = CodeBlock {
                    stmts,
                    end_semicolon_token: (),
                };

                let else_token = end;
                end = else_end;
                semicolon_token = Some(end);

                Some(WhileExprElse::Block(WhileExprElseBlock {
                    else_token,
                    block: else_block,
                }))
            } else {
                let else_expr = NestedExpr(self.expression()?);
                let else_token = end;

                semicolon_token = None;

                Some(WhileExprElse::Ternary(WhileExprElseTernary {
                    else_token,
                    else_expr,
                }))
            }
        } else {
            None
        };

        return Ok(WhileExpr {
            id: NodeId::gen(),
            while_token,
            label,
            condition,
            block,
            then,
            else_,
            semicolon_token,
            resolved_terminal: None,
            resolved_type: None,
        });
    }

    fn var_decl_statement(&mut self, var_mut: VarDeclMut) -> Result<VarDeclStmt> {
        let target = self.var_decl_target()?;

        // TODO: parse explicit type

        let value = if let Some(token) = self.match_any(&[TokenType::Equal], WithNewlines::One) {
            Some(VarDeclValue {
                eq_token: token,
                value: NestedExpr(self.break_expr()?),
            })
        } else {
            None
        };

        return Ok(VarDeclStmt {
            id: NodeId::gen(),
            var_mut,
            target,
            explicit_type: None,
            value,
        });
    }

    fn var_decl_target(&mut self) -> Result<VarDeclTarget> {
        // TODO

        return Ok(VarDeclTarget::Ident(Arc::new(Mutex::new(IdentExpr {
            id: NodeId::gen(),
            ident: self.consume(
                &TokenType::Ident,
                "TODO: Handle more complicated assignment patterns",
            )?,
            resolved_type: (),
        }))));
    }

    fn if_statement(&mut self, if_token: Arc<Token>) -> Result<IfStmt> {
        // TODO: Maybe if condition should be special to allow assigning or naming condition?
        let condition = NestedExpr(self.expression()?);

        let _ = self.consume(&TokenType::Newline, "Expected newline after if condition")?;

        let (stmts, mut end_token) =
            self.code_block_with_any_end(&[TokenType::Semicolon, TokenType::Else])?;

        let then = CodeBlock {
            stmts,
            end_semicolon_token: (),
        };

        let mut else_ifs = vec![];
        let mut else_ = None;

        if end_token.token_type == TokenType::Else {
            while let Some(if_token) = self.match_any(&[TokenType::If], WithNewlines::None) {
                let condition = NestedExpr(self.expression()?);

                let mut should_continue = false;

                let (stmts, end) =
                    self.code_block_with_any_end(&[TokenType::Semicolon, TokenType::Else])?;

                let then = CodeBlock {
                    stmts,
                    end_semicolon_token: (),
                };

                let else_token = end_token;
                end_token = end;

                if end_token.token_type == TokenType::Else {
                    should_continue = true;
                }

                let else_if = IfStmtElseIf {
                    else_token,
                    if_token,
                    condition,
                    then,
                };

                else_ifs.push(else_if);

                if !should_continue {
                    break;
                }
            }

            if end_token.token_type == TokenType::Else {
                let _ = self.allow_one_newline();

                let (stmts, end) = self.code_block_with_any_end(&[TokenType::Semicolon])?;

                let then = CodeBlock {
                    stmts,
                    end_semicolon_token: (),
                };

                let else_token = end_token;
                end_token = end;

                else_ = Some(IfStmtElse { else_token, then });
            }
        }

        return Ok(IfStmt {
            id: NodeId::gen(),
            if_token,
            condition,
            then,
            else_ifs,
            else_,
            semicolon_token: end_token,
            resolved_terminal: None,
        });
    }

    fn if_expr(&mut self, if_token: Arc<Token>) -> Result<IfExpr> {
        let label = self.match_any(&[TokenType::Label], WithNewlines::None);

        // TODO: Maybe if condition should be special to allow assigning or naming condition?
        let condition = NestedExpr(self.expression()?);

        let mut semicolon_token = None;

        let then = if let Some(then_token) = self.match_any(&[TokenType::Then], WithNewlines::None)
        {
            if label.is_some() {
                todo!("Unexpected label!");
            }

            let then_expr = NestedExpr(self.expression()?);

            if let Some(else_token) = self.match_any(&[TokenType::Else], WithNewlines::One) {
                semicolon_token = Some(else_token);
            }

            IfExprThen::Ternary(IfExprThenTernary {
                then_token,
                then_expr,
            })
        } else {
            let _ = self.consume(&TokenType::Newline, "Expected newline after if condition")?;

            let (stmts, end) =
                self.code_block_with_any_end(&[TokenType::Semicolon, TokenType::Else])?;

            semicolon_token = Some(end);

            let block = CodeBlock {
                stmts,
                end_semicolon_token: (),
            };

            IfExprThen::Block(IfExprThenBlock { label, block })
        };

        let mut else_ifs = vec![];
        let mut else_ = None;

        if let Some(mut end_token) = semicolon_token.clone() {
            if end_token.token_type == TokenType::Else {
                while let Some(if_token) = self.match_any(&[TokenType::If], WithNewlines::None) {
                    let label = self.match_any(&[TokenType::Label], WithNewlines::None);

                    let condition = NestedExpr(self.expression()?);

                    let mut should_continue = false;

                    let else_if = if let Some(then_token) =
                        self.match_any(&[TokenType::Then], WithNewlines::None)
                    {
                        if label.is_some() {
                            todo!("Unexpected label");
                        }

                        let then_expr = NestedExpr(self.expression()?);

                        let else_token = end_token.clone();

                        if let Some(end) = self.match_any(&[TokenType::Else], WithNewlines::One) {
                            end_token = end;
                            semicolon_token = Some(end_token.clone());

                            should_continue = true;
                        } else {
                            semicolon_token = None;
                        }

                        IfExprElseIf::Ternary(IfExprElseIfTernary {
                            else_token,
                            if_token,
                            condition,
                            then_token,
                            expr: then_expr,
                        })
                    } else {
                        let (stmts, end) =
                            self.code_block_with_any_end(&[TokenType::Semicolon, TokenType::Else])?;

                        let block = CodeBlock {
                            stmts,
                            end_semicolon_token: (),
                        };

                        let else_token = end_token;
                        end_token = end;
                        semicolon_token = Some(end_token.clone());

                        if end_token.token_type == TokenType::Else {
                            should_continue = true;
                        }

                        IfExprElseIf::Block(IfExprElseIfBlock {
                            else_token,
                            if_token,
                            label,
                            condition,
                            block,
                        })
                    };

                    else_ifs.push(else_if);

                    if !should_continue {
                        break;
                    }
                }

                if end_token.token_type == TokenType::Else {
                    let label = self.match_any(&[TokenType::Label], WithNewlines::None);

                    if self.allow_one_newline() {
                        let (stmts, end) = self.code_block_with_any_end(&[TokenType::Semicolon])?;

                        let block = CodeBlock {
                            stmts,
                            end_semicolon_token: (),
                        };

                        let else_token = end_token;
                        end_token = end;
                        semicolon_token = Some(end_token);

                        else_ = Some(IfExprElse::Block(IfExprElseBlock {
                            else_token,
                            label,
                            block,
                        }));
                    } else {
                        if label.is_some() {
                            dbg!(&label);
                            todo!("Unexpected label!");
                        }

                        let else_expr = NestedExpr(self.expression()?);

                        let else_token = end_token;
                        semicolon_token = None;

                        else_ = Some(IfExprElse::Ternary(IfExprElseTernary {
                            else_token,
                            else_expr,
                        }));
                    }
                }
            }
        }

        return Ok(IfExpr {
            id: NodeId::gen(),
            if_token,
            condition,
            then,
            else_ifs,
            else_,
            semicolon_token,
            resolved_terminal: None,
            resolved_type: None,
        });
    }

    fn return_statement(&mut self, return_token: Arc<Token>) -> Result<ReturnStmt> {
        let value = if self.check(&TokenType::Newline) {
            None
        } else {
            Some(NestedExpr(self.break_expr()?))
        };

        return Ok(ReturnStmt {
            id: NodeId::gen(),
            return_token,
            value,
        });
    }

    fn break_statement(&mut self, break_token: Arc<Token>) -> Result<BreakStmt> {
        let label = self.match_any(&[TokenType::Label], WithNewlines::None);

        let value = if self.check(&TokenType::Newline) {
            None
        } else {
            Some(NestedExpr(self.expression()?))
        };

        return Ok(BreakStmt {
            id: NodeId::gen(),
            break_token,
            label,
            value,
            resolved_type: None,
            handler: None,
        });
    }

    fn then_statement(&mut self, then_token: Arc<Token>) -> Result<ThenStmt> {
        let label = self.match_any(&[TokenType::Label], WithNewlines::None);

        let value = NestedExpr(self.expression()?);

        return Ok(ThenStmt {
            id: NodeId::gen(),
            then_token,
            label,
            value,
            resolved_type: (),
            handler: None,
        });
    }

    fn expr_or_assign_statement(&mut self) -> Result<Arc<Mutex<Stmt>>> {
        let expr = self.expression()?;

        if let Some(op_token) = self.match_any(
            &[
                TokenType::Equal,
                TokenType::PlusEqual,
                TokenType::MinusEqual,
            ],
            WithNewlines::One,
        ) {
            let op = match op_token.token_type {
                TokenType::Equal => AssignOp::Eq(op_token),
                TokenType::PlusEqual => AssignOp::PlusEq(op_token),
                TokenType::MinusEqual => AssignOp::MinusEq(op_token),
                _ => {
                    return Err(self
                        .error(
                            format!("[{}:{}] Expected '=' or '+='", file!(), line!()),
                            op_token,
                        )
                        .into())
                }
            };

            let value = self.break_expr()?;

            return Ok(Arc::new(Mutex::new(Stmt::Assign(Arc::new(Mutex::new(
                AssignStmt {
                    id: NodeId::gen(),
                    target: NestedExpr(expr),
                    op,
                    value: NestedExpr(value),
                },
            ))))));
        } else {
            return Ok(Arc::new(Mutex::new(Stmt::Expr(Arc::new(Mutex::new(
                ExprStmt {
                    id: NodeId::gen(),
                    expr,
                },
            ))))));
        }
    }

    fn break_expr(&mut self) -> Result<Arc<Mutex<Expr>>> {
        if let Some(token) = self.match_any(&[TokenType::Loop], WithNewlines::Many) {
            return Ok(Arc::new(Mutex::new(Expr::Loop(Arc::new(Mutex::new(
                self.loop_expr(token)?,
            ))))));
        }

        if let Some(token) = self.match_any(&[TokenType::While], WithNewlines::Many) {
            return Ok(Arc::new(Mutex::new(Expr::While(Arc::new(Mutex::new(
                self.while_expr(token)?,
            ))))));
        }

        if let Some(token) = self.match_any(&[TokenType::If], WithNewlines::Many) {
            return Ok(Arc::new(Mutex::new(Expr::If(Arc::new(Mutex::new(
                self.if_expr(token)?,
            ))))));
        }

        return self.expression();
    }

    fn expression(&mut self) -> Result<Arc<Mutex<Expr>>> {
        return self.or();
    }

    fn or(&mut self) -> Result<Arc<Mutex<Expr>>> {
        let expr = self.and()?;

        /*
        while let Some(operator) = self.match_any(&[TokenType::Or], WithNewlines::One) {
            let right = self.and()?;

            expr = Expr::Logical(LogicalExpr {
                id: NodeId::gen(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        */

        return Ok(expr);
    }

    fn and(&mut self) -> Result<Arc<Mutex<Expr>>> {
        let expr = self.equality()?;

        /*
        while let Some(operator) = self.match_any(&[TokenType::And], WithNewlines::One) {
            let right = self.equality()?;

            expr = Expr::Logical(LogicalExpr {
                id: NodeId::gen(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        */

        return Ok(expr);
    }

    fn equality(&mut self) -> Result<Arc<Mutex<Expr>>> {
        let expr = self.comparison()?;

        /*
        while let Some(op_token) = self.match_any(
            &[TokenType::BangEqual, TokenType::EqualEqual],
            WithNewlines::One,
        ) {
            let op = match op_token.token_type {
                TokenType::BangEqual => (BinaryOp::NotEqual, op_token),
                TokenType::EqualEqual => (BinaryOp::EqualEqual, op_token),

                _ => {
                    return Err(self.error(
                        format!("[{}:{}] Expected '!=' or '=='", file!(), line!()),
                        op_token,
                    ).into())
                }
            };

            let right = self.comparison()?;

            expr = Expr::Binary(BinaryExpr {
                id: NodeId::gen(),
                left: Box::new(expr),
                op,
                right: Box::new(right),
            });
        }
        */

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Arc<Mutex<Expr>>> {
        let mut expr = self.range()?;

        while let Some(op_token) = self.match_any(
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
            WithNewlines::One,
        ) {
            let op = match op_token.token_type {
                TokenType::Greater => BinaryOp::Greater(op_token),
                TokenType::GreaterEqual => BinaryOp::GreaterEq(op_token),
                TokenType::Less => BinaryOp::Less(op_token),
                TokenType::LessEqual => BinaryOp::LessEq(op_token),

                _ => {
                    return Err(self
                        .error(
                            format!("[{}:{}] Expected '>', '>=', '<', or '<='", file!(), line!()),
                            op_token,
                        )
                        .into());
                }
            };

            let right = self.range()?;

            expr = Arc::new(Mutex::new(Expr::Binary(Arc::new(Mutex::new(BinaryExpr {
                id: NodeId::gen(),
                lhs: NestedExpr(expr),
                op,
                rhs: NestedExpr(right),
                resolved_type: (),
            })))));
        }

        return Ok(expr);
    }

    fn range(&mut self) -> Result<Arc<Mutex<Expr>>> {
        let expr = self.term()?;

        /*
        while let Some(op_token) = self.match_any(&[TokenType::DotDot], WithNewlines::One) {
            let op = match op_token.token_type {
                TokenType::DotDot => (BinaryOp::Range, op_token),

                _ => {
                    return Err(self
                        .error(format!("[{}:{}] Expected '..'", file!(), line!()), op_token)
                        .into());
                }
            };

            let right = self.term()?;

            expr = Expr::Binary(BinaryExpr {
                id: NodeId::gen(),
                left: Box::new(expr),
                op,
                right: Box::new(right),
            });
        }
        */

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Arc<Mutex<Expr>>> {
        let mut expr = self.factor()?;

        while let Some(op_token) =
            self.match_any(&[TokenType::Minus, TokenType::Plus], WithNewlines::One)
        {
            let op = match op_token.token_type {
                TokenType::Minus => BinaryOp::Subtract(op_token),
                TokenType::Plus => BinaryOp::Add(op_token),

                _ => {
                    return Err(self
                        .error(
                            format!("[{}:{}] Expected '+' or '-'", file!(), line!()),
                            op_token,
                        )
                        .into())
                }
            };

            let right = self.factor()?;

            expr = Arc::new(Mutex::new(Expr::Binary(Arc::new(Mutex::new(BinaryExpr {
                id: NodeId::gen(),
                lhs: NestedExpr(expr),
                op,
                rhs: NestedExpr(right),
                resolved_type: (),
            })))));
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Arc<Mutex<Expr>>> {
        let expr = self.modulo()?;

        /*
        while let Some(op_token) =
            self.match_any(&[TokenType::Slash, TokenType::Asterisk], WithNewlines::One)
        {
            let op = match op_token.token_type {
                TokenType::Slash => (BinaryOp::Divide, op_token),
                TokenType::Asterisk => (BinaryOp::Times, op_token),

                _ => {
                    return Err(self
                        .error(
                            format!("[{}:{}] Expected '/' or '*'", file!(), line!()),
                            op_token,
                        )
                        .into())
                }
            };

            let right = self.modulo()?;

            expr = Expr::Binary(BinaryExpr {
                id: NodeId::gen(),
                left: Box::new(expr),
                op,
                right: Box::new(right),
            });
        }
        */

        return Ok(expr);
    }

    fn modulo(&mut self) -> Result<Arc<Mutex<Expr>>> {
        let expr = self.unary()?;

        /*
        while let Some(op_token) = self.match_any(&[TokenType::Percent], WithNewlines::One) {
            let op = match op_token.token_type {
                TokenType::Percent => (BinaryOp::Modulo, op_token),

                _ => {
                    return Err(self
                        .error(format!("[{}:{}] Expected '%'", file!(), line!()), op_token)
                        .into())
                }
            };

            let right = self.unary()?;

            expr = Expr::Binary(BinaryExpr {
                id: NodeId::gen(),
                left: Box::new(expr),
                op,
                right: Box::new(right),
            });
        }
        */

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Arc<Mutex<Expr>>> {
        // TODO: Do we want to stop 'not' and '&' chains?

        if let Some(op_token) = self.match_any(
            &[TokenType::Not, /*TokenType::Minus,*/ TokenType::Amp],
            WithNewlines::One,
        ) {
            let op = match &op_token.token_type {
                TokenType::Not => UnaryOp::Not(op_token),
                // TokenType::Minus => (UnaryOp::Minus, op_token),
                TokenType::Amp => {
                    if let Some(mut_token) = self.match_any(&[TokenType::Mut], WithNewlines::None) {
                        UnaryOp::Ref(RefType::Mut {
                            ref_token: op_token,
                            mut_token,
                        })
                    } else {
                        let const_token = self.match_any(&[TokenType::Const], WithNewlines::None);

                        UnaryOp::Ref(RefType::Shared {
                            ref_token: op_token,
                            const_token,
                        })
                    }
                }

                _ => {
                    return Err(self
                        .error(
                            format!("[{}:{}] Expected '!' or '-'", file!(), line!()),
                            op_token,
                        )
                        .into())
                }
            };

            let value = self.unary()?;

            return Ok(Arc::new(Mutex::new(Expr::Unary(Arc::new(Mutex::new(
                UnaryExpr {
                    id: NodeId::gen(),
                    op,
                    value: NestedExpr(value),
                    resolved_type: (),
                },
            ))))));
        }

        return self.call();
    }

    fn call(&mut self) -> Result<Arc<Mutex<Expr>>> {
        let mut expr = self.ident()?;

        loop {
            if let Some(open_paren_token) =
                self.match_any(&[TokenType::OpenParen], WithNewlines::None)
            {
                expr = self.finish_call(expr, open_paren_token)?;
            } else if let Some(dot_token) = self.match_any(&[TokenType::Dot], WithNewlines::One) {
                let name = self.consume(&TokenType::Ident, "Expect property name '.'")?;

                expr = Arc::new(Mutex::new(Expr::Get(Arc::new(Mutex::new(GetExpr {
                    id: NodeId::gen(),
                    target: NestedExpr(expr),
                    dot_token,
                    name,
                    resolved_type: (),
                })))));
            } else {
                break;
            }
        }

        return Ok(expr);
    }

    fn finish_call(
        &mut self,
        callee: Arc<Mutex<Expr>>,
        open_paren_token: Arc<Token>,
    ) -> Result<Arc<Mutex<Expr>>> {
        let mut args = vec![];

        let pre_comma_token = self.match_any(&[TokenType::Comma], WithNewlines::Many);
        self.allow_many_newlines();

        if !self.check(&TokenType::CloseParen) {
            loop {
                // TODO: Do I care?
                if args.len() >= 255 {
                    let t = self.peek().ok_or_else(|| self.eof_err())?;
                    return Err(self
                        .error("Can't have more than 255 arguments".to_string(), t)
                        .into());
                }

                let value = NestedExpr(self.expression()?);
                self.allow_many_newlines();
                let post_comma_token = self.match_any(&[TokenType::Comma], WithNewlines::Many);
                self.allow_many_newlines();

                args.push(CallArg {
                    param_name: None,
                    value,
                    post_comma_token: post_comma_token.clone(),
                    resolved_type: (),
                });

                if post_comma_token.is_none() {
                    break;
                }
            }
        }

        let close_paren_token = self.consume(
            &TokenType::CloseParen,
            "Expect ')' after arguments".to_string(),
        )?;

        return Ok(Arc::new(Mutex::new(Expr::Call(Arc::new(Mutex::new(
            CallExpr {
                id: NodeId::gen(),
                callee: NestedExpr(callee),
                open_paren_token,
                pre_comma_token,
                args,
                close_paren_token,
                resolved_type: None,
            },
        ))))));
    }

    fn ident(&mut self) -> Result<Arc<Mutex<Expr>>> {
        let ident_expr = if self.check(&TokenType::DoubleColon)
            || (self.check(&TokenType::Ident) && self.check_offset(1, &TokenType::DoubleColon))
        {
            Some(ConstructTarget::StaticPath(self.static_path()?))
        } else {
            self.match_any(&[TokenType::Ident], WithNewlines::None)
                .map(|ident| {
                    ConstructTarget::Ident(Arc::new(Mutex::new(IdentExpr {
                        id: NodeId::gen(),
                        ident,
                        resolved_type: (),
                    })))
                })
        };

        if let Some(ident_expr) = ident_expr {
            if let Some(open_squirly_brace) =
                self.match_any(&[TokenType::OpenSquirlyBrace], WithNewlines::One)
            {
                let mut args = vec![];
                let close_squirly_brace = loop {
                    if let Some(token) =
                        self.match_any(&[TokenType::CloseSquirlyBrace], WithNewlines::One)
                    {
                        break token;
                    }

                    self.allow_many_newlines();

                    let name = self.consume(&TokenType::Ident, "Expected field name")?;
                    let colon_token = self.consume(&TokenType::Colon, "Expected ':'")?;

                    self.allow_many_newlines();
                    let value = NestedExpr(self.expression()?);

                    self.allow_many_newlines();
                    let comma_token = self.match_any(&[TokenType::Comma], WithNewlines::One);
                    let is_done = comma_token.is_none();

                    args.push(ConstructArg::Field(ConstructField {
                        name,
                        colon_token,
                        value,
                        comma_token,
                    }));

                    if is_done {
                        break self.consume(&TokenType::CloseSquirlyBrace, "Expected '}'")?;
                    }
                };

                return Ok(Arc::new(Mutex::new(Expr::Construct(Arc::new(Mutex::new(
                    ConstructExpr {
                        id: NodeId::gen(),
                        target: ident_expr,
                        open_squirly_brace,
                        args,
                        close_squirly_brace,
                        resolved_type: (),
                    },
                ))))));
            }

            let ident_expr = match ident_expr {
                ConstructTarget::Ident(ident) => Expr::Ident(ident),
                ConstructTarget::StaticPath(static_path) => {
                    Expr::StaticRef(Arc::new(Mutex::new(StaticRefExpr {
                        id: NodeId::gen(),
                        static_path,
                        resolved_type: (),
                    })))
                }
            };

            return Ok(Arc::new(Mutex::new(ident_expr)));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Arc<Mutex<Expr>>> {
        match self.advance().as_ref().map(|t| (t.clone(), &t.token_type)) {
            Some((t, TokenType::PlainString)) => {
                return Ok(Arc::new(Mutex::new(Expr::PlainStringLiteral(Arc::new(
                    Mutex::new(PlainStringLiteralExpr {
                        id: NodeId::gen(),
                        literal: t,
                        resolved_type: (),
                    }),
                )))))
            }

            Some((t, TokenType::OpenFmtString)) => {
                let mut rest = vec![];
                loop {
                    let mut done = false;

                    let expr = NestedExpr(self.expression()?);

                    let string =
                        match self.match_any(&[TokenType::MidFmtString], WithNewlines::None) {
                            Some(t) => t.lexeme.clone(),
                            None => {
                                done = true;
                                self.consume(
                                    &TokenType::CloseFmtString,
                                    "Expected format string to be closed",
                                )?
                                .lexeme
                                .clone()
                            }
                        };

                    rest.push(FmtStringPart { expr, string });

                    if done {
                        break;
                    }
                }

                return Ok(Arc::new(Mutex::new(Expr::FmtStringLiteral(Arc::new(
                    Mutex::new(FmtStringLiteralExpr {
                        id: NodeId::gen(),
                        first: t,
                        rest,
                        resolved_type: (),
                    }),
                )))));
            }

            Some((t, TokenType::IntegerNumber)) => {
                return Ok(Arc::new(Mutex::new(Expr::NumberLiteral(Arc::new(
                    Mutex::new(NumberLiteralExpr {
                        id: NodeId::gen(),
                        details: NumberLiteralDetails::Integer(t.lexeme.parse()?),
                        literal: t,
                        resolved_type: (),
                    }),
                )))));
            }
            Some((t, TokenType::DecimalNumber)) => {
                return Ok(Arc::new(Mutex::new(Expr::NumberLiteral(Arc::new(
                    Mutex::new(NumberLiteralExpr {
                        id: NodeId::gen(),
                        details: NumberLiteralDetails::Decimal(t.lexeme.parse()?),
                        literal: t,
                        resolved_type: (),
                    }),
                )))));
            }

            Some((t, TokenType::True | TokenType::False)) => {
                return Ok(Arc::new(Mutex::new(Expr::BoolLiteral(Arc::new(
                    Mutex::new(BoolLiteralExpr {
                        id: NodeId::gen(),
                        literal: t,
                        resolved_type: (),
                    }),
                )))));
            }

            /*
            Some(
                t @ Token {
                    token_type:
                        TokenType::False | TokenType::True | TokenType::Number | TokenType::Char..,
                },
            ) => {
                let literal_type = match t.token_type {
                    TokenType::False => PlainLiteralType::False,
                    TokenType::True => PlainLiteralType::True,
                    TokenType::Number => PlainLiteralType::Number,
                    TokenType::Char => PlainLiteralType::Char,
                    _ => unreachable!(),
                };

                return Ok(Arc::new(Mutex::new(Expr::PlainLiteral(PlainLiteralExpr {
                    id: NodeId::gen(),
                    literal_type,
                    token: t,
                }))));
            }

            Some(
                open_token @ Token {
                    token_type: TokenType::FormatStringOpen,
                    ..
                },
            ) => {
                let mut parts = vec![];

                loop {
                    let left_token =
                        self.consume(&TokenType::LeftBrace, "Expected '{' in format string")?;

                    let expr = self.expression()?;

                    let right_token =
                        self.consume(&TokenType::RightBrace, "Expected '}' in format string")?;

                    let fmt_str_part = self.peek().ok_or_else(|| self.eof_err())?;
                    let is_done = match fmt_str_part.token_type {
                        TokenType::FormatStringMid => false,
                        TokenType::FormatStringClose => true,

                        _ => {
                            return Err(self
                                .error(
                                    "Expected part of a format string.".to_string(),
                                    fmt_str_part,
                                )
                                .into())
                        }
                    };
                    let _ = self.advance();

                    parts.push(FormatStringExprPart {
                        left_brace: left_token,
                        expr: Box::new(expr),
                        right_brace: right_token,
                        fmt_str_part,
                    });

                    if is_done {
                        break;
                    }
                }

                return Ok(Expr::FormatString(FormatStringExpr {
                    id: NodeId::gen(),
                    open: open_token,
                    parts,
                }));
            }

            Some(
                t @ Token {
                    token_type: TokenType::SelfVal,
                    ..
                },
            ) => {
                return Ok(Expr::SelfVal(SelfValExpr {
                    id: NodeId::gen(),
                    keyword: t,
                }));
            }

            Some(Token {
                token_type: TokenType::Crash,
                mut span,
                ..
            }) => {
                let error = if let Some(Token {
                    token_type: TokenType::Newline,
                    ..
                }) = self.peek()
                {
                    None
                } else {
                    let error = self.expression()?;

                    if let Some(prev) = self.previous() {
                        span.end = prev.span.end.clone();
                    }

                    Some(Box::new(error))
                };

                return Ok(Expr::Crash(CrashExpr {
                    id: NodeId::gen(),
                    error,
                    span,
                }));
            }
            */
            Some((peek, _)) => {
                return Err(self
                    .error(
                        format!(
                            "[{}:{}] Expected some expression. Found {peek:#?}",
                            file!(),
                            line!()
                        ),
                        peek,
                    )
                    .into());
            }

            None => return Err(self.eof_err().into()),
        }
    }

    fn consume(
        &mut self,
        token_type: &TokenType,
        err_msg: impl Into<String>,
    ) -> Result<Arc<Token>> {
        if self.check(token_type) {
            return self.advance().ok_or_else(|| unreachable!());
        }

        let t = self.peek().ok_or_else(|| self.eof_err())?;

        return Err(self.error(err_msg.into(), t).into());
    }

    fn eof_err(&mut self) -> ParserError {
        let message = format!("[{}:{}] Unexpected end of file.", file!(), line!());
        // self.error_ctx.error(Span::new(), message.clone());

        return ParserError::Error { message };
    }

    fn error(&mut self, message: String, _t: Arc<Token>) -> ParserError {
        // self.error_ctx.token_error(t, message.clone());

        return ParserError::Error { message };
    }

    fn allow_many_newlines(&mut self) -> usize {
        let mut any_newlines = 0;

        while self.allow_one_newline() {
            any_newlines += 1;
        }

        return any_newlines;
    }

    fn allow_one_newline(&mut self) -> bool {
        return self
            .match_any(&[TokenType::Newline], WithNewlines::None)
            .is_some();
    }

    fn match_any(
        &mut self,
        token_types: &[TokenType],
        with_newlines: WithNewlines,
    ) -> Option<Arc<Token>> {
        let newlines: usize = match with_newlines {
            WithNewlines::None => 0,
            WithNewlines::One => {
                if self.allow_one_newline() {
                    1
                } else {
                    0
                }
            }

            WithNewlines::Many => self.allow_many_newlines(),
        };

        for token_type in token_types {
            if self.check(token_type) {
                return self.advance();
            }
        }

        for _ in 0..newlines {
            self.backtrack();
        }

        return None;
    }

    fn check(&self, token_type: &TokenType) -> bool {
        return self.check_offset(0, token_type);
    }

    fn check_offset(&self, offset: usize, token_type: &TokenType) -> bool {
        return self
            .tokens
            .get(self.current_idx + offset)
            .map(|peek| peek.token_type == *token_type)
            .unwrap_or(false);
    }

    fn advance(&mut self) -> Option<Arc<Token>> {
        if !self.is_at_end() {
            self.current_idx += 1;
        }

        return self.previous();
    }

    fn backtrack(&mut self) -> Option<Arc<Token>> {
        if self.current_idx == 0 {
            return None;
        }

        self.current_idx -= 1;

        return self.peek();
    }

    fn is_at_end(&self) -> bool {
        return self.current_idx >= self.tokens.len();
    }

    fn peek(&self) -> Option<Arc<Token>> {
        return self.tokens.get(self.current_idx).cloned();
    }

    fn previous(&self) -> Option<Arc<Token>> {
        if self.current_idx == 0 {
            return None;
        }

        return self.tokens.get(self.current_idx - 1).cloned();
    }
}
