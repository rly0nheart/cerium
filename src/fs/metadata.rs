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
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

/// Minimal POSIX-like metadata struct loaded via libc::lstat
#[derive(Clone, Debug)]
pub(crate) struct Metadata {
    pub(crate) mode: u32,
    pub(crate) size: u64,
    pub(crate) ino: u64,
    pub(crate) nlink: u64,
    pub(crate) uid: u32,
    pub(crate) gid: u32,
    pub(crate) blocks: u64,
    pub(crate) blksize: u64,
    pub(crate) atime: i64,
    pub(crate) mtime: i64,
    pub(crate) ctime: i64,
}

impl Metadata {
    /// Load metadata using libc::lstat (does not follow symlinks).
    /// Returns io::Error from errno on failure, or Ok(LibcMetadata).
    pub(crate) fn load(path: &PathBuf) -> io::Result<Self> {
        let c_path = CString::new(path.as_os_str().as_bytes()).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "path contains interior nul")
        })?;

        unsafe {
            let mut st: libc::stat = std::mem::zeroed();

            if libc::lstat(c_path.as_ptr(), &mut st) != 0 {
                return Err(io::Error::last_os_error());
            }

            Ok(Self {
                mode: st.st_mode as u32,
                size: st.st_size as u64,
                ino: st.st_ino as u64,
                nlink: st.st_nlink as u64,
                uid: st.st_uid,
                gid: st.st_gid,
                blocks: st.st_blocks as u64,
                blksize: st.st_blksize as u64,
                atime: st.st_atime,
                mtime: st.st_mtime,
                ctime: st.st_ctime,
            })
        }
    }

    pub(crate) fn empty() -> Self {
        Self {
            mode: 0,
            size: 0,
            ino: 0,
            nlink: 0,
            uid: 0,
            gid: 0,
            blocks: 0,
            blksize: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
        }
    }
}
