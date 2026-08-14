// SPDX-License-Identifier: MIT

//! SELinux security context retrieval.

use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::sync::Arc;

const SELINUX_XATTR: &str = "security.selinux";

/// Gets the SELinux security context for a file.
///
/// # Parameters
/// - `path`: Path to the file to query.
///
/// # Returns
/// The SELinux context string (e.g. `"system_u:object_r:usr_t:s0"`),
/// or `"?"` if SELinux is not enabled or the context cannot be retrieved.
pub(crate) fn get(path: &Path) -> Arc<str> {
    match get_context(path) {
        Ok(ctx) => ctx.into(),
        Err(_) => "?".into(),
    }
}

/// Reads the `security.selinux` extended attribute via `lgetxattr`.
///
/// Uses a two-pass approach: first call to determine buffer size,
/// second call to read the data.
///
/// # Parameters
/// - `path`: Path to the file to query.
///
/// # Returns
/// `Ok(String)` containing the context, or `Err(())` if the attribute
/// is missing, empty, the path contains a null byte, or the value is not valid UTF-8.
fn get_context(path: &Path) -> Result<String, ()> {
    let path_c = CString::new(path.as_os_str().as_bytes()).map_err(|_| ())?;
    let name_c = CString::new(SELINUX_XATTR).map_err(|_| ())?;

    // First call to get size needed
    let size =
        unsafe { libc::lgetxattr(path_c.as_ptr(), name_c.as_ptr(), std::ptr::null_mut(), 0) };

    if size < 0 {
        return Err(());
    }

    if size == 0 {
        return Err(());
    }

    // Second call to get actual data
    let mut buffer = vec![0u8; size as usize];
    let result = unsafe {
        libc::lgetxattr(
            path_c.as_ptr(),
            name_c.as_ptr(),
            buffer.as_mut_ptr() as *mut libc::c_void,
            size as usize,
        )
    };

    if result < 0 {
        return Err(());
    }

    // Remove trailing null byte if present
    if buffer.last() == Some(&0) {
        buffer.pop();
    }

    String::from_utf8(buffer).map_err(|_| ())
}
