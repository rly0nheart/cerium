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

/// The direction cells are laid out in the grid.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Direction {
    /// Cells fill columns top-to-bottom, then move to the next column.
    TopToBottom,
    /// Cells fill rows left-to-right, then move to the next row.
    LeftToRight,
}

/// The separator between columns in the grid.
#[derive(Debug, Clone, Copy)]
pub enum Filling {
    /// Use the specified number of spaces between columns.
    Spaces(usize),
}

/// Options for configuring grid layout.
#[derive(Debug, Clone)]
pub struct GridOptions {
    /// The direction cells are laid out.
    pub direction: Direction,
    /// The separator between columns.
    pub filling: Filling,
}
