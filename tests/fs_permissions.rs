use cerium::fs::permissions::Permissions;
use libc::{S_IFDIR, S_IFLNK, S_IFREG, S_ISGID, S_ISUID, S_ISVTX};
use std::fs::File;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_file_type_regular_file() {
    let mode = S_IFREG | 0o644;
    assert_eq!(Permissions::file_type_char(mode), '.');
}

#[test]
fn test_file_type_directory() {
    let mode = S_IFDIR | 0o755;
    assert_eq!(Permissions::file_type_char(mode), 'd');
}

#[test]
fn test_file_type_symlink() {
    let mode = S_IFLNK | 0o777;
    assert_eq!(Permissions::file_type_char(mode), 'l');
}

#[test]
fn test_permission_from_mode_644() {
    let mode = 0o644;
    let path = PathBuf::from("/tmp");
    let perm = Permissions::from_mode(mode, &path);

    assert!(perm.user_read);
    assert!(perm.user_write);
    assert!(!perm.user_execute);

    assert!(perm.group_read);
    assert!(!perm.group_write);
    assert!(!perm.group_execute);

    assert!(perm.other_read);
    assert!(!perm.other_write);
    assert!(!perm.other_execute);
}

#[test]
fn test_permission_from_mode_755() {
    let mode = 0o755;
    let path = PathBuf::from("/tmp");
    let perm = Permissions::from_mode(mode, &path);

    assert!(perm.user_read);
    assert!(perm.user_write);
    assert!(perm.user_execute);

    assert!(perm.group_read);
    assert!(!perm.group_write);
    assert!(perm.group_execute);

    assert!(perm.other_read);
    assert!(!perm.other_write);
    assert!(perm.other_execute);
}

#[test]
fn test_permission_setuid() {
    let mode = S_ISUID | 0o755;
    let path = PathBuf::from("/tmp");
    let perm = Permissions::from_mode(mode, &path);

    assert!(perm.setuid);
    assert!(!perm.setgid);
    assert!(!perm.sticky);
}

#[test]
fn test_permission_setgid() {
    let mode = S_ISGID | 0o755;
    let path = PathBuf::from("/tmp");
    let perm = Permissions::from_mode(mode, &path);

    assert!(!perm.setuid);
    assert!(perm.setgid);
    assert!(!perm.sticky);
}

#[test]
fn test_permission_sticky() {
    let mode = S_ISVTX | 0o755;
    let path = PathBuf::from("/tmp");
    let perm = Permissions::from_mode(mode, &path);

    assert!(!perm.setuid);
    assert!(!perm.setgid);
    assert!(perm.sticky);
}

#[test]
fn test_xattr_detection() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    File::create(&file_path).unwrap();

    // Test that check_xattr doesn't crash on a regular file
    let has_xattr = Permissions::check_xattr(&file_path);
    // Most temp files won't have xattrs, so typically false
    let _: bool = has_xattr; // Just ensure it returns a bool
}
