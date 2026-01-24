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

//! File entry type for regular files.

use crate::fs::metadata::Metadata;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Represents a regular file in the filesystem.
#[derive(Clone, Debug)]
pub(crate) struct FileEntry {
    /// The display name of the file.
    pub(crate) name: Arc<str>,
    /// The full path to the file.
    pub(crate) path: PathBuf,
    /// The file extension in lowercase, or empty if none.
    pub(crate) extension: Arc<str>,
    /// Optional metadata (lazily loaded).
    pub(crate) metadata: Option<Metadata>,
}

impl FileEntry {
    /// Creates a new FileEntry.
    ///
    /// # Parameters
    ///
    /// * `name` - The display name of the file
    /// * `path` - The full path to the file
    pub(crate) fn new(name: Arc<str>, path: PathBuf) -> Self {
        let extension = Self::get_extension(&path);
        Self {
            name,
            path,
            extension,
            metadata: None,
        }
    }

    /// Returns the file extension in lowercase.
    fn get_extension(path: &Path) -> Arc<str> {
        path.extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default()
            .into()
    }
}
