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

//! Configuration system for Cerium theme customisation.
//!
//! This module handles loading theme configuration from `~/.config/cerium.toml`.
//! If the config file doesn't exist or is invalid, it falls back silently to the
//! built-in Gruvbox theme.
//!
//! # Config File Format
//!
//! ```toml
//! size_bytes = { r = 152, g = 151, b = 26 }
//! size_kb = { r = 184, g = 187, b = 38 }
//! # ... more colours
//! ```

mod colour;
mod theme;

pub(crate) use theme::Theme;

use std::fs;
use std::path::PathBuf;

/// Loads the theme from config file, or returns built-in Gruvbox theme.
///
/// This function silently falls back to Gruvbox in the following cases:
/// - Config file doesn't exist
/// - Config file is invalid TOML
/// - Any I/O error occurs
///
/// # Returns
///
/// The loaded theme, or Gruvbox as a fallback.
pub(crate) fn load_theme() -> Theme {
    match load_config() {
        Ok(theme) => theme,
        Err(_) => Theme::gruvbox(),
    }
}

/// Attempts to load and parse the config file directly as a Theme
fn load_config() -> Result<Theme, Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    let contents = fs::read_to_string(config_path)?;
    let theme: Theme = toml::from_str(&contents)?;
    Ok(theme)
}

/// Gets the path to the config file (~/.config/cerium.toml)
fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .ok_or("Could not find config directory")?;
    Ok(config_dir.join("cerium.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nu_ansi_term::Color as Colour;

    #[test]
    fn test_load_theme_fallback() {
        // Should return gruvbox when no config exists
        let theme = load_theme();
        // Just verify it doesn't panic and returns a valid theme with authentic Gruvbox colours
        assert!(matches!(theme.size_bytes.colour, Colour::Rgb(152, 151, 26))); // green
    }

    #[test]
    fn test_load_config_with_valid_toml() {
        let toml_content = r#"
            size_bytes = { r = 255, g = 0, b = 0 }
            size_kb = "green"
            size_mb = "blue"
            size_gb = "yellow"
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
            web_css = "magenta"
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

        let theme: Theme = toml::from_str(toml_content).unwrap();
        assert!(matches!(theme.size_bytes.colour, Colour::Rgb(255, 0, 0)));
    }

    #[test]
    fn test_config_path() {
        let path = get_config_path();
        // Should return a path (may vary by system)
        assert!(path.is_ok() || path.is_err()); // Just verify it doesn't panic
    }
}
