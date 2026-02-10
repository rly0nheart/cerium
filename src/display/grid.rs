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
use crate::display::layout::alignment::Alignment;
use crate::display::layout::column::Column;
use crate::display::layout::term_grid::{
    Cell as GridCell, Direction, Filling, GridOptions, TermGrid,
};
use crate::display::layout::width::Width;
use crate::display::mode::DisplayMode;
use crate::display::output::quotes::Quotes;
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
            None => Width::terminal_width(),
            Some(0) => usize::MAX, // 0 means no limit
            Some(w) => w,
        };

        // Add an alignment space in any entries that have got special characters (quotable)
        let add_alignment_space = entries
            .iter()
            .any(|entry| Quotes::is_quotable(entry.name()));

        // Convert entries into term_grid Cells
        let cells: Vec<GridCell> = entries
            .iter()
            .map(|entry| {
                let styled_column =
                    ColumnStyle::get(entry, &Column::Name, &self.args, add_alignment_space);
                let entry_width = Width::measure_ansi_text(&styled_column);
                GridCell {
                    width: entry_width,
                    contents: styled_column,
                    alignment: Alignment::Left,
                }
            })
            .collect();

        // Create the grid
        let mut grid = TermGrid::new(GridOptions {
            filling: Filling::Spaces(2),
            direction: Direction::TopToBottom, // column-first layout
        });

        for cell in &cells {
            grid.add(cell.clone());
        }

        Self::fit_grid(grid, terminal_width, entries.len())
    }

    /// Fits the grid into the terminal width and prints it.
    ///
    /// # Parameters
    /// - `grid`: The fully populated [`TermGrid`] to print.
    /// - `terminal_width`: The visible width of the terminal in characters.
    /// - `entries_length`: The number of entries (caps the column count).
    fn fit_grid(grid: TermGrid, terminal_width: usize, entries_length: usize) {
        // Try the easy fit first
        if let Some(fit) = grid.fit_into_width(terminal_width) {
            print!("{fit}");
            return;
        }

        // Fallback: binary search for maximum columns that fit
        let mut low = 1usize;
        let mut high = entries_length.max(1);
        let mut best_fit = None;

        while low <= high {
            let mid = low + (high - low) / 2;
            let fitted = grid.fit_into_columns(mid);
            let max_line_width = fitted
                .to_string()
                .lines()
                .map(Width::measure_ansi_text)
                .max()
                .unwrap_or(0);

            if max_line_width <= terminal_width {
                // This fits, try more columns
                best_fit = Some(fitted);
                low = mid + 1;
            } else {
                // Too wide, try fewer columns
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
        }

        // Print best fit or fall back to single column
        if let Some(best) = best_fit {
            print!("{best}");
        } else {
            let single = grid.fit_into_columns(1);
            print!("{single}");
        }
    }
}
