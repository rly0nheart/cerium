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
use crate::display::layout::column::Column;
use crate::display::output::populate::Populate;
use crate::fs::entry::Entry;
use std::sync::Arc;

/// Extracts and formats column values from a filesystem entry.
pub(crate) struct Row<'a> {
    pub entry: &'a Entry,
    pub(crate) args: &'a Args,
}

impl<'a> Row<'a> {
    /// Creates a new [`Row`] for the given entry.
    ///
    /// # Parameters
    /// - `entry`: The filesystem entry to wrap.
    /// - `args`: Command-line arguments controlling formatting.
    pub(crate) fn new(entry: &'a Entry, args: &'a Args) -> Self {
        Self { entry, args }
    }

    /// Returns the formatted value for a specific column.
    ///
    /// # Parameters
    /// - `column`: The column to retrieve the value for.
    ///
    /// # Returns
    /// An `Arc<str>` containing the formatted column value (without styling).
    pub(crate) fn value(&self, column: &Column) -> Arc<str> {
        let populate = Populate::new(self.entry, column, self.args);
        populate.value()
    }
}
