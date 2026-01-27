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

use crate::cli::args::Args;
use crate::cli::flags::SortBy;
use crate::display::theme::icons::IconSettings;
use crate::fs::entry::Entry;
use crate::fs::glob::Glob;
use std::fs;
use std::path::PathBuf;

pub(crate) struct DirReader {
    path: PathBuf,
}

impl DirReader {
    pub(crate) fn from(path: PathBuf) -> Self {
        Self { path }
    }

    pub(crate) fn path(&self) -> &PathBuf {
        &self.path
    }

    pub(crate) fn list(&self, args: &Args) -> Vec<Entry> {
        let mut entries: Vec<Entry> = Vec::new();

        if self.path.is_dir() {
            for mut entry in self
                .path
                .read_dir()
                .into_iter()
                .flatten()
                .filter_map(|dir_entry| {
                    let e = dir_entry.ok()?;
                    // Use from_dir_entry to leverage d_type and avoid stat calls
                    Some(Entry::from_dir_entry(&e, args.long))
                })
            {
                // Hidden files (use entry.name())
                if !args.all && entry.name().starts_with('.') {
                    continue;
                }

                // Explicit type filters (use is_dir_like for symlinks to directories)
                if args.dirs && !entry.is_dir_like() {
                    continue;
                }
                if args.files && entry.is_dir_like() {
                    continue;
                }

                if args.prune && entry.is_dir() {
                    if !entry.compute_has_children() {
                        continue;
                    }
                }

                entry.conditional_metadata(args);

                // Compute has_children for directories when icons are enabled
                // (needed to show empty folder icon)
                if entry.is_dir() && IconSettings::enabled() {
                    entry.compute_has_children();
                }

                entries.push(entry);
            }

            if !args.hide.is_empty() {
                self.hide_entries(&mut entries, &args.hide, args.verbose);
            }
        } else if fs::symlink_metadata(&self.path).is_ok() {
            // lstat() handles all file types including broken symlinks
            let mut entry = Entry::from_path(self.path.to_path_buf(), args.long);
            entry.conditional_metadata(args);
            // Compute has_children for directories when icons are enabled
            if entry.is_dir() && IconSettings::enabled() {
                entry.compute_has_children();
            }
            entries.push(entry);
        }

        self.sort(&mut entries, args);
        entries
    }

