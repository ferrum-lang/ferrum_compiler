use anyhow::Context;
use ferrum_compiler::helpers::run_full;
use ferrum_compiler::result::Result;

use std::path::PathBuf;
use std::{env, fs};

const CARGO_MANIFEST_DIR: &'static str = "CARGO_MANIFEST_DIR";

#[test]
fn test_examples() -> Result {
    let root_dir = PathBuf::from(env::var(CARGO_MANIFEST_DIR)?);
    let projects_dir = root_dir.join("tests/integration/projects");

    for project_dir in projects_dir.read_dir()? {
        let project_dir = project_dir?;

        if project_dir.file_type()?.is_dir() {
            // Setup
            let project_dir = project_dir.path();

            // Run
            let out = run_full((&project_dir).into())
                .with_context(|| format!("Error running {:#?}", project_dir))?;

            let actual_stdout = String::from_utf8(out.stdout)?;
            let actual_stderr = String::from_utf8(out.stderr)?;

            // Build expectation
            let expected_stdout_path = project_dir.join("stdout.txt");
            let expected_stderr_path = project_dir.join("stderr.txt");

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
            assert_eq!(actual_stdout, expected_stdout, "test: {:?}", project_dir);
            assert_eq!(actual_stderr, expected_stderr, "test: {:?}", project_dir);
        }
    }

    return Ok(());
}
