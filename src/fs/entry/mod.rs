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

pub mod directory;
pub mod file;
pub mod symlink;

pub use directory::DirectoryEntry;
pub use file::FileEntry;
pub use symlink::SymlinkEntry;

use crate::cli::args;
use crate::cli::args::Args;
use crate::fs::cache::Cache;
use crate::fs::metadata::Metadata;
use crate::fs::symlink as symlink_utils;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// A filesystem entry that can be a file, directory, or symlink.
#[derive(Clone, Debug)]
pub enum Entry {
    File(FileEntry),
    Directory(DirectoryEntry),
    Symlink(SymlinkEntry),
}

impl Entry {
    /// Creates a new entry from a DirEntry, using d_type to avoid stat calls.
    ///
    /// # Parameters
    ///
    /// * `dir_entry` - The directory entry from readdir
    /// * `show_link_target` - If `true`, include symlink target in display name
    ///
    /// # Performance
    ///
    /// This avoids separate is_dir() and is_symlink() calls by using DirEntry::file_type()
    /// which reads from the cached d_type field on Linux.
    pub fn from_dir_entry(dir_entry: &DirEntry, show_link_target: bool) -> Self {
        let path = dir_entry.path();

        let (is_dir, is_symlink) = match dir_entry.file_type() {
            Ok(filetype) => (filetype.is_dir(), filetype.is_symlink()),
            // Fallback to stat only if d_type is unavailable
            Err(_) => (path.is_dir(), path.is_symlink()),
        };

        Self::create(path, is_dir, is_symlink, show_link_target)
    }

    /// Creates a new entry for a known path (e.g., root path for tree traversal).
    ///
    /// This constructor is used when we don't have a DirEntry (e.g., for the root
    /// path passed on command line). It does require stat calls.
    ///
    /// Prefer `from_dir_entry()` when iterating directory contents as it avoids
    /// stat syscalls by using the cached d_type from readdir.
    ///
    /// # Parameters
    ///
    /// * `path` - The path to the file, directory, or symlink.
    /// * `show_link_target` - If `true`, the entry will include the symlink target
    ///   in its display name (e.g., `"name ⇒ target"`).
    pub fn from_path(path: PathBuf, show_link_target: bool) -> Self {
        // For root paths we need to stat - but this is only called once per listing
        let is_symlink = path.is_symlink();
        let is_dir = path.is_dir();

        Self::create(path, is_dir, is_symlink, show_link_target)
    }

    /// Internal helper to create the appropriate Entry variant.
    fn create(path: PathBuf, is_dir: bool, is_symlink: bool, show_link_target: bool) -> Self {
        let name = Self::get_name(&path, is_symlink, show_link_target);

        if is_symlink {
            // For symlinks, we need to check if the target is a directory and if it exists
            let target_exists = path.exists(); // follows symlink
            let target_is_dir = target_exists && path.is_dir();

            Entry::Symlink(SymlinkEntry::new(name, path, target_is_dir, target_exists))
        } else if is_dir {
            Entry::Directory(DirectoryEntry::new(name, path))
        } else {
            Entry::File(FileEntry::new(name, path))
        }
    }

    /// Returns true only for actual directories (not symlinks to directories).
    pub fn is_dir(&self) -> bool {
        matches!(self, Entry::Directory(_))
    }

    /// Returns true only for symlinks.
    pub fn is_symlink(&self) -> bool {
        matches!(self, Entry::Symlink(_))
    }

    #[allow(dead_code)]
    /// Returns true only for regular files.
    pub fn is_file(&self) -> bool {
        matches!(self, Entry::File(_))
    }

    /// Returns true if this entry is empty.
    /// For directories, checks whether the directory has no children.
    /// For files, checks whether the file is 0 bytes via a lightweight stat call.
    pub fn is_empty(&self) -> bool {
        match self {
            Entry::Directory(directory) => !directory.has_children(),
            Entry::File(file) => std::fs::symlink_metadata(&file.path)
                .map(|metadata| metadata.len() == 0)
                .unwrap_or(false),
            _ => false,
        }
    }

    /// Returns true for directories AND symlinks pointing to directories.
    /// Used for filtering with --dirs/--files flags.
    pub fn is_dir_like(&self) -> bool {
        match self {
            Entry::Directory(_) => true,
            Entry::Symlink(s) => s.target_is_dir,
            Entry::File(_) => false,
        }
    }

    /// Returns a reference to the entry's display name.
    pub fn name(&self) -> &Arc<str> {
        match self {
            Entry::File(file) => &file.name,
            Entry::Directory(directory) => &directory.name,
            Entry::Symlink(symlink) => &symlink.name,
        }
    }

