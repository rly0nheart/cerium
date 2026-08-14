// SPDX-License-Identifier: MIT

use super::color::{ThemeColor, color_from_value};
use nu_ansi_term::Color;
use std::collections::HashMap;

/// Theme configuration containing every color Cerium can paint with.
///
/// Role names follow the conventions established by `lsd` (`permission`,
/// `date`, `size`, `tree-edge`) and `eza` (`filekind`, `file_type`), so a
/// theme written for those tools translates key-for-key.
#[derive(Debug, Clone)]
pub struct Theme {
    // [size] — magnitude buckets
    pub size_none: ThemeColor,
    pub size_small: ThemeColor,
    pub size_medium: ThemeColor,
    pub size_large: ThemeColor,

    // [date] — age buckets, youngest first
    pub date_minute_old: ThemeColor,
    pub date_hour_old: ThemeColor,
    pub date_day_old: ThemeColor,
    pub date_week_old: ThemeColor,
    pub date_month_old: ThemeColor,
    pub date_older: ThemeColor,

    // [permission]
    pub permission_read: ThemeColor,
    pub permission_write: ThemeColor,
    pub permission_exec: ThemeColor,
    pub permission_no_access: ThemeColor,
    pub permission_exec_sticky: ThemeColor,
    pub permission_filetype: ThemeColor,
    pub permission_acl: ThemeColor,
    pub permission_context: ThemeColor,
    pub permission_attribute: ThemeColor,

    // [filekind]
    pub filekind_normal: ThemeColor,
    pub filekind_directory: ThemeColor,
    pub filekind_symlink: ThemeColor,

    // [file_type] — groups
    pub file_type_source: ThemeColor,
    pub file_type_build: ThemeColor,
    pub file_type_document: ThemeColor,
    pub file_type_image: ThemeColor,
    pub file_type_video: ThemeColor,
    pub file_type_music: ThemeColor,
    pub file_type_compressed: ThemeColor,
    pub file_type_crypto: ThemeColor,

    // [file_type] — per-language and per-format refinements
    pub file_type_rust: ThemeColor,
    pub file_type_python: ThemeColor,
    pub file_type_javascript: ThemeColor,
    pub file_type_c: ThemeColor,
    pub file_type_go: ThemeColor,
    pub file_type_java: ThemeColor,
    pub file_type_ruby: ThemeColor,
    pub file_type_php: ThemeColor,
    pub file_type_lua: ThemeColor,
    pub file_type_html: ThemeColor,
    pub file_type_css: ThemeColor,
    pub file_type_json: ThemeColor,
    pub file_type_xml: ThemeColor,
    pub file_type_yaml: ThemeColor,
    pub file_type_markdown: ThemeColor,
    pub file_type_pdf: ThemeColor,
    pub file_type_text: ThemeColor,

    // Top-level roles
    pub user: ThemeColor,
    pub group: ThemeColor,
    pub tree_edge: ThemeColor,
    pub header: ThemeColor,
    pub path: ThemeColor,
    pub numeric: ThemeColor,
    pub punctuation: ThemeColor,
    pub summary: ThemeColor,
    pub checksum: ThemeColor,
    pub magic: ThemeColor,
    pub mountpoint: ThemeColor,

    // [cli_help]
    pub cli_help_header: ThemeColor,
    pub cli_help_usage: ThemeColor,
    pub cli_help_literal: ThemeColor,
    pub cli_help_placeholder: ThemeColor,
}

impl Theme {
    /// Reads a theme from TOML source.
    ///
    /// # Parameters
    /// - `source`: The config file contents.
    ///
    /// # Returns
    /// A fully-populated [`Theme`] for any syntactically valid TOML, or the
    /// parse error. Unknown or unresolvable roles keep their defaults.
    pub fn parse(source: &str) -> Result<Self, toml::de::Error> {
        Ok(Theme::from_value(&toml::from_str::<toml::Value>(source)?))
    }

