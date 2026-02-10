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
use crate::display::styles::element::ElementStyle;
use crate::fs::dir::DirReader;
use crate::fs::entry::Entry;
use std::cell::Cell;
use std::path::Path;

/// Trait for renderers that support recursive directory traversal.
///
/// This trait provides a default implementation of recursive directory listing
/// that eliminates code duplication between List and Grid renderers. The common
/// pattern is:
/// 1. Print optional section title (directory path)
/// 2. Render current level using renderer-specific logic
/// 3. Descend into subdirectories
///
/// # Example
///
/// ```text
/// impl RecursiveTraversal for List {
///     fn render_level(&self, entries: &[Entry], args: &Args) {
///         // List-specific rendering
///     }
///
///     fn get_args(&self) -> &Args {
///         &self.args
///     }
/// }
/// ```
pub(crate) trait RecursiveTraversal {
    /// Renders entries at a single directory level.
    ///
    /// # Parameters
    /// - `entries`: The entries to render at this level.
    /// - `args`: Command-line arguments controlling display options.
    fn render_level(&self, entries: &[Entry], args: &Args);

    /// Returns a reference to the Args for this renderer.
    ///
    /// This allows the trait to access the renderer's configuration
    /// without requiring it to be passed as a parameter.
    fn get_args(&self) -> &Args;

    /// Returns a reference to the accumulated directory count.
    fn dir_count(&self) -> &Cell<usize>;

    /// Returns a reference to the accumulated file count.
    fn file_count(&self) -> &Cell<usize>;

    /// Recursively renders entries with directory titles, descending into subdirectories.
    ///
    /// Accumulates directory and file counts during traversal so that
    /// the summary can be printed instantly without re-reading the filesystem.
    ///
    /// # Parameters
    /// - `entries`: The entries to display at the current level.
    /// - `title`: Optional path to display as a section header; `None` for the root call.
    fn render_recursive(&self, entries: &[Entry], title: Option<&Path>) {
        // Print section title if provided
        if let Some(path) = title {
            println!("\n{}:", ElementStyle::path_header(path.display()));
        }

        // Render current level using renderer-specific logic
        let args = self.get_args();
        self.render_level(entries, args);

        // Accumulate counts from this level
        for entry in entries {
            if entry.is_dir() {
                self.dir_count().set(self.dir_count().get() + 1);
            } else {
                self.file_count().set(self.file_count().get() + 1);
            }
        }

        // Descend into subdirectories
        for entry in entries.iter().filter(|e| e.is_dir()) {
            let path = entry.path();
            let dir_reader = DirReader::from(path.to_path_buf());
            let children = dir_reader.list(args);
            self.render_recursive(&children, Some(path));
        }
    }
}
