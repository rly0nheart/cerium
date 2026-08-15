// SPDX-License-Identifier: MIT

use crate::cli::flags::PermissionFormat;
use crate::fs::permissions::Permissions;

use std::sync::Arc;

/// Formats a mode bitmask as symbolic, octal, or hex.
///
/// # Parameters
/// - `mode`: The raw permission mode bits from stat.
/// - `format_mode`: The display format (symbolic, octal, or hex).
/// - `has_xattr`: Whether the entry carries extended attributes (appends `@`).
pub fn format(mode: u32, format_mode: PermissionFormat, has_xattr: bool) -> Arc<str> {
    let file_type = Permissions::file_type_char(mode);
    let permission = Permissions::from_mode(mode);

    match format_mode {
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
            if has_xattr {
                out.push('@');
            }

            out.into()
        }

        PermissionFormat::Octal => {
            // Full 4-digit octal, including special bits
            // Example: -4755@, d2750, etc.
            let mut out = format!("{}{:04o}", file_type, mode & 0o7777);
            if has_xattr {
                out.push('@');
            }
            out.into()
        }

        PermissionFormat::Hex => {
            // Full hex representation
            let mut out = format!("{}{:x}", file_type, mode);
            if has_xattr {
                out.push('@');
            }
            out.into()
        }
    }
}
