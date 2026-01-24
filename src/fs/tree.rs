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

//! Tree structure for hierarchical directory representation.

use crate::cli::args::Args;
use crate::fs::dir::DirReader;
use crate::fs::entry::Entry;
use std::path::PathBuf;

/// A node in a directory tree.
#[derive(Debug, Clone)]
pub(crate) struct TreeNode {
    pub(crate) entry: Entry,
    pub(crate) children: Vec<TreeNode>,
}

/// Builds a tree representation of a directory.
pub(crate) struct TreeBuilder {
    path: PathBuf,
}

impl TreeBuilder {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Builds the complete tree structure.
    pub(crate) fn build(&self, args: &Args) -> TreeNode {
        // Create the root entry (requires stat since we only have a path)
        let mut root_entry = Entry::from_path(self.path.clone(), args.long);
        root_entry.conditional_metadata(args);
        self.build_node(root_entry, args)
    }

    /// Recursively builds a tree node from an existing Entry.
    ///
    /// Takes an Entry directly to avoid redundant stat calls - child entries
    /// are already created efficiently via `from_dir_entry()` in `DirReader::list()`.
    fn build_node(&self, entry: Entry, args: &Args) -> TreeNode {
        let is_dir = entry.is_dir();
        let path = entry.path().clone();

        let mut node = TreeNode {
            entry,
            children: Vec::new(),
        };

        if is_dir {
            let dir_reader = DirReader::from(path);
            let entries = dir_reader.list(args);

            for child_entry in entries {
                // Recursively build, reusing the Entry created by from_dir_entry()
                node.children.push(self.build_node(child_entry, args));
            }
        }

        node
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
            block_size: false,
            blocks: false,
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
            permission_format: PermissionsFormat::Symbolic,
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

        fs::create_dir(base.join("subdir")).unwrap();
        File::create(base.join("subdir/nested.txt")).unwrap();

        temp_dir
    }

    #[test]
    fn test_tree_build() {
        let temp_dir = setup_test_dir();
        let builder = TreeBuilder::new(temp_dir.path().to_path_buf());
        let args = default_args();

        let tree = builder.build(&args);

        assert!(tree.entry.is_dir());
        assert!(!tree.children.is_empty());
    }

    #[test]
    fn test_tree_nested() {
        let temp_dir = setup_test_dir();
        let builder = TreeBuilder::new(temp_dir.path().to_path_buf());
        let args = default_args();

        let tree = builder.build(&args);

        let subdir = tree
            .children
            .iter()
            .find(|n| n.entry.name().as_ref() == "subdir");

        assert!(subdir.is_some());
        assert!(!subdir.unwrap().children.is_empty());
    }

    #[test]
    fn test_tree_node_structure() {
        let temp_dir = setup_test_dir();
        let entry = Entry::from_path(temp_dir.path().to_path_buf(), false);

        let node = TreeNode {
            entry: entry.clone(),
            children: vec![],
        };

        assert_eq!(node.entry.path(), entry.path());
        assert!(node.children.is_empty());
    }
}
