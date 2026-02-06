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

/// Searches for files matching a glob pattern.
pub(crate) struct Search {
    glob: Glob,
    base_path: PathBuf,
}

impl Search {
    /// Creates a new search with the given glob pattern.
    pub(crate) fn new(pattern: &str, base_path: PathBuf) -> Result<Self, String> {
        let glob = Glob::new(pattern)?;
        Ok(Self { glob, base_path })
    }

    /// Executes the search and returns matching entries.
    ///
    /// If `args.recursive` is true, searches subdirectories as well.
    pub(crate) fn find(&self, args: &Args) -> Vec<Entry> {
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

    /// Creates the display name with relative path prefix.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::flags::*;
    use std::fs::{self, File};
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
            headers: false,
            hide: Vec::new(),
            inode: false,
            verbose: false,
            xattr: false,
            prune: false,
            colours: ShowColour::Always,
            icons: ShowIcons::Auto,
            hyperlink: ShowHyperlink::Never,
            find: "".to_string(),
            #[cfg(all(feature = "magic", not(target_os = "android")))]
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

    fn setup_test_dir() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let base = temp_dir.path();

        File::create(base.join("file1.txt")).unwrap();
        File::create(base.join("file2.rs")).unwrap();
        File::create(base.join("other.txt")).unwrap();

        fs::create_dir(base.join("subdir")).unwrap();
        File::create(base.join("subdir/nested.txt")).unwrap();
        File::create(base.join("subdir/nested.rs")).unwrap();

        temp_dir
    }

    #[test]
    fn test_search_glob() {
        let temp_dir = setup_test_dir();
        let search = Search::new("*.txt", temp_dir.path().to_path_buf()).unwrap();
        let args = default_args();

        let matches = search.find(&args);

        assert_eq!(matches.len(), 2);
        assert!(matches.iter().any(|e| e.name().contains("file1.txt")));
        assert!(matches.iter().any(|e| e.name().contains("other.txt")));
    }

    #[test]
    fn test_search_recursive() {
        let temp_dir = setup_test_dir();
        let search = Search::new("*.txt", temp_dir.path().to_path_buf()).unwrap();
        let mut args = default_args();
        args.recursive = true;

        let matches = search.find(&args);

        assert_eq!(matches.len(), 3);
        assert!(matches.iter().any(|e| e.name().contains("nested.txt")));
    }

    #[test]
    fn test_search_case_insensitive() {
        let temp_dir = setup_test_dir();
        let search = Search::new("FILE*", temp_dir.path().to_path_buf()).unwrap();
        let args = default_args();

        let matches = search.find(&args);

        assert_eq!(matches.len(), 2);
    }
}
