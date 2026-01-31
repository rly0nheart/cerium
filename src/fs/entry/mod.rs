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

mod directory;
mod file;
mod symlink;

pub(crate) use directory::DirectoryEntry;
pub(crate) use file::FileEntry;
pub(crate) use symlink::SymlinkEntry;

use crate::cli::args::{Args, args_need_metadata};
use crate::fs::cache::Cache;
use crate::fs::metadata::Metadata;
use crate::fs::symlink as symlink_utils;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// A filesystem entry that can be a file, directory, or symlink.
#[derive(Clone, Debug)]
pub(crate) enum Entry {
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
    pub(crate) fn from_dir_entry(dir_entry: &DirEntry, show_link_target: bool) -> Self {
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
    pub(crate) fn from_path(path: PathBuf, show_link_target: bool) -> Self {
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
    pub(crate) fn is_dir(&self) -> bool {
        matches!(self, Entry::Directory(_))
    }

    /// Returns true only for symlinks.
    pub(crate) fn is_symlink(&self) -> bool {
        matches!(self, Entry::Symlink(_))
    }

    #[allow(dead_code)]
    /// Returns true only for regular files.
    pub(crate) fn is_file(&self) -> bool {
        matches!(self, Entry::File(_))
    }

    /// Returns true for directories AND symlinks pointing to directories.
    /// Used for filtering with --dirs/--files flags.
    pub(crate) fn is_dir_like(&self) -> bool {
        match self {
            Entry::Directory(_) => true,
            Entry::Symlink(s) => s.target_is_dir,
            Entry::File(_) => false,
        }
    }

    /// Returns a reference to the entry's display name.
    pub(crate) fn name(&self) -> &Arc<str> {
        match self {
            Entry::File(file) => &file.name,
            Entry::Directory(directory) => &directory.name,
            Entry::Symlink(symlink) => &symlink.name,
        }
    }

    /// Returns a reference to the entry's path.
    pub(crate) fn path(&self) -> &PathBuf {
        match self {
            Entry::File(file) => &file.path,
            Entry::Directory(directory) => &directory.path,
            Entry::Symlink(symlink) => &symlink.path,
        }
    }

    /// Returns a reference to the entry's extension.
    /// For directories, returns an empty string.
    pub(crate) fn extension(&self) -> &Arc<str> {
        static EMPTY: std::sync::OnceLock<Arc<str>> = std::sync::OnceLock::new();
        let empty = EMPTY.get_or_init(|| Arc::from(""));

        match self {
            Entry::File(f) => &f.extension,
            Entry::Directory(_) => empty,
            Entry::Symlink(symlink) => &symlink.extension,
        }
    }

    /// Returns a reference to the entry's metadata if loaded.
    pub(crate) fn metadata(&self) -> Option<&Metadata> {
        match self {
            Entry::File(file) => file.metadata.as_ref(),
            Entry::Directory(directory) => directory.metadata.as_ref(),
            Entry::Symlink(symlink) => symlink.metadata.as_ref(),
        }
    }

    /// Sets the entry's display name.
    /// Used by search.rs for highlighting matches.
    pub(crate) fn set_name(&mut self, name: Arc<str>) {
        match self {
            Entry::File(file) => file.name = name,
            Entry::Directory(directory) => directory.name = name,
            Entry::Symlink(symlink) => symlink.name = name,
        }
    }

    /// Returns whether this directory has children (for icon display).
    /// Returns true by default if not yet computed or if not a directory.
    pub(crate) fn has_children(&self) -> bool {
        match self {
            Entry::Directory(directory) => directory.has_children(),
            Entry::Symlink(symlink) if symlink.target_is_dir => true, // Assume symlinks to dirs have children
            _ => false,
        }
    }

    /// Computes and caches whether this directory has children.
    /// Returns false for non-directories.
    pub(crate) fn compute_has_children(&mut self) -> bool {
        match self {
            Entry::Directory(directory) => directory.compute_has_children(),
            _ => false,
        }
    }

    #[allow(dead_code)]
    /// Returns whether this entry is a broken symlink.
    pub(crate) fn is_broken_symlink(&self) -> bool {
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
    pub(crate) fn conditional_metadata(&mut self, args: &Args) {
        if !args_need_metadata(args) {
            return;
        }

        self.unconditional_metadata();
    }

    /// Unconditionally loads metadata for sorting purposes.
    /// This bypasses the args_request_metadata check since sorting
    /// needs metadata even when display flags don't request it.
    pub(crate) fn unconditional_metadata(&mut self) {
        // Skip if already loaded (check both size and ino for robustness)
        if let Some(meta) = self.metadata()
            && (meta.size != 0 || meta.ino != 0)
        {
            return;
        }

        let path = self.path().clone();
        let metadata = match Cache::metadata(&path) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::args::Args;
    use crate::cli::flags::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::os::unix::fs as unix_fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn default_args() -> Args {
        Args {
            path: PathBuf::from("."),
            oneline: false,
            accessed: false,
            all: false,
            blocks: false,
            block_size: false,
            long: false,
            modified: false,
            permission: false,
            recursive: false,
            tree: false,
            true_size: false,
            reverse: false,
            sort: SortBy::Name,
            dirs: false,
            files: false,
            group: false,
            column_headers: false,
            hide: Vec::new(),
            inode: false,
            verbose: false,
            colours: ShowColour::Always,
            icons: ShowIcons::Auto,
            hyperlink: ShowHyperlink::Never,
            xattr: false,
            acl: false,
            context: false,
            width: None,
            prune: false,
            find: "".to_string(),
            #[cfg(all(feature = "magic", not(target_os = "android")))]
            magic: false,
            #[cfg(feature = "checksum")]
            checksum: None,
            date_format: DateFormat::Locale,
            number_format: NumberFormat::Humanly,
            permission_format: PermissionFormat::Symbolic,
            created: false,
            hard_links: false,
            quote_name: QuoteStyle::Auto,
            size: false,
            user: false,
            size_format: SizeFormat::Bytes,
            mountpoint: false,
            ownership_format: OwnershipFormat::Name,
        }
    }

    #[test]
    fn test_entry_new_regular_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let entry = Entry::from_path(file_path.clone(), false);

        assert_eq!(entry.name().as_ref(), "test.txt");
        assert_eq!(entry.path(), &file_path);
        assert!(!entry.is_dir());
        assert!(!entry.is_symlink());
        assert!(entry.is_file());
        assert_eq!(entry.extension().as_ref(), "txt");
        assert!(entry.metadata().is_none());
    }

    #[test]
    fn test_entry_new_directory() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().join("test_dir");
        fs::create_dir(&dir_path).unwrap();

        let entry = Entry::from_path(dir_path.clone(), false);

        assert_eq!(entry.name().as_ref(), "test_dir");
        assert_eq!(entry.path(), &dir_path);
        assert!(entry.is_dir());
        assert!(!entry.is_symlink());
        assert!(!entry.is_file());
        assert_eq!(entry.extension().as_ref(), "");
    }

    #[test]
    fn test_entry_new_symlink_without_target() {
        let temp_dir = TempDir::new().unwrap();
        let target_path = temp_dir.path().join("target.txt");
        let link_path = temp_dir.path().join("link.txt");

        File::create(&target_path).unwrap();
        unix_fs::symlink(&target_path, &link_path).unwrap();

        let entry = Entry::from_path(link_path.clone(), false);

        assert_eq!(entry.name().as_ref(), "link.txt");
        assert!(entry.is_symlink());
        assert!(!entry.is_dir());
        assert!(!entry.is_file());
        // Name should not contain arrow when show_link_target is false
        assert!(!entry.name().contains("⇒"));
    }

    #[test]
    fn test_entry_new_symlink_with_target() {
        let temp_dir = TempDir::new().unwrap();
        let target_path = temp_dir.path().join("target.txt");
        let link_path = temp_dir.path().join("link.txt");

        File::create(&target_path).unwrap();
        unix_fs::symlink(&target_path, &link_path).unwrap();

        let entry = Entry::from_path(link_path.clone(), true);

        assert!(entry.is_symlink());
        // Name should contain arrow and target when show_link_target is true
        assert!(entry.name().contains("⇒"));
        assert!(entry.name().contains("link.txt"));
        assert!(entry.name().contains("target.txt"));
    }

    #[test]
    fn test_is_dir_like_for_symlink_to_directory() {
        let temp_dir = TempDir::new().unwrap();
        let target_dir = temp_dir.path().join("target_dir");
        let link_path = temp_dir.path().join("link_to_dir");

        fs::create_dir(&target_dir).unwrap();
        unix_fs::symlink(&target_dir, &link_path).unwrap();

        let entry = Entry::from_path(link_path.clone(), false);

        assert!(entry.is_symlink());
        assert!(!entry.is_dir()); // is_dir() returns false for symlinks
        assert!(entry.is_dir_like()); // is_dir_like() returns true for symlinks to directories
    }

    #[test]
    fn test_broken_symlink() {
        let temp_dir = TempDir::new().unwrap();
        let link_path = temp_dir.path().join("broken_link");

        // Create symlink to non-existent target
        unix_fs::symlink("/nonexistent/target", &link_path).unwrap();

        let entry = Entry::from_path(link_path.clone(), false);

        assert!(entry.is_symlink());
        assert!(entry.is_broken_symlink());
        assert!(!entry.is_dir_like());
    }

    #[test]
    fn test_get_extension_lowercase() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("Test.TXT");
        File::create(&file_path).unwrap();

        let entry = Entry::from_path(file_path, false);

        // Extension should be lowercase
        assert_eq!(entry.extension().as_ref(), "txt");
    }

    #[test]
    fn test_get_extension_no_extension() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("README");
        File::create(&file_path).unwrap();

        let entry = Entry::from_path(file_path, false);

        assert_eq!(entry.extension().as_ref(), "");
    }

