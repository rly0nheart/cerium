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
use crate::fs::dir::DirReader;
use crate::fs::entry::Entry;
use crate::fs::tree::TreeNode;
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
                Self::traverse_and_print(parent_entry, &Vec::new(), &self.args);
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

                if self.args.column_headers {
                    Column::headers(&widths, &self.args);
                }

                Self::add_node(node, &widths, &Vec::new(), &self.args, add_alignment_space);
            }
        }
    }
}

/// A tree-structured renderer that displays filesystem entries in a hierarchical format.
///
/// `Tree` presents directory contents as a visual tree using Unicode box-drawing
/// characters to show parent-child relationships, similar to the `tree` command.
/// Entries can also include additional column data (size, permissions, etc.) when
/// combined with table mode.
///
/// # Visual Format
///
/// ```text
/// root/
/// ├── file1.txt
/// ├── subdir/
/// │   ├── file2.txt
/// │   ╰── file3.txt
/// ╰── file4.txt
/// ```
///
/// # Features
///
/// * **Hierarchical display**: Shows directory structure with visual connectors
/// * **Column support**: Can include table columns alongside tree structure
/// * **Headers**: Optional column headers
/// * **Recursive traversal**: Automatically descends into subdirectories
/// * **Streaming mode**: Prints entries as discovered for instant feedback
///
/// # Box-Drawing Characters
///
/// * `│` - Vertical line (continues parent connection)
/// * `├─` - Branch connector (more children follow)
/// * `╰─` - Corner connector (last child)
///
/// # Examples
///
/// ```rust
/// // Table mode (pre-built tree)
/// let tree = Tree::new_table(root_node, args);
/// tree.print();
///
/// // Streaming mode (on-demand traversal)
/// let tree = Tree::new_streaming(root_path, args);
/// tree.print();
/// ```
pub(crate) enum TreeData {
    /// Pre-built tree structure for table mode with columns
    Table(TreeNode),
    /// Root path for streaming mode without columns
    Streaming(PathBuf),
}

pub(crate) struct Tree {
    /// The tree data source (pre-built or path for streaming)
    data: TreeData,
    /// Command-line arguments controlling display options
    args: Args,
}

impl Tree {
    /// Creates a new `Tree` renderer in table mode with a pre-built tree structure.
    ///
    /// Table mode is used when metadata columns are requested and requires width
    /// calculations across all entries before rendering.
    ///
    /// # Parameters
    ///
    /// * `node` - The root node of the directory tree to render
    /// * `args` - Command-line arguments controlling columns, colours, headers, etc.
    ///
    /// # Returns
    ///
    /// A new `Tree` instance ready to render in table mode
    pub(crate) fn new_table(node: TreeNode, args: Args) -> Self {
        Self {
            data: TreeData::Table(node),
            args,
        }
    }

    /// Creates a new `Tree` renderer in streaming mode for on-demand traversal.
    ///
    /// Streaming mode traverses and prints the filesystem tree on-demand, providing
    /// instant output for large directory structures. Used when no metadata columns
    /// are requested.
    ///
    /// # Parameters
    ///
    /// * `path` - The root path to traverse
    /// * `args` - Command-line arguments controlling display options
    ///
    /// # Returns
    ///
    /// A new `Tree` instance ready to render in streaming mode
    pub(crate) fn new_streaming(path: PathBuf, args: Args) -> Self {
        Self {
            data: TreeData::Streaming(path),
            args,
        }
    }

