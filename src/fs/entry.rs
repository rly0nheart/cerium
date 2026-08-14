// SPDX-License-Identifier: MIT

use crate::cli::args::Args;
use crate::fs::metadata::Metadata;
use crate::fs::symlink as symlink_utils;
use std::cell::Cell;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// What a filesystem entry is. Everything else about an entry is shared.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    File,
    Directory,
    /// A symlink, carrying whether its target resolves to a directory.
    Symlink {
        target_is_dir: bool,
    },
}

/// A filesystem entry: a file, directory, or symlink.
#[derive(Clone, Debug)]
pub struct Entry {
    /// The display name (a symlink may render as `name -> target`).
    pub name: Arc<str>,
    /// The full path.
    pub path: PathBuf,
    /// The lowercase extension, empty for directories and extensionless names.
    pub extension: Arc<str>,
    /// Lazily loaded metadata.
    pub metadata: Option<Metadata>,
    /// Which of the three kinds this is.
    pub kind: Kind,
    /// Lazily computed; `None` means not yet checked.
    has_children: Cell<Option<bool>>,
}

impl Entry {
    /// Creates an entry from a [`DirEntry`], using `d_type` to avoid stat calls.
    ///
    /// # Parameters
    /// - `dir_entry`: The directory entry from readdir.
    /// - `show_link_target`: If `true`, includes the symlink target in the display name.
    pub fn from_dir_entry(dir_entry: &DirEntry, show_link_target: bool) -> Self {
        let path = dir_entry.path();

        let (is_dir, is_symlink) = match dir_entry.file_type() {
            Ok(filetype) => (filetype.is_dir(), filetype.is_symlink()),
            // Fallback to stat only if d_type is unavailable
            Err(_) => (path.is_dir(), path.is_symlink()),
        };

        Self::create(path, is_dir, is_symlink, show_link_target)
    }

    /// Creates an entry for a known path (e.g., root path for tree traversal).
    ///
    /// # Parameters
    /// - `path`: The path to the file, directory, or symlink.
    /// - `show_link_target`: If `true`, includes the symlink target in the display name.
    pub fn from_path(path: PathBuf, show_link_target: bool) -> Self {
        // For root paths we need to stat - but this is only called once per listing
        let is_symlink = path.is_symlink();
        let is_dir = path.is_dir();

        Self::create(path, is_dir, is_symlink, show_link_target)
    }

    /// Builds an entry from pre-computed type flags.
    ///
    /// # Parameters
    /// - `path`: The filesystem path.
    /// - `is_dir`: Whether the path is a directory.
    /// - `is_symlink`: Whether the path is a symbolic link.
    /// - `show_link_target`: If `true`, includes the symlink target in the display name.
    fn create(path: PathBuf, is_dir: bool, is_symlink: bool, show_link_target: bool) -> Self {
        let kind = if is_symlink {
            Kind::Symlink {
                // One stat through the link, so a broken link reads as a non-directory.
                target_is_dir: std::fs::metadata(&path).is_ok_and(|meta| meta.is_dir()),
            }
        } else if is_dir {
            Kind::Directory
        } else {
            Kind::File
        };

        Self {
            name: Self::get_name(&path, is_symlink, show_link_target),
            extension: Self::get_extension(&path, kind),
            path,
            metadata: None,
            kind,
            has_children: Cell::new(None),
        }
    }

    /// Returns true only for actual directories (not symlinks to directories).
    pub fn is_dir(&self) -> bool {
        self.kind == Kind::Directory
    }

    /// Returns true only for symlinks.
    pub fn is_symlink(&self) -> bool {
        matches!(self.kind, Kind::Symlink { .. })
    }

    /// Returns true for directories AND symlinks pointing to directories.
    /// Used for filtering with --dirs/--files flags.
    pub fn is_dir_like(&self) -> bool {
        match self.kind {
            Kind::Directory => true,
            Kind::Symlink { target_is_dir } => target_is_dir,
            Kind::File => false,
        }
    }

