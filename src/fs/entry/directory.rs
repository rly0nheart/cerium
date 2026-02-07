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

//! Directory entry type for directories.

use crate::fs::metadata::Metadata;
use std::cell::Cell;
use std::path::PathBuf;
use std::sync::Arc;

/// Represents a directory in the filesystem.
#[derive(Clone, Debug)]
pub struct DirectoryEntry {
    /// The display name of the directory.
    pub name: Arc<str>,
    /// The full path to the directory.
    pub path: PathBuf,
    /// Optional metadata (lazily loaded).
    pub metadata: Option<Metadata>,
    /// Lazily computed - None means not yet checked.
    has_children: Cell<Option<bool>>,
}

impl DirectoryEntry {
    /// Creates a new DirectoryEntry.
    ///
    /// # Parameters
    ///
    /// * `name` - The display name of the directory
    /// * `path` - The full path to the directory
    pub fn new(name: Arc<str>, path: PathBuf) -> Self {
        Self {
            name,
            path,
            metadata: None,
            has_children: Cell::new(None),
        }
    }

    /// Returns whether this directory has children.
    /// Computes and caches the result on first call.
    pub fn has_children(&self) -> bool {
        if let Some(has) = self.has_children.get() {
            return has;
        }

        let has = if let Ok(mut entries) = std::fs::read_dir(&self.path) {
            entries.next().is_some()
        } else {
            false
        };

        self.has_children.set(Some(has));
        has
    }
}