    /// Determines if the tree requires table layout with column width calculations.
    ///
    /// Table layout is needed when any metadata columns or table-specific columns are
    /// requested. Without these, the tree can stream output immediately as entries
    /// are discovered, providing instant feedback for large directory trees.
    ///
    /// # Columns requiring table layout:
    ///
    /// **Metadata columns:**
    /// - `--long`, `--size`, `--permission`, `--user`, `--group`
    /// - `--created`, `--modified`, `--accessed`
    /// - `--inode`, `--blocks`, `--hard-links`, `--block-size`
    ///
    /// **Table-specific columns:**
    /// - `--magic` (file type detection)
    /// - `--hash` (file hash computation)
    /// - `--xattr` (extended attributes)
    /// - `--acl` (ACL indicator)
    /// - `--head` (first N bytes preview)
    /// - `--tail` (last N bytes preview)
    /// - `--oneline` (force single column)
    ///
    /// # Parameters
    ///
    /// * `args` - Command-line arguments to check
    ///
    /// # Returns
    ///
    /// `true` if table layout is required, `false` for streaming mode
    pub(crate) fn needs_table_layout(args: &Args) -> bool {
        // Metadata columns
        if args.long
            || args.size
            || args.permission
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
    /// This method performs on-demand filesystem traversal and prints entries
    /// immediately as they are discovered, providing instant feedback for large
    /// directory structures. Unlike the table mode which pre-builds the entire
    /// tree, this approach only traverses directories as needed.
    ///
    /// # Parameters
    ///
    /// * `entry` - The current entry to render
    /// * `parents_last` - Boolean flags indicating whether each ancestor is the last child
    /// * `args` - Command-line arguments controlling display options
    ///
    /// # Performance
    ///
    /// This is significantly faster for large trees because:
    /// - No upfront tree building
    /// - No width calculations across all entries
    /// - Output appears immediately as filesystem is traversed
    /// - Directories are only read when needed
    fn traverse_and_print(entry: Entry, parents_last: &Vec<bool>, args: &Args) {
        let connector = Self::draw_connector(parents_last);

        // Get styled entry for name display (no alignment space for tree)
        let styled_entry = StyledEntry::new(&entry);
        let entry_view = styled_entry.load(args, false);

        // Print: [connector] [name]
        println!(
            "{}{}",
            TextStyle::tree_connector(&connector),
            TextStyle::name(&entry_view.name, entry_view.colour),
        );

        // If this is a directory, traverse and print its children
        if entry.is_dir() {
            let dir_reader = DirReader::from(entry.path().clone());
            let children = dir_reader.list(args);

            let count = children.len();
            for (i, mut child_entry) in children.into_iter().enumerate() {
                child_entry.conditional_metadata(args);
                let mut new_parents = parents_last.clone();
                new_parents.push(i == count - 1);
                Self::traverse_and_print(child_entry, &new_parents, args);
            }
        }
    }

    /// Flattens a hierarchical tree structure into a linear vector of entries.
    ///
    /// This depth-first traversal collects all entries in the tree, which is
    /// necessary for calculating column widths that accommodate all entries
    /// before rendering begins.
    ///
    /// # Parameters
    ///
    /// * `node` - The root node to flatten
    /// * `entries` - Mutable vector to populate with entries (initially empty)
    ///
    /// # Traversal Order
    ///
    /// Depth-first, pre-order traversal:
    /// 1. Process current node
    /// 2. Recursively process all children
    fn flatten(node: &TreeNode, entries: &mut Vec<Entry>) {
        entries.push(node.entry.clone());
        for child in &node.children {
            Self::flatten(child, entries);
        }
    }

    /// Recursively renders a node and all its children with proper tree connectors.
    ///
    /// This is the core rendering function that traverses the tree structure and
    /// generates the visual output with appropriate indentation and connector symbols.
    ///
    /// # Parameters
    ///
    /// * `node` - The current node to render
    /// * `widths` - Pre-calculated column widths for alignment
    /// * `parents_last` - Boolean flags indicating whether each ancestor in the path
    ///   is the last child of its parent. Used to determine connector style.
    /// * `args` - Command-line arguments controlling display options
    ///
    /// # Connector Logic
    ///
    /// The `parents_last` vector tracks the tree path:
    /// * `[false]` → Current node has siblings below it → use `├──`
    /// * `[true]` → Current node is the last child → use `╰──`
    /// * `[false, false]` → Parent has siblings, current has siblings → `│   ├──`
    /// * `[false, true]` → Parent has siblings, current is last → `│   ╰──`
    /// * `[true, false]` → Parent is last, current has siblings → `    ├──`
    ///
    /// # Recursion
    ///
    /// For each child, the function:
    /// 1. Clones the current `parents_last` state
    /// 2. Appends whether the child is the last sibling
    /// 3. Recursively calls itself with the updated state
    fn add_node(
        node: &TreeNode,
        widths: &HashMap<Column, usize>,
        parents_last: &Vec<bool>,
        args: &Args,
        add_alignment_space: bool,
    ) {
        let entry = &node.entry;
        let connector = Self::draw_connector(parents_last);

        // Render the row with tree connectors
        Self::render_tree_row(entry, widths, &connector, args, add_alignment_space);

        let count = node.children.len();
        for (i, child) in node.children.iter().enumerate() {
            let mut new_parents = parents_last.clone();
            new_parents.push(i == count - 1);
            Self::add_node(child, widths, &new_parents, args, add_alignment_space);
        }
    }

    /// Renders a single row in tree mode with connectors and column data.
    ///
    /// This method combines tabular column data with tree visualization,
    /// producing output that shows both hierarchical structure and detailed
    /// entry information.
    ///
    /// # Parameters
    ///
    /// * `entry` - The entry to render
    /// * `widths` - Pre-calculated column widths for alignment
    /// * `connector` - Tree connector string (e.g., "├── ", "│   ╰── ")
    /// * `args` - Command-line arguments controlling display options
    /// * `add_alignment_space` - Whether any entries in this batch need quoting
    ///
    /// # Output Format
    ///
    /// ```text
    /// 1.2 KB  2025-01-15  ├── file1.txt
    /// 45 KB   2025-01-14  │   ╰── subfile.txt
    /// ```
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

    /// Builds the connector string with proper box-drawing characters for a tree node.
    ///
    /// This function constructs the visual prefix that appears before each entry name,
    /// using Unicode box-drawing characters to represent the hierarchical structure.
    ///
    /// # Parameters
    ///
    /// * `parents_last` - A vector of boolean flags where each element indicates
    ///   whether the corresponding ancestor in the path is the last child of its parent
    ///
    /// # Returns
    ///
    /// A string containing the appropriate combination of vertical lines, spaces,
    /// and branch connectors to visually represent the node's position in the tree
    ///
    /// # Construction Logic
    ///
    /// 1. For each ancestor (except the immediate parent):
    ///    - If ancestor is last child: add 4 spaces (no vertical line needed)
    ///    - If ancestor has siblings: add `│   ` (vertical line continues)
    /// 2. For the immediate parent:
    ///    - If current node is last child: add `╰── ` (corner connector)
    ///    - If current node has siblings: add `├── ` (branch connector)
    ///
    /// # Visual Reference
    ///
    /// ```text
    /// root/                    depth: 0, connector: ""
    /// ├── file1               depth: 1, parents_last: [false], connector: "├── "
    /// ├── dir1/               depth: 1, parents_last: [false], connector: "├── "
    /// │   ├── file2           depth: 2, parents_last: [false, false], connector: "│   ├── "
    /// │   ╰── file3           depth: 2, parents_last: [false, true], connector: "│   ╰── "
    /// ╰── file4               depth: 1, parents_last: [true], connector: "╰── "
    /// ```
    fn draw_connector(parents_last: &Vec<bool>) -> String {
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
