use cerium::cli::flags::PermissionFormat;
use cerium::display::output::formats::format::Format;
use cerium::display::output::formats::permission::Permission;
use libc::{S_IFDIR, S_IFREG, S_ISGID, S_ISUID, S_ISVTX};
use std::path::PathBuf;

#[test]
fn test_format_symbolic_regular_file() {
    let path = PathBuf::from("/tmp/test");
    let formatter = Permission::new(PermissionFormat::Symbolic, path);
    let mode = S_IFREG | 0o644;
    let result = formatter.format(mode);

    assert!(result.starts_with(".rw-r--r--"));
}

#[test]
fn test_format_symbolic_directory() {
    let path = PathBuf::from("/tmp");
    let formatter = Permission::new(PermissionFormat::Symbolic, path);
    let mode = S_IFDIR | 0o755;
    let result = formatter.format(mode);

    assert!(result.starts_with("drwxr-xr-x"));
}

#[test]
fn test_format_symbolic_with_setuid() {
    let path = PathBuf::from("/tmp/test");
    let formatter = Permission::new(PermissionFormat::Symbolic, path);
    let mode = S_IFREG | S_ISUID | 0o755;
    let result = formatter.format(mode);

    assert!(result.starts_with(".rwsr-xr-x"));
}

#[test]
fn test_format_symbolic_with_setgid() {
    let path = PathBuf::from("/tmp/test");
    let formatter = Permission::new(PermissionFormat::Symbolic, path);
    let mode = S_IFREG | S_ISGID | 0o755;
    let result = formatter.format(mode);

    assert!(result.starts_with(".rwxr-sr-x"));
}

#[test]
fn test_format_symbolic_with_sticky() {
    let path = PathBuf::from("/tmp");
    let formatter = Permission::new(PermissionFormat::Symbolic, path);
    let mode = S_IFDIR | S_ISVTX | 0o755;
    let result = formatter.format(mode);

    assert!(result.starts_with("drwxr-xr-t"));
}

#[test]
fn test_format_symbolic_sticky_no_execute() {
    let path = PathBuf::from("/tmp");
    let formatter = Permission::new(PermissionFormat::Symbolic, path);
    let mode = S_IFDIR | S_ISVTX | 0o644;
    let result = formatter.format(mode);

    assert!(result.starts_with("drw-r--r-T"));
}

#[test]
fn test_format_octal() {
    let path = PathBuf::from("/tmp/test");
    let formatter = Permission::new(PermissionFormat::Octal, path);
    let mode = S_IFREG | 0o644;
    let result = formatter.format(mode);

    assert!(result.starts_with(".0644"));
}

#[test]
fn test_format_octal_with_special_bits() {
    let path = PathBuf::from("/tmp/test");
    let formatter = Permission::new(PermissionFormat::Octal, path);
    let mode = S_IFREG | S_ISUID | 0o755;
    let result = formatter.format(mode);

    assert!(result.starts_with(".4755"));
}

#[test]
fn test_format_hex() {
    let path = PathBuf::from("/tmp/test");
    let formatter = Permission::new(PermissionFormat::Hex, path);
    let mode = S_IFREG | 0o644;
    let result = formatter.format(mode);

    assert!(result.starts_with('.'));
    assert!(result.len() > 1);
}
