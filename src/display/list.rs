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
use crate::display::traversal::RecursiveTraversal;
use crate::fs::entry::Entry;
use std::collections::HashMap;

impl DisplayMode for List {
    /// Prints the table output, either recursively or non-recursively based on args.
    ///
    /// # Behaviour
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

/// Tabular renderer that shows filesystem entries in aligned columns.
pub(crate) struct List {
    /// The filesystem entries to display
    entries: Vec<Entry>,
    /// Command-line arguments controlling display options
    args: Args,
}

impl List {
    /// Creates a new [`List`] renderer.
    ///
    /// # Parameters
    /// - `entries`: The filesystem entries to display.
    /// - `args`: Command-line arguments controlling columns and formatting.
    pub(crate) fn new(entries: Vec<Entry>, args: Args) -> Self {
        Self { entries, args }
    }

    /// Displays entries in a single, non-recursive table with aligned columns.
    ///
    /// # Parameters
    /// - `entries`: The entries to display.
    /// - `args`: Command-line arguments controlling column selection and formatting.
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

        if args.headers {
            Column::headers(&widths, args);
        }

        for entry in entries {
            Self::render_row(entry, &widths, &columns, args, add_alignment_space);
        }
    }

    /// Renders a single row in list format with styled and aligned columns.
    ///
    /// # Parameters
    /// - `entry`: The entry to render.
    /// - `widths`: Pre-calculated column widths for alignment.
    /// - `columns`: The columns to display.
    /// - `args`: Command-line arguments controlling display options.
    /// - `add_alignment_space`: Whether to add a space for quote-alignment.
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
