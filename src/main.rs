use ferrum_compiler::code_gen::*;
use ferrum_compiler::executor::*;
use ferrum_compiler::ir::*;
use ferrum_compiler::lexer::*;
use ferrum_compiler::parser::*;
use ferrum_compiler::project_gen::*;
use ferrum_compiler::source::*;
use ferrum_compiler::syntax::*;
use ferrum_compiler::type_resolver::*;

use std::collections::HashMap;
use std::env;
use std::path;
use std::path::PathBuf;
use std::process;
use std::sync::{Arc, Mutex};

fn main() -> ferrum_compiler::result::Result {
    // TODO: Make code available in compiler lib to do this
    let source = read_source()?;

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

    // println!("{}", String::from_utf8(out.stderr)?);
    println!("Output:\n------\n");

    println!("{}", String::from_utf8(out.stdout)?);

    return Ok(());
}

fn read_source() -> ferrum_compiler::result::Result<Arc<Mutex<FeSourcePackage>>> {
    fn build_dir_pkg(
        dir: PathBuf,
        name: SourcePackageName,
    ) -> ferrum_compiler::result::Result<FeSourcePackage> {
        let src_dir_entries = dir.read_dir()?;
        let mut local_packages = HashMap::new();

        for pkg in src_dir_entries {
            let pkg = pkg?;

            let name = SourcePackageName(
                pkg.path()
                    .file_stem()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap()
                    .into(),
            );

            if name.0.as_ref() == "_pkg" {
                continue;
            }

            if pkg.file_type()?.is_dir() {
                let pkg = build_dir_pkg(pkg.path(), name.clone())?;

                local_packages.insert(name, Arc::new(Mutex::new(pkg)));
            } else {
                let path = pkg.path().into_os_string().into_string().unwrap().into();
                let content = std::fs::read_to_string(&path)?.into();

                let pkg = FeSourcePackage::File(FeSourceFile {
                    name: name.clone(),
                    path,
                    content,
                });

                local_packages.insert(name, Arc::new(Mutex::new(pkg)));
            }
        }

        let pkg = dir.join("_pkg.fe");
        if !pkg.is_file() {
            panic!("Expected package {:?} to contain a '_pkg.fe' file", dir);
        }
        let pkg_content = std::fs::read_to_string(&pkg)?.into();

        let entry_file = FeSourceFile {
            name: SourcePackageName("_pkg".into()),
            path: pkg,
            content: pkg_content,
        };

        return Ok(FeSourcePackage::Dir(FeSourceDir {
            name,
            path: dir,
            entry_file,
            local_packages,
        }));
    }

    if env::args().count() != 2 {
        panic!("Expected 1 arg: path to the root of the project directory");
    }

    let root_dir = env::args().last().unwrap();
    let root_dir = PathBuf::from(root_dir);
    if !root_dir.is_dir() {
        panic!(
            "Expected arg 1 to be a directory to the project root, found {:?}",
            root_dir
        );
    }

    let src_dir = root_dir.join("src");
    if !src_dir.is_dir() {
        panic!("Expected the project root to contain a 'src' directory");
    }

    let src_dir_entries = src_dir.read_dir()?;
    let mut local_packages = HashMap::new();

    for pkg in src_dir_entries {
        let pkg = pkg?;

        let name = SourcePackageName(
            pkg.path()
                .file_stem()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap()
                .into(),
        );

        if name.0.as_ref() == "_main" {
            continue;
        }

        if pkg.file_type()?.is_dir() {
            let pkg = build_dir_pkg(pkg.path(), name.clone())?;

            local_packages.insert(name, Arc::new(Mutex::new(pkg)));
        } else {
            let path = pkg.path().into_os_string().into_string().unwrap().into();
            let content = std::fs::read_to_string(&path)?.into();

            let pkg = FeSourcePackage::File(FeSourceFile {
                name: name.clone(),
                path,
                content,
            });

            local_packages.insert(name, Arc::new(Mutex::new(pkg)));
        }
    }

    let main = src_dir.join("_main.fe");
    if !main.is_file() {
        panic!("Expected project root to contain a '_main.fe' file");
    }
    let main_content = std::fs::read_to_string(main)?;

    let source = FeSourcePackage::Dir(FeSourceDir {
        name: SourcePackageName("src".into()),
        path: "src/".into(),
        entry_file: FeSourceFile {
            name: SourcePackageName("_main".into()),
            path: "src/_main.fe".into(),
            content: main_content.into(),
        },
        local_packages,
    });

    return Ok(Arc::new(Mutex::new(source)));
}
