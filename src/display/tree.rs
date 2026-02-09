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
use crate::display::layout::alignment::Align;
use crate::display::layout::column;
use crate::display::layout::column::Column;
use crate::display::layout::width::Width;
use crate::display::mode::DisplayMode;
use crate::display::output::quotes::Quotes;
use crate::display::styles::column::ColumnStyle;
use crate::display::styles::entry::StyledEntry;
use crate::display::styles::text::TextStyle;
use crate::display::summary::Summary;
use crate::fs::dir::DirReader;
use crate::fs::entry::Entry;
use crate::fs::tree::TreeNode;
use std::cell::Cell;
use std::collections::HashMap;
use std::path::PathBuf;

/// Unicode box drawing character for vertical line with spaces (│   )
const LINE_CONNECTOR: &str = "\u{2502}\u{0020}\u{0020}\u{0020}";
/// Unicode box drawing character for branch connector (├── )
const EDGE_CONNECTOR: &str = "\u{251C}\u{2500}\u{2500}\u{0020}";
/// Unicode box drawing character for last branch connector (╰── )
const CORNER_CONNECTOR: &str = "\u{2570}\u{2500}\u{2500}\u{0020}";
/// Four space characters for indentation
const FOUR_SPACES: &str = "\u{0020}\u{0020}\u{0020}\u{0020}";

impl DisplayMode for Tree {
    /// Prints the tree-structured directory listing with visual hierarchy.
    ///
    /// # Process
    ///
    /// **Streaming mode** (no columns/table needed):
    /// - Traverses filesystem on-demand
    /// - Prints entries immediately as they are discovered
    /// - Provides instant feedback for large directory trees
    ///
    /// **Table mode** (columns requested):
    /// 1. Flattens the tree structure to extract all entries
    /// 2. Calculates optimal column widths based on all entries
    /// 3. Prints optional column headers
    /// 4. Recursively renders the tree with proper connectors
    fn print(&self) {
        match &self.data {
            TreeData::Streaming(path) => {
                // Streaming mode: traverse and print on-demand
                let mut parent_entry = Entry::from_path(path.clone(), self.args.long);
                parent_entry.conditional_metadata(&self.args);
                self.traverse_and_print(parent_entry, &Vec::new());
            }
            TreeData::Table(node) => {
                // Table mode: use pre-built tree with width calculations
                let mut entries = Vec::new();
                Self::flatten(node, &mut entries);

                // Add an alignment space in any entries in have got special characters and will get quoted
                let add_alignment_space = entries.iter().any(|e| Quotes::is_quotable(e.name()));

                let columns = column::Selector::select(&self.args);
                let mut width_calc = Width::new();
                let widths = width_calc.calculate(&entries, &columns, &self.args);

                if self.args.headers {
                    Column::headers(&widths, &self.args);
                }

                Self::add_node(node, &widths, &Vec::new(), &self.args, add_alignment_space);
            }
        }

        self.print_summary();
    }
}

/// Backing data for the tree renderer.
pub(crate) enum TreeData {
    /// Pre-built tree structure for table mode with columns
    Table(TreeNode),
    /// Root path for streaming mode without columns
    Streaming(PathBuf),
}

/// Hierarchical renderer using Unicode box-drawing connectors.
pub(crate) struct Tree {
    data: TreeData,
    args: Args,
    dir_count: Cell<usize>,
    file_count: Cell<usize>,
}

impl Summary for Tree {
    /// Returns the accumulated directory and file counts.
    ///
    /// For table mode, counts are computed from the pre-built tree.
    /// For streaming mode, counts are accumulated during traversal.
    fn counts(&self) -> (usize, usize) {
        match &self.data {
            TreeData::Table(node) => crate::display::summary::count_tree_children(node),
            TreeData::Streaming(_) => (self.dir_count.get(), self.file_count.get()),
        }
    }
}

impl Tree {
    /// Creates a [`Tree`] renderer in table mode with a pre-built tree.
    ///
    /// # Parameters
    /// - `node`: The root node of the directory tree.
    /// - `args`: Command-line arguments controlling display options.
    pub(crate) fn new_table(node: TreeNode, args: Args) -> Self {
        Self {
            data: TreeData::Table(node),
            args,
            dir_count: Cell::new(0),
            file_count: Cell::new(0),
        }
    }

    /// Creates a [`Tree`] renderer in streaming mode for on-demand traversal.
    ///
    /// # Parameters
    /// - `path`: The root path to traverse.
    /// - `args`: Command-line arguments controlling display options.
    pub(crate) fn new_streaming(path: PathBuf, args: Args) -> Self {
        Self {
            data: TreeData::Streaming(path),
            args,
            dir_count: Cell::new(0),
            file_count: Cell::new(0),
        }
    }

    /// Checks whether the tree requires table layout with column width calculations.
    ///
    /// # Parameters
    /// - `args`: Command-line arguments to check.
    ///
    /// # Returns
    /// `true` if any metadata or table-specific columns are requested.
    pub(crate) fn needs_table_layout(args: &Args) -> bool {
        // Metadata columns
        if args.long
            || args.size
            || args.permissions
            || args.user
            || args.group
            || args.created
            || args.modified
            || args.accessed
            || args.inode
            || args.blocks
            || args.hard_links
            || args.block_size
        {
            return true;
        }

        // Table-specific columns
        #[cfg(all(feature = "magic", not(target_os = "android")))]
        if args.magic {
            return true;
        }

        #[cfg(feature = "checksum")]
        if args.checksum.is_some() {
            return true;
        }

        if args.xattr || args.acl || args.context || args.mountpoint || args.oneline {
            return true;
        }

        false
    }

