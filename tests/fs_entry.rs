mod common;

use cerium::fs::entry::Entry;
use cerium::fs::metadata::Metadata;
use common::default_args;
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs as unix_fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

#[test]
fn test_entry_new_regular_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    File::create(&file_path).unwrap();

    let entry = Entry::from_path(file_path.clone(), false);

    assert_eq!(entry.name().as_ref(), "test.txt");
    assert_eq!(entry.path(), &file_path);
    assert!(!entry.is_dir());
    assert!(!entry.is_symlink());
    assert!(entry.is_file());
    assert_eq!(entry.extension().as_ref(), "txt");
    assert!(entry.metadata().is_none());
}

#[test]
fn test_entry_new_directory() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().join("test_dir");
    fs::create_dir(&dir_path).unwrap();

    let entry = Entry::from_path(dir_path.clone(), false);

    assert_eq!(entry.name().as_ref(), "test_dir");
    assert_eq!(entry.path(), &dir_path);
    assert!(entry.is_dir());
    assert!(!entry.is_symlink());
    assert!(!entry.is_file());
    assert_eq!(entry.extension().as_ref(), "");
}

#[test]
fn test_entry_new_symlink_without_target() {
    let temp_dir = TempDir::new().unwrap();
    let target_path = temp_dir.path().join("target.txt");
    let link_path = temp_dir.path().join("link.txt");

    File::create(&target_path).unwrap();
    unix_fs::symlink(&target_path, &link_path).unwrap();

    let entry = Entry::from_path(link_path.clone(), false);

    assert_eq!(entry.name().as_ref(), "link.txt");
    assert!(entry.is_symlink());
    assert!(!entry.is_dir());
    assert!(!entry.is_file());
    // Name should not contain arrow when show_link_target is false
    assert!(!entry.name().contains("⇒"));
}

#[test]
fn test_entry_new_symlink_with_target() {
    let temp_dir = TempDir::new().unwrap();
    let target_path = temp_dir.path().join("target.txt");
    let link_path = temp_dir.path().join("link.txt");

    File::create(&target_path).unwrap();
    unix_fs::symlink(&target_path, &link_path).unwrap();

    let entry = Entry::from_path(link_path.clone(), true);

    assert!(entry.is_symlink());
    // Name should contain arrow and target when show_link_target is true
    assert!(entry.name().contains("⇒"));
    assert!(entry.name().contains("link.txt"));
    assert!(entry.name().contains("target.txt"));
}

#[test]
fn test_is_dir_like_for_symlink_to_directory() {
    let temp_dir = TempDir::new().unwrap();
    let target_dir = temp_dir.path().join("target_dir");
    let link_path = temp_dir.path().join("link_to_dir");

    fs::create_dir(&target_dir).unwrap();
    unix_fs::symlink(&target_dir, &link_path).unwrap();

    let entry = Entry::from_path(link_path.clone(), false);

    assert!(entry.is_symlink());
    assert!(!entry.is_dir()); // is_dir() returns false for symlinks
    assert!(entry.is_dir_like()); // is_dir_like() returns true for symlinks to directories
}

#[test]
fn test_broken_symlink() {
    let temp_dir = TempDir::new().unwrap();
    let link_path = temp_dir.path().join("broken_link");

    // Create symlink to non-existent target
    unix_fs::symlink("/nonexistent/target", &link_path).unwrap();

    let entry = Entry::from_path(link_path.clone(), false);

    assert!(entry.is_symlink());
    assert!(entry.is_broken_symlink());
    assert!(!entry.is_dir_like());
}

#[test]
fn test_get_extension_lowercase() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("Test.TXT");
    File::create(&file_path).unwrap();

    let entry = Entry::from_path(file_path, false);

    // Extension should be lowercase
    assert_eq!(entry.extension().as_ref(), "txt");
}

#[test]
fn test_get_extension_no_extension() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("README");
    File::create(&file_path).unwrap();

    let entry = Entry::from_path(file_path, false);

    assert_eq!(entry.extension().as_ref(), "");
}

#[test]
fn test_get_extension_multiple_dots() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("archive.tar.gz");
    File::create(&file_path).unwrap();

    let entry = Entry::from_path(file_path, false);

    // Should only get the last extension
    assert_eq!(entry.extension().as_ref(), "gz");
}

