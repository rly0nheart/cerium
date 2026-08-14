// SPDX-License-Identifier: MIT

use crate::display::layout::alignment::Alignment;
use crate::display::layout::term_grid::cell::Cell;
use crate::display::layout::term_grid::direction::Direction;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Spaces printed between columns.
const SEPARATOR_WIDTH: usize = 2;

/// A grid layout calculator for arranging cells in columns.
#[derive(Debug, Clone)]
pub struct TermGrid {
    cells: Vec<Cell>,
    direction: Direction,
}

impl TermGrid {
    /// Creates an empty grid that fills in the given direction.
    ///
    /// # Parameters
    /// - `direction`: Whether cells run down columns or across rows.
    pub fn new(direction: Direction) -> Self {
        Self {
            cells: Vec::new(),
            direction,
        }
    }

    /// Adds a cell to the grid.
    ///
    /// # Parameters
    /// - `cell`: The cell to append.
    pub fn add(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    /// Fits the grid into the widest column count that stays within `width`.
    ///
    /// # Parameters
    /// - `width`: The available terminal width in columns.
    pub fn fit_into_width(&self, width: usize) -> GridDisplay<'_> {
        // Start with single-column as fallback (always valid)
        let mut best = self.fit_into_columns(1);

        // Try increasing column counts until it doesn't fit
        for num_cols in 2..=self.cells.len() {
            let display = self.fit_into_columns(num_cols);

            if display.total_width() > width {
                // Once we exceed width, stop (more columns will be wider)
                break;
            }

            best = display;
        }

        best
    }

    /// Fits the grid into exactly the specified number of columns.
    ///
    /// # Parameters
    /// - `num_columns`: The desired column count (clamped to at least 1).
    pub fn fit_into_columns(&self, num_columns: usize) -> GridDisplay<'_> {
        if self.cells.is_empty() {
            return GridDisplay {
                cells: &self.cells,
                column_widths: Vec::new(),
                num_columns: 0,
                direction: self.direction,
                separator_width: SEPARATOR_WIDTH,
            };
        }

        let num_columns = num_columns.max(1);
        let num_rows = self.cells.len().div_ceil(num_columns);

        GridDisplay {
            cells: &self.cells,
            column_widths: self.calculate_column_widths(num_columns, num_rows),
            num_columns,
            direction: self.direction,
            separator_width: SEPARATOR_WIDTH,
        }
    }

    /// Calculates the width of each column.
    ///
    /// # Parameters
    /// - `num_columns`: Number of columns in the layout.
    /// - `num_rows`: Number of rows in the layout.
    fn calculate_column_widths(&self, num_columns: usize, num_rows: usize) -> Vec<usize> {
        let mut widths = vec![0usize; num_columns];

        for (index, cell) in self.cells.iter().enumerate() {
            let col = match self.direction {
                Direction::TopToBottom => index / num_rows,
                Direction::LeftToRight => index % num_columns,
            };

            if col < num_columns {
                widths[col] = widths[col].max(cell.width);
            }
        }

        widths
    }
}

/// A computed grid layout ready for display, borrowing the grid's cells.
#[derive(Debug, Clone)]
pub struct GridDisplay<'a> {
    cells: &'a [Cell],
    column_widths: Vec<usize>,
    num_columns: usize,
    direction: Direction,
    separator_width: usize,
}

impl GridDisplay<'_> {
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
    ///
    /// # Parameters
    /// - `row`: The row index.
    /// - `col`: The column index.
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
    ///
    /// # Parameters
    /// - `cell`: The cell to pad.
    /// - `width`: The target column width.
    fn pad_cell(&self, cell: &Cell, width: usize) -> String {
        let padding = width.saturating_sub(cell.width);

        match cell.alignment {
            Alignment::Left => format!("{}{}", cell.contents, " ".repeat(padding)),
            Alignment::Right => format!("{}{}", " ".repeat(padding), cell.contents),
        }
    }
}

impl Display for GridDisplay<'_> {
    /// Writes the grid layout to the formatter, one row per line.
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
