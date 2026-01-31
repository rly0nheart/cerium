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

use crate::display::layout::alignment::Alignment;
use crate::display::layout::term_grid::cell::Cell;
use crate::display::layout::term_grid::options::{Direction, Filling, GridOptions};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A grid layout calculator for arranging cells in columns.
#[derive(Debug, Clone)]
pub(crate) struct TermGrid {
    cells: Vec<Cell>,
    options: GridOptions,
}

impl TermGrid {
    /// Creates a new empty grid with the given options.
    pub(crate) fn new(options: GridOptions) -> Self {
        Self {
            cells: Vec::new(),
            options,
        }
    }

    /// Adds a cell to the grid.
    pub(crate) fn add(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    /// Attempts to fit the grid into the given terminal width.
    ///
    /// Returns `Some(GridDisplay)` with the best-fitting layout (maximising columns),
    /// or `None` only if there are no cells.
    pub(crate) fn fit_into_width(&self, width: usize) -> Option<GridDisplay> {
        if self.cells.is_empty() {
            return Some(GridDisplay {
                cells: Vec::new(),
                column_widths: Vec::new(),
                num_columns: 0,
                direction: self.options.direction,
                separator_width: self.separator_width(),
            });
        }

        // Start with single-column as fallback (always valid)
        let mut best = self.fit_into_columns(1);

        // Try increasing column counts until it doesn't fit
        for num_cols in 2..=self.cells.len() {
            let display = self.fit_into_columns(num_cols);
            let total_width = display.total_width();

            if total_width <= width {
                best = display;
            } else {
                // Once we exceed width, stop (more columns will be wider)
                break;
            }
        }

        Some(best)
    }

    /// Fits the grid into exactly the specified number of columns.
    pub(crate) fn fit_into_columns(&self, num_columns: usize) -> GridDisplay {
        let num_columns = num_columns.max(1);

        if self.cells.is_empty() {
            return GridDisplay {
                cells: self.cells.clone(),
                column_widths: Vec::new(),
                num_columns: 0,
                direction: self.options.direction,
                separator_width: self.separator_width(),
            };
        }

        // Calculate number of rows needed
        let num_rows = self.cells.len().div_ceil(num_columns);

        // Calculate column widths based on direction
        let column_widths = self.calculate_column_widths(num_columns, num_rows);

        GridDisplay {
            cells: self.cells.clone(),
            column_widths,
            num_columns,
            direction: self.options.direction,
            separator_width: self.separator_width(),
        }
    }

    /// Calculates the width of each column.
    fn calculate_column_widths(&self, num_columns: usize, num_rows: usize) -> Vec<usize> {
        let mut widths = vec![0usize; num_columns];

        for (index, cell) in self.cells.iter().enumerate() {
            let col = match self.options.direction {
                Direction::TopToBottom => index / num_rows,
                Direction::LeftToRight => index % num_columns,
            };

            if col < num_columns {
                widths[col] = widths[col].max(cell.width);
            }
        }

        widths
    }

    /// Returns the width of the separator between columns.
    fn separator_width(&self) -> usize {
        match self.options.filling {
            Filling::Spaces(n) => n,
        }
    }
}

/// A computed grid layout ready for display.
#[derive(Debug, Clone)]
pub(crate) struct GridDisplay {
    cells: Vec<Cell>,
    column_widths: Vec<usize>,
    num_columns: usize,
    direction: Direction,
    separator_width: usize,
}

impl GridDisplay {
    /// Returns the total width of the grid including separators.
    fn total_width(&self) -> usize {
        if self.column_widths.is_empty() {
            return 0;
        }

        let content_width: usize = self.column_widths.iter().sum();
        let separator_total = self.separator_width * (self.column_widths.len().saturating_sub(1));

        content_width + separator_total
    }

    /// Returns the cell at the given row and column position, if any.
    fn cell_at(&self, row: usize, col: usize) -> Option<&Cell> {
        let num_rows = if self.num_columns > 0 {
            self.cells.len().div_ceil(self.num_columns)
        } else {
            return None;
        };

        let index = match self.direction {
            Direction::TopToBottom => col * num_rows + row,
            Direction::LeftToRight => row * self.num_columns + col,
        };

        self.cells.get(index)
    }

    /// Pads the cell content to the specified width based on alignment.
    fn pad_cell(&self, cell: &Cell, width: usize) -> String {
        let padding = width.saturating_sub(cell.width);

        match cell.alignment {
            Alignment::Left => format!("{}{}", cell.contents, " ".repeat(padding)),
            Alignment::Right => format!("{}{}", " ".repeat(padding), cell.contents),
        }
    }
}

impl Display for GridDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.cells.is_empty() || self.num_columns == 0 {
            return Ok(());
        }

        let num_rows = self.cells.len().div_ceil(self.num_columns);
        let separator = " ".repeat(self.separator_width);

        for row in 0..num_rows {
            let mut line_parts: Vec<String> = Vec::new();

            for col in 0..self.num_columns {
                if let Some(cell) = self.cell_at(row, col) {
                    let width = self.column_widths.get(col).copied().unwrap_or(0);

                    // Don't pad the last column on the row
                    let is_last_cell =
                        col == self.num_columns - 1 || self.cell_at(row, col + 1).is_none();

                    if is_last_cell {
                        line_parts.push(cell.contents.clone());
                    } else {
                        line_parts.push(self.pad_cell(cell, width));
                    }
                }
            }

            // Join with separator and trim trailing whitespace
            let line = line_parts.join(&separator);
            writeln!(f, "{}", line.trim_end())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cell(contents: &str) -> Cell {
        Cell {
            width: contents.len(),
            contents: contents.to_string(),
            alignment: Alignment::Left,
        }
    }

    #[test]
    fn test_empty_grid() {
        let grid = TermGrid::new(GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
        });

        let display = grid.fit_into_width(80);
        assert!(display.is_some());
        assert_eq!(display.unwrap().to_string(), "");
    }

    #[test]
    fn test_single_cell() {
        let mut grid = TermGrid::new(GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
        });

        grid.add(make_cell("hello"));

        let display = grid.fit_into_width(80).unwrap();
        assert_eq!(display.to_string(), "hello\n");
    }

    #[test]
    fn test_multiple_cells_top_to_bottom() {
        let mut grid = TermGrid::new(GridOptions {
            direction: Direction::TopToBottom,
            filling: Filling::Spaces(2),
        });

        grid.add(make_cell("a"));
        grid.add(make_cell("b"));
        grid.add(make_cell("c"));
        grid.add(make_cell("d"));

        let display = grid.fit_into_columns(2);
        let output = display.to_string();

        // With TopToBottom and 2 columns, 4 cells:
        // Layout: col0=[a,b], col1=[c,d]
        // Row 0: a, c
        // Row 1: b, d
        assert!(output.contains("a"));
        assert!(output.contains("b"));
        assert!(output.contains("c"));
        assert!(output.contains("d"));
    }

    #[test]
    fn test_fit_into_width() {
        let mut grid = TermGrid::new(GridOptions {
            direction: Direction::LeftToRight,
            filling: Filling::Spaces(2),
        });

        // Add cells with varying widths
        grid.add(make_cell("short"));
        grid.add(make_cell("medium_len"));
        grid.add(make_cell("x"));

        // With width 80, should fit multiple columns
        let display = grid.fit_into_width(80);
        assert!(display.is_some());

        // With width 5, should only fit 1 column
        let display = grid.fit_into_width(5);
        assert!(display.is_some());
        let output = display.unwrap().to_string();
        // Each cell should be on its own line
        assert_eq!(output.lines().count(), 3);
    }
}
