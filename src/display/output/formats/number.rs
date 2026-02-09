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

use crate::cli::flags::NumberFormat;
use crate::display::output::formats::format::Format;
use humanly::HumanNumber;
use std::sync::Arc;

impl Format<u64> for Number {
    /// Formats a `u64` according to the configured number format.
    fn format(&self, input: u64) -> Arc<str> {
        self.format_number(input)
    }
}

/// Formats numeric values according to the selected [`NumberFormat`].
pub(crate) struct Number {
    number_format: NumberFormat,
}

impl Number {
    /// Creates a new [`Number`] formatter.
    ///
    /// # Parameters
    /// - `number_format`: The display format to use.
    pub(crate) fn new(number_format: NumberFormat) -> Self {
        Self { number_format }
    }

    /// Formats a number as human-readable or natural.
    ///
    /// # Parameters
    /// - `number`: The value to format.
    fn format_number(&self, number: u64) -> Arc<str> {
        match self.number_format {
            NumberFormat::Humanly => HumanNumber::from(number as f64).concise().into(),
            NumberFormat::Natural => number.to_string().into(),
        }
    }
}
