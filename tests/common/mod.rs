#[allow(unused)]
use cerium::cli::args::Args;
#[allow(unused)]
use clap::Parser;
#[allow(unused)]
use std::fs::{self, File};
#[allow(unused)]
use tempfile::TempDir;

#[allow(dead_code)]
pub fn default_args() -> Args {
    Args::parse_from(["ce", "."])
}

#[allow(dead_code)]
pub fn setup_test_dir() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    // Create files
    File::create(base.join("file1.txt")).unwrap();
    File::create(base.join("file2.rs")).unwrap();
    File::create(base.join(".hidden")).unwrap();

    // Create subdirectory with files
    fs::create_dir(base.join("subdir")).unwrap();
    File::create(base.join("subdir/nested.txt")).unwrap();
    File::create(base.join("subdir/.hidden_nested")).unwrap();

    // Create empty directory
    fs::create_dir(base.join("empty_dir")).unwrap();

    temp_dir
}
