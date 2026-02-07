mod common;

use cerium::fs::search::Search;
use common::default_args;
use std::fs::{self, File};
use tempfile::TempDir;

fn setup_test_dir() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    File::create(base.join("file1.txt")).unwrap();
    File::create(base.join("file2.rs")).unwrap();
    File::create(base.join("other.txt")).unwrap();

    fs::create_dir(base.join("subdir")).unwrap();
    File::create(base.join("subdir/nested.txt")).unwrap();
    File::create(base.join("subdir/nested.rs")).unwrap();

    temp_dir
}

#[test]
fn test_search_glob() {
    let temp_dir = setup_test_dir();
    let search = Search::new("*.txt", temp_dir.path().to_path_buf()).unwrap();
    let args = default_args();

    let matches = search.find(&args);

    assert_eq!(matches.len(), 2);
    assert!(matches.iter().any(|e| e.name().contains("file1.txt")));
    assert!(matches.iter().any(|e| e.name().contains("other.txt")));
}

#[test]
fn test_search_recursive() {
    let temp_dir = setup_test_dir();
    let search = Search::new("*.txt", temp_dir.path().to_path_buf()).unwrap();
    let mut args = default_args();
    args.recursive = true;

    let matches = search.find(&args);

    assert_eq!(matches.len(), 3);
    assert!(matches.iter().any(|e| e.name().contains("nested.txt")));
}

#[test]
fn test_search_case_insensitive() {
    let temp_dir = setup_test_dir();
    let search = Search::new("FILE*", temp_dir.path().to_path_buf()).unwrap();
    let args = default_args();

    let matches = search.find(&args);

    assert_eq!(matches.len(), 2);
}
