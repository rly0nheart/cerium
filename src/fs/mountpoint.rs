// SPDX-License-Identifier: MIT

use std::cmp::Reverse;
use std::fs;
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
    let mounts = MOUNT_POINTS.get_or_init(|| parse_mounts().unwrap_or_default());

    match find_mountpoint(path, mounts) {
        Some(mount) => mount.into(),
        None => "-".into(),
    }
}

/// Parses `/proc/mounts` to extract all mount points.
///
/// # Returns
/// The mount paths sorted by length (longest first), or `Err(())` if
/// `/proc/mounts` cannot be read.
fn parse_mounts() -> Result<Vec<PathBuf>, ()> {
    let content = fs::read_to_string("/proc/mounts").map_err(|_| ())?;

    let mut mounts: Vec<PathBuf> = content
        .lines()
        // Field 1 is the mount point; the rest (device, fs type, options) is unused.
        .filter_map(|line| line.split_whitespace().nth(1))
        .map(|mount| PathBuf::from(unescape_mount_path(mount)))
        .collect();

    // Sort by path length (longest first) to ensure we match the most specific mount
    mounts.sort_by_key(|mount| Reverse(mount.as_os_str().len()));

    Ok(mounts)
}

/// Unescapes octal sequences in mount point paths from `/proc/mounts`.
///
/// # Parameters
/// - `path`: The escaped path string from `/proc/mounts`.
///
/// # Returns
/// The unescaped path string.
fn unescape_mount_path(path: &str) -> String {
    // Fast path: mount points rarely contain escapes.
    if !path.contains('\\') {
        return path.to_string();
    }

    let mut result = String::new();
    let mut chars = path.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            // Try to read next 3 characters as octal
            let octal: String = chars.by_ref().take(3).collect();
            if octal.len() == 3
                && let Ok(code) = u8::from_str_radix(&octal, 8)
            {
                result.push(code as char);
                continue;
            }
            // If parsing failed, just add the backslash and what we read
            result.push('\\');
            result.push_str(&octal);
        } else {
            result.push(ch);
        }
    }

    result
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
