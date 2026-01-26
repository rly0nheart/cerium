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

use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::sync::Arc;

pub struct Acl;

impl Acl {
    /// Checks if a file has ACLs beyond standard permissions.
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the file
    ///
    /// # Returns
    ///
    /// "+" if ACLs present, "-" if none/error
    pub fn check(path: &Path) -> Arc<str> {
        match Self::has_acl(path) {
            Ok(true) => "+".into(),
            _ => "-".into(),
        }
    }

    /// Internal ACL check using libc
    fn has_acl(path: &Path) -> Result<bool, ()> {
        let path_c = CString::new(path.as_os_str().as_bytes()).map_err(|_| ())?;

        // Use listxattr to check for system.posix_acl_access
        let size = unsafe { libc::listxattr(path_c.as_ptr(), std::ptr::null_mut(), 0) };

        if size < 0 {
            return Ok(false);
        }

        if size == 0 {
            return Ok(false);
        }

        let mut buffer = vec![0u8; size as usize];
        let result = unsafe {
            libc::listxattr(
                path_c.as_ptr(),
                buffer.as_mut_ptr() as *mut libc::c_char,
                size as usize,
            )
        };

        if result < 0 {
            return Ok(false);
        }

        // Check if system.posix_acl_access exists
        let attrs_str = String::from_utf8_lossy(&buffer);
        Ok(attrs_str.contains("system.posix_acl_access"))
    }
}
