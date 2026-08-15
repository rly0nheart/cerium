// SPDX-License-Identifier: MIT

use crate::cli::args::Args;
use crate::render::layout::column;
use crate::render::layout::column::Column;
use crate::render::Renderer;
use crate::render::output::quotes;
use crate::render::style::element::ElementStyle;
use crate::render::style::entry::StyledEntry;
use crate::render::style::value::ValueStyle;
use crate::render::summary::Summary;
use crate::fs::dir::DirReader;
use crate::fs::entry::Entry;
use crate::fs::tree::TreeNode;
use std::cell::Cell;
use std::path::PathBuf;

/// An ancestor directory that still has siblings below it.
const LINE: &str = "│   ";
/// An entry with siblings after it.
const EDGE: &str = "├── ";
/// The last entry in its directory.
const CORNER: &str = "╰── ";
/// An ancestor directory with nothing left below it.
const BLANK: &str = "    ";

impl Renderer for Tree {
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
    fn show(&self) {
        match &self.data {
            TreeData::Streaming(path) => {
                // Streaming mode: traverse and print on-demand
                let mut parent_entry = Entry::from_path(path.clone(), self.args.long);
                parent_entry.conditional_metadata(&self.args);
                self.traverse_and_print(parent_entry, &mut Vec::new());
            }
            TreeData::Table(node) => {
                // Table mode: render every row first, then size the columns to fit them
                let add_alignment_space = Self::any_quotable(node);
                let columns = column::Selector::select(&self.args);

                let mut rows = Vec::new();
                Self::collect_rows(
                    node,
                    &mut Vec::new(),
                    &columns,
                    &self.args,
                    add_alignment_space,
                    &mut rows,
                );

                let widths = column::widths(
                    &columns,
                    rows.iter().map(|row| row.cells.as_slice()),
                    self.args.headers,
                );

                let mut out = String::new();
                if self.args.headers {
                    column::write_headers(&mut out, &columns, &widths);
                }
                for row in &rows {
                    out.push_str(&column::row_text(&row.cells, &columns, &widths));
                    out.push(' ');
                    out.push_str(&row.connector);
                    out.push_str(&row.name);
                    out.push('\n');
                }

                print!("{out}");
            }
        }

        self.print_summary();
    }
}

/// One fully rendered tree row: table columns, connector, and styled name.
struct TreeRow {
    cells: Vec<column::Cell>,
    connector: String,
    name: String,
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
            TreeData::Table(node) => crate::render::summary::count_tree_children(node),
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
    fn traverse_and_print(&self, entry: Entry, parents_last: &mut Vec<bool>) {
        let connector = Self::draw_connector(parents_last);

        // Get styled entry for name display (no alignment space for tree)
        let styled_entry = StyledEntry::new(&entry);
        let entry_view = styled_entry.load(&self.args, false);

        // Print: [connector] [name]
        println!(
            "{}{}",
            ElementStyle::tree_connector(&connector),
            ValueStyle::name(&entry_view.name, entry_view.color),
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
                parents_last.push(i == count - 1);
                self.traverse_and_print(child_entry, parents_last);
                parents_last.pop();
            }
        }
    }

    /// Reports whether any entry in the tree will be quoted.
    ///
    /// # Parameters
    /// - `node`: The node to inspect, along with its descendants.
    fn any_quotable(node: &TreeNode) -> bool {
        quotes::is_quotable(node.entry.name()) || node.children.iter().any(Self::any_quotable)
    }

    /// Recursively renders every node into a printable row.
    ///
    /// # Parameters
    /// - `node`: The current node to render.
    /// - `parents_last`: Flags indicating whether each ancestor is the last child.
    /// - `columns`: The columns to display.
    /// - `args`: Command-line arguments controlling display options.
    /// - `add_alignment_space`: Whether to add a space for quote-alignment.
    /// - `rows`: Accumulator for the rendered rows, in print order.
    fn collect_rows(
        node: &TreeNode,
        parents_last: &mut Vec<bool>,
        columns: &[Column],
        args: &Args,
        add_alignment_space: bool,
        rows: &mut Vec<TreeRow>,
    ) {
        let entry_view = StyledEntry::new(&node.entry).load(args, false);

        rows.push(TreeRow {
            cells: column::render_row(&node.entry, columns, args, add_alignment_space),
            connector: ElementStyle::tree_connector(&Self::draw_connector(parents_last)),
            name: ValueStyle::name(&entry_view.name, entry_view.color),
        });

        let count = node.children.len();
        for (i, child) in node.children.iter().enumerate() {
            parents_last.push(i == count - 1);
            Self::collect_rows(
                child,
                parents_last,
                columns,
                args,
                add_alignment_space,
                rows,
            );
            parents_last.pop();
        }
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
                connector.push_str(if last { BLANK } else { LINE });
            }
            connector.push_str(if parents_last[depth - 1] { CORNER } else { EDGE });
        }
        connector
    }
}
