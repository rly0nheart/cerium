// SPDX-License-Identifier: MIT

use std::cmp::Reverse;
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

/// Global cache of mount points parsed from /proc/mounts
static MOUNT_POINTS: OnceLock<Vec<PathBuf>> = OnceLock::new();

/// Determines the mount point for a given path.
///
/// # Parameters
/// - `path`: The file or directory path to check.
///
/// # Returns
/// The mount point path as an `Arc<str>`, or `"-"` if unavailable.
pub(crate) fn of(path: &Path) -> Arc<str> {
    let mounts = MOUNT_POINTS.get_or_init(read_mounts);

    match find_mountpoint(path, mounts) {
        Some(mount) => mount.into(),
        None => "-".into(),
    }
}

/// Reads the mount table, letting libc unescape the octal sequences in each path.
///
/// # Returns
/// The mount paths sorted by length (longest first), or empty if the table
/// cannot be opened.
fn read_mounts() -> Vec<PathBuf> {
    let mut mounts = Vec::new();

    // `getmntent` returns a pointer into a static buffer, so it is only safe to
    // call from one thread at a time. `OnceLock` guarantees a single caller here,
    // and each path is copied out before the next iteration overwrites it.
    unsafe {
        let table = libc::setmntent(c"/proc/mounts".as_ptr(), c"r".as_ptr());
        if table.is_null() {
            return mounts;
        }

        loop {
            let entry = libc::getmntent(table);
            if entry.is_null() {
                break;
            }

            let dir = (*entry).mnt_dir;
            if !dir.is_null() {
                let bytes = CStr::from_ptr(dir).to_bytes();
                mounts.push(PathBuf::from(OsStr::from_bytes(bytes)));
            }
        }

        libc::endmntent(table);
    }

    // Sort by path length (longest first) to ensure we match the most specific mount
    mounts.sort_by_key(|mount| Reverse(mount.as_os_str().len()));

    mounts
}

/// Finds the most specific mount point for a given path.
///
/// # Parameters
/// - `path`: The path to find the mount point for.
/// - `mounts`: List of `(mount_path, fs_type)` tuples, sorted longest first.
///
/// # Returns
/// The mount point path as a `String`, or `None` if no match is found.
fn find_mountpoint(path: &Path, mounts: &[PathBuf]) -> Option<String> {
    // Canonicalise the path to resolve symlinks and get absolute path
    let canonical_path = path.canonicalize().ok()?;

    mounts
        .iter()
        .find(|mount| canonical_path.starts_with(mount))
        .map(|mount| mount.display().to_string())
}
