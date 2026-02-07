mod common;

use cerium::fs::entry::Entry;
use cerium::fs::tree::{TreeBuilder, TreeNode};
use common::default_args;
use std::fs::{self, File};
use tempfile::TempDir;

fn setup_test_dir() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    File::create(base.join("file1.txt")).unwrap();
    File::create(base.join("file2.rs")).unwrap();

    fs::create_dir(base.join("subdir")).unwrap();
    File::create(base.join("subdir/nested.txt")).unwrap();

    temp_dir
}

#[test]
fn test_tree_build() {
    let temp_dir = setup_test_dir();
    let builder = TreeBuilder::new(temp_dir.path().to_path_buf());
    let args = default_args();

    let tree = builder.build(&args);

    assert!(tree.entry.is_dir());
    assert!(!tree.children.is_empty());
}

#[test]
fn test_tree_nested() {
    let temp_dir = setup_test_dir();
    let builder = TreeBuilder::new(temp_dir.path().to_path_buf());
    let args = default_args();

    let tree = builder.build(&args);

    let subdir = tree
        .children
        .iter()
        .find(|n| n.entry.name().as_ref() == "subdir");

    assert!(subdir.is_some());
    assert!(!subdir.unwrap().children.is_empty());
}

#[test]
fn test_tree_node_structure() {
    let temp_dir = setup_test_dir();
    let entry = Entry::from_path(temp_dir.path().to_path_buf(), false);

    let node = TreeNode {
        entry: entry.clone(),
        children: vec![],
    };

    assert_eq!(node.entry.path(), entry.path());
    assert!(node.children.is_empty());
}
