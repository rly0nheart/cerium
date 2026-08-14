// SPDX-License-Identifier: MIT

use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::sync::Arc;

/// Reads a file's extended attribute names via a two-pass `listxattr` call.
///
/// The first call sizes the buffer, the second fills it with the
/// null-terminated name list.
///
/// # Parameters
/// - `path`: Path to the file to query.
///
/// # Returns
/// The raw name list, empty if the file has no extended attributes or the
/// path/call is unusable.
pub(crate) fn list_names(path: &Path) -> Vec<u8> {
    let Ok(path_c) = CString::new(path.as_os_str().as_bytes()) else {
        return Vec::new();
    };

    // First call to get size needed
    let size = unsafe { libc::listxattr(path_c.as_ptr(), std::ptr::null_mut(), 0) };

    if size <= 0 {
        return Vec::new();
    }

    // Second call to get actual data
    let mut buffer = vec![0u8; size as usize];
    // c_char is i8 on most platforms but u8 on Android
    let read = unsafe {
        libc::listxattr(
            path_c.as_ptr(),
            buffer.as_mut_ptr() as *mut libc::c_char,
            size as usize,
        )
    };

    if read < 0 {
        return Vec::new();
    }

    buffer.truncate(read as usize);
    buffer
}

/// Lists extended attribute names for a file as a display string.
///
/// # Parameters
/// - `path`: Path to the file to query.
///
/// # Returns
/// A comma-separated list of xattr names (e.g. `"user.mime_type, security.selinux"`),
/// or `"-"` if the file has no extended attributes or an error occurs.
pub(crate) fn list(path: &Path) -> Arc<str> {
    let names = list_names(path);

    let attrs: Vec<&str> = names
        .split(|&byte| byte == 0)
        .filter(|name| !name.is_empty())
        .filter_map(|name| std::str::from_utf8(name).ok())
        .collect();

    if attrs.is_empty() {
        "-".into()
    } else {
        attrs.join(", ").into()
    }
}
