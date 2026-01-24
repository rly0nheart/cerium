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
use crate::output::layout::column::Column;
use crate::output::populate::Populate;
use std::sync::Arc;

/// Builds row data by extracting and formatting column values from filesystem entries.
///
/// `Row` is a pure data builder that knows how to extract values from entries
/// but does NOT handle rendering, styling, or output. This separation eliminates
/// circular dependencies and makes the code more testable.
///
/// # Responsibilities
///
/// * Extract column values from entries via the Populate system
/// * Provide a simple interface for accessing formatted column data
///
/// # Non-Responsibilities (handled by renderers)
///
/// * Styling and colour application
/// * Padding and alignment
/// * Tree connectors and formatting
/// * Output to stdout
pub(crate) struct Row<'a> {
    /// The filesystem entry this row represents
    pub entry: &'a Entry,
    /// Command-line arguments controlling formatting options
    pub(crate) args: &'a Args,
}

impl<'a> Row<'a> {
    /// Creates a new `Row` wrapping the given filesystem entry.
    ///
    /// # Parameters
    ///
    /// * `entry` - A reference to the filesystem entry to wrap
    /// * `args` - Command-line arguments controlling formatting
    ///
    /// # Returns
    ///
    /// A new `Row` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let row = Row::new(&entry, &args);
    /// let size = row.value(&Column::Size);
    /// ```
    pub(crate) fn new(entry: &'a Entry, args: &'a Args) -> Self {
        Self { entry, args }
    }

    /// Retrieves the formatted value for a specific column.
    ///
    /// This method delegates to the `Populate` system to extract and format
    /// the appropriate data from the entry (size, permissions, timestamps, etc.)
    /// based on the column type.
    ///
    /// # Parameters
    ///
    /// * `column` - The column to retrieve the value for
    ///
    /// # Returns
    ///
    /// An `Arc<str>` containing the formatted column value (without styling)
    pub(crate) fn value(&self, column: &Column) -> Arc<str> {
        let populate = Populate::new(self.entry, column, self.args);
        populate.value()
    }
}
