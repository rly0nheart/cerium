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

use crate::display::layout::width::Width;

/// Text alignment direction within a column.
#[derive(Debug, Copy, Clone)]
pub enum Alignment {
    Left,
    Right,
}

/// Pads strings to a target width according to an [`Alignment`].
pub struct Align;

impl Align {
    /// Pads a string to the target width using the given alignment.
    ///
    /// # Parameters
    /// - `value`: The string to pad (may contain ANSI codes).
    /// - `width`: The target display width.
    /// - `alignment`: Whether to left- or right-align the value.
    ///
    /// # Returns
    /// The padded string.
    pub fn pad(value: &String, width: usize, alignment: Alignment) -> String {
        let visible = Width::measure_ansi_text(value);
        let padding = width.saturating_sub(visible);
        match alignment {
            Alignment::Right => format!("{}{}", " ".repeat(padding), value),
            Alignment::Left => format!("{}{}", value, " ".repeat(padding)),
        }
    }
}
