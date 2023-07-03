use ferrum_compiler::*;

fn main() {
    let syn = SyntaxTree {
        uses: vec![],
        declarations: vec![],
    };
    dbg!(&syn);

    let res = compile_syntax_tree(syn);
    dbg!(&res);
}
