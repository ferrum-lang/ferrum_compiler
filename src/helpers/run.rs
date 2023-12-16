use crate::borrow_checker::FeBorrowChecker;
use crate::code_gen::RustCodeGen;
use crate::config::Config;
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

use std::process;
use std::sync::{Arc, Mutex};

pub fn run_full(cfg: Config) -> Result<process::Output> {
    let cfg = Arc::new(cfg);

    // let dev_build_dir = builds_dir.join("dev");
    // let test_build_dir = builds_dir.join("test");
    // let release_build_dir = builds_dir.join("release");

    // Read source files
    let source = Arc::new(Mutex::new(SourceReader::read_src_files(cfg.clone())?));

    // Scan to tokens
    let tokens = Arc::new(Mutex::new(FeLexer::scan_package(cfg.clone(), source)?));

    log::debug!(&tokens);

    // Parse to AST
    let pkg = FeSyntaxParser::parse_package(cfg.clone(), tokens)?;

    // Resolve AST types
    let typed_pkg = Arc::new(Mutex::new(FeTypeResolver::resolve_package(
        cfg.clone(),
        pkg,
    )?));

    // Check ownership / borrowing
    // FeBorrowChecker::check_package(cfg.clone(), typed_pkg.clone())?;

    log::debug!(&typed_pkg);

    // Compile to Rust IR
    let rust_ir = Arc::new(Mutex::new(RustSyntaxCompiler::compile_package(
        cfg.clone(),
        typed_pkg,
    )?));

    // Generate Rust code
    let rust_code = Arc::new(Mutex::new(RustCodeGen::generate_code(
        cfg.clone(),
        rust_ir,
    )?));

    log::debug!(&rust_code);

    // Write Rust output source files
    let generated = RustProjectGen::generate_project_files(cfg.clone(), rust_code)?;

    log::debug!(&generated);

    // Run generated Rust project
    let out = RustExecutor::cargo_run(cfg.clone())?;

    return Ok(out);
}