    /// Traverses the filesystem and prints the tree in streaming mode.
    ///
    /// Accumulates directory and file counts (excluding the root) into the
    /// struct's [`Cell`] fields for later retrieval via [`Summary::counts`].
    ///
    /// # Parameters
    /// - `entry`: The current entry to render.
    /// - `parents_last`: Boolean flags indicating whether each ancestor is the last child.
    fn traverse_and_print(&self, entry: Entry, parents_last: &[bool]) {
        let connector = Self::draw_connector(parents_last);

        // Get styled entry for name display (no alignment space for tree)
        let styled_entry = StyledEntry::new(&entry);
        let entry_view = styled_entry.load(&self.args, false);

        // Print: [connector] [name]
        println!(
            "{}{}",
            TextStyle::tree_connector(&connector),
            TextStyle::name(&entry_view.name, entry_view.colour),
        );

        // Count non-root entries (root has empty parents_last)
        if !parents_last.is_empty() {
            if entry.is_dir() {
                self.dir_count.set(self.dir_count.get() + 1);
            } else {
                self.file_count.set(self.file_count.get() + 1);
            }
        }

        // If this is a directory, traverse and print its children
        if entry.is_dir() {
            let dir_reader = DirReader::from(entry.path().clone());
            let children = dir_reader.list(&self.args);

            let count = children.len();
            for (i, mut child_entry) in children.into_iter().enumerate() {
                child_entry.conditional_metadata(&self.args);
                let mut new_parents = parents_last.to_owned();
                new_parents.push(i == count - 1);
                self.traverse_and_print(child_entry, &new_parents);
            }
        }
    }

    /// Flattens a tree into a linear vector of entries for width calculation.
    ///
    /// # Parameters
    /// - `node`: The root node to flatten.
    /// - `entries`: Mutable vector to populate with entries.
    fn flatten(node: &TreeNode, entries: &mut Vec<Entry>) {
        entries.push(node.entry.clone());
        for child in &node.children {
            Self::flatten(child, entries);
        }
    }

    /// Recursively renders a node and its children with tree connectors.
    ///
    /// # Parameters
    /// - `node`: The current node to render.
    /// - `widths`: Pre-calculated column widths for alignment.
    /// - `parents_last`: Flags indicating whether each ancestor is the last child.
    /// - `args`: Command-line arguments controlling display options.
    /// - `add_alignment_space`: Whether to add a space for quote-alignment.
    fn add_node(
        node: &TreeNode,
        widths: &HashMap<Column, usize>,
        parents_last: &[bool],
        args: &Args,
        add_alignment_space: bool,
    ) {
        let entry = &node.entry;
        let connector = Self::draw_connector(parents_last);

        // Render the row with tree connectors
        Self::render_tree_row(entry, widths, &connector, args, add_alignment_space);

        let count = node.children.len();
        for (i, child) in node.children.iter().enumerate() {
            let mut new_parents = parents_last.to_owned();
            new_parents.push(i == count - 1);
            Self::add_node(child, widths, &new_parents, args, add_alignment_space);
        }
    }

    /// Renders a single row in tree mode with connectors and column data.
    ///
    /// # Parameters
    /// - `entry`: The entry to render.
    /// - `widths`: Pre-calculated column widths for alignment.
    /// - `connector`: Tree connector string (e.g., `"├── "`).
    /// - `args`: Command-line arguments controlling display options.
    /// - `add_alignment_space`: Whether to add a space for quote-alignment.
    fn render_tree_row(
        entry: &Entry,
        widths: &HashMap<Column, usize>,
        connector: &str,
        args: &Args,
        add_alignment_space: bool,
    ) {
        let columns = column::Selector::select(args);
        let mut parts = Vec::new();

        // Build column data
        for column in &columns {
            let styled_column = ColumnStyle::get(entry, column, args, add_alignment_space);
            let width = *widths
                .get(column)
                .unwrap_or(&Width::measure_ansi_text(&styled_column));
            let padded = Align::pad(&styled_column, width, column.alignment());
            parts.push(padded);
        }

        // Get styled entry for name display (no alignment space for tree)
        let styled_entry = StyledEntry::new(entry);
        let entry_view = styled_entry.load(args, false);

        // Print: [table columns] [connector] [name]
        println!(
            "{} {}{}",
            parts.join(" "),
            TextStyle::tree_connector(connector),
            TextStyle::name(&entry_view.name, entry_view.colour),
        );
    }

    /// Builds the connector string with box-drawing characters for a tree node.
    ///
    /// # Parameters
    /// - `parents_last`: Flags indicating whether each ancestor is the last child.
    ///
    /// # Returns
    /// A string of box-drawing characters representing the node's position in the tree.
    fn draw_connector(parents_last: &[bool]) -> String {
        let mut connector = String::new();
        let depth = parents_last.len();
        if depth > 0 {
            for &last in &parents_last[..depth - 1] {
                connector.push_str(if last { FOUR_SPACES } else { LINE_CONNECTOR });
            }
            let is_last = parents_last[depth - 1];
            connector.push_str(if is_last {
                CORNER_CONNECTOR
            } else {
                EDGE_CONNECTOR
            });
        }
        connector
    }
}
