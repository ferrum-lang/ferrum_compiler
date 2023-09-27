#![allow(clippy::needless_return)]

use ferrum_compiler::utils;

use std::env;
use std::path;
use std::process;

fn main() -> ferrum_compiler::result::Result {
    let root_dir = get_root_dir();

    let out = utils::run_full(root_dir)?;

    let _ = process::Command::new("clear").status()?;

    // println!("{}", String::from_utf8(out.stderr)?);
    // println!("Output:\n------\n");

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
