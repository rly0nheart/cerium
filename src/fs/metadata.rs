// SPDX-License-Identifier: MIT

use std::ffi::CString;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

/// Minimal POSIX-like metadata struct loaded via `libc::lstat` or `libc::stat`.
///
/// [`Default`] gives the all-zero placeholder used when a stat call fails.
#[derive(Clone, Debug, Default)]
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
    /// Loads metadata for a path using a raw libc stat call.
    ///
    /// # Parameters
    /// - `path`: The filesystem path to query.
    /// - `dereference`: If `true`, uses `libc::stat` (follows symlinks);
    ///   if `false`, uses `libc::lstat` (returns the symlink's own metadata).
    ///
    /// # Returns
    /// The populated [`Metadata`], or an I/O error if the stat call fails
    /// or the path contains an interior null byte.
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
}
