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

//! File search functionality using glob patterns.

use crate::cli::args::Args;
use crate::fs::dir::DirReader;
use crate::fs::entry::Entry;
use crate::fs::glob::Glob;
use std::path::PathBuf;

/// Searches for files matching a glob pattern under a base directory.
pub struct Search {
    glob: Glob,
    base_path: PathBuf,
}

impl Search {
    /// Creates a new search with the given glob pattern rooted at `base_path`.
    ///
    /// # Parameters
    /// - `pattern`: A glob string where `*` matches any sequence and `?` matches any single character.
    /// - `base_path`: The root directory to search from.
    ///
    /// # Returns
    /// A configured [`Search`], or an error if the pattern fails to compile.
    pub fn new(pattern: &str, base_path: PathBuf) -> Result<Self, String> {
        let glob = Glob::new(pattern)?;
        Ok(Self { glob, base_path })
    }

    /// Executes the search and returns matching entries.
    ///
    /// If `args.recursive` is true, searches subdirectories as well.
    ///
    /// # Parameters
    /// - `args`: CLI arguments controlling recursion, filters, verbosity, and metadata.
    ///
    /// # Returns
    /// A `Vec<Entry>` of all entries whose names match the glob pattern.
    pub fn find(&self, args: &Args) -> Vec<Entry> {
        let mut matches = Vec::new();
        let dir_reader = DirReader::from(self.base_path.clone());
        self.search_dir(&dir_reader, args, &mut matches);

        if args.verbose {
            println!(
                "Found {} matches in {}\n",
                matches.len(),
                self.base_path.display()
            );
        }

        matches
    }

    /// Recursively searches a directory, appending matched entries to `matches`.
    ///
    /// # Parameters
    /// - `dir_reader`: The directory to scan.
    /// - `args`: CLI arguments controlling filters, recursion, and verbosity.
    /// - `matches`: Accumulator for entries whose names match the glob.
    fn search_dir(&self, dir_reader: &DirReader, args: &Args, matches: &mut Vec<Entry>) {
        if args.verbose {
            println!("Searching in {} ...", dir_reader.path().display());
        }

        for mut entry in dir_reader.list(args) {
            let is_dir_like = entry.is_dir_like();

            // Check if entry matches (respecting --dirs/--files filters)
            let dominated_match = if (args.dirs && !is_dir_like) || (args.files && is_dir_like) {
                false
            } else {
                self.glob.is_match(entry.name())
            };

            if dominated_match {
                if args.verbose {
                    println!("Match: {}", entry.path().display());
                }

                entry.conditional_metadata(args);

                // Prepend relative path from base
                let display_name = self.relative_display_name(&entry);
                entry.set_name(display_name.into());

                matches.push(entry.clone());
            }

            // Recurse into subdirectories if -R flag is set
            if args.recursive && is_dir_like {
                let subdir = DirReader::from(entry.path().clone());
                self.search_dir(&subdir, args, matches);
            }
        }
    }

    /// Builds a display name with the relative path prefix from `base_path`.
    ///
    /// # Parameters
    /// - `entry`: The entry to produce a display name for.
    ///
    /// # Returns
    /// A string like `"subdir/file.txt"` relative to the search root, or just
    /// the entry name if it lives directly under `base_path`.
    fn relative_display_name(&self, entry: &Entry) -> String {
        let parent_prefix = entry
            .path()
            .parent()
            .and_then(|p| p.strip_prefix(&self.base_path).ok())
            .map(|p| format!("{}/", p.display()))
            .unwrap_or_default();

        format!("{}{}", parent_prefix, entry.name())
    }
}
