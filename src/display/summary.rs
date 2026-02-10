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

use crate::display::styles::element::ElementStyle;
use crate::fs::entry::Entry;
use crate::fs::tree::TreeNode;
use humanly::HumanNumber;

/// Counts directories and files in a flat slice of entries.
///
/// # Parameters
/// - `entries`: The entries to count.
///
/// # Returns
/// A tuple of (directory count, file count).
pub(crate) fn count_entries(entries: &[Entry]) -> (usize, usize) {
    let mut dirs = 0;
    let mut files = 0;
    for entry in entries {
        if entry.is_dir() {
            dirs += 1;
        } else {
            files += 1;
        }
    }
    (dirs, files)
}

/// Recursively counts directories and files in a tree, excluding the root.
///
/// # Parameters
/// - `root`: The root node whose children are counted.
///
/// # Returns
/// A tuple of (directory count, file count) across all descendants.
pub(crate) fn count_tree_children(root: &TreeNode) -> (usize, usize) {
    /// Recursively counts a single tree node and all its descendants.
    ///
    /// # Parameters
    /// - `node`: The node to count.
    /// - `dirs`: Accumulator for directory count.
    /// - `files`: Accumulator for file count.
    fn count_tree_node(node: &TreeNode, dirs: &mut usize, files: &mut usize) {
        if node.entry.is_dir() {
            *dirs += 1;
        } else {
            *files += 1;
        }
        for child in &node.children {
            count_tree_node(child, dirs, files);
        }
    }
    let (mut dirs, mut files) = (0, 0);
    for child in &root.children {
        count_tree_node(child, &mut dirs, &mut files);
    }
    (dirs, files)
}

/// Provides a directory and file count summary line after listing output.
///
/// Implementors supply their own counting logic via [`Summary::counts`],
/// while formatting and printing use shared default methods.
pub(crate) trait Summary {
    /// Returns the directory and file counts for this renderer's entries.
    fn counts(&self) -> (usize, usize);

    /// Formats the counts as a human-readable string.
    ///
    /// Produces output like "3 directories and 5 files", using singular forms
    /// when counts are 1. Omits the directory part when there are no directories,
    /// and the file part when there are no files.
    ///
    /// # Returns
    /// The formatted summary, or an empty string if both counts are zero.
    fn format(&self) -> String {
        let (dir_count, file_count) = self.counts();

        let dirs = match dir_count {
            0 => None,
            1 => Some("1 directory".to_string()),
            number => Some(format!("{} directories", HumanNumber::from(number as f64))),
        };

        let files = match file_count {
            0 => None,
            1 => Some("1 file".to_string()),
            number => Some(format!("{} files", HumanNumber::from(number as f64))),
        };

        match (dirs, files) {
            (Some(d), Some(f)) => format!("{d} and {f}"),
            (Some(d), None) => d,
            (None, Some(f)) => f,
            (None, None) => String::new(),
        }
    }

    /// Prints the formatted and styled summary line to stdout.
    fn print_summary(&self) {
        let text = self.format();
        if !text.is_empty() {
            println!("\n{}.", ElementStyle::summary(&text));
        }
    }
}
