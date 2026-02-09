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

use libc::{
    S_IFBLK, S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFMT, S_IFREG, S_IFSOCK, S_IRGRP, S_IROTH,
    S_IRUSR, S_ISGID, S_ISUID, S_ISVTX, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR,
};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

/// A parsed representation of Unix permissions derived from libc `mode_t` bitmasks,
/// including special bits (setuid, setgid, sticky) and extended attribute presence.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Permissions {
    pub user_read: bool,
    pub user_write: bool,
    pub user_execute: bool,

    pub group_read: bool,
    pub group_write: bool,
    pub group_execute: bool,

    pub other_read: bool,
    pub other_write: bool,
    pub other_execute: bool,

    pub sticky: bool,
    pub setgid: bool,
    pub setuid: bool,

    pub has_xattr: bool,
}

impl Permissions {
    /// Parses permissions from a raw `mode_t` value and checks for extended attributes.
    ///
    /// # Parameters
    /// - `mode`: The `st_mode` value from a stat call.
    /// - `path`: The file path, used to query extended attributes via `listxattr`.
    ///
    /// # Returns
    /// A fully populated [`Permissions`] struct.
    pub fn from_mode(mode: u32, path: &Path) -> Self {
        // Helper closure to check bits using libc constants
        let has_bit = |bit: u32| (mode & bit) == bit;

        let has_xattr = Self::check_xattr(path);

        Self {
            user_read: has_bit(S_IRUSR),
            user_write: has_bit(S_IWUSR),
            user_execute: has_bit(S_IXUSR),

            group_read: has_bit(S_IRGRP),
            group_write: has_bit(S_IWGRP),
            group_execute: has_bit(S_IXGRP),

            other_read: has_bit(S_IROTH),
            other_write: has_bit(S_IWOTH),
            other_execute: has_bit(S_IXOTH),

            sticky: has_bit(S_ISVTX),
            setgid: has_bit(S_ISGID),
            setuid: has_bit(S_ISUID),

            has_xattr,
        }
    }

    /// Checks if a file has any extended attributes using `listxattr`.
    ///
    /// # Parameters
    /// - `path`: The file path to query.
    ///
    /// # Returns
    /// `true` if the file has at least one extended attribute, `false` otherwise
    /// or if the path contains a null byte.
    pub fn check_xattr(path: &Path) -> bool {
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

    /// Determines the file type indicator character from a raw `mode_t` value.
    ///
    /// # Parameters
    /// - `mode`: The `st_mode` value from a stat call.
    ///
    /// # Returns
    /// A single character: `'d'` (directory), `'.'` (regular file), `'l'` (symlink),
    /// `'b'` (block device), `'c'` (char device), `'p'` (FIFO), `'s'` (socket),
    /// or `'?'` (unknown).
    pub fn file_type_char(mode: u32) -> char {
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
}