    /// Returns true if this entry is empty.
    /// For directories, checks whether the directory has no children.
    /// For files, checks whether the file is 0 bytes via a lightweight stat call.
    pub fn is_empty(&self) -> bool {
        match self.kind {
            Kind::Directory => !self.has_children(),
            Kind::File => std::fs::symlink_metadata(&self.path)
                .map(|metadata| metadata.len() == 0)
                .unwrap_or(false),
            Kind::Symlink { .. } => false,
        }
    }

    /// Returns a reference to the entry's display name.
    pub fn name(&self) -> &Arc<str> {
        &self.name
    }

    /// Returns a reference to the entry's path.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Returns the entry's extension, empty when it has none.
    pub fn extension(&self) -> &str {
        &self.extension
    }

    /// Returns a reference to the entry's metadata if loaded.
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }

    /// Sets the entry's display name.
    ///
    /// # Parameters
    /// - `name`: The new display name.
    pub fn set_name(&mut self, name: Arc<str>) {
        self.name = name;
    }

    /// Returns whether this entry has children, computing it once per entry.
    ///
    /// Symlinks to directories answer `true` without reading the target.
    pub fn has_children(&self) -> bool {
        match self.kind {
            Kind::Symlink { target_is_dir } => return target_is_dir,
            Kind::File => return false,
            Kind::Directory => {}
        }

        if let Some(has) = self.has_children.get() {
            return has;
        }

        let has = std::fs::read_dir(&self.path).is_ok_and(|mut entries| entries.next().is_some());
        self.has_children.set(Some(has));
        has
    }

    /// Loads metadata for this entry only if the arguments request it.
    ///
    /// # Parameters
    /// - `args`: Parsed command-line arguments that determine which metadata fields to populate.
    pub fn conditional_metadata(&mut self, args: &Args) {
        if !args.wants_metadata() {
            return;
        }

        self.unconditional_metadata(args.dereference);
    }

    /// Unconditionally loads metadata, bypassing display-flag checks.
    ///
    /// # Parameters
    /// - `dereference`: If `true`, follows symlinks so metadata reflects the target.
    pub fn unconditional_metadata(&mut self, dereference: bool) {
        // Skip if already loaded (check both size and ino for robustness)
        if let Some(meta) = &self.metadata
            && (meta.size != 0 || meta.ino != 0)
        {
            return;
        }

        self.metadata = Some(Metadata::load(&self.path, dereference).unwrap_or_default());
    }

    /// Builds a display name for a file, directory, or symlink.
    ///
    /// # Parameters
    /// - `path`: The filesystem path.
    /// - `is_symlink`: Whether the path is a symbolic link.
    /// - `show_link_target`: If `true` and the path is a symlink, appends the target.
    fn get_name(path: &Path, is_symlink: bool, show_link_target: bool) -> Arc<str> {
        if !is_symlink {
            return path
                .file_name()
                .map(|name| name.to_string_lossy().into())
                .unwrap_or_else(|| {
                    std::env::current_dir()
                        .ok()
                        .and_then(|dir| dir.file_name().map(|name| name.to_string_lossy().into()))
                        .unwrap_or_else(|| ".".into())
                });
        }

        let name = path.file_name().unwrap_or_default().to_string_lossy();

        if show_link_target {
            // Include arrow and target only if long format
            let target = symlink_utils::read_symlink_target(path);
            symlink_utils::format_symlink(&name, &target).into()
        } else {
            name.into()
        }
    }

    /// Returns the lowercase extension, empty for directories.
    ///
    /// # Parameters
    /// - `path`: The path to read the extension from.
    /// - `kind`: What the entry is; directories never carry one.
    fn get_extension(path: &Path, kind: Kind) -> Arc<str> {
        if kind == Kind::Directory {
            return "".into();
        }

        path.extension()
            .and_then(|extension| extension.to_str())
            .map(str::to_lowercase)
            .unwrap_or_default()
            .into()
    }
}
