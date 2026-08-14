// SPDX-License-Identifier: MIT

use crate::cli::args::Args;
use crate::display::layout::column;
use crate::display::mode::DisplayMode;
use crate::display::output::quotes;
use crate::display::summary;
use crate::display::summary::Summary;
use crate::display::traversal::RecursiveTraversal;
use crate::fs::entry::Entry;
use std::cell::Cell;

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

        self.print_summary();
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

    fn dir_count(&self) -> &Cell<usize> {
        &self.dir_count
    }

    fn file_count(&self) -> &Cell<usize> {
        &self.file_count
    }
}

/// Tabular renderer that shows filesystem entries in aligned columns.
pub(crate) struct List {
    /// The filesystem entries to display
    entries: Vec<Entry>,
    /// Command-line arguments controlling display options
    args: Args,
    /// Accumulated directory count during recursive traversal
    dir_count: Cell<usize>,
    /// Accumulated file count during recursive traversal
    file_count: Cell<usize>,
}

impl Summary for List {
    /// Returns directory and file counts for List view.
    ///
    /// In recursive mode, returns counts accumulated during traversal.
    /// In non-recursive mode, counts the flat entry slice.
    fn counts(&self) -> (usize, usize) {
        if self.args.recursive {
            (self.dir_count.get(), self.file_count.get())
        } else {
            summary::count_entries(&self.entries)
        }
    }
}

impl List {
    /// Creates a new [`List`] renderer.
    ///
    /// # Parameters
    /// - `entries`: The filesystem entries to display.
    /// - `args`: Command-line arguments controlling columns and formatting.
    pub(crate) fn new(entries: Vec<Entry>, args: Args) -> Self {
        Self {
            entries,
            args,
            dir_count: Cell::new(0),
            file_count: Cell::new(0),
        }
    }

    /// Displays entries in a single, non-recursive table with aligned columns.
    ///
    /// Each cell is rendered once, up front: the column widths are measured from
    /// the very strings that get printed.
    ///
    /// # Parameters
    /// - `entries`: The entries to display.
    /// - `args`: Command-line arguments controlling column selection and formatting.
    fn nonrecursive(entries: &[Entry], args: &Args) {
        if entries.is_empty() {
            return;
        }

        let columns = column::Selector::select(args);

        // Add an alignment space in any entries in have got special characters and will get quoted
        let add_alignment_space = entries
            .iter()
            .any(|entry| quotes::is_quotable(entry.name()));

        let rows = column::render_rows(entries, &columns, args, add_alignment_space);
        let widths = column::widths(&columns, rows.iter().map(Vec::as_slice), args.headers);

        let mut out = String::new();
        if args.headers {
            column::write_headers(&mut out, &columns, &widths);
        }
        for row in &rows {
            column::write_row(&mut out, row, &columns, &widths);
        }

        print!("{out}");
    }
}
