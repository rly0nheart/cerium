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

mod cli;
mod display;
mod fs;

use crate::cli::args::Args;
use crate::display::factory::DisplayFactory;
use crate::display::output::banner;
use crate::display::theme::colours::{ColourSettings, RgbColours};
use crate::display::theme::config;
use crate::display::theme::icons::IconSettings;
use crate::fs::dir::DirReader;
use crate::fs::hyperlink::HyperlinkSettings;
use clap::{CommandFactory, FromArgMatches};
use std::process;

/// Application entry point.
///
/// # Description
/// Parses CLI arguments, validates the target directory, prepares display options,
/// and invokes the appropriate display mode. Handles warnings and errors for invalid
/// paths, non-directory paths, or empty directories.
fn main() {
    // Load theme from config file (or use built-in Gruvbox) BEFORE parsing args
    let theme = config::load_theme();

    // Generate banner with theme colours
    let banner_gradient = cli::args::banner_gradient_from_theme(&theme);
    let banner = banner::get_banner(&banner_gradient);

    // Apply theme colours to CLI and parse arguments
    let args = Args::command()
        .about(banner)
        .styles(cli::args::styles_from_theme(&theme))
        .get_matches();
    let args = Args::from_arg_matches(&args).expect("Failed to parse arguments");

    // Initialise theme system for output
    RgbColours::init(theme);

    // Setup colours, icons, and hyperlinks
    ColourSettings::setup(args.colours);
    IconSettings::setup(args.icons);
    HyperlinkSettings::setup(args.hyperlink);

    // Convert input path to PathBuf
    let path = &args.path;
    let dir_reader = DirReader::from(path.to_path_buf());

    // Validate that the path exists (use lstat to handle broken symlinks)
    if std::fs::symlink_metadata(path).is_err() {
        println!("file or directory not found: {}", &path.display());
        process::exit(1);
    }

    if path.is_dir() && dir_reader.is_empty() {
        println!("empty directory: {}", &path.display());
        process::exit(1);
    }

    // Use the factory to create the appropriate display mode
    let display = DisplayFactory::create(&dir_reader, args);
    display.print();
}
