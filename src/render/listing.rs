// SPDX-License-Identifier: MIT

use crate::cli::args::Args;
use crate::fs::entry::Entry;
use crate::render::Renderer;
use crate::render::grid::{GridCell, GridLayout};
use crate::render::layout::alignment::Alignment;
use crate::render::layout::column;
use crate::render::layout::column::Column;
use crate::render::layout::width;
use crate::render::output::quotes;
use crate::render::output::terminal;
use crate::render::style::column::ColumnStyle;
use crate::render::summary;
use crate::render::summary::Summary;
use crate::render::traversal::RecursiveTraversal;
use std::cell::Cell;

/// How a flat listing arranges its entries.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Mode {
    /// Names only, in as many columns as the terminal width allows.
    Grid,
    /// One entry per row, in aligned metadata columns.
    Long,
}

/// Renders a listing that is not a tree, in either mode.
pub(crate) struct Listing {
    /// The filesystem entries to display
    entries: Vec<Entry>,
    /// Command-line arguments controlling display options
    args: Args,
    /// Which arrangement to draw
    mode: Mode,
    /// Accumulated directory count during recursive traversal
    dir_count: Cell<usize>,
    /// Accumulated file count during recursive traversal
    file_count: Cell<usize>,
}

impl Renderer for Listing {
    /// Prints the listing, either recursively or non-recursively based on args.
    ///
    /// # Behaviour
    ///
    /// * If `args.recursive` is true, displays entries in a hierarchical format
    ///   with directory titles, recursing into subdirectories
    /// * Otherwise, displays a single level in the configured mode
    fn show(&self) {
        if self.args.recursive {
            self.render_recursive(&self.entries, None);
        } else {
            self.render_level(&self.entries, &self.args);
        }

        self.print_summary();
    }
}

impl RecursiveTraversal for Listing {
    /// Renders entries at a single directory level in the configured mode.
    fn render_level(&self, entries: &[Entry], args: &Args) {
        if entries.is_empty() {
            return;
        }

        match self.mode {
            Mode::Grid => self.grid(entries),
            Mode::Long => Self::list(entries, args),
        }
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

impl Summary for Listing {
    /// Returns directory and file counts.
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

impl Listing {
    /// Creates a new [`Listing`] renderer.
    ///
    /// # Parameters
    /// - `entries`: The filesystem entries to display.
    /// - `args`: Command-line arguments controlling formatting.
    /// - `mode`: Which arrangement to draw.
    pub(crate) fn new(entries: Vec<Entry>, args: Args, mode: Mode) -> Self {
        Self {
            entries,
            args,
            mode,
            dir_count: Cell::new(0),
            file_count: Cell::new(0),
        }
    }

    /// Draws entries as a multi-column grid fitted to the terminal width.
    ///
    /// # Parameters
    /// - `entries`: The entries to display.
    fn grid(&self, entries: &[Entry]) {
        let terminal_width = match self.args.width {
            None => terminal::width(),
            Some(0) => usize::MAX, // 0 means no limit
            Some(w) => w,
        };

        // Add an alignment space in any entries that have got special characters (quotable)
        let add_alignment_space = entries
            .iter()
            .any(|entry| quotes::is_quotable(entry.name()));

        let mut grid = GridLayout::new();

        for entry in entries {
            let contents = ColumnStyle::get(entry, &Column::Name, &self.args, add_alignment_space);
            grid.add(GridCell {
                width: width::measure(&contents),
                contents,
                alignment: Alignment::Left,
            });
        }

        print!("{}", grid.fit_into_width(terminal_width));
    }

    /// Draws entries as a table with aligned columns.
    ///
    /// Each cell is rendered once, up front: the column widths are measured from
    /// the very strings that get printed.
    ///
    /// # Parameters
    /// - `entries`: The entries to display.
    /// - `args`: Command-line arguments controlling column selection and formatting.
    fn list(entries: &[Entry], args: &Args) {
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
