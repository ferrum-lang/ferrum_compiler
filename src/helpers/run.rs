use crate::code_gen::RustCodeGen;
use crate::executor::RustExecutor;
use crate::ir::RustSyntaxCompiler;
use crate::lexer::FeLexer;
use crate::parser::FeSyntaxParser;
use crate::project_gen::RustProjectGen;
use crate::reader::SourceReader;
use crate::type_resolver::FeTypeResolver;

use crate::log;
use crate::result::Result;

use crate::code_gen::CodeGen;
use crate::project_gen::ProjectGen;
use crate::syntax::SyntaxCompiler;

use std::path::PathBuf;
use std::process;
use std::sync::{Arc, Mutex};

pub fn run_full(root_dir: PathBuf) -> Result<process::Output> {
    let src_root_dir = root_dir.join("src");
    let gen_output_dir = root_dir.join(".ferrum/compiled_rust");

    // Read source files
    let source = Arc::new(Mutex::new(SourceReader::read_src_files(src_root_dir)?));

    // Scan to tokens
    let tokens = Arc::new(Mutex::new(FeLexer::scan_package(source)?));

    log::debug!(&tokens);

    // Parse to AST
    let pkg = FeSyntaxParser::parse_package(tokens)?;

    // Resolve AST types
    let typed_pkg = Arc::new(Mutex::new(FeTypeResolver::resolve_package(pkg)?));

    log::debug!(&typed_pkg);

    // Compile to Rust IR
    let rust_ir = Arc::new(Mutex::new(RustSyntaxCompiler::compile_package(typed_pkg)?));

    // Generate Rust code
    let rust_code = Arc::new(Mutex::new(RustCodeGen::generate_code(rust_ir)?));

    log::debug!(&rust_code);

    // Write Rust output source files
    let generated = RustProjectGen::generate_project_files(rust_code, &gen_output_dir)?;

    log::debug!(&generated);

    // Run generated Rust project
    let out = RustExecutor::cargo_run(&gen_output_dir)?;

    return Ok(out);
}