    #[test]
    fn test_get_extension_multiple_dots() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("archive.tar.gz");
        File::create(&file_path).unwrap();

        let entry = Entry::from_path(file_path, false);

        // Should only get the last extension
        assert_eq!(entry.extension().as_ref(), "gz");
    }

    #[test]
    fn test_get_extension_directory() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().join("folder.dir");
        fs::create_dir(&dir_path).unwrap();

        let entry = Entry::from_path(dir_path, false);

        // Directories should have empty extension
        assert_eq!(entry.extension().as_ref(), "");
    }

    #[test]
    fn test_metadata_loading() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Hello, World!").unwrap();
        drop(file);

        let mut entry = Entry::from_path(file_path, false);
        let mut args = default_args();
        args.long = true; // This should trigger metadata loading

        entry.conditional_metadata(&args);

        assert!(entry.metadata().is_some());
        let meta = entry.metadata().unwrap();
        assert_eq!(meta.size, 13); // "Hello, World!" is 13 bytes
        assert!(meta.mode > 0);
        assert!(meta.ino > 0);
    }

    #[test]
    fn test_metadata_not_loaded_when_not_needed() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let mut entry = Entry::from_path(file_path, false);
        let args = default_args(); // No flags that require metadata

        entry.conditional_metadata(&args);

        // Metadata should not be loaded if not needed
        assert!(entry.metadata().is_none());
    }

    #[test]
    fn test_metadata_cached() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Test").unwrap();
        drop(file);

        let mut entry = Entry::from_path(file_path, false);
        let mut args = default_args();
        args.long = true;

        // Load metadata first time
        entry.conditional_metadata(&args);
        let first_size = entry.metadata().map(|m| m.size);

        // Load metadata again
        entry.conditional_metadata(&args);
        let second_size = entry.metadata().map(|m| m.size);

        // Should be the same (cached)
        assert!(first_size.is_some());
        assert!(second_size.is_some());
        assert_eq!(first_size, second_size);
    }

    #[test]
    fn test_metadata_load_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Content").unwrap();
        drop(file);

        let result = Metadata::load(&file_path);

        assert!(result.is_ok());
        let meta = result.unwrap();
        assert_eq!(meta.size, 7);
        assert!(meta.mode > 0);
        assert!(meta.ino > 0);
        assert!(meta.nlink > 0);
        assert!(meta.blksize > 0);
    }

    #[test]
    fn test_metadata_load_nonexistent_file() {
        let path = PathBuf::from("/nonexistent/file/path.txt");
        let result = Metadata::load(&path);

        assert!(result.is_err());
    }

    #[test]
    fn test_metadata_load_directory() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().to_path_buf();

        let result = Metadata::load(&dir_path);

        assert!(result.is_ok());
        let meta = result.unwrap();
        // Directories have S_IFDIR bit set in mode
        assert_ne!(meta.mode & libc::S_IFDIR, 0);
    }

    #[test]
    fn test_metadata_load_symlink() {
        let temp_dir = TempDir::new().unwrap();
        let target_path = temp_dir.path().join("target.txt");
        let link_path = temp_dir.path().join("link.txt");

        File::create(&target_path).unwrap();
        unix_fs::symlink(&target_path, &link_path).unwrap();

        let result = Metadata::load(&link_path);

        assert!(result.is_ok());
        let meta = result.unwrap();
        // Symlinks have S_IFLNK bit set in mode (lstat doesn't follow symlinks)
        assert_ne!(meta.mode & libc::S_IFLNK, 0);
    }

    #[test]
    fn test_metadata_timestamps() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let result = Metadata::load(&file_path);

        assert!(result.is_ok());
        let meta = result.unwrap();
        // Timestamps should be reasonable (after year 2000)
        assert!(meta.atime > 946_684_800); // Jan 1, 2000
        assert!(meta.mtime > 946_684_800);
        assert!(meta.ctime > 946_684_800);
    }

    #[test]
    fn test_metadata_empty() {
        let meta = Metadata::empty();

        assert_eq!(meta.mode, 0);
        assert_eq!(meta.size, 0);
        assert_eq!(meta.ino, 0);
        assert_eq!(meta.nlink, 0);
        assert_eq!(meta.uid, 0);
        assert_eq!(meta.gid, 0);
        assert_eq!(meta.blocks, 0);
        assert_eq!(meta.blksize, 0);
        assert_eq!(meta.atime, 0);
        assert_eq!(meta.mtime, 0);
        assert_eq!(meta.ctime, 0);
    }

    #[test]
    fn test_metadata_permissions() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        // Set specific permissions
        let perms = fs::Permissions::from_mode(0o644);
        fs::set_permissions(&file_path, perms).unwrap();

        let result = Metadata::load(&file_path);
        assert!(result.is_ok());
        let meta = result.unwrap();

        // Check that the permission bits are preserved
        let perm_bits = meta.mode & 0o777;
        assert_eq!(perm_bits, 0o644);
    }

    #[test]
    fn test_entry_clone() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let entry1 = Entry::from_path(file_path.clone(), false);
        let entry2 = entry1.clone();

        assert_eq!(entry1.name(), entry2.name());
        assert_eq!(entry1.path(), entry2.path());
        assert_eq!(entry1.is_dir(), entry2.is_dir());
        assert_eq!(entry1.has_children(), entry2.has_children());
        assert_eq!(entry1.is_symlink(), entry2.is_symlink());
        assert_eq!(entry1.extension(), entry2.extension());
    }

    #[test]
    fn test_metadata_clone() {
        let meta1 = Metadata {
            mode: 0o644,
            size: 1024,
            ino: 12345,
            nlink: 1,
            uid: 1000,
            gid: 1000,
            blocks: 8,
            blksize: 4096,
            atime: 1000000000,
            mtime: 1000000001,
            ctime: 1000000002,
        };

        let meta2 = meta1.clone();

        assert_eq!(meta1.mode, meta2.mode);
        assert_eq!(meta1.size, meta2.size);
        assert_eq!(meta1.ino, meta2.ino);
        assert_eq!(meta1.nlink, meta2.nlink);
        assert_eq!(meta1.uid, meta2.uid);
        assert_eq!(meta1.gid, meta2.gid);
        assert_eq!(meta1.blocks, meta2.blocks);
        assert_eq!(meta1.blksize, meta2.blksize);
        assert_eq!(meta1.atime, meta2.atime);
        assert_eq!(meta1.mtime, meta2.mtime);
        assert_eq!(meta1.ctime, meta2.ctime);
    }

    #[test]
    fn test_entry_with_hidden_file() {
        let temp_dir = TempDir::new().unwrap();
        let hidden_path = temp_dir.path().join(".hidden");
        File::create(&hidden_path).unwrap();

        let entry = Entry::from_path(hidden_path, false);

        assert_eq!(entry.name().as_ref(), ".hidden");
        assert!(entry.name().starts_with('.'));
    }

    #[test]
    fn test_entry_with_special_characters() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("file with spaces.txt");
        File::create(&file_path).unwrap();

        let entry = Entry::from_path(file_path, false);

        assert_eq!(entry.name().as_ref(), "file with spaces.txt");
        assert_eq!(entry.extension().as_ref(), "txt");
    }

    #[test]
    fn test_metadata_uid_gid() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let result = Metadata::load(&file_path);
        assert!(result.is_ok());

        let meta = result.unwrap();
        // UID and GID should be set (we can't predict exact values, but they exist)
        // Both are u32, so they're always >= 0 by definition
        assert!(meta.uid < u32::MAX);
        assert!(meta.gid < u32::MAX);
    }

    #[test]
    fn test_metadata_blocks_and_blksize() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(&[0u8; 8192]).unwrap(); // Write 8KB
        drop(file);

        let result = Metadata::load(&file_path);
        assert!(result.is_ok());

        let meta = result.unwrap();
        assert!(meta.blocks > 0);
        assert!(meta.blksize > 0);
        // Block size is typically 512 or 4096
        assert!(meta.blksize == 512 || meta.blksize == 4096 || meta.blksize == 8192);
    }

    #[test]
    fn test_set_name() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let mut entry = Entry::from_path(file_path, false);
        assert_eq!(entry.name().as_ref(), "test.txt");

        entry.set_name(Arc::from("new_name.txt"));
        assert_eq!(entry.name().as_ref(), "new_name.txt");
    }
}
