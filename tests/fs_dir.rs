mod common;

use cerium::cli::flags::SortBy;
use cerium::fs::dir::DirReader;
use common::{default_args, setup_test_dir};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_new_directory() {
    let path = PathBuf::from("/tmp");
    let dir_reader = DirReader::from(path.clone());
    assert_eq!(dir_reader.path(), &path);
}

#[test]
fn test_list_basic() {
    let temp_dir = setup_test_dir();
    let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
    let args = default_args();

    let entries = dir_reader.list(&args);

    // Should not include hidden files by default
    assert_eq!(entries.len(), 4); // file1.txt, file2.rs, subdir, empty_dir
    assert!(entries.iter().any(|e| e.name().as_ref() == "file1.txt"));
    assert!(entries.iter().any(|e| e.name().as_ref() == "file2.rs"));
    assert!(entries.iter().any(|e| e.name().as_ref() == "subdir"));
}

#[test]
fn test_list_with_all_flag() {
    let temp_dir = setup_test_dir();
    let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
    let mut args = default_args();
    args.all = true;

    let entries = dir_reader.list(&args);

    // Should include hidden files
    assert!(entries.iter().any(|e| e.name().as_ref() == ".hidden"));
    assert!(entries.len() >= 4);
}

#[test]
fn test_list_dirs_only() {
    let temp_dir = setup_test_dir();
    let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
    let mut args = default_args();
    args.dirs = true;

    let entries = dir_reader.list(&args);

    // Should only return directories
    assert!(entries.iter().all(|e| e.is_dir_like()));
    assert!(entries.iter().any(|e| e.name().as_ref() == "subdir"));
    assert!(entries.iter().any(|e| e.name().as_ref() == "empty_dir"));
}

#[test]
fn test_list_files_only() {
    let temp_dir = setup_test_dir();
    let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
    let mut args = default_args();
    args.files = true;

    let entries = dir_reader.list(&args);

    // Should only return files
    assert!(entries.iter().all(|e| !e.is_dir_like()));
    assert!(entries.iter().any(|e| e.name().as_ref() == "file1.txt"));
    assert!(entries.iter().any(|e| e.name().as_ref() == "file2.rs"));
}

#[test]
fn test_list_single_file() {
    let temp_dir = setup_test_dir();
    let file_path = temp_dir.path().join("file1.txt");
    let dir_reader = DirReader::from(file_path.clone());
    let args = default_args();

    let entries = dir_reader.list(&args);

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].name().as_ref(), "file1.txt");
}

#[test]
fn test_hide_entries() {
    let temp_dir = setup_test_dir();
    let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
    let mut args = default_args();
    args.hide = vec!["file1.txt".to_string(), "subdir".to_string()];

    let entries = dir_reader.list(&args);

    // Should not include hidden entries
    assert!(!entries.iter().any(|e| e.name().as_ref() == "file1.txt"));
    assert!(!entries.iter().any(|e| e.name().as_ref() == "subdir"));
    assert!(entries.iter().any(|e| e.name().as_ref() == "file2.rs"));
}

#[test]
fn test_sort_by_name() {
    let temp_dir = setup_test_dir();
    let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
    let mut args = default_args();
    args.sort = SortBy::Name;

    let entries = dir_reader.list(&args);

    // Check if sorted alphabetically
    for i in 0..entries.len().saturating_sub(1) {
        assert!(entries[i].name().to_lowercase() <= entries[i + 1].name().to_lowercase());
    }
}

#[test]
fn test_sort_by_extension() {
    let temp_dir = setup_test_dir();
    let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
    let mut args = default_args();
    args.sort = SortBy::Extension;

    let entries = dir_reader.list(&args);

    // Check if sorted by extension
    for i in 0..entries.len().saturating_sub(1) {
        assert!(entries[i].extension().to_lowercase() <= entries[i + 1].extension().to_lowercase());
    }
}

#[test]
fn test_reverse_sort() {
    let temp_dir = setup_test_dir();
    let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
    let mut args = default_args();
    args.sort = SortBy::Name;
    args.reverse = true;

    let entries = dir_reader.list(&args);

    // Check if sorted in reverse alphabetical order
    for i in 0..entries.len().saturating_sub(1) {
        assert!(entries[i].name().to_lowercase() >= entries[i + 1].name().to_lowercase());
    }
}

#[test]
fn test_true_size_with_hidden() {
    let temp_dir = setup_test_dir();
    let base = temp_dir.path();

    // Write some data to files
    let mut file1 = File::create(base.join("file1.txt")).unwrap();
    file1.write_all(b"Hello").unwrap();

    let mut hidden = File::create(base.join(".hidden")).unwrap();
    hidden.write_all(b"Secret").unwrap();

    let dir_reader = DirReader::from(base.to_path_buf());

    let size_with_hidden = dir_reader.true_size(true);
    let size_without_hidden = dir_reader.true_size(false);

    assert!(size_with_hidden > size_without_hidden);
    assert!(size_with_hidden >= 11); // At least "Hello" + "Secret"
}

#[test]
fn test_true_size_recursive() {
    let temp_dir = setup_test_dir();
    let base = temp_dir.path();

    // Write data to nested file
    let mut nested = File::create(base.join("subdir/nested.txt")).unwrap();
    nested.write_all(b"Nested content").unwrap();

    let dir_reader = DirReader::from(base.to_path_buf());
    let size = dir_reader.true_size(true);

    // Should include nested files
    assert!(size >= 14); // At least "Nested content"
}

#[test]
fn test_true_size_non_directory() {
    let temp_dir = setup_test_dir();
    let file_path = temp_dir.path().join("file1.txt");

    let dir_reader = DirReader::from(file_path);
    let size = dir_reader.true_size(true);

    assert_eq!(size, 0); // Non-directories return 0
}

#[cfg(unix)]
#[test]
fn test_list_special_file_types() {
    use std::os::unix::net::UnixListener;

    let temp_dir = TempDir::new().unwrap();

    // Create a Unix socket (special file type, not regular file or directory)
    let socket_path = temp_dir.path().join("test.sock");
    let _listener = UnixListener::bind(&socket_path).unwrap();

    // Test that DirReader::list returns the socket when passed directly
    let dir_reader = DirReader::from(socket_path.clone());
    let args = default_args();
    let entries = dir_reader.list(&args);

    // Should return exactly 1 entry (the socket itself)
    assert_eq!(entries.len(), 1, "Special file types should be listed");
    assert_eq!(entries[0].name().as_ref(), "test.sock");
}
