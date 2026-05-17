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

use super::colour::{ThemeColour, colour_from_value};
use nu_ansi_term::Color as Colour;
use serde::Deserialize;
use std::collections::HashMap;

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
#[derive(Debug, Clone)]
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
    pub summary: ThemeColour,

    // CLI help colours
    pub cli_help_header: ThemeColour,
    pub cli_help_usage: ThemeColour,
    pub cli_help_literal: ThemeColour,
    pub cli_help_placeholder: ThemeColour,
}

impl<'de> Deserialize<'de> for Theme {
    /// Deserialises a theme from the config file.
    ///
    /// The whole document is read into a [`toml::Value`], then
    /// [`Theme::from_value`] applies the palette layer and per-field
    /// fallbacks to the built-in Catppuccin Mocha default.
    ///
    /// # Parameters
    /// - `deserializer`: The serde deserialiser to read from.
    ///
    /// # Returns
    /// A fully-populated [`Theme`]. Succeeds for any syntactically valid TOML.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = toml::Value::deserialize(deserializer)?;
        Ok(Theme::from_value(&value))
    }
}

impl Theme {
    /// Builds a theme from a parsed TOML value.
    ///
    /// Resolution per key:
    /// 1. the `[palette]` table is resolved into named colours;
    /// 2. each semantic key is looked up under `[colors]`, then at the top
    ///    level (flat form);
    /// 3. its value is resolved (RGB / hex / palette reference / named);
    /// 4. anything absent or unresolvable uses the built-in Catppuccin
    ///    Mocha default for that key.
    ///
    /// Always returns a complete theme.
    ///
    /// # Parameters
    /// - `value`: The parsed TOML document.
    ///
    /// # Returns
    /// A complete [`Theme`].
    pub(crate) fn from_value(value: &toml::Value) -> Self {
        let root = value.as_table();

        // Palette entries don't reference each other, so they resolve against
        // an empty map.
        let empty = HashMap::new();
        let palette: HashMap<String, Colour> = root
            .and_then(|t| t.get("palette"))
            .and_then(toml::Value::as_table)
            .map(|table| {
                table
                    .iter()
                    .filter_map(|(name, v)| {
                        colour_from_value(v, &empty).map(|c| (name.clone(), c))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let colors = root
            .and_then(|t| t.get("colors"))
            .and_then(toml::Value::as_table);

        // A key may live under [colors] or at the top level; [colors] takes
        // precedence.
        let pick = |name: &str, fallback: ThemeColour| -> ThemeColour {
            let raw = colors
                .and_then(|t| t.get(name))
                .or_else(|| root.and_then(|t| t.get(name)));
            match raw.and_then(|v| colour_from_value(v, &palette)) {
                Some(colour) => ThemeColour { colour },
                None => fallback,
            }
        };

        let d = Theme::default();

        Theme {
            size_bytes: pick("size_bytes", d.size_bytes),
            size_kb: pick("size_kb", d.size_kb),
            size_mb: pick("size_mb", d.size_mb),
            size_gb: pick("size_gb", d.size_gb),

            date_recent: pick("date_recent", d.date_recent),
            date_hours: pick("date_hours", d.date_hours),
            date_days: pick("date_days", d.date_days),
            date_weeks: pick("date_weeks", d.date_weeks),
            date_months: pick("date_months", d.date_months),
            date_old: pick("date_old", d.date_old),

            perm_read: pick("perm_read", d.perm_read),
            perm_write: pick("perm_write", d.perm_write),
            perm_execute: pick("perm_execute", d.perm_execute),
            perm_none: pick("perm_none", d.perm_none),
            perm_special: pick("perm_special", d.perm_special),
            perm_filetype: pick("perm_filetype", d.perm_filetype),

            entry_directory: pick("entry_directory", d.entry_directory),
            entry_symlink: pick("entry_symlink", d.entry_symlink),
            entry_file: pick("entry_file", d.entry_file),

            user: pick("user", d.user),
            group: pick("group", d.group),

            code_rust: pick("code_rust", d.code_rust),
            code_python: pick("code_python", d.code_python),
            code_javascript: pick("code_javascript", d.code_javascript),
            code_c: pick("code_c", d.code_c),
            code_go: pick("code_go", d.code_go),
            code_java: pick("code_java", d.code_java),
            code_ruby: pick("code_ruby", d.code_ruby),
            code_php: pick("code_php", d.code_php),
            code_lua: pick("code_lua", d.code_lua),

            web_html: pick("web_html", d.web_html),
            web_css: pick("web_css", d.web_css),
            web_json: pick("web_json", d.web_json),
            web_xml: pick("web_xml", d.web_xml),
            web_yaml: pick("web_yaml", d.web_yaml),

            doc_text: pick("doc_text", d.doc_text),
            doc_markdown: pick("doc_markdown", d.doc_markdown),
            doc_pdf: pick("doc_pdf", d.doc_pdf),

            media_image: pick("media_image", d.media_image),
            media_video: pick("media_video", d.media_video),
            media_audio: pick("media_audio", d.media_audio),

            archive: pick("archive", d.archive),

            tree_connector: pick("tree_connector", d.tree_connector),
            table_header: pick("table_header", d.table_header),
            path_display: pick("path_display", d.path_display),
            checksum: pick("checksum", d.checksum),
            magic: pick("magic", d.magic),
            xattr: pick("xattr", d.xattr),
            acl: pick("acl", d.acl),
            mountpoint: pick("mountpoint", d.mountpoint),
            numeric: pick("numeric", d.numeric),
            placeholder: pick("placeholder", d.placeholder),
            summary: pick("summary", d.summary),

            cli_help_header: pick("cli_help_header", d.cli_help_header),
            cli_help_usage: pick("cli_help_usage", d.cli_help_usage),
            cli_help_literal: pick("cli_help_literal", d.cli_help_literal),
            cli_help_placeholder: pick("cli_help_placeholder", d.cli_help_placeholder),
        }
    }
}

impl Default for Theme {
    /// Returns the built-in Catppuccin Mocha theme.
    ///
    /// # Returns
    /// A [`Theme`] using the Catppuccin Mocha palette
    /// (<https://github.com/catppuccin/catppuccin>).
    fn default() -> Self {
        // Catppuccin Mocha palette
        let text = color_rgb(205, 214, 244);
        let red = color_rgb(243, 139, 168);
        let maroon = color_rgb(235, 160, 172);
        let peach = color_rgb(250, 179, 135);
        let yellow = color_rgb(249, 226, 175);
        let green = color_rgb(166, 227, 161);
        let teal = color_rgb(148, 226, 213);
        let sky = color_rgb(137, 220, 235);
        let sapphire = color_rgb(116, 199, 236);
        let blue = color_rgb(137, 180, 250);
        let lavender = color_rgb(180, 190, 254);
        let mauve = color_rgb(203, 166, 247);
        let pink = color_rgb(245, 194, 231);
        let overlay0 = color_rgb(108, 112, 134);
        let surface2 = color_rgb(88, 91, 112);

        Theme {
            // Size gradients (smallest to largest)
            size_bytes: green.clone(),
            size_kb: green.clone(),
            size_mb: teal.clone(),
            size_gb: yellow.clone(),

            // Date gradients (recent to old)
            date_recent: sky.clone(),
            date_hours: sapphire.clone(),
            date_days: blue.clone(),
            date_weeks: lavender.clone(),
            date_months: overlay0.clone(),
            date_old: surface2.clone(),

            // Permission colours
            perm_read: yellow.clone(),
            perm_write: red.clone(),
            perm_execute: green.clone(),
            perm_none: overlay0.clone(),
            perm_special: pink.clone(),
            perm_filetype: blue.clone(),

            // Entry types
            entry_directory: blue.clone(),
            entry_symlink: sky.clone(),
            entry_file: text.clone(),

            // User/Group
            user: yellow.clone(),
            group: peach.clone(),

            // Code file types
            code_rust: peach.clone(),
            code_python: sapphire.clone(),
            code_javascript: yellow.clone(),
            code_c: teal.clone(),
            code_go: sky.clone(),
            code_java: peach.clone(),
            code_ruby: red.clone(),
            code_php: mauve.clone(),
            code_lua: blue.clone(),

            // Web file types
            web_html: maroon.clone(),
            web_css: mauve.clone(),
            web_json: pink.clone(),
            web_xml: text.clone(),
            web_yaml: teal.clone(),

            // Document types
            doc_text: text.clone(),
            doc_markdown: text.clone(),
            doc_pdf: red.clone(),

            // Media types
            media_image: pink.clone(),
            media_video: peach.clone(),
            media_audio: green.clone(),

            // Archive types
            archive: yellow.clone(),

            // UI colours
            tree_connector: overlay0.clone(),
            table_header: yellow.clone(),
            path_display: blue.clone(),
            checksum: teal.clone(),
            magic: pink.clone(),
            xattr: sky.clone(),
            acl: green.clone(),
            mountpoint: mauve.clone(),
            numeric: sky.clone(),
            placeholder: overlay0.clone(),
            summary: text.clone(),

            // CLI help colours
            cli_help_header: yellow.clone(),
            cli_help_usage: green.clone(),
            cli_help_literal: sky.clone(),
            cli_help_placeholder: peach.clone(),
        }
    }
}

/// Creates a [`ThemeColour`] from RGB values.
///
/// # Parameters
/// - `r`: Red channel (0–255).
/// - `g`: Green channel (0–255).
/// - `b`: Blue channel (0–255).
///
/// # Returns
/// A [`ThemeColour`] wrapping the specified RGB colour.
fn color_rgb(r: u8, g: u8, b: u8) -> ThemeColour {
    ThemeColour {
        colour: Colour::Rgb(r, g, b),
    }
}
