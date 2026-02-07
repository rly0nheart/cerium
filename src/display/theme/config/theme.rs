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

use super::colour::ThemeColour;
use nu_ansi_term::Color as Colour;
use serde::Deserialize;

/// Theme configuration containing all customisable colours for Cerium.
///
/// Colours are organised into semantic categories:
/// - Size gradients (for file sizes from bytes to gigabytes)
/// - Date gradients (for timestamps from recent to old)
/// - Permission colours (read, write, execute, etc.)
/// - Entry type colours (files, directories, symlinks)
/// - File type colours (code, web, documents, media, archives)
/// - UI colours (tree connectors, headers, paths, etc.)
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Theme {
    // Size gradients (bytes → gigabytes)
    pub size_bytes: ThemeColour,
    pub size_kb: ThemeColour,
    pub size_mb: ThemeColour,
    pub size_gb: ThemeColour,

    // Date gradients (recent → old)
    pub date_recent: ThemeColour,
    pub date_hours: ThemeColour,
    pub date_days: ThemeColour,
    pub date_weeks: ThemeColour,
    pub date_months: ThemeColour,
    pub date_old: ThemeColour,

    // Permission colours
    pub perm_read: ThemeColour,
    pub perm_write: ThemeColour,
    pub perm_execute: ThemeColour,
    pub perm_none: ThemeColour,
    pub perm_special: ThemeColour,
    pub perm_filetype: ThemeColour,

    // Entry types
    pub entry_directory: ThemeColour,
    pub entry_symlink: ThemeColour,
    pub entry_file: ThemeColour,

    // User/Group
    pub user: ThemeColour,
    pub group: ThemeColour,

    // Code file types
    pub code_rust: ThemeColour,
    pub code_python: ThemeColour,
    pub code_javascript: ThemeColour,
    pub code_c: ThemeColour,
    pub code_go: ThemeColour,
    pub code_java: ThemeColour,
    pub code_ruby: ThemeColour,
    pub code_php: ThemeColour,
    pub code_lua: ThemeColour,

    // Web file types
    pub web_html: ThemeColour,
    pub web_css: ThemeColour,
    pub web_json: ThemeColour,
    pub web_xml: ThemeColour,
    pub web_yaml: ThemeColour,

    // Document types
    pub doc_text: ThemeColour,
    pub doc_markdown: ThemeColour,
    pub doc_pdf: ThemeColour,

    // Media types
    pub media_image: ThemeColour,
    pub media_video: ThemeColour,
    pub media_audio: ThemeColour,

    // Archive types
    pub archive: ThemeColour,

    // Misc UI colours
    pub tree_connector: ThemeColour,
    pub table_header: ThemeColour,
    pub path_display: ThemeColour,
    pub checksum: ThemeColour,
    pub magic: ThemeColour,
    pub xattr: ThemeColour,
    pub acl: ThemeColour,
    pub mountpoint: ThemeColour,
    pub numeric: ThemeColour,
    pub placeholder: ThemeColour,

    // CLI help colours
    pub cli_help_header: ThemeColour,
    pub cli_help_usage: ThemeColour,
    pub cli_help_literal: ThemeColour,
    pub cli_help_placeholder: ThemeColour,
}

impl Theme {
    /// Built-in Gruvbox Dark theme.
    ///
    /// Uses the authentic Gruvbox colour palette by Pavel Pertsev (morhetz).
    /// Color palette: https://github.com/morhetz/gruvbox
    pub fn default() -> Self {
        // Authentic Gruvbox Dark palette
        let fg = color_rgb(235, 219, 178); // fg (light1)
        let red = color_rgb(204, 36, 29); // red
        let bright_red = color_rgb(251, 73, 52); // bright_red
        let green = color_rgb(152, 151, 26); // green
        let bright_green = color_rgb(184, 187, 38); // bright_green
        let yellow = color_rgb(215, 153, 33); // yellow
        let bright_yellow = color_rgb(250, 189, 47); // bright_yellow
        let blue = color_rgb(69, 133, 136); // blue
        let bright_blue = color_rgb(131, 165, 152); // bright_blue
        let purple = color_rgb(177, 98, 134); // purple
        let bright_purple = color_rgb(211, 134, 155); // bright_purple
        let aqua = color_rgb(104, 157, 106); // aqua
        let bright_aqua = color_rgb(142, 192, 124); // bright_aqua
        let gray = color_rgb(146, 131, 116); // gray
        let orange = color_rgb(214, 93, 14); // orange
        let bright_orange = color_rgb(254, 128, 25); // bright_orange

        Theme {
            // Size gradients (green tones - smallest to largest)
            size_bytes: green.clone(),
            size_kb: bright_green.clone(),
            size_mb: bright_aqua.clone(),
            size_gb: bright_yellow.clone(),

            // Date gradients (blue/aqua tones - recent to old)
            date_recent: bright_aqua.clone(),
            date_hours: aqua.clone(),
            date_days: bright_blue.clone(),
            date_weeks: blue.clone(),
            date_months: blue.clone(),
            date_old: gray.clone(),

            // Permission colours (traffic light pattern)
            perm_read: yellow.clone(),
            perm_write: red.clone(),
            perm_execute: green.clone(),
            perm_none: gray.clone(),
            perm_special: purple.clone(),
            perm_filetype: blue.clone(),

            // Entry types
            entry_directory: bright_blue.clone(),
            entry_symlink: bright_aqua.clone(),
            entry_file: fg.clone(),

            // User/Group
            user: bright_yellow.clone(),
            group: bright_orange.clone(),

            // Code file types
            code_rust: orange.clone(),
            code_python: blue.clone(),
            code_javascript: bright_yellow.clone(),
            code_c: aqua.clone(),
            code_go: bright_blue.clone(),
            code_java: bright_orange.clone(),
            code_ruby: red.clone(),
            code_php: purple.clone(),
            code_lua: blue.clone(),

            // Web file types
            web_html: bright_red.clone(),
            web_css: purple.clone(),
            web_json: bright_purple.clone(),
            web_xml: fg.clone(),
            web_yaml: aqua.clone(),

            // Document types
            doc_text: fg.clone(),
            doc_markdown: fg.clone(),
            doc_pdf: bright_red.clone(),

            // Media types
            media_image: bright_purple.clone(),
            media_video: bright_orange.clone(),
            media_audio: bright_aqua.clone(),

            // Archive types
            archive: yellow.clone(),

            // UI colours
            tree_connector: gray.clone(),
            table_header: bright_yellow.clone(),
            path_display: blue.clone(),
            checksum: bright_aqua.clone(),
            magic: bright_purple.clone(),
            xattr: aqua.clone(),
            acl: green.clone(),
            mountpoint: purple.clone(),
            numeric: bright_blue.clone(),
            placeholder: gray.clone(),

            // CLI help colours
            cli_help_header: bright_yellow.clone(),
            cli_help_usage: bright_green.clone(),
            cli_help_literal: bright_aqua.clone(),
            cli_help_placeholder: yellow.clone(),
        }
    }
}

/// Helper to create a ThemeColor from RGB values
fn color_rgb(r: u8, g: u8, b: u8) -> ThemeColour {
    ThemeColour {
        colour: Colour::Rgb(r, g, b),
    }
}
