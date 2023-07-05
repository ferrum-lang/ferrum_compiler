use ferrum_compiler::syntax::*;
use ferrum_compiler::token::*;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    /*
    use ::fe::print

    pub fn main()
        print("Hello, World!")
    ;
    */
    let pkg = Rc::new(RefCell::new(FePackage::File(FeFile {
        name: PackageName("_main".into()),
        path: "src/_main.fe".into(),
        syntax: Rc::new(RefCell::new(SyntaxTree {
            uses: vec![Rc::new(RefCell::new(Use {
                id: NodeId::gen(),
                use_token: Token {
                    token_type: TokenType::Use,
                    lexeme: "use".into(),
                    span: Span::zero(),
                },
                use_mod: None,
                pre_double_colon_token: Some(Token {
                    token_type: TokenType::DoubleColon,
                    lexeme: "::".into(),
                    span: Span::zero(),
                }),
                path: UseStaticPath {
                    name: Token {
                        token_type: TokenType::Ident,
                        lexeme: "fe".into(),
                        span: Span::zero(),
                    },
                    next: Some(UseStaticPathNext::Single(UseStaticPathNextSingle {
                        double_colon_token: Token {
                            token_type: TokenType::DoubleColon,
                            lexeme: "::".into(),
                            span: Span::zero(),
                        },
                        path: Box::new(UseStaticPath {
                            name: Token {
                                token_type: TokenType::Ident,
                                lexeme: "print".into(),
                                span: Span::zero(),
                            },
                            next: None,
                        }),
                    })),
                },
            }))],
            decls: vec![Rc::new(RefCell::new(Decl::Fn(FnDecl {
                id: NodeId::gen(),
                decl_mod: Some(DeclMod::Pub(Token {
                    token_type: TokenType::Pub,
                    lexeme: "pub".into(),
                    span: Span::zero(),
                })),
                fn_token: Token {
                    token_type: TokenType::Fn,
                    lexeme: "fn".into(),
                    span: Span::zero(),
                },
                fn_mod: None,
                name: Token {
                    token_type: TokenType::Ident,
                    lexeme: "main".into(),
                    span: Span::zero(),
                },
                generics: None,
                open_paren_token: Token {
                    token_type: TokenType::OpenParen,
                    lexeme: "(".into(),
                    span: Span::zero(),
                },
                params: vec![],
                close_paren_token: Token {
                    token_type: TokenType::CloseParen,
                    lexeme: ")".into(),
                    span: Span::zero(),
                },
                return_type: None,
                body: FnDeclBody::Block(CodeBlock {
                    stmts: vec![Stmt::Expr(ExprStmt {
                        id: NodeId::gen(),
                        expr: Expr::Call(CallExpr {
                            id: NodeId::gen(),
                            callee: Box::new(Expr::Ident(IdentExpr {
                                id: NodeId::gen(),
                                ident: Token {
                                    token_type: TokenType::Ident,
                                    lexeme: "print".into(),
                                    span: Span::zero(),
                                },
                            })),
                            open_paren_token: Token {
                                token_type: TokenType::OpenParen,
                                lexeme: "(".into(),
                                span: Span::zero(),
                            },
                            args: vec![CallArg {
                                param_name: None,
                                value: Box::new(Expr::StringLiteral(StringLiteralExpr {
                                    id: NodeId::gen(),
                                    literal: Token {
                                        token_type: TokenType::StringLiteral,
                                        lexeme: r#""Hello, World!""#.into(),
                                        span: Span::zero(),
                                    },
                                })),
                            }],
                            close_paren_token: Token {
                                token_type: TokenType::CloseParen,
                                lexeme: ")".into(),
                                span: Span::zero(),
                            },
                        }),
                    })],
                    end_semicolon_token: Token {
                        token_type: TokenType::Semicolon,
                        lexeme: ";".into(),
                        span: Span::zero(),
                    },
                }),
            })))],
        })),
    })));
    dbg!(&pkg);

    let res = RustSyntaxCompiler::compile_package(pkg);
    dbg!(&res);
}