    /// Builds a theme from a parsed TOML value.
    ///
    /// Resolution per key:
    /// 1. the `[palette]` table is resolved into named colors;
    /// 2. each role is looked up by its section and key (`permission.read`),
    ///    or at the top level for unsectioned roles;
    /// 3. its value is resolved (RGB / hex / palette reference / named);
    /// 4. anything absent or unresolvable uses the built-in Catppuccin
    ///    Mocha default for that role.
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
        let palette: HashMap<String, Color> = root
            .and_then(|table| table.get("palette"))
            .and_then(toml::Value::as_table)
            .map(|table| {
                table
                    .iter()
                    .filter_map(|(name, value)| {
                        color_from_value(value, &empty).map(|color| (name.clone(), color))
                    })
                    .collect()
            })
            .unwrap_or_default();

        // `path` is a dotted role name: "permission.read" reads key `read`
        // from table `[permission]`, "user" reads the top-level key.
        let pick = |path: &str, fallback: ThemeColor| -> ThemeColor {
            let raw = match path.split_once('.') {
                Some((section, key)) => root
                    .and_then(|table| table.get(section))
                    .and_then(toml::Value::as_table)
                    .and_then(|table| table.get(key)),
                None => root.and_then(|table| table.get(path)),
            };

            match raw.and_then(|value| color_from_value(value, &palette)) {
                Some(color) => ThemeColor { color },
                None => fallback,
            }
        };

        let d = Theme::default();

