use ferrum_compiler::code_gen::*;
use ferrum_compiler::executor::*;
use ferrum_compiler::ir::*;
use ferrum_compiler::lexer::*;
use ferrum_compiler::parser::*;
use ferrum_compiler::project_gen::*;
use ferrum_compiler::reader::*;
use ferrum_compiler::syntax::*;
use ferrum_compiler::type_resolver::*;

use std::env;
use std::path;
use std::sync::{Arc, Mutex};

fn main() -> ferrum_compiler::result::Result {
    let root_dir = get_root_dir();

    let source = read_project_files(root_dir)?;

    let tokens = Arc::new(Mutex::new(FeLexer::scan_package(source)?));
    dbg!(&tokens);

    let pkg = FeSyntaxParser::parse_package(tokens)?;
    // dbg!(&pkg);

    let typed_pkg = Arc::new(Mutex::new(FeTypeResolver::resolve_package(pkg)?));
    // dbg!(&typed_pkg);

    let rust_ir = Arc::new(Mutex::new(RustSyntaxCompiler::compile_package(typed_pkg)?));
    // dbg!(&rust_ir);

    let rust_code = Arc::new(Mutex::new(RustCodeGen::generate_code(rust_ir)?));
    // dbg!(&rust_code);

    println!("\n\nCompiled Files:\n------\n");
    for file in &rust_code.lock().unwrap().files {
        println!("// {:?}\n{}", file.path, file.content);
    }
    // todo!();

    let dst = path::PathBuf::from("./.ferrum/compiled_rust");

    let rust_project = RustProjectGen::generate_project_files(rust_code, &dst)?;
    // dbg!(&rust_project);
    let _ = rust_project;

    // // //

    // process::Command::new("clear").status()?;

    let out = RustExecutor::cargo_run(&dst)?;

    println!("{}", String::from_utf8(out.stderr)?);
    println!("Output:\n------\n");

    println!("{}", String::from_utf8(out.stdout)?);

    return Ok(());
}

fn get_root_dir() -> path::PathBuf {
    if env::args().count() != 2 {
        panic!("Expected 1 arg: path to the root of the project directory");
    }

    let root_dir = env::args().last().unwrap();
    let root_dir = path::PathBuf::from(root_dir);
    if !root_dir.is_dir() {
        panic!(
            "Expected arg 1 to be a directory to the project root, found {:?}",
            root_dir
        );
    }

    return root_dir;
}
