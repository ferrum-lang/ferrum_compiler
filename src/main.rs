use ferrum_compiler::code_gen::*;
use ferrum_compiler::lexer::*;
use ferrum_compiler::parser::*;
use ferrum_compiler::source::*;
use ferrum_compiler::syntax::*;
use ferrum_compiler::type_resolver::*;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn main() -> ferrum_compiler::result::Result {
    // let mut local_packages = HashMap::new();

    // local_packages.insert(
    //     SourcePackageName("hello".into()),
    //     Arc::new(Mutex::new(FeSourcePackage::File(FeSourceFile {
    //         name: SourcePackageName("hello".into()),
    //         path: "src/hello.fe".into(),
    //         content: r#"
    //         use ::fe::print

    //         pub fn hello()
    //             print("hello")
    //         ;
    //     "#
    //         .into(),
    //     }))),
    // );

    let source = Arc::new(Mutex::new(FeSourcePackage::Dir(FeSourceDir {
        name: SourcePackageName("src".into()),
        path: "src/".into(),
        entry_file: FeSourceFile {
            name: SourcePackageName("_main".into()),
            path: "src/_main.fe".into(),
            content: r#"
            use ::fe::print

            pub fn main()
                const name = "Adam"

                print("Hello {name} :)")
            ;
       "#
            .into(),
        },
        // local_packages,
        local_packages: HashMap::new(),
    })));

    let tokens = Arc::new(Mutex::new(FeLexer::scan_package(source)?));
    // dbg!(&tokens);

    let pkg = FeSyntaxParser::parse_package(tokens)?;
    // dbg!(&pkg);

    let typed_pkg = Arc::new(Mutex::new(FeTypeResolver::resolve_package(pkg)?));
    // dbg!(&typed_pkg);

    let rust_ir = Arc::new(Mutex::new(RustSyntaxCompiler::compile_package(typed_pkg)?));
    // dbg!(&rust_ir);

    let rust_code = RustCodeGen::generate_code(rust_ir)?;
    // dbg!(&rust_code);

    println!("\n");

    for file in &rust_code.files {
        println!("// {:?}\n{}", file.path, file.content);
    }

    return Ok(());
}
