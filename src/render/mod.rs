// SPDX-License-Identifier: MIT

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

pub(crate) mod classify;
pub mod grid;
pub mod layout;
pub(crate) mod listing;
pub mod output;
pub mod style;
pub(crate) mod summary;
pub mod theme;
pub(crate) mod traversal;
pub(crate) mod tree;

use crate::cli::args::Args;
use crate::fs::dir::DirReader;
use crate::fs::search::Search;
use crate::render::listing::{Listing, Mode};
use crate::render::style::element::ElementStyle;
use crate::render::tree::Tree;

/// A renderer that writes a directory listing to standard output.
pub trait Renderer {
    fn show(&self);
}

/// Creates the appropriate renderer based on the command-line arguments.
///
/// # Parameters
/// - `dir_reader`: The directory reader to use.
/// - `args`: Command-line arguments controlling output options.
///
/// # Returns
/// A boxed [`Renderer`] trait object ready to produce output.
pub fn create(dir_reader: &DirReader, args: Args) -> Box<dyn Renderer> {
    // Find/Search mode
    if !args.find.is_empty() {
        let search = match Search::new(&args.find, dir_reader.path().to_path_buf()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Invalid pattern '{}': {}", args.find, e);
                return Box::new(Listing::new(Vec::new(), args, Mode::Long));
            }
        };
        let matches = search.find(&args);
        let mode = mode_for(&args);
        return Box::new(Listing::new(matches, args, mode));
    }

    // Tree mode
    if args.tree {
        // Use streaming mode for instant output when no table columns are needed
        return if Tree::needs_table_layout(&args) {
            let node = crate::fs::tree::build(dir_reader.path().to_path_buf(), &args);
            Box::new(Tree::new_table(node, args))
        } else {
            Box::new(Tree::new_streaming(dir_reader.path().to_path_buf(), args))
        };
    }

    // List vs Grid mode
    let entries = dir_reader.list(&args);

    // Print directory title for recursive mode
    if args.recursive {
        println!(
            "{}:",
            ElementStyle::path_header(dir_reader.path().display())
        );
    }

    let mode = mode_for(&args);
    Box::new(Listing::new(entries, args, mode))
}

/// Chooses the mode a listing should be rendered in.
///
/// # Parameters
/// - `args`: Command-line arguments to examine.
///
/// # Returns
/// [`Mode::Long`] if metadata or table-specific columns are requested,
/// otherwise [`Mode::Grid`].
fn mode_for(args: &Args) -> Mode {
    if args.wants_metadata() || args.wants_table_column() {
        Mode::Long
    } else {
        Mode::Grid
    }
}
