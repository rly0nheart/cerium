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
pub struct TreeNode {
    pub entry: Entry,
    pub children: Vec<TreeNode>,
}

/// Builds a tree representation of a directory.
pub struct TreeBuilder {
    path: PathBuf,
}

impl TreeBuilder {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Builds the complete tree structure.
    pub fn build(&self, args: &Args) -> TreeNode {
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
