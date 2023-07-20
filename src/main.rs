use ferrum_compiler::code_gen::*;
use ferrum_compiler::parser::*;
use ferrum_compiler::syntax::*;
use ferrum_compiler::token::*;

use std::sync::{Arc, Mutex};

fn main() -> ferrum_compiler::result::Result {
    /*
    use ::fe::print

    pub fn main()
        print("Hello, World!")
    ;
    */

    let tokens = Arc::new(Mutex::new(FeTokenPackage::File(FeTokenFile {
        name: TokenPackageName("_main".into()),
        path: "src/_main.fe".into(),
        tokens: Arc::new(Mutex::new(vec![
            Arc::new(Token {
                token_type: TokenType::Use,
                lexeme: "use".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::DoubleColon,
                lexeme: "::".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::Ident,
                lexeme: "fe".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::DoubleColon,
                lexeme: "::".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::Ident,
                lexeme: "print".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::Newline,
                lexeme: "\n".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::Newline,
                lexeme: "\n".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::Pub,
                lexeme: "pub".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::Fn,
                lexeme: "fn".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::Ident,
                lexeme: "main".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::OpenParen,
                lexeme: "(".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::CloseParen,
                lexeme: ")".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::Newline,
                lexeme: "\n".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::Ident,
                lexeme: "print".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::OpenParen,
                lexeme: "(".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::StringLiteral,
                lexeme: r#""Hello, World!""#.into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::CloseParen,
                lexeme: ")".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::Semicolon,
                lexeme: ";".into(),
                span: Span::zero(),
            }),
        ])),
    })));

    let pkg = Arc::new(Mutex::new(FeSyntaxParser::parse_package(tokens)?));
    /*
    let pkg = Arc::new(Mutex::new(FeSyntaxPackage::File(FeSyntaxFile {
        name: PackageName("_main".into()),
        path: "src/_main.fe".into(),
        syntax: Arc::new(Mutex::new(SyntaxTree {
            uses: vec![Arc::new(Mutex::new(Use {
                id: NodeId::gen(),
                use_token: Token {
                    token_type: TokenType::Use,
                    lexeme: "".into(),
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
                            lexeme: "".into(),
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
            decls: vec![Arc::new(Mutex::new(Decl::Fn(FnDecl {
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
    */
    dbg!(&pkg);

    let rust_ir = Arc::new(Mutex::new(RustSyntaxCompiler::compile_package(pkg)?));
    dbg!(&rust_ir);

    let rust_code = RustCodeGen::generate_code(rust_ir)?;
    dbg!(&rust_code);

    println!("\n\nRUST CODE:\n\n{}\n\n", rust_code.files[0].content);

    return Ok(());
}
