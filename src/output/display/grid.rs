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
use crate::output::layout::alignment::Alignment;
use crate::output::layout::column::Column;
use crate::output::layout::term_grid::{Cell, Direction, Filling, GridOptions, TermGrid};
use crate::output::layout::width::Width;
use crate::output::quotes::Quotes;
use crate::output::styles::column::ColumnStyle;

impl DisplayMode for Grid {
    /// Prints the grid output, either recursively or non-recursively based on args.
    ///
    /// # Behavior
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
}

/// A grid-based renderer that displays filesystem entries in a multi-column layout.
///
/// `Grid` arranges entries into columns that fit within the terminal width.
/// The layout is optimised to use available horizontal space efficiently while maintaining readability.
///
/// # Layout Strategy
///
/// * **Direction**: Top-to-bottom filling (entries flow down columns, then across)
/// * **Spacing**: 2 spaces between columns
/// * **Width calculation**: Measures actual rendered width including ANSI codes
/// * **Fitting algorithm**: Maximises column count while respecting terminal width
///
/// # Examples
///
/// ```rust
/// let grid = Grid::new(entries, args);
/// grid.print(); // Outputs multi-column layout
/// ```
pub(crate) struct Grid {
    /// The filesystem entries to display
    entries: Vec<Entry>,
    /// Command-line arguments controlling display options
    args: Args,
}

impl Grid {
    /// Creates a new `Grid` renderer with the given entries and arguments.
    ///
    /// # Parameters
    ///
    /// * `entries` - The filesystem entries to display in the grid
    /// * `args` - Command-line arguments that control formatting, colours, icons, etc.
    ///
    /// # Returns
    ///
    /// A new `Grid` instance ready to render
    ///
    /// # Examples
    ///
    /// ```rust
    /// let entries = directory.list(&args);
    /// let grid = Grid::new(entries, args);
    /// grid.print();
    /// ```
    pub(crate) fn new(entries: Vec<Entry>, args: Args) -> Self {
        Self { entries, args }
    }

    /// Displays entries in a non-recursive grid layout fitted to the terminal width.
    ///
    /// This method creates a compact multi-column layout where entries are arranged
    /// to maximise the use of available terminal width while maintaining consistent
    /// spacing and alignment.
    ///
    /// # Parameters
    ///
    /// * `entries` - The entries to display
    ///
    /// # Behavior
    ///
    /// 1. Returns early if entries list is empty
    /// 2. Queries current terminal width
    /// 3. Converts entries to `Cell` objects with styled content and measured widths
    /// 4. Creates a grid with 2-space column separation and top-to-bottom filling
    /// 5. Uses `fit_grid` to find optimal column layout for terminal width
    ///
    /// # Grid Configuration
    ///
    /// * **Filling**: 2 spaces between columns
    /// * **Direction**: TopToBottom (entries fill columns vertically before moving right)
    /// * **Alignment**: Left-aligned entries
    fn nonrecursive(&self, entries: &[Entry]) {
        if entries.is_empty() {
            return;
        }

        let terminal_width = match self.args.width {
            None => Width::terminal_width(),
            Some(0) => usize::MAX, // 0 means no limit
            Some(w) => w,
        };

        // Add an alignment space in any entries in have got special characters and will get quoted
        let add_alignment_space = entries
            .iter()
            .any(|entry| Quotes::is_quotable(entry.name()));

        // Convert entries into term_grid Cells
        let cells: Vec<Cell> = entries
            .iter()
            .map(|entry| {
                let styled_column =
                    ColumnStyle::get(entry, &Column::Name, &self.args, add_alignment_space);
                let entry_width = Width::measure_ansi_text(&styled_column);
                Cell {
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

    /// Attempts to print a `TermGrid` so that it fits cleanly within the
    /// available terminal width.
    ///
    /// This function implements an intelligent fitting algorithm that maximises
    /// the number of columns while ensuring all content fits within the terminal.
    ///
    /// # Parameters
    ///
    /// * `grid` - The fully populated `TermGrid` to be rendered
    /// * `terminal_width` - The visible width of the terminal in characters
    /// * `entries_length` - The number of entries being displayed; used to
    ///   determine the maximum possible column count
    ///
    /// # Algorithm
    ///
    /// The function uses a two-phase approach:
    ///
    /// 1. **Fast path**: Attempts `fit_into_width()` which lets term_grid
    ///    compute an ideal layout automatically. If successful, uses that layout.
    ///
    /// 2. **Fallback path**: If the fast path fails, uses binary search to find
    ///    the maximum number of columns that fits within terminal width.
    ///
    /// # Output
    ///
    /// Prints the grid directly to stdout using the optimal layout found.
    ///
    /// # Performance
    ///
    /// * Fast path: O(1) - immediate if term_grid's algorithm succeeds
    /// * Fallback: O(log n) - binary search for optimal column count
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
                .map(|line| Width::measure_ansi_text(line))
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
