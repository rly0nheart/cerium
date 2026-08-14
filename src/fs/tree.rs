// SPDX-License-Identifier: MIT

//! Tree structure for hierarchical directory representation.

use crate::cli::args::Args;
use crate::fs::dir::DirReader;
use crate::fs::entry::Entry;
use std::path::PathBuf;

/// A node in a directory tree, holding an entry and its recursive children.
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub entry: Entry,
    pub children: Vec<TreeNode>,
}

/// Builds the tree rooted at `path`.
///
/// # Parameters
/// - `path`: The root directory to build the tree from.
/// - `args`: CLI arguments controlling filters, metadata, and sorting.
///
/// # Returns
/// A [`TreeNode`] for the root, with children populated recursively.
pub fn build(path: PathBuf, args: &Args) -> TreeNode {
    // Create the root entry (requires stat since we only have a path)
    let mut root_entry = Entry::from_path(path, args.long);
    root_entry.conditional_metadata(args);
    build_node(root_entry, args)
}

/// Recursively builds a tree node from an existing entry.
///
/// Takes an [`Entry`] directly to avoid redundant stat calls — child entries
/// are already created efficiently via `from_dir_entry()` in [`DirReader::list`].
///
/// # Parameters
/// - `entry`: The pre-built entry for this node.
/// - `args`: CLI arguments controlling filters, metadata, and sorting.
///
/// # Returns
/// A [`TreeNode`] with children populated recursively if the entry is a directory.
fn build_node(entry: Entry, args: &Args) -> TreeNode {
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
            node.children.push(build_node(child_entry, args));
        }
    }

    node
}
