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
use std::path::Path;

/// Minimal POSIX-like metadata struct loaded via libc::lstat
#[derive(Clone, Debug)]
pub struct Metadata {
    pub mode: u32,
    pub size: u64,
    pub ino: u64,
    pub nlink: u64,
    pub uid: u32,
    pub gid: u32,
    pub blocks: u64,
    pub blksize: u64,
    pub atime: i64,
    pub mtime: i64,
    pub ctime: i64,
}

impl Metadata {
    /// Load metadata for a path.
    ///
    /// When `dereference` is `false` (the default), uses `libc::lstat` which
    /// returns the symlink's own metadata.  When `true`, uses `libc::stat`
    /// which follows the symlink and returns the target's metadata.
    pub fn load(path: &Path, dereference: bool) -> io::Result<Self> {
        let c_path = CString::new(path.as_os_str().as_bytes()).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "path contains interior nul")
        })?;

        unsafe {
            let mut st: libc::stat = std::mem::zeroed();

            let stat_fn = if dereference { libc::stat } else { libc::lstat };
            if stat_fn(c_path.as_ptr(), &mut st) != 0 {
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

    pub fn empty() -> Self {
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
