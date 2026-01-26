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
pub(crate) struct Theme {
    // Size gradients (bytes → gigabytes)
    pub(crate) size_bytes: ThemeColour,
    pub(crate) size_kb: ThemeColour,
    pub(crate) size_mb: ThemeColour,
    pub(crate) size_gb: ThemeColour,

    // Date gradients (recent → old)
    pub(crate) date_recent: ThemeColour,
    pub(crate) date_hours: ThemeColour,
    pub(crate) date_days: ThemeColour,
    pub(crate) date_weeks: ThemeColour,
    pub(crate) date_months: ThemeColour,
    pub(crate) date_old: ThemeColour,

    // Permission colours
    pub(crate) perm_read: ThemeColour,
    pub(crate) perm_write: ThemeColour,
    pub(crate) perm_execute: ThemeColour,
    pub(crate) perm_none: ThemeColour,
    pub(crate) perm_special: ThemeColour,
    pub(crate) perm_filetype: ThemeColour,

    // Entry types
    pub(crate) entry_directory: ThemeColour,
    pub(crate) entry_symlink: ThemeColour,
    pub(crate) entry_file: ThemeColour,

    // User/Group
    pub(crate) user: ThemeColour,
    pub(crate) group: ThemeColour,

    // Code file types
    pub(crate) code_rust: ThemeColour,
    pub(crate) code_python: ThemeColour,
    pub(crate) code_javascript: ThemeColour,
    pub(crate) code_c: ThemeColour,
    pub(crate) code_go: ThemeColour,
    pub(crate) code_java: ThemeColour,
    pub(crate) code_ruby: ThemeColour,
    pub(crate) code_php: ThemeColour,
    pub(crate) code_lua: ThemeColour,

    // Web file types
    pub(crate) web_html: ThemeColour,
    pub(crate) web_css: ThemeColour,
    pub(crate) web_json: ThemeColour,
    pub(crate) web_xml: ThemeColour,
    pub(crate) web_yaml: ThemeColour,

    // Document types
    pub(crate) doc_text: ThemeColour,
    pub(crate) doc_markdown: ThemeColour,
    pub(crate) doc_pdf: ThemeColour,

    // Media types
    pub(crate) media_image: ThemeColour,
    pub(crate) media_video: ThemeColour,
    pub(crate) media_audio: ThemeColour,

    // Archive types
    pub(crate) archive: ThemeColour,

    // Misc UI colours
    pub(crate) tree_connector: ThemeColour,
    pub(crate) table_header: ThemeColour,
    pub(crate) path_display: ThemeColour,
    pub(crate) checksum: ThemeColour,
    pub(crate) magic: ThemeColour,
    pub(crate) xattr: ThemeColour,
    pub(crate) acl: ThemeColour,
    pub(crate) mountpoint: ThemeColour,
    pub(crate) numeric: ThemeColour,
    pub(crate) placeholder: ThemeColour,

    // CLI help colours
    pub(crate) cli_help_header: ThemeColour,
    pub(crate) cli_help_usage: ThemeColour,
    pub(crate) cli_help_literal: ThemeColour,
    pub(crate) cli_help_placeholder: ThemeColour,

    // Banner gradient colours (7 colours for the ASCII art banner)
    pub(crate) banner_gradient_1: ThemeColour,
    pub(crate) banner_gradient_2: ThemeColour,
    pub(crate) banner_gradient_3: ThemeColour,
    pub(crate) banner_gradient_4: ThemeColour,
    pub(crate) banner_gradient_5: ThemeColour,
    pub(crate) banner_gradient_6: ThemeColour,
    pub(crate) banner_gradient_7: ThemeColour,
}

impl Theme {
    /// Built-in Gruvbox Dark theme.
    ///
    /// Uses the authentic Gruvbox colour palette by Pavel Pertsev (morhetz).
    /// Color palette: https://github.com/morhetz/gruvbox
    pub(crate) fn default() -> Self {
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

            // Banner gradient colours (authentic Gruvbox gradient)
            banner_gradient_1: aqua.clone(),   // aqua
            banner_gradient_2: green.clone(),  // green
            banner_gradient_3: yellow.clone(), // yellow
            banner_gradient_4: orange.clone(), // orange
            banner_gradient_5: red.clone(),    // red
            banner_gradient_6: purple.clone(), // purple
            banner_gradient_7: blue.clone(),   // blue
        }
    }
}

/// Helper to create a ThemeColor from RGB values
fn color_rgb(r: u8, g: u8, b: u8) -> ThemeColour {
    ThemeColour {
        colour: Colour::Rgb(r, g, b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gruvbox_theme_creation() {
        let theme = Theme::default();
        // Verify a few key authentic Gruvbox colours
        assert!(matches!(theme.size_bytes.colour, Colour::Rgb(152, 151, 26))); // green
        assert!(matches!(theme.perm_read.colour, Colour::Rgb(215, 153, 33))); // yellow
        assert!(matches!(
            theme.entry_directory.colour,
            Colour::Rgb(131, 165, 152) // bright_blue
        ));
        assert!(matches!(
            theme.entry_file.colour,
            Colour::Rgb(235, 219, 178)
        )); // fg
        assert!(matches!(theme.code_rust.colour, Colour::Rgb(214, 93, 14))); // orange
    }

    #[test]
    fn test_theme_deserialisation() {
        let toml = r#"
            size_bytes = { r = 255, g = 0, b = 0 }
            size_kb = "green"
            size_mb = { r = 0, g = 255, b = 0 }
            size_gb = "blue"
            date_recent = "white"
            date_hours = "white"
            date_days = "white"
            date_weeks = "white"
            date_months = "white"
            date_old = "white"
            perm_read = "yellow"
            perm_write = "red"
            perm_execute = "green"
            perm_none = "darkgray"
            perm_special = "magenta"
            perm_filetype = "blue"
            entry_directory = "blue"
            entry_symlink = "cyan"
            entry_file = "white"
            user = "white"
            group = "white"
            code_rust = "red"
            code_python = "blue"
            code_javascript = "yellow"
            code_c = "cyan"
            code_go = "blue"
            code_java = "red"
            code_ruby = "red"
            code_php = "blue"
            code_lua = "blue"
            web_html = "red"
            web_css = "purple"
            web_json = "magenta"
            web_xml = "white"
            web_yaml = "cyan"
            doc_text = "white"
            doc_markdown = "white"
            doc_pdf = "white"
            media_image = "magenta"
            media_video = "red"
            media_audio = "green"
            archive = "yellow"
            tree_connector = "darkgray"
            table_header = "white"
            path_display = "blue"
            checksum = "white"
            magic = "white"
            xattr = "cyan"
            acl = "green"
            mountpoint = "magenta"
            numeric = "cyan"
            placeholder = "darkgray"
            cli_help_header = "yellow"
            cli_help_usage = "green"
            cli_help_literal = "cyan"
            cli_help_placeholder = "yellow"
            banner_gradient_1 = "cyan"
            banner_gradient_2 = "green"
            banner_gradient_3 = "yellow"
            banner_gradient_4 = "red"
            banner_gradient_5 = "red"
            banner_gradient_6 = "magenta"
            banner_gradient_7 = "blue"
        "#;

        let theme: Theme = toml::from_str(toml).unwrap();
        assert!(matches!(theme.size_bytes.colour, Colour::Rgb(255, 0, 0)));
        assert!(matches!(theme.size_kb.colour, Colour::Green));
    }
}
