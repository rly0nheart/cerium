use cerium::cli::flags::PermissionFormat;
use cerium::display::output::formats::permission;
use libc::{S_IFDIR, S_IFREG, S_ISGID, S_ISUID, S_ISVTX};

#[test]
fn test_format_symbolic_regular_file() {
    let mode = S_IFREG | 0o644;
    let result = permission::format(mode, PermissionFormat::Symbolic, false);

    assert!(result.starts_with(".rw-r--r--"));
}

#[test]
fn test_format_symbolic_directory() {
    let mode = S_IFDIR | 0o755;
    let result = permission::format(mode, PermissionFormat::Symbolic, false);

    assert!(result.starts_with("drwxr-xr-x"));
}

#[test]
fn test_format_symbolic_with_setuid() {
    let mode = S_IFREG | S_ISUID | 0o755;
    let result = permission::format(mode, PermissionFormat::Symbolic, false);

    assert!(result.starts_with(".rwsr-xr-x"));
}

#[test]
fn test_format_symbolic_with_setgid() {
    let mode = S_IFREG | S_ISGID | 0o755;
    let result = permission::format(mode, PermissionFormat::Symbolic, false);

    assert!(result.starts_with(".rwxr-sr-x"));
}

#[test]
fn test_format_symbolic_with_sticky() {
    let mode = S_IFDIR | S_ISVTX | 0o755;
    let result = permission::format(mode, PermissionFormat::Symbolic, false);

    assert!(result.starts_with("drwxr-xr-t"));
}

#[test]
fn test_format_symbolic_sticky_no_execute() {
    let mode = S_IFDIR | S_ISVTX | 0o644;
    let result = permission::format(mode, PermissionFormat::Symbolic, false);

    assert!(result.starts_with("drw-r--r-T"));
}

#[test]
fn test_format_octal() {
    let mode = S_IFREG | 0o644;
    let result = permission::format(mode, PermissionFormat::Octal, false);

    assert!(result.starts_with(".0644"));
}

#[test]
fn test_format_octal_with_special_bits() {
    let mode = S_IFREG | S_ISUID | 0o755;
    let result = permission::format(mode, PermissionFormat::Octal, false);

    assert!(result.starts_with(".4755"));
}

#[test]
fn test_format_hex() {
    let mode = S_IFREG | 0o644;
    let result = permission::format(mode, PermissionFormat::Hex, false);

    assert!(result.starts_with('.'));
    assert!(result.len() > 1);
}
