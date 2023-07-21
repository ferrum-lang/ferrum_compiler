use ferrum_compiler::code_gen::*;
use ferrum_compiler::parser::*;
use ferrum_compiler::syntax::*;
use ferrum_compiler::token::*;

use std::sync::{Arc, Mutex};

fn main() -> ferrum_compiler::result::Result {
    /*
         use ::fe::print
         use ::fe::print_err

         pub fn main()
             print("Hello, world!")
             print("Howdy Twitch chat! :D")
         ;
    */

    let tokens = Arc::new(Mutex::new(FeTokenPackage::File(FeTokenFile {
        name: TokenPackageName("_main".into()),
        path: "src/_main.fe".into(),
        tokens: Arc::new(Mutex::new(vec![
            // use
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
            // use
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
                lexeme: "print_err".into(),
                span: Span::zero(),
            }),
            Arc::new(Token {
                token_type: TokenType::Newline,
                lexeme: "\n".into(),
                span: Span::zero(),
            }),
            // fn
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
            // print
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
                token_type: TokenType::PlainString,
                lexeme: r#""Hello, World!""#.into(),
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
            // print
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
                token_type: TokenType::PlainString,
                lexeme: r#""Howdy Twitch chat! :D""#.into(),
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
            // ;
            Arc::new(Token {
                token_type: TokenType::Semicolon,
                lexeme: ";".into(),
                span: Span::zero(),
            }),
        ])),
    })));

    let pkg = Arc::new(Mutex::new(FeSyntaxParser::parse_package(tokens)?));
    dbg!(&pkg);

    let rust_ir = Arc::new(Mutex::new(RustSyntaxCompiler::compile_package(pkg)?));
    dbg!(&rust_ir);

    let rust_code = RustCodeGen::generate_code(rust_ir)?;
    dbg!(&rust_code);

    println!("\n\nRUST CODE:\n\n{}\n\n", rust_code.files[0].content);

    return Ok(());
}
