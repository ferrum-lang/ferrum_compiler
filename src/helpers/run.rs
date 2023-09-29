use crate::code_gen::RustCodeGen;
use crate::executor::RustExecutor;
use crate::ir::RustSyntaxCompiler;
use crate::lexer::FeLexer;
use crate::parser::FeSyntaxParser;
use crate::project_gen::RustProjectGen;
use crate::reader::Reader;
use crate::type_resolver::FeTypeResolver;

use crate::result::Result;

// Traits
use crate::code_gen::CodeGen;
use crate::project_gen::ProjectGen;
use crate::syntax::SyntaxCompiler;

use std::path::PathBuf;
use std::process;
use std::sync::{Arc, Mutex};

pub fn run_full(root_dir: PathBuf) -> Result<process::Output> {
    let gen_output_dir = root_dir.join(".ferrum/compiled_rust");

    // Read source files
    let source = Arc::new(Mutex::new(Reader::read_project_files(root_dir)?));

    // Scan to tokens
    let tokens = Arc::new(Mutex::new(FeLexer::scan_package(source)?));
    // dbg!(&tokens);

    // Parse to AST
    let pkg = FeSyntaxParser::parse_package(tokens)?;

    // Resolve AST types
    let typed_pkg = Arc::new(Mutex::new(FeTypeResolver::resolve_package(pkg)?));

    // Compile to Rust IR
    let rust_ir = Arc::new(Mutex::new(RustSyntaxCompiler::compile_package(typed_pkg)?));

    // Generate Rust code
    let rust_code = Arc::new(Mutex::new(RustCodeGen::generate_code(rust_ir)?));

    // Write Rust output source files
    let _ = RustProjectGen::generate_project_files(rust_code, &gen_output_dir)?;

    // Run generated Rust project
    let out = RustExecutor::cargo_run(&gen_output_dir)?;

    return Ok(out);
}