    /// Returns a reference to the entry's path.
    pub fn path(&self) -> &PathBuf {
        match self {
            Entry::File(file) => &file.path,
            Entry::Directory(directory) => &directory.path,
            Entry::Symlink(symlink) => &symlink.path,
        }
    }

    /// Returns a reference to the entry's extension.
    /// For directories, returns an empty string.
    pub fn extension(&self) -> &Arc<str> {
        static EMPTY: std::sync::OnceLock<Arc<str>> = std::sync::OnceLock::new();
        let empty = EMPTY.get_or_init(|| Arc::from(""));

        match self {
            Entry::File(f) => &f.extension,
            Entry::Directory(_) => empty,
            Entry::Symlink(symlink) => &symlink.extension,
        }
    }

    /// Returns a reference to the entry's metadata if loaded.
    pub fn metadata(&self) -> Option<&Metadata> {
        match self {
            Entry::File(file) => file.metadata.as_ref(),
            Entry::Directory(directory) => directory.metadata.as_ref(),
            Entry::Symlink(symlink) => symlink.metadata.as_ref(),
        }
    }

    /// Sets the entry's display name.
    /// Used by search.rs for highlighting matches.
    pub fn set_name(&mut self, name: Arc<str>) {
        match self {
            Entry::File(file) => file.name = name,
            Entry::Directory(directory) => directory.name = name,
            Entry::Symlink(symlink) => symlink.name = name,
        }
    }

    /// Returns whether this directory has children (for icon display).
    /// Returns true by default if not yet computed or if not a directory.
    pub fn has_children(&self) -> bool {
        match self {
            Entry::Directory(directory) => directory.has_children(),
            Entry::Symlink(symlink) if symlink.target_is_dir => true, // Assume symlinks to dirs have children
            _ => false,
        }
    }

    #[allow(dead_code)]
    /// Returns whether this entry is a broken symlink.
    pub fn is_broken_symlink(&self) -> bool {
        match self {
            Entry::Symlink(symlink) => symlink.is_broken(),
            _ => false,
        }
    }

    /// Conditionally loads metadata for this entry if requested by any args option.
    ///
    /// # Parameters
    /// - `args`: Reference to args options that determine which metadata fields to populate.
    ///
    /// # Description
    /// Populates fields like size, timestamps, permissions, owner/group, inode, links, and block info
    /// only if the corresponding display args are enabled. Skips loading if metadata is already populated.
    pub fn conditional_metadata(&mut self, args: &Args) {
        if !args::is_args_requesting_metadata(args) {
            return;
        }

        self.unconditional_metadata(args.dereference);
    }

    /// Unconditionally loads metadata for sorting purposes.
    /// This bypasses the args_request_metadata check since sorting
    /// needs metadata even when display flags don't request it.
    ///
    /// When `dereference` is true, symlinks are followed (stat) so
    /// metadata reflects the link target rather than the link itself.
    pub fn unconditional_metadata(&mut self, dereference: bool) {
        // Skip if already loaded (check both size and ino for robustness)
        if let Some(meta) = self.metadata()
            && (meta.size != 0 || meta.ino != 0)
        {
            return;
        }

        let path = self.path().clone();
        let metadata = match Cache::metadata(&path, dereference) {
            Ok(raw) => Some(raw.clone()),
            Err(_) => Some(Metadata::empty()),
        };

        match self {
            Entry::File(file) => file.metadata = metadata,
            Entry::Directory(directory) => directory.metadata = metadata,
            Entry::Symlink(symlink) => symlink.metadata = metadata,
        }
    }

    /// Returns a display name for a file, directory, or symlink.
    ///
    /// # Parameters
    ///
    /// * `path` - The path to the file, directory, or symlink.
    /// * `is_symlink` - Whether the path is a symbolic link.
    /// * `show_link_target` - If `true` and `path` is a symlink, the returned name
    ///   will include the symlink target in the format `"name ⇒ target"`.
    fn get_name(path: &Path, is_symlink: bool, show_link_target: bool) -> Arc<str> {
        if !is_symlink {
            path.file_name()
                .map(|s| s.to_string_lossy().into())
                .unwrap_or_else(|| {
                    std::env::current_dir()
                        .ok()
                        .and_then(|p| p.file_name().map(|s| s.to_string_lossy().into()))
                        .unwrap_or_else(|| ".".into())
                })
        } else {
            let name = path.file_name().unwrap_or_default().to_string_lossy();

            if show_link_target {
                // Include arrow and target only if long format
                let target = symlink_utils::read_symlink_target(path);
                symlink_utils::format_symlink(&name, &target).into()
            } else {
                // Just the symlink name
                name.into()
            }
        }
    }
}