#[test]
fn test_get_extension_directory() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().join("folder.dir");
    fs::create_dir(&dir_path).unwrap();

    let entry = Entry::from_path(dir_path, false);

    // Directories should have empty extension
    assert_eq!(entry.extension().as_ref(), "");
}

#[test]
fn test_metadata_loading() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"Hello, World!").unwrap();
    drop(file);

    let mut entry = Entry::from_path(file_path, false);
    let mut args = default_args();
    args.long = true; // This should trigger metadata loading

    entry.conditional_metadata(&args);

    assert!(entry.metadata().is_some());
    let meta = entry.metadata().unwrap();
    assert_eq!(meta.size, 13); // "Hello, World!" is 13 bytes
    assert!(meta.mode > 0);
    assert!(meta.ino > 0);
}

#[test]
fn test_metadata_not_loaded_when_not_needed() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    File::create(&file_path).unwrap();

    let mut entry = Entry::from_path(file_path, false);
    let args = default_args(); // No flags that require metadata

    entry.conditional_metadata(&args);

    // Metadata should not be loaded if not needed
    assert!(entry.metadata().is_none());
}

#[test]
fn test_metadata_cached() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"Test").unwrap();
    drop(file);

    let mut entry = Entry::from_path(file_path, false);
    let mut args = default_args();
    args.long = true;

    // Load metadata first time
    entry.conditional_metadata(&args);
    let first_size = entry.metadata().map(|m| m.size);

    // Load metadata again
    entry.conditional_metadata(&args);
    let second_size = entry.metadata().map(|m| m.size);

    // Should be the same (cached)
    assert!(first_size.is_some());
    assert!(second_size.is_some());
    assert_eq!(first_size, second_size);
}

#[test]
fn test_metadata_load_success() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"Content").unwrap();
    drop(file);

    let result = Metadata::load(&file_path, false);

    assert!(result.is_ok());
    let meta = result.unwrap();
    assert_eq!(meta.size, 7);
    assert!(meta.mode > 0);
    assert!(meta.ino > 0);
    assert!(meta.nlink > 0);
    assert!(meta.blksize > 0);
}

#[test]
fn test_metadata_load_nonexistent_file() {
    let path = PathBuf::from("/nonexistent/file/path.txt");
    let result = Metadata::load(&path, false);

    assert!(result.is_err());
}

#[test]
fn test_metadata_load_directory() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();

    let result = Metadata::load(&dir_path, false);

    assert!(result.is_ok());
    let meta = result.unwrap();
    // Directories have S_IFDIR bit set in mode
    assert_ne!(meta.mode & libc::S_IFDIR, 0);
}

#[test]
fn test_metadata_load_symlink() {
    let temp_dir = TempDir::new().unwrap();
    let target_path = temp_dir.path().join("target.txt");
    let link_path = temp_dir.path().join("link.txt");

    File::create(&target_path).unwrap();
    unix_fs::symlink(&target_path, &link_path).unwrap();

    let result = Metadata::load(&link_path, false);

    assert!(result.is_ok());
    let meta = result.unwrap();
    // Symlinks have S_IFLNK bit set in mode (lstat doesn't follow symlinks)
    assert_ne!(meta.mode & libc::S_IFLNK, 0);
}

#[test]
fn test_metadata_timestamps() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    File::create(&file_path).unwrap();

    let result = Metadata::load(&file_path, false);

    assert!(result.is_ok());
    let meta = result.unwrap();
    // Timestamps should be reasonable (after year 2000)
    assert!(meta.atime > 946_684_800); // Jan 1, 2000
    assert!(meta.mtime > 946_684_800);
    assert!(meta.ctime > 946_684_800);
}

#[test]
fn test_metadata_empty() {
    let meta = Metadata::empty();

    assert_eq!(meta.mode, 0);
    assert_eq!(meta.size, 0);
    assert_eq!(meta.ino, 0);
    assert_eq!(meta.nlink, 0);
    assert_eq!(meta.uid, 0);
    assert_eq!(meta.gid, 0);
    assert_eq!(meta.blocks, 0);
    assert_eq!(meta.blksize, 0);
    assert_eq!(meta.atime, 0);
    assert_eq!(meta.mtime, 0);
    assert_eq!(meta.ctime, 0);
}

