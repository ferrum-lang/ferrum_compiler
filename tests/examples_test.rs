use ferrum_compiler::result::Result;
use ferrum_compiler::utils;

use std::path::PathBuf;
use std::{env, fs};

const CARGO_MANIFEST_DIR: &'static str = "CARGO_MANIFEST_DIR";

#[test]
fn test_that_examples_work() -> Result {
    let root_dir = PathBuf::from(env::var(CARGO_MANIFEST_DIR)?);
    let examples_dir = root_dir.join("examples");

    for example_dir in examples_dir.read_dir()? {
        let example_dir = example_dir?;

        if example_dir.file_type()?.is_dir() {
            // Setup
            let example_dir = example_dir.path();

            // Run
            let out = utils::run_full(example_dir.clone())?;

            // Build expectation
            let actual_stdout = String::from_utf8(out.stdout)?;
            let actual_stderr = String::from_utf8(out.stderr)?;

            let expected_stdout_path = example_dir.join("stdout.txt");
            let expected_stderr_path = example_dir.join("stderr.txt");

            let expected_stdout = if expected_stdout_path.is_file() {
                fs::read_to_string(expected_stdout_path)?
            } else {
                String::new()
            };

            let expected_stderr = if expected_stderr_path.is_file() {
                fs::read_to_string(expected_stderr_path)?
            } else {
                String::new()
            };

            // Assertions
            assert_eq!(actual_stdout, expected_stdout, "example: {:?}", example_dir);
            assert_eq!(actual_stderr, expected_stderr, "example: {:?}", example_dir);
        }
    }

    return Ok(());
}
