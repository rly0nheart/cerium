// SPDX-License-Identifier: MIT

use cerium::cli::args::Args;
use cerium::render;
use cerium::render::style::cli_help;
use cerium::render::theme::colors;
use cerium::render::theme::config;
use cerium::render::theme::icons;
use cerium::fs::dir::DirReader;
use cerium::fs::hyperlink;
use clap::{CommandFactory, FromArgMatches};
use std::process;

/// Parses CLI arguments, validates the target directory, and invokes the appropriate renderer.
fn main() {
    // Load theme from config file (or use built-in Gruvbox) BEFORE parsing args
    let theme = config::load_theme();

    // Initialise theme system for cli help
    let help_style = cli_help::HelpStyle::new(&theme);

    // Apply theme colors to CLI and parse arguments
    let arg_matches = Args::command()
        .styles(help_style.get_styles())
        .get_matches();
    let args = Args::from_arg_matches(&arg_matches).expect("Failed to parse arguments");

    // Initialise theme system for output
    colors::init(theme);

    // Setup colors, icons, and hyperlinks
    colors::setup(args.colors);
    icons::setup(args.icons);
    hyperlink::setup(args.hyperlink);

    // Convert input path to PathBuf
    let path = &args.path;
    let dir_reader = DirReader::from(path.to_path_buf());

    // Validate that the path exists (use lstat to handle broken symlinks)
    if std::fs::symlink_metadata(path).is_err() {
        println!("file or directory not found: {}", path.display());
        process::exit(1);
    }

    // Create the appropriate renderer for the requested output mode
    let renderer = render::create(&dir_reader, args);
    renderer.show();
}
