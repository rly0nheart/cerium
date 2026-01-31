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

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

/// Global cache of mount points parsed from /proc/mounts
static MOUNT_POINTS: OnceLock<Vec<(PathBuf, String)>> = OnceLock::new();

pub struct Mountpoint;

impl Mountpoint {
    /// Determines the mount point for a given path.
    ///
    /// Returns the mount point path as a string. If the mount point cannot be
    /// determined, returns "-".
    ///
    /// # Implementation
    ///
    /// This function reads `/proc/mounts` (only once, cached globally) to get all
    /// active mount points, then finds the longest matching prefix for the given path.
    ///
    /// # Parameters
    ///
    /// * `path` - The file or directory path to check
    ///
    /// # Returns
    ///
    /// The mount point path as an Arc<str>, or "-" if unavailable
    pub fn get(path: &Path) -> Arc<str> {
        let mounts = MOUNT_POINTS.get_or_init(|| Self::parse_mounts().unwrap_or_default());

        match Self::find_mountpoint(path, mounts) {
            Some(mount) => mount.into(),
            None => "-".into(),
        }
    }

    /// Parses /proc/mounts to extract all mount points.
    ///
    /// # Format
    ///
    /// Each line in /proc/mounts has the format:
    /// ```text
    /// device mountpoint filesystem options dump pass
    /// ```
    ///
    /// Example:
    /// ```text
    /// /dev/sda1 / ext4 rw,relatime 0 0
    /// tmpfs /tmp tmpfs rw,nosuid,nodev 0 0
    /// ```
    ///
    /// # Returns
    ///
    /// A vector of (mount_path, filesystem_type) tuples, sorted by path length
    /// (longest first) to ensure correct prefix matching.
    fn parse_mounts() -> Result<Vec<(PathBuf, String)>, ()> {
        let content = fs::read_to_string("/proc/mounts").map_err(|_| ())?;

        let mut mounts: Vec<(PathBuf, String)> = content
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    // parts[1] is mount point, parts[2] is filesystem type
                    let mount_path = Self::unescape_mount_path(parts[1]);
                    let fs_type = parts[2].to_string();
                    Some((PathBuf::from(mount_path), fs_type))
                } else {
                    None
                }
            })
            .collect();

        // Sort by path length (longest first) to ensure we match the most specific mount
        mounts.sort_by(|a, b| b.0.as_os_str().len().cmp(&a.0.as_os_str().len()));

        Ok(mounts)
    }

    /// Unescapes octal sequences in mount point paths from /proc/mounts.
    ///
    /// /proc/mounts escapes special characters (like spaces) as octal sequences.
    /// For example, "/mnt/my\040folder" represents "/mnt/my folder".
    ///
    /// # Parameters
    ///
    /// * `path` - The escaped path string from /proc/mounts
    ///
    /// # Returns
    ///
    /// The unescaped path string
    fn unescape_mount_path(path: &str) -> String {
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

    /// Finds the mount point for a given path by matching against known mounts.
    ///
    /// # Algorithm
    ///
    /// Since mounts are sorted by length (longest first), we iterate through them
    /// and return the first one where the path starts with the mount point.
    /// This ensures we match the most specific mount.
    ///
    /// # Parameters
    ///
    /// * `path` - The path to find the mount point for
    /// * `mounts` - List of (mount_path, fs_type) tuples, sorted longest first
    ///
    /// # Returns
    ///
    /// The mount point path as a String, or None if no match found
    fn find_mountpoint(path: &Path, mounts: &[(PathBuf, String)]) -> Option<String> {
        // Canonicalise the path to resolve symlinks and get absolute path
        let canonical_path = path.canonicalize().ok()?;

        for (mount_path, _fs_type) in mounts {
            if canonical_path.starts_with(mount_path) {
                return Some(mount_path.display().to_string());
            }
        }

        None
    }
}
