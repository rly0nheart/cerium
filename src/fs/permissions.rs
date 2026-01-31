use libc::{
    S_IFBLK, S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFMT, S_IFREG, S_IFSOCK, S_IRGRP, S_IROTH,
    S_IRUSR, S_ISGID, S_ISUID, S_ISVTX, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR,
};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

/// A parsed representation of Unix permissions (using libc bitmasks)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Permissions {
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

impl Permissions {
    /// Parse permissions from a `mode_t` value from `libc::lstat`.
    /// Also checks for extended attributes using `listxattr`.
    pub(crate) fn from_mode(mode: u32, path: &Path) -> Self {
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
    pub(crate) fn check_xattr(path: &Path) -> bool {
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
    pub(crate) fn file_type_char(mode: u32) -> char {
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

#[cfg(test)]
mod tests {
    use super::*;
    use libc::{S_IFDIR, S_IFLNK, S_IFREG};
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
}
