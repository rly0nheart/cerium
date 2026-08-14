// SPDX-License-Identifier: MIT

use crate::display::layout::alignment::Alignment;

/// A single cell in the grid containing content to display.
#[derive(Debug, Clone)]
pub struct Cell {
    /// The display width of the cell content (excluding ANSI codes).
    pub width: usize,
    /// The cell content (may contain ANSI escape codes).
    pub contents: String,
    /// The alignment of the cell content within its column.
    pub alignment: Alignment,
}
