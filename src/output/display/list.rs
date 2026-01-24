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
use crate::fs::entry::Entry;
use crate::output::display::mode::DisplayMode;
use crate::output::display::traversal::RecursiveTraversal;
use crate::output::layout::alignment::Align;
use crate::output::layout::column;
use crate::output::layout::column::Column;
use crate::output::layout::width::Width;
use crate::output::quotes::Quotes;
use crate::output::styles::column::ColumnStyle;
use std::collections::HashMap;

impl DisplayMode for List {
    /// Prints the table output, either recursively or non-recursively based on args.
    ///
    /// # Behavior
    ///
    /// * If `args.recursive` is true, displays entries in a hierarchical format
    ///   with directory titles, recursing into subdirectories
    /// * Otherwise, displays a single table with properly aligned columns
    fn print(&self) {
        if self.args.recursive {
            self.render_recursive(&self.entries, None);
        } else {
            Self::nonrecursive(&self.entries, &self.args);
        }
    }
}

impl RecursiveTraversal for List {
    /// Renders entries at a single directory level in list format.
    ///
    /// This implementation delegates to the existing `nonrecursive()` method
    /// which handles column width calculation and formatted table output.
    fn render_level(&self, entries: &[Entry], args: &Args) {
        Self::nonrecursive(entries, args);
    }

    /// Returns a reference to the Args for this renderer.
    fn get_args(&self) -> &Args {
        &self.args
    }
}

/// A tabular display mode that shows filesystem entries in aligned columns with headers.
///
/// `List` formats entries in a structured table layout with properly aligned columns,
/// similar to `ls -l`. Each entry occupies one row, and columns are automatically
/// sized to accommodate the widest content while maintaining clean alignment.
///
/// # Features
///
/// * **Column alignment**: Automatically calculates optimal column widths
/// * **Headers**: Optional column headers (when `args.headers` is true)
/// * **Recursive mode**: Can display nested directory structures with section titles
/// * **Configurable columns**: Supports various column types (size, permissions, dates, etc.)
///
/// # Output Format
///
/// ```text
/// Name        Size    Modified
/// file1.txt   1.2 KB  2025-01-15
/// image.png   45 KB   2025-01-14
/// document    890 B   2025-01-13
/// ```
pub(crate) struct List {
    /// The filesystem entries to display
    entries: Vec<Entry>,
    /// Command-line arguments controlling display options
    args: Args,
}

impl List {
    /// Creates a new `List` display mode with the given entries and arguments.
    ///
    /// # Parameters
    ///
    /// * `entries` - The filesystem entries to display in the table
    /// * `args` - Command-line arguments that control columns, formatting, colours, etc.
    ///
    /// # Returns
    ///
    /// A new `List` instance ready to render
    ///
    /// # Examples
    ///
    /// ```rust
    /// let entries = directory.list(&args);
    /// let list = List::new(entries, args);
    /// list.print();
    /// ```
    pub(crate) fn new(entries: Vec<Entry>, args: Args) -> Self {
        Self { entries, args }
    }

    /// Displays entries in a single, non-recursive table with aligned columns.
    ///
    /// This method creates a formatted table where:
    /// 1. Column widths are calculated based on the widest content in each column
    /// 2. Optional headers are printed if enabled
    /// 3. Each entry is printed as a row with properly padded columns
    ///
    /// # Parameters
    ///
    /// * `entries` - The entries to display
    /// * `args` - Command-line arguments controlling column selection and formatting
    ///
    /// # Behavior
    ///
    /// * Returns early if the entries list is empty
    /// * Selects columns based on args (default or user-specified)
    /// * Calculates optimal width for each column
    /// * Prints column headers if `args.headers` is true
    /// * Prints each entry as a formatted row
    ///
    /// # Column Width Calculation
    ///
    /// Widths are determined by scanning all entries to find the maximum
    /// rendered width for each column, ensuring proper alignment without
    /// excessive whitespace.
    fn nonrecursive(entries: &[Entry], args: &Args) {
        if entries.is_empty() {
            return;
        }

        let columns = column::Selector::select(args);
        let mut width_calc = Width::new();
        let widths = width_calc.calculate(entries, &columns, args);

        // Add an alignment space in any entries in have got special characters and will get quoted
        let add_alignment_space = entries
            .iter()
            .any(|entry| Quotes::is_quotable(entry.name()));

        if args.column_headers {
            Column::headers(&widths, args);
        }

        for entry in entries {
            Self::render_row(entry, &widths, &columns, args, add_alignment_space);
        }
    }

    /// Renders a single row in list format with styled and aligned columns.
    ///
    /// This method handles all rendering logic for a single entry:
    /// 1. Builds column data via Row
    /// 2. Applies styling via ColumnStyle
    /// 3. Applies padding and alignment
    /// 4. Outputs the formatted row
    ///
    /// # Parameters
    ///
    /// * `entry` - The entry to render
    /// * `widths` - Pre-calculated column widths for alignment
    /// * `columns` - The columns to display
    /// * `args` - Command-line arguments controlling display options
    /// * `add_alignment_space` - Add an alignment space in any entries in have got special characters and will get quoted
    fn render_row(
        entry: &Entry,
        widths: &HashMap<Column, usize>,
        columns: &[Column],
        args: &Args,
        add_alignment_space: bool,
    ) {
        let mut parts = Vec::new();

        for column in columns {
            let styled_column = ColumnStyle::get(entry, column, args, add_alignment_space);
            let width = *widths
                .get(column)
                .unwrap_or(&Width::measure_ansi_text(&styled_column));
            let padded = Align::pad(&styled_column, width, column.alignment());
            parts.push(padded);
        }

        println!("{}", parts.join(" "));
    }
}
