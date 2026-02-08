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

/// Factory for creating appropriate display modes based on command-line arguments.
///
/// `DisplayFactory` encapsulates all the logic for selecting which display mode
/// (List, Grid, or Tree) to use based on the user's CLI flags and the type
/// of directory operation (list, search, tree).
///
/// This separation moves display selection logic out of the CLI layer and into
/// the output system where it belongs, making the codebase more maintainable.
///
/// # Responsibilities
///
/// * Determine which display mode to use based on Args
/// * Create and configure the appropriate display mode
/// * Handle special cases (find/search, tree mode, recursive mode)
///
/// # Examples
///
/// ```text
/// let dir_reader = DirReader::from(path);
/// let display = DisplayFactory::create(&dir_reader, args);
/// display.print();
/// ```
pub struct DisplayFactory;

impl DisplayFactory {
    /// Creates the appropriate display mode based on the command-line arguments.
    ///
    /// This is the main entry point for display mode selection. It examines the
    /// args to determine the user's intent and creates the corresponding display mode.
    ///
    /// # Selection Logic
    ///
    /// 1. **Find/Search mode**: If `args.find` is non-empty, use List display
    ///    with search results
    /// 2. **Tree mode**: If `args.tree` is true, use Tree display
    /// 3. **List/Grid mode**: Otherwise, determine if List or Grid is appropriate:
    ///    - List if metadata is needed or table-specific columns are requested
    ///    - Grid otherwise (compact view)
    ///
    /// # Parameters
    ///
    /// * `dir_reader` - The directory reader to use
    /// * `args` - Command-line arguments controlling display options
    ///
    /// # Returns
    ///
    /// A boxed DisplayMode trait object ready to render output
    pub fn create(dir_reader: &DirReader, args: Args) -> Box<dyn DisplayMode> {
        // 1. Find/Search mode
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

        // 2. Tree mode
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

        // 3. List vs Grid mode
        let entries = dir_reader.list(&args);
        let mut files_count = 0;
        let mut dirs_count = 0;

        for entry in &entries {
            if entry.is_file() {
                files_count += 1;
            } else if entry.is_dir() {
                dirs_count += 1;
            }
        }

        println!("{dirs_count} directories, {files_count} files");

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

    /// Determines whether the List renderer should be used instead of Grid.
    ///
    /// List renderer is needed when:
    /// * Metadata columns are requested (size, dates, permissions, etc.)
    /// * Table-specific columns are requested (magic, head, tail, oneline)
    ///
    /// # Parameters
    ///
    /// * `args` - Command-line arguments to examine
    ///
    /// # Returns
    ///
    /// `true` if List renderer should be used, `false` for Grid
    fn needs_list_renderer(args: &Args) -> bool {
        Args::is_args_requesting_metadata(args) || Args::is_args_requesting_table_column(args)
    }
}
