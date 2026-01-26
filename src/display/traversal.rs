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
use crate::display::styles::text::TextStyle;
use crate::fs::dir::DirReader;
use crate::fs::entry::Entry;
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
/// ```rust
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
    /// This method is implemented by each renderer to provide its specific
    /// rendering logic (list, grid, etc.) without the recursion overhead.
    ///
    /// # Parameters
    ///
    /// * `entries` - The entries to render at this level
    /// * `args` - Command-line arguments controlling display options
    fn render_level(&self, entries: &[Entry], args: &Args);

    /// Returns a reference to the Args for this renderer.
    ///
    /// This allows the trait to access the renderer's configuration
    /// without requiring it to be passed as a parameter.
    fn get_args(&self) -> &Args;

    /// Recursively renders entries with directory titles, descending into subdirectories.
    ///
    /// This default implementation provides the common recursion pattern used by
    /// multiple renderers, eliminating code duplication. It:
    /// 1. Prints an optional title (directory path) as a section header
    /// 2. Calls `render_level()` for renderer-specific output
    /// 3. Recursively processes all subdirectories
    ///
    /// # Parameters
    ///
    /// * `entries` - The entries to display at the current level
    /// * `title` - Optional path to display as a section title before the entries.
    ///   `None` for the initial/root call, `Some(path)` for subdirectories
    ///
    /// # Output Format
    ///
    /// ```text
    /// /path/to/directory:
    /// [entries rendered by render_level()]
    ///
    /// /path/to/directory/subdir:
    /// [entries rendered by render_level()]
    /// ```
    ///
    /// # Performance
    ///
    /// This method performs a depth-first traversal, rendering each directory
    /// before descending into its subdirectories. For large directory trees,
    /// this can be memory-intensive as it maintains the full call stack.
    fn render_recursive(&self, entries: &[Entry], title: Option<&Path>) {
        // 1. Print section title if provided
        if let Some(path) = title {
            println!("\n{}:", TextStyle::path_display(path.display()));
        }

        // 2. Render current level using renderer-specific logic
        let args = self.get_args();
        self.render_level(entries, args);

        // 3. Descend into subdirectories
        for entry in entries.iter().filter(|e| e.is_dir()) {
            let path = entry.path();
            let dir_reader = DirReader::from(path.to_path_buf());
            let children = dir_reader.list(args);
            self.render_recursive(&children, Some(path));
        }
    }
}
