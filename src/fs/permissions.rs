use libc::{
    S_IFBLK, S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFMT, S_IFREG, S_IFSOCK, S_IRGRP, S_IROTH,
    S_IRUSR, S_ISGID, S_ISUID, S_ISVTX, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR,
};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

/// A parsed representation of Unix permissions (using libc bitmasks)
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
    /// Parse permissions from a `mode_t` value from `libc::lstat`.
    /// Also checks for extended attributes using `listxattr`.
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

    /// Check if a file has extended attributes using libc's listxattr
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

    /// Determine file type char using libc S_IF* constants
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
