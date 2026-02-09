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

use crate::cli::flags::SizeFormat;
use crate::display::output::formats::format::Format;
use humanly::HumanSize;
use std::sync::Arc;

/// Formats byte sizes according to the selected [`SizeFormat`].
pub(crate) struct Size {
    size_mode: SizeFormat,
}

impl Size {
    /// Creates a new [`Size`] formatter.
    ///
    /// # Parameters
    /// - `size_mode`: The display format (bytes, binary, or decimal).
    pub(crate) fn new(size_mode: SizeFormat) -> Self {
        Self { size_mode }
    }

    /// Formats a byte count as human-readable or raw.
    ///
    /// # Parameters
    /// - `bytes`: The byte count to format.
    pub(crate) fn format_size(&self, bytes: u64) -> Arc<str> {
        match self.size_mode {
            SizeFormat::Binary => HumanSize::from(bytes).binary().concise().into(),
            SizeFormat::Decimal => HumanSize::from(bytes).decimal().concise().into(),
            SizeFormat::Bytes => bytes.to_string().into(),
        }
    }
}

impl Format<u64> for Size {
    /// Formats a `u64` byte count according to the configured size format.
    fn format(&self, input: u64) -> Arc<str> {
        self.format_size(input)
    }
}