        Theme {
            size_none: pick("size.none", d.size_none),
            size_small: pick("size.small", d.size_small),
            size_medium: pick("size.medium", d.size_medium),
            size_large: pick("size.large", d.size_large),

            date_minute_old: pick("date.minute-old", d.date_minute_old),
            date_hour_old: pick("date.hour-old", d.date_hour_old),
            date_day_old: pick("date.day-old", d.date_day_old),
            date_week_old: pick("date.week-old", d.date_week_old),
            date_month_old: pick("date.month-old", d.date_month_old),
            date_older: pick("date.older", d.date_older),

            permission_read: pick("permission.read", d.permission_read),
            permission_write: pick("permission.write", d.permission_write),
            permission_exec: pick("permission.exec", d.permission_exec),
            permission_no_access: pick("permission.no-access", d.permission_no_access),
            permission_exec_sticky: pick("permission.exec-sticky", d.permission_exec_sticky),
            permission_filetype: pick("permission.filetype", d.permission_filetype),
            permission_acl: pick("permission.acl", d.permission_acl),
            permission_context: pick("permission.context", d.permission_context),
            permission_attribute: pick("permission.attribute", d.permission_attribute),

            filekind_normal: pick("filekind.normal", d.filekind_normal),
            filekind_directory: pick("filekind.directory", d.filekind_directory),
            filekind_symlink: pick("filekind.symlink", d.filekind_symlink),

            file_type_source: pick("file_type.source", d.file_type_source),
            file_type_build: pick("file_type.build", d.file_type_build),
            file_type_document: pick("file_type.document", d.file_type_document),
            file_type_image: pick("file_type.image", d.file_type_image),
            file_type_video: pick("file_type.video", d.file_type_video),
            file_type_music: pick("file_type.music", d.file_type_music),
            file_type_compressed: pick("file_type.compressed", d.file_type_compressed),
            file_type_crypto: pick("file_type.crypto", d.file_type_crypto),

            file_type_rust: pick("file_type.rust", d.file_type_rust),
            file_type_python: pick("file_type.python", d.file_type_python),
            file_type_javascript: pick("file_type.javascript", d.file_type_javascript),
            file_type_c: pick("file_type.c", d.file_type_c),
            file_type_go: pick("file_type.go", d.file_type_go),
            file_type_java: pick("file_type.java", d.file_type_java),
            file_type_ruby: pick("file_type.ruby", d.file_type_ruby),
            file_type_php: pick("file_type.php", d.file_type_php),
            file_type_lua: pick("file_type.lua", d.file_type_lua),
            file_type_html: pick("file_type.html", d.file_type_html),
            file_type_css: pick("file_type.css", d.file_type_css),
            file_type_json: pick("file_type.json", d.file_type_json),
            file_type_xml: pick("file_type.xml", d.file_type_xml),
            file_type_yaml: pick("file_type.yaml", d.file_type_yaml),
            file_type_markdown: pick("file_type.markdown", d.file_type_markdown),
            file_type_pdf: pick("file_type.pdf", d.file_type_pdf),
            file_type_text: pick("file_type.text", d.file_type_text),

            user: pick("user", d.user),
            group: pick("group", d.group),
            tree_edge: pick("tree-edge", d.tree_edge),
            header: pick("header", d.header),
            path: pick("path", d.path),
            numeric: pick("numeric", d.numeric),
            punctuation: pick("punctuation", d.punctuation),
            summary: pick("summary", d.summary),
            checksum: pick("checksum", d.checksum),
            magic: pick("magic", d.magic),
            mountpoint: pick("mountpoint", d.mountpoint),

            cli_help_header: pick("cli_help.header", d.cli_help_header),
            cli_help_usage: pick("cli_help.usage", d.cli_help_usage),
            cli_help_literal: pick("cli_help.literal", d.cli_help_literal),
            cli_help_placeholder: pick("cli_help.placeholder", d.cli_help_placeholder),
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
            // Size buckets (smallest to largest)
            size_none: green.clone(),
            size_small: green.clone(),
            size_medium: teal.clone(),
            size_large: yellow.clone(),

            // Date buckets (youngest to oldest)
            date_minute_old: sky.clone(),
            date_hour_old: sapphire.clone(),
            date_day_old: blue.clone(),
            date_week_old: lavender.clone(),
            date_month_old: overlay0.clone(),
            date_older: surface2.clone(),

            // Permissions
            permission_read: yellow.clone(),
            permission_write: red.clone(),
            permission_exec: green.clone(),
            permission_no_access: overlay0.clone(),
            permission_exec_sticky: pink.clone(),
            permission_filetype: blue.clone(),
            permission_acl: green.clone(),
            permission_context: mauve.clone(),
            permission_attribute: sky.clone(),

            // Entry kinds
            filekind_normal: text.clone(),
            filekind_directory: blue.clone(),
            filekind_symlink: sky.clone(),

            // File type groups
            file_type_source: blue.clone(),
            file_type_build: teal.clone(),
            file_type_document: text.clone(),
            file_type_image: pink.clone(),
            file_type_video: peach.clone(),
            file_type_music: green.clone(),
            file_type_compressed: yellow.clone(),
            file_type_crypto: red.clone(),

            // File type refinements
            file_type_rust: peach.clone(),
            file_type_python: sapphire.clone(),
            file_type_javascript: yellow.clone(),
            file_type_c: teal.clone(),
            file_type_go: sky.clone(),
            file_type_java: peach.clone(),
            file_type_ruby: red.clone(),
            file_type_php: mauve.clone(),
            file_type_lua: blue.clone(),
            file_type_html: maroon.clone(),
            file_type_css: mauve.clone(),
            file_type_json: pink.clone(),
            file_type_xml: text.clone(),
            file_type_yaml: teal.clone(),
            file_type_markdown: text.clone(),
            file_type_pdf: red.clone(),
            file_type_text: text.clone(),

            // Top-level roles
            user: yellow.clone(),
            group: peach.clone(),
            tree_edge: overlay0.clone(),
            header: yellow.clone(),
            path: blue.clone(),
            numeric: sky.clone(),
            punctuation: overlay0.clone(),
            summary: text.clone(),
            checksum: teal.clone(),
            magic: pink.clone(),
            mountpoint: mauve.clone(),

            // CLI help
            cli_help_header: yellow.clone(),
            cli_help_usage: green.clone(),
            cli_help_literal: sky.clone(),
            cli_help_placeholder: peach.clone(),
        }
    }
}

/// Creates a [`ThemeColor`] from RGB values.
///
/// # Parameters
/// - `r`: Red channel (0–255).
/// - `g`: Green channel (0–255).
/// - `b`: Blue channel (0–255).
///
/// # Returns
/// A [`ThemeColor`] wrapping the specified RGB color.
fn color_rgb(r: u8, g: u8, b: u8) -> ThemeColor {
    ThemeColor {
        color: Color::Rgb(r, g, b),
    }
}