    pub(crate) fn true_size(&self, include_hidden: bool) -> u64 {
        fn dir_size(path: &PathBuf, include_hidden: bool) -> u64 {
            let mut size = 0;

            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.filter_map(Result::ok) {
                    let path = entry.path();

                    // Skip hidden files if not including them
                    if !include_hidden {
                        if let Some(name) = path.file_name() {
                            if name.to_string_lossy().starts_with('.') {
                                continue;
                            }
                        }
                    }

                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            size += metadata.len();
                        } else if metadata.is_dir() {
                            // Recursive call for subdirectory
                            size += dir_size(&path, include_hidden);
                        }
                    }
                }
            }

            size
        }

        if !self.path.is_dir() {
            0
        } else {
            dir_size(&self.path, include_hidden)
        }
    }

    /// Returns true if this directory is empty.
    /// Callers should ensure the path is a directory before calling.
    pub(crate) fn is_empty(&self) -> bool {
        // read_dir will fail if not a directory, which is fine - return false
        if let Ok(mut entries) = fs::read_dir(&self.path) {
            entries.next().is_none()
        } else {
            false
        }
    }

    fn hide_entries(
        &self,
        entries: &mut Vec<Entry>,
        hide_patterns: &[String],
        verbose: bool,
    ) -> usize {
        if hide_patterns.is_empty() {
            return 0;
        }

        // Compile glob patterns
        let globs: Vec<_> = hide_patterns
            .iter()
            .filter_map(|p| match Glob::new(p) {
                Ok(g) => Some(g),
                Err(e) => {
                    if verbose {
                        eprintln!("Invalid hide pattern '{}': {}", p, e);
                    }
                    None
                }
            })
            .collect();

        let original_len = entries.len();

        // Retain entries that don't match any hide pattern
        entries.retain(|entry| !globs.iter().any(|g| g.is_match(entry.name())));

        let removed = original_len - entries.len();

        if removed == 0 && verbose {
            println!(
                "Hide pattern(s) {:?} matched nothing in '{}'",
                hide_patterns,
                self.path.display()
            );
        }

        removed
    }

    fn sort(&self, entries: &mut Vec<Entry>, args: &Args) {
        // Load metadata for all entries if we're sorting by metadata fields
        let needs_metadata = matches!(
            args.sort,
            SortBy::Size | SortBy::Modified | SortBy::Created | SortBy::Accessed | SortBy::Inode
        );

        if needs_metadata {
            for entry in entries.iter_mut() {
                entry.unconditional_metadata();
            }
        }

        match args.sort {
            SortBy::Size => {
                entries.sort_by_cached_key(|entry| entry.metadata().map(|m| m.size).unwrap_or(0));
            }
            SortBy::Modified => {
                entries.sort_by_cached_key(|entry| entry.metadata().map(|m| m.mtime).unwrap_or(0));
            }
            SortBy::Created => {
                entries.sort_by_cached_key(|entry| entry.metadata().map(|m| m.ctime).unwrap_or(0));
            }
            SortBy::Accessed => {
                entries.sort_by_cached_key(|entry| entry.metadata().map(|m| m.atime).unwrap_or(0));
            }
            SortBy::Inode => {
                entries.sort_by_cached_key(|entry| entry.metadata().map(|m| m.ino).unwrap_or(0));
            }
            SortBy::Extension => {
                entries.sort_by_cached_key(|entry| entry.extension().to_lowercase());
            }
            SortBy::Name => {
                entries.sort_by_cached_key(|entry| entry.name().to_lowercase());
            }
        }

        if args.reverse {
            entries.reverse();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::args::Args;
    use crate::cli::flags::{
        DateFormat, NumberFormat, OwnershipFormat, PermissionFormat, QuoteStyle, ShowColour,
        ShowHyperlink, ShowIcons, SizeFormat, SortBy,
    };
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // Helper function to create a test directory structure
    fn setup_test_dir() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let base = temp_dir.path();

        // Create files
        File::create(base.join("file1.txt")).unwrap();
        File::create(base.join("file2.rs")).unwrap();
        File::create(base.join(".hidden")).unwrap();

        // Create subdirectory with files
        fs::create_dir(base.join("subdir")).unwrap();
        File::create(base.join("subdir/nested.txt")).unwrap();
        File::create(base.join("subdir/.hidden_nested")).unwrap();

        // Create empty directory
        fs::create_dir(base.join("empty_dir")).unwrap();

        temp_dir
    }

    fn default_args() -> Args {
        Args {
            path: PathBuf::from("."),
            oneline: false,
            accessed: false,
            blocks: false,
            all: false,
            block_size: false,
            long: false,
            modified: false,
            mountpoint: false,
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
            xattr: false,
            prune: false,
            colours: ShowColour::Always,
            icons: ShowIcons::Auto,
            hyperlink: ShowHyperlink::Never,
            find: "".to_string(),
            #[cfg(feature = "magic")]
            magic: false,
            #[cfg(feature = "checksum")]
            checksum: None,
            date_format: DateFormat::Locale,
            number_format: NumberFormat::Humanly,
            ownership_format: OwnershipFormat::Name,
            permission_format: PermissionFormat::Symbolic,
            created: false,
            hard_links: false,
            quote_name: QuoteStyle::Auto,
            size: false,
            user: false,
            size_format: SizeFormat::Bytes,
            acl: false,
            context: false,
            width: None,
        }
    }

    #[test]
    fn test_new_directory() {
        let path = PathBuf::from("/tmp");
        let dir_reader = DirReader::from(path.clone());
        assert_eq!(dir_reader.path, path);
    }

    #[test]
    fn test_list_basic() {
        let temp_dir = setup_test_dir();
        let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
        let args = default_args();

        let entries = dir_reader.list(&args);

        // Should not include hidden files by default
        assert_eq!(entries.len(), 4); // file1.txt, file2.rs, subdir, empty_dir
        assert!(entries.iter().any(|e| e.name().as_ref() == "file1.txt"));
        assert!(entries.iter().any(|e| e.name().as_ref() == "file2.rs"));
        assert!(entries.iter().any(|e| e.name().as_ref() == "subdir"));
    }

    #[test]
    fn test_list_with_all_flag() {
        let temp_dir = setup_test_dir();
        let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
        let mut args = default_args();
        args.all = true;

        let entries = dir_reader.list(&args);

        // Should include hidden files
        assert!(entries.iter().any(|e| e.name().as_ref() == ".hidden"));
        assert!(entries.len() >= 4);
    }

    #[test]
    fn test_list_dirs_only() {
        let temp_dir = setup_test_dir();
        let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
        let mut args = default_args();
        args.dirs = true;

        let entries = dir_reader.list(&args);

        // Should only return directories
        assert!(entries.iter().all(|e| e.is_dir_like()));
        assert!(entries.iter().any(|e| e.name().as_ref() == "subdir"));
        assert!(entries.iter().any(|e| e.name().as_ref() == "empty_dir"));
    }

    #[test]
    fn test_list_files_only() {
        let temp_dir = setup_test_dir();
        let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
        let mut args = default_args();
        args.files = true;

        let entries = dir_reader.list(&args);

        // Should only return files
        assert!(entries.iter().all(|e| !e.is_dir_like()));
        assert!(entries.iter().any(|e| e.name().as_ref() == "file1.txt"));
        assert!(entries.iter().any(|e| e.name().as_ref() == "file2.rs"));
    }

    #[test]
    fn test_list_single_file() {
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.path().join("file1.txt");
        let dir_reader = DirReader::from(file_path.clone());
        let args = default_args();

        let entries = dir_reader.list(&args);

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name().as_ref(), "file1.txt");
    }

    #[test]
    fn test_hide_entries() {
        let temp_dir = setup_test_dir();
        let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
        let mut args = default_args();
        args.hide = vec!["file1.txt".to_string(), "subdir".to_string()];

        let entries = dir_reader.list(&args);

        // Should not include hidden entries
        assert!(!entries.iter().any(|e| e.name().as_ref() == "file1.txt"));
        assert!(!entries.iter().any(|e| e.name().as_ref() == "subdir"));
        assert!(entries.iter().any(|e| e.name().as_ref() == "file2.rs"));
    }

    #[test]
    fn test_sort_by_name() {
        let temp_dir = setup_test_dir();
        let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
        let mut args = default_args();
        args.sort = SortBy::Name;

        let entries = dir_reader.list(&args);

        // Check if sorted alphabetically
        for i in 0..entries.len().saturating_sub(1) {
            assert!(entries[i].name().to_lowercase() <= entries[i + 1].name().to_lowercase());
        }
    }

    #[test]
    fn test_sort_by_extension() {
        let temp_dir = setup_test_dir();
        let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
        let mut args = default_args();
        args.sort = SortBy::Extension;

        let entries = dir_reader.list(&args);

        // Check if sorted by extension
        for i in 0..entries.len().saturating_sub(1) {
            assert!(
                entries[i].extension().to_lowercase() <= entries[i + 1].extension().to_lowercase()
            );
        }
    }

    #[test]
    fn test_reverse_sort() {
        let temp_dir = setup_test_dir();
        let dir_reader = DirReader::from(temp_dir.path().to_path_buf());
        let mut args = default_args();
        args.sort = SortBy::Name;
        args.reverse = true;

        let entries = dir_reader.list(&args);

        // Check if sorted in reverse alphabetical order
        for i in 0..entries.len().saturating_sub(1) {
            assert!(entries[i].name().to_lowercase() >= entries[i + 1].name().to_lowercase());
        }
    }

    #[test]
    fn test_true_size_with_hidden() {
        let temp_dir = setup_test_dir();
        let base = temp_dir.path();

        // Write some data to files
        let mut file1 = File::create(base.join("file1.txt")).unwrap();
        file1.write_all(b"Hello").unwrap();

        let mut hidden = File::create(base.join(".hidden")).unwrap();
        hidden.write_all(b"Secret").unwrap();

        let dir_reader = DirReader::from(base.to_path_buf());

        let size_with_hidden = dir_reader.true_size(true);
        let size_without_hidden = dir_reader.true_size(false);

        assert!(size_with_hidden > size_without_hidden);
        assert!(size_with_hidden >= 11); // At least "Hello" + "Secret"
    }

    #[test]
    fn test_true_size_recursive() {
        let temp_dir = setup_test_dir();
        let base = temp_dir.path();

        // Write data to nested file
        let mut nested = File::create(base.join("subdir/nested.txt")).unwrap();
        nested.write_all(b"Nested content").unwrap();

        let dir_reader = DirReader::from(base.to_path_buf());
        let size = dir_reader.true_size(true);

        // Should include nested files
        assert!(size >= 14); // At least "Nested content"
    }

    #[test]
    fn test_true_size_non_directory() {
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.path().join("file1.txt");

        let dir_reader = DirReader::from(file_path);
        let size = dir_reader.true_size(true);

        assert_eq!(size, 0); // Non-directories return 0
    }

    #[test]
    fn test_is_empty_true() {
        let temp_dir = TempDir::new().unwrap();
        let empty_dir = temp_dir.path().join("empty");
        fs::create_dir(&empty_dir).unwrap();

        let dir_reader = DirReader::from(empty_dir);
        assert!(dir_reader.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let temp_dir = setup_test_dir();
        let dir_reader = DirReader::from(temp_dir.path().to_path_buf());

        assert!(!dir_reader.is_empty());
    }

    #[test]
    fn test_is_empty_non_directory() {
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.path().join("file1.txt");

        let dir_reader = DirReader::from(file_path);
        assert!(!dir_reader.is_empty()); // Non-directories return false
    }

    #[cfg(unix)]
    #[test]
    fn test_list_special_file_types() {
        use std::os::unix::net::UnixListener;

        let temp_dir = TempDir::new().unwrap();

        // Create a Unix socket (special file type, not regular file or directory)
        let socket_path = temp_dir.path().join("test.sock");
        let _listener = UnixListener::bind(&socket_path).unwrap();

        // Test that DirReader::list returns the socket when passed directly
        let dir_reader = DirReader::from(socket_path.clone());
        let args = default_args();
        let entries = dir_reader.list(&args);

        // Should return exactly 1 entry (the socket itself)
        assert_eq!(entries.len(), 1, "Special file types should be listed");
        assert_eq!(entries[0].name().as_ref(), "test.sock");
    }
}