#[test]
fn test_metadata_permissions() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    File::create(&file_path).unwrap();

    // Set specific permissions
    let perms = fs::Permissions::from_mode(0o644);
    fs::set_permissions(&file_path, perms).unwrap();

    let result = Metadata::load(&file_path, false);
    assert!(result.is_ok());
    let meta = result.unwrap();

    // Check that the permission bits are preserved
    let perm_bits = meta.mode & 0o777;
    assert_eq!(perm_bits, 0o644);
}

#[test]
fn test_entry_clone() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    File::create(&file_path).unwrap();

    let entry1 = Entry::from_path(file_path.clone(), false);
    let entry2 = entry1.clone();

    assert_eq!(entry1.name(), entry2.name());
    assert_eq!(entry1.path(), entry2.path());
    assert_eq!(entry1.is_dir(), entry2.is_dir());
    assert_eq!(entry1.has_children(), entry2.has_children());
    assert_eq!(entry1.is_symlink(), entry2.is_symlink());
    assert_eq!(entry1.extension(), entry2.extension());
}

#[test]
fn test_metadata_clone() {
    let meta1 = Metadata {
        mode: 0o644,
        size: 1024,
        ino: 12345,
        nlink: 1,
        uid: 1000,
        gid: 1000,
        blocks: 8,
        blksize: 4096,
        atime: 1000000000,
        mtime: 1000000001,
        ctime: 1000000002,
    };

    let meta2 = meta1.clone();

    assert_eq!(meta1.mode, meta2.mode);
    assert_eq!(meta1.size, meta2.size);
    assert_eq!(meta1.ino, meta2.ino);
    assert_eq!(meta1.nlink, meta2.nlink);
    assert_eq!(meta1.uid, meta2.uid);
    assert_eq!(meta1.gid, meta2.gid);
    assert_eq!(meta1.blocks, meta2.blocks);
    assert_eq!(meta1.blksize, meta2.blksize);
    assert_eq!(meta1.atime, meta2.atime);
    assert_eq!(meta1.mtime, meta2.mtime);
    assert_eq!(meta1.ctime, meta2.ctime);
}

#[test]
fn test_entry_with_hidden_file() {
    let temp_dir = TempDir::new().unwrap();
    let hidden_path = temp_dir.path().join(".hidden");
    File::create(&hidden_path).unwrap();

    let entry = Entry::from_path(hidden_path, false);

    assert_eq!(entry.name().as_ref(), ".hidden");
    assert!(entry.name().starts_with('.'));
}

#[test]
fn test_entry_with_special_characters() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("file with spaces.txt");
    File::create(&file_path).unwrap();

    let entry = Entry::from_path(file_path, false);

    assert_eq!(entry.name().as_ref(), "file with spaces.txt");
    assert_eq!(entry.extension().as_ref(), "txt");
}

#[test]
fn test_metadata_uid_gid() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    File::create(&file_path).unwrap();

    let result = Metadata::load(&file_path, false);
    assert!(result.is_ok());

    let meta = result.unwrap();
    // UID and GID should be set (we can't predict exact values, but they exist)
    // Both are u32, so they're always >= 0 by definition
    assert!(meta.uid < u32::MAX);
    assert!(meta.gid < u32::MAX);
}

#[test]
fn test_metadata_blocks_and_blksize() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(&[0u8; 8192]).unwrap(); // Write 8KB
    drop(file);

    let result = Metadata::load(&file_path, false);
    assert!(result.is_ok());

    let meta = result.unwrap();
    assert!(meta.blocks > 0);
    assert!(meta.blksize > 0);
    // Block size is typically 512 or 4096
    assert!(meta.blksize == 512 || meta.blksize == 4096 || meta.blksize == 8192);
}

#[test]
fn test_set_name() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    File::create(&file_path).unwrap();

    let mut entry = Entry::from_path(file_path, false);
    assert_eq!(entry.name().as_ref(), "test.txt");

    entry.set_name(Arc::from("new_name.txt"));
    assert_eq!(entry.name().as_ref(), "new_name.txt");
}
