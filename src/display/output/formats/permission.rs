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

use crate::cli::flags::PermissionFormat;
use crate::display::output::formats::format::Format;
use crate::fs::permissions::Permissions;

use std::path::PathBuf;
use std::sync::Arc;

impl Format<u32> for Permission {
    fn format(&self, input: u32) -> Arc<str> {
        self.format_permission(input)
    }
}

pub(crate) struct Permission {
    permission_flag: PermissionFormat,
    path: PathBuf,
}

impl Permission {
    pub(crate) fn new(permission_flag: PermissionFormat, path: PathBuf) -> Self {
        Self {
            permission_flag,
            path,
        }
    }

    /// Symbolic ("drwxr-xr-t@") or octal/hex formatting derived from libc's mode bits.
    /// The '@' suffix indicates extended attributes are present.
    fn format_permission(&self, mode: u32) -> Arc<str> {
        let file_type = Permissions::file_type_char(mode);
        let permission = Permissions::from_mode(mode, &self.path);

        match self.permission_flag {
            PermissionFormat::Symbolic => {
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

            PermissionFormat::Octal => {
                // Full 4-digit octal, including special bits
                // Example: -4755@, d2750, etc.
                let mut out = format!("{}{:04o}", file_type, mode & 0o7777);
                if permission.has_xattr {
                    out.push('@');
                }
                out.into()
            }

            PermissionFormat::Hex => {
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

#[cfg(test)]
mod tests {
    use super::*;
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
}
