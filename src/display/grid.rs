// SPDX-License-Identifier: MIT

use crate::cli::args::Args;
use crate::display::layout::alignment::Alignment;
use crate::display::layout::column::Column;
use crate::display::layout::term_grid::{Cell as GridCell, Direction, TermGrid};
use crate::display::layout::width;
use crate::display::mode::DisplayMode;
use crate::display::output::quotes;
use crate::display::styles::column::ColumnStyle;
use crate::display::summary;
use crate::display::summary::Summary;
use crate::display::traversal::RecursiveTraversal;
use crate::fs::entry::Entry;
use std::cell::Cell;

impl DisplayMode for Grid {
    /// Prints the grid output, either recursively or non-recursively based on args.
    ///
    /// # Behaviour
    ///
    /// * If `args.recursive` is true, displays entries in a hierarchical format
    ///   with directory titles, recursing into subdirectories
    /// * Otherwise, displays entries in a compact grid layout
    fn print(&self) {
        if self.args.recursive {
            self.render_recursive(&self.entries, None);
        } else {
            self.nonrecursive(&self.entries);
        }

        self.print_summary();
    }
}

impl RecursiveTraversal for Grid {
    /// Renders entries at a single directory level in grid format.
    ///
    /// This implementation delegates to the existing `nonrecursive()` method
    /// which handles grid layout calculation and multi-column rendering.
    fn render_level(&self, entries: &[Entry], _args: &Args) {
        self.nonrecursive(entries);
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

/// Multi-column renderer that arranges entries to fit the terminal width.
pub(crate) struct Grid {
    /// The filesystem entries to display
    entries: Vec<Entry>,
    /// Command-line arguments controlling display options
    args: Args,
    /// Accumulated directory count during recursive traversal
    dir_count: Cell<usize>,
    /// Accumulated file count during recursive traversal
    file_count: Cell<usize>,
}

impl Summary for Grid {
    /// Returns directory and file counts for Grid view.
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

impl Grid {
    /// Creates a new [`Grid`] renderer.
    ///
    /// # Parameters
    /// - `entries`: The filesystem entries to display.
    /// - `args`: Command-line arguments controlling formatting.
    pub(crate) fn new(entries: Vec<Entry>, args: Args) -> Self {
        Self {
            entries,
            args,
            dir_count: Cell::new(0),
            file_count: Cell::new(0),
        }
    }

    /// Displays entries in a non-recursive grid layout fitted to the terminal width.
    ///
    /// # Parameters
    /// - `entries`: The entries to display.
    fn nonrecursive(&self, entries: &[Entry]) {
        if entries.is_empty() {
            return;
        }

        let terminal_width = match self.args.width {
            None => width::terminal(),
            Some(0) => usize::MAX, // 0 means no limit
            Some(w) => w,
        };

        // Add an alignment space in any entries that have got special characters (quotable)
        let add_alignment_space = entries
            .iter()
            .any(|entry| quotes::is_quotable(entry.name()));

        // Create the grid
        // Column-first, like ls
        let mut grid = TermGrid::new(Direction::TopToBottom);

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
}
