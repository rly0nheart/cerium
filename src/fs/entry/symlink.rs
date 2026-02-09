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

//! Symlink entry type for symbolic links.

use crate::fs::metadata::Metadata;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Represents a symbolic link in the filesystem.
#[derive(Clone, Debug)]
pub struct SymlinkEntry {
    /// The display name of the symlink (may include target if show_link_target is true).
    pub name: Arc<str>,
    /// The full path to the symlink.
    pub path: PathBuf,
    /// The file extension in lowercase (from the symlink name), or empty if none.
    pub extension: Arc<str>,
    /// Optional metadata (lazily loaded).
    pub metadata: Option<Metadata>,
    /// Whether the symlink target is a directory.
    pub target_is_dir: bool,
    #[allow(dead_code)]
    /// Whether the symlink target exists (false for broken symlinks).
    pub target_exists: bool,
}

impl SymlinkEntry {
    /// Creates a new SymlinkEntry.
    ///
    /// # Parameters
    /// - `name`: The display name of the symlink.
    /// - `path`: The full path to the symlink.
    /// - `target_is_dir`: Whether the symlink points to a directory.
    /// - `target_exists`: Whether the symlink target exists.
    pub fn new(name: Arc<str>, path: PathBuf, target_is_dir: bool, target_exists: bool) -> Self {
        let extension = Self::get_extension(&path);
        Self {
            name,
            path,
            extension,
            metadata: None,
            target_is_dir,
            target_exists,
        }
    }

    /// Returns the file extension in lowercase from the symlink name.
    ///
    /// # Parameters
    /// - `path`: The symlink path to extract the extension from.
    fn get_extension(path: &Path) -> Arc<str> {
        path.extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default()
            .into()
    }

    #[allow(dead_code)]
    /// Returns whether this symlink is broken (target doesn't exist).
    pub fn is_broken(&self) -> bool {
        !self.target_exists
    }
}
