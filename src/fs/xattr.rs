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

pub struct Xattr;

impl Xattr {
    /// Lists extended attributes for a file.
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the file
    ///
    /// # Returns
    ///
    /// Comma-separated list of xattr names, or "-" if none/error
    pub fn list(path: &Path) -> Arc<str> {
        match Self::list_xattrs(path) {
            Ok(attrs) if !attrs.is_empty() => attrs.join(", ").into(),
            _ => "-".into(),
        }
    }

    /// Internal xattr listing using libc
    fn list_xattrs(path: &Path) -> Result<Vec<String>, ()> {
        let path_c = CString::new(path.as_os_str().as_bytes()).map_err(|_| ())?;

        // First call to get size needed
        let size = unsafe { libc::listxattr(path_c.as_ptr(), std::ptr::null_mut(), 0) };

        if size < 0 {
            return Err(());
        }

        if size == 0 {
            return Ok(Vec::new());
        }

        // Second call to get actual data
        let mut buffer = vec![0u8; size as usize];
        let result = unsafe {
            libc::listxattr(
                path_c.as_ptr(),

                #[cfg(target_os = "android")]
                buffer.as_mut_ptr() as *mut u8,

                #[cfg(not(target_os = "android"))]
                buffer.as_mut_ptr() as *mut i8,
                
                size as usize,
            )
        };

        if result < 0 {
            return Err(());
        }

        // Parse null-terminated attribute names
        let mut attrs = Vec::new();
        let mut start = 0;
        for (i, &byte) in buffer.iter().enumerate() {
            if byte == 0 && start < i {
                if let Ok(name) = std::str::from_utf8(&buffer[start..i]) {
                    attrs.push(name.to_string());
                }
                start = i + 1;
            }
        }

        Ok(attrs)
    }
}
