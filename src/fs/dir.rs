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
use crate::fs::entry::Entry;
use crate::fs::glob::Glob;
use std::fs;
use std::path::PathBuf;

pub struct DirReader {
    path: PathBuf,
}

impl DirReader {
    pub fn from(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn list(&self, args: &Args) -> Vec<Entry> {
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

                // Omit empty entries (childless directories and 0-byte files)
                if args.prune && entry.is_empty() {
                    continue;
                }

                entry.conditional_metadata(args);

                entries.push(entry);
            }

            if !args.hide.is_empty() {
                self.hide_entries(&mut entries, &args.hide, args.verbose);
            }
        } else if fs::symlink_metadata(&self.path).is_ok() {
            // lstat() handles all file types including broken symlinks
            let mut entry = Entry::from_path(self.path.to_path_buf(), args.long);
            entry.conditional_metadata(args);
            entries.push(entry);
        }

        self.sort(&mut entries, args);
        entries
    }

    pub fn true_size(&self, include_hidden: bool) -> u64 {
        fn dir_size(path: &PathBuf, include_hidden: bool) -> u64 {
            let mut size = 0;

            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.filter_map(Result::ok) {
                    let path = entry.path();

                    // Skip hidden files if not including them
                    if !include_hidden
                        && let Some(name) = path.file_name()
                        && name.to_string_lossy().starts_with('.')
                    {
                        continue;
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

    fn sort(&self, entries: &mut [Entry], args: &Args) {
        // Load metadata for all entries if we're sorting by metadata fields
        let needs_metadata = matches!(
            args.sort,
            SortBy::Size | SortBy::Modified | SortBy::Created | SortBy::Accessed | SortBy::Inode
        );

        if needs_metadata {
            for entry in entries.iter_mut() {
                entry.unconditional_metadata(args.dereference);
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
