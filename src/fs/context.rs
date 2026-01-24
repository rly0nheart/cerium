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

//! SELinux security context retrieval.

use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::sync::Arc;

const SELINUX_XATTR: &str = "security.selinux";

pub(crate) struct Context;

impl Context {
    /// Gets the SELinux security context for a file.
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the file
    ///
    /// # Returns
    ///
    /// The SELinux context string (e.g., "system_u:object_r:usr_t:s0"),
    /// or "?" if SELinux is not enabled or context cannot be retrieved.
    pub(crate) fn get(path: &Path) -> Arc<str> {
        match Self::get_context(path) {
            Ok(ctx) => ctx.into(),
            Err(_) => "?".into(),
        }
    }

    /// Internal context retrieval using lgetxattr.
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
}
