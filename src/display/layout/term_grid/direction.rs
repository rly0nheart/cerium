// SPDX-License-Identifier: MIT

/// The direction cells are laid out in the grid.
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    /// Cells fill columns top-to-bottom, then move to the next column.
    TopToBottom,
    /// Cells fill rows left-to-right, then move to the next row.
    LeftToRight,
}
