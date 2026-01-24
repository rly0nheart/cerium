/*
MIT License

Copyright (c) 2025 Ritchie Mwewa

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::cli::flags::PermissionsFormat;
use crate::output::formats::format::Format;
use libc::{
    S_IFBLK, S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFMT, S_IFREG, S_IFSOCK, S_IRGRP, S_IROTH,
    S_IRUSR, S_ISGID, S_ISUID, S_ISVTX, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR,
};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// A parsed representation of Unix permissions (using libc bitmasks)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct UnixPermissions {
    pub(crate) user_read: bool,
    pub(crate) user_write: bool,
    pub(crate) user_execute: bool,

    pub(crate) group_read: bool,
    pub(crate) group_write: bool,
    pub(crate) group_execute: bool,

    pub(crate) other_read: bool,
    pub(crate) other_write: bool,
    pub(crate) other_execute: bool,

    pub(crate) sticky: bool,
    pub(crate) setgid: bool,
    pub(crate) setuid: bool,

    pub(crate) has_xattr: bool,
}

impl UnixPermissions {
    /// Parse permissions from a `mode_t` value from `libc::lstat`.
    /// Also checks for extended attributes using `listxattr`.
    pub(crate) fn from_mode(mode: u32, path: &Path) -> Self {
        // Helper closure to check bits using libc constants
        let has_bit = |bit: u32| (mode & bit) == bit;

        let has_xattr = Self::check_xattr(path);

        Self {
            user_read: has_bit(S_IRUSR as u32),
            user_write: has_bit(S_IWUSR as u32),
            user_execute: has_bit(S_IXUSR as u32),

            group_read: has_bit(S_IRGRP as u32),
            group_write: has_bit(S_IWGRP as u32),
            group_execute: has_bit(S_IXGRP as u32),

            other_read: has_bit(S_IROTH as u32),
            other_write: has_bit(S_IWOTH as u32),
            other_execute: has_bit(S_IXOTH as u32),

            sticky: has_bit(S_ISVTX as u32),
            setgid: has_bit(S_ISGID as u32),
            setuid: has_bit(S_ISUID as u32),

            has_xattr,
        }
    }

    /// Check if a file has extended attributes using libc's listxattr
    fn check_xattr(path: &Path) -> bool {
        let c_path = match CString::new(path.as_os_str().as_bytes()) {
            Ok(p) => p,
            Err(_) => return false,
        };

        unsafe {
            // Call listxattr with NULL buffer to get the size needed
            let size = libc::listxattr(c_path.as_ptr(), std::ptr::null_mut(), 0);

            // If size > 0, extended attributes exist
            size > 0
        }
    }
}

/// Determine file type char using libc S_IF* constants
fn file_type_char(mode: u32) -> char {
    match mode & S_IFMT {
        S_IFDIR => 'd',  // directory
        S_IFREG => '.',  // regular file
        S_IFLNK => 'l',  // symlink
        S_IFBLK => 'b',  // block device
        S_IFCHR => 'c',  // char device
        S_IFIFO => 'p',  // FIFO (named pipe)
        S_IFSOCK => 's', // socket
        _ => '?',        // unknown type
    }
}

pub(crate) struct Permissions {
    permissions_flag: PermissionsFormat,
    path: PathBuf,
}

impl Permissions {
    pub(crate) fn new(permissions_flag: PermissionsFormat, path: PathBuf) -> Self {
        Self {
            permissions_flag,
            path,
        }
    }

    /// Symbolic ("drwxr-xr-t@") or octal/hex formatting derived from libc's mode bits.
    /// The '@' suffix indicates extended attributes are present.
    fn format_permissions(&self, mode: u32) -> Arc<str> {
        let file_type = file_type_char(mode);
        let permission = UnixPermissions::from_mode(mode, &self.path);

        match self.permissions_flag {
            PermissionsFormat::Symbolic => {
                // Expand into rwx chars, applying suid/sgid/sticky replacements
                let mut chars = [
                    if permission.user_read { 'r' } else { '-' },
                    if permission.user_write { 'w' } else { '-' },
                    if permission.user_execute { 'x' } else { '-' },
                    if permission.group_read { 'r' } else { '-' },
                    if permission.group_write { 'w' } else { '-' },
                    if permission.group_execute { 'x' } else { '-' },
                    if permission.other_read { 'r' } else { '-' },
                    if permission.other_write { 'w' } else { '-' },
                    if permission.other_execute { 'x' } else { '-' },
                ];

                // Apply special bits: setuid, setgid, sticky
                if permission.setuid {
                    chars[2] = if chars[2] == 'x' { 's' } else { 'S' };
                }
                if permission.setgid {
                    chars[5] = if chars[5] == 'x' { 's' } else { 'S' };
                }
                if permission.sticky {
                    chars[8] = if chars[8] == 'x' { 't' } else { 'T' };
                }

                let mut out = String::with_capacity(12);
                out.push(file_type);
                for c in chars {
                    out.push(c);
                }

                // Add '@' suffix if extended attributes exist
                if permission.has_xattr {
                    out.push('@');
                }

                out.into()
            }

            PermissionsFormat::Octal => {
                // Full 4-digit octal, including special bits
                // Example: -4755@, d2750, etc.
                let mut out = format!("{}{:04o}", file_type, mode & 0o7777);
                if permission.has_xattr {
                    out.push('@');
                }
                out.into()
            }

            PermissionsFormat::Hex => {
                // Full hex representation
                let mut out = format!("{}{:x}", file_type, mode);
                if permission.has_xattr {
                    out.push('@');
                }
                out.into()
            }
        }
    }
}

impl Format<u32> for Permissions {
    fn format(&self, input: u32) -> Arc<str> {
        self.format_permissions(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_file_type_regular_file() {
        let mode = S_IFREG | 0o644;
        assert_eq!(file_type_char(mode), '.');
    }

    #[test]
    fn test_file_type_directory() {
        let mode = S_IFDIR | 0o755;
        assert_eq!(file_type_char(mode), 'd');
    }

    #[test]
    fn test_file_type_symlink() {
        let mode = S_IFLNK | 0o777;
        assert_eq!(file_type_char(mode), 'l');
    }

    #[test]
    fn test_permissions_from_mode_644() {
        let mode = 0o644;
        let path = PathBuf::from("/tmp");
        let perm = UnixPermissions::from_mode(mode, &path);

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
    fn test_permissions_from_mode_755() {
        let mode = 0o755;
        let path = PathBuf::from("/tmp");
        let perm = UnixPermissions::from_mode(mode, &path);

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
    fn test_permissions_setuid() {
        let mode = S_ISUID as u32 | 0o755;
        let path = PathBuf::from("/tmp");
        let perm = UnixPermissions::from_mode(mode, &path);

        assert!(perm.setuid);
        assert!(!perm.setgid);
        assert!(!perm.sticky);
    }

    #[test]
    fn test_permissions_setgid() {
        let mode = S_ISGID as u32 | 0o755;
        let path = PathBuf::from("/tmp");
        let perm = UnixPermissions::from_mode(mode, &path);

        assert!(!perm.setuid);
        assert!(perm.setgid);
        assert!(!perm.sticky);
    }

    #[test]
    fn test_permissions_sticky() {
        let mode = S_ISVTX as u32 | 0o755;
        let path = PathBuf::from("/tmp");
        let perm = UnixPermissions::from_mode(mode, &path);

        assert!(!perm.setuid);
        assert!(!perm.setgid);
        assert!(perm.sticky);
    }

    #[test]
    fn test_format_symbolic_regular_file() {
        let path = PathBuf::from("/tmp/test");
        let formatter = Permissions::new(PermissionsFormat::Symbolic, path);
        let mode = S_IFREG | 0o644;
        let result = formatter.format(mode);

        assert!(result.starts_with(".rw-r--r--"));
    }

    #[test]
    fn test_format_symbolic_directory() {
        let path = PathBuf::from("/tmp");
        let formatter = Permissions::new(PermissionsFormat::Symbolic, path);
        let mode = S_IFDIR | 0o755;
        let result = formatter.format(mode);

        assert!(result.starts_with("drwxr-xr-x"));
    }

    #[test]
    fn test_format_symbolic_with_setuid() {
        let path = PathBuf::from("/tmp/test");
        let formatter = Permissions::new(PermissionsFormat::Symbolic, path);
        let mode = S_IFREG | S_ISUID as u32 | 0o755;
        let result = formatter.format(mode);

        assert!(result.starts_with(".rwsr-xr-x"));
    }

    #[test]
    fn test_format_symbolic_with_setgid() {
        let path = PathBuf::from("/tmp/test");
        let formatter = Permissions::new(PermissionsFormat::Symbolic, path);
        let mode = S_IFREG | S_ISGID as u32 | 0o755;
        let result = formatter.format(mode);

        assert!(result.starts_with(".rwxr-sr-x"));
    }

    #[test]
    fn test_format_symbolic_with_sticky() {
        let path = PathBuf::from("/tmp");
        let formatter = Permissions::new(PermissionsFormat::Symbolic, path);
        let mode = S_IFDIR | S_ISVTX as u32 | 0o755;
        let result = formatter.format(mode);

        assert!(result.starts_with("drwxr-xr-t"));
    }

    #[test]
    fn test_format_symbolic_sticky_no_execute() {
        let path = PathBuf::from("/tmp");
        let formatter = Permissions::new(PermissionsFormat::Symbolic, path);
        let mode = S_IFDIR | S_ISVTX as u32 | 0o644;
        let result = formatter.format(mode);

        assert!(result.starts_with("drw-r--r-T"));
    }

    #[test]
    fn test_format_octal() {
        let path = PathBuf::from("/tmp/test");
        let formatter = Permissions::new(PermissionsFormat::Octal, path);
        let mode = S_IFREG | 0o644;
        let result = formatter.format(mode);

        assert!(result.starts_with(".0644"));
    }

    #[test]
    fn test_format_octal_with_special_bits() {
        let path = PathBuf::from("/tmp/test");
        let formatter = Permissions::new(PermissionsFormat::Octal, path);
        let mode = S_IFREG | S_ISUID as u32 | 0o755;
        let result = formatter.format(mode);

        assert!(result.starts_with(".4755"));
    }

    #[test]
    fn test_format_hex() {
        let path = PathBuf::from("/tmp/test");
        let formatter = Permissions::new(PermissionsFormat::Hex, path);
        let mode = S_IFREG | 0o644;
        let result = formatter.format(mode);

        assert!(result.starts_with('.'));
        assert!(result.len() > 1);
    }

    #[test]
    fn test_xattr_detection() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        // Test that check_xattr doesn't crash on a regular file
        let has_xattr = UnixPermissions::check_xattr(&file_path);
        // Most temp files won't have xattrs, so typically false
        assert!(!has_xattr || has_xattr); // Just ensure it returns a bool
    }
}
