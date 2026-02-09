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
use crate::display::grid::Grid;
use crate::display::list::List;
use crate::display::mode::DisplayMode;
use crate::display::styles::text::TextStyle;
use crate::display::tree::Tree;
use crate::fs::dir::DirReader;
use crate::fs::search::Search;
use crate::fs::tree::TreeBuilder;

/// Selects and creates the appropriate display mode based on CLI arguments.
pub struct DisplayFactory;

impl DisplayFactory {
    /// Creates the appropriate display mode based on the command-line arguments.
    ///
    /// # Parameters
    /// - `dir_reader`: The directory reader to use.
    /// - `args`: Command-line arguments controlling display options.
    ///
    /// # Returns
    /// A boxed [`DisplayMode`] trait object ready to produce output.
    pub fn create(dir_reader: &DirReader, args: Args) -> Box<dyn DisplayMode> {
        // Find/Search mode
        if !args.find.is_empty() {
            let search = match Search::new(&args.find, dir_reader.path().clone()) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Invalid pattern '{}': {}", args.find, e);
                    return Box::new(List::new(Vec::new(), args));
                }
            };
            let matches = search.find(&args);
            return if Self::needs_list_renderer(&args) {
                Box::new(List::new(matches, args))
            } else {
                Box::new(Grid::new(matches, args))
            };
        }

        // Tree mode
        if args.tree {
            // Use streaming mode for instant output when no table columns are needed
            return if Tree::needs_table_layout(&args) {
                let builder = TreeBuilder::new(dir_reader.path().clone());
                let node = builder.build(&args);
                Box::new(Tree::new_table(node, args))
            } else {
                Box::new(Tree::new_streaming(dir_reader.path().clone(), args))
            };
        }

        // List vs Grid mode
        let entries = dir_reader.list(&args);

        // Print directory title for recursive mode
        if args.recursive {
            println!("{}:", TextStyle::path_display(dir_reader.path().display()));
        }

        if Self::needs_list_renderer(&args) {
            Box::new(List::new(entries, args))
        } else {
            Box::new(Grid::new(entries, args))
        }
    }

    /// Checks whether the List renderer should be used instead of Grid.
    ///
    /// # Parameters
    /// - `args`: Command-line arguments to examine.
    ///
    /// # Returns
    /// `true` if metadata or table-specific columns are requested.
    fn needs_list_renderer(args: &Args) -> bool {
        Args::is_args_requesting_metadata(args) || Args::is_args_requesting_table_column(args)
    }
}
