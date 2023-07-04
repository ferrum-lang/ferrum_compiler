use ferrum_compiler::syntax::*;

fn main() {
    let pkg = FePackage::File(FeFile {
        name: PackageName("_main".to_string()),
        path: "src/_main.fe".into(),
        syntax: SyntaxTree {
            uses: vec![],
            decls: vec![],
        },
    });
    dbg!(&pkg);

    let res = RustSyntaxCompiler::compile_package(pkg);
    dbg!(&res);
}
