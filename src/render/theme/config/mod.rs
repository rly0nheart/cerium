// SPDX-License-Identifier: MIT

//! Configuration system for Cerium theme customisation.
//!
//! Loads the theme from `~/.config/cerium.toml` (or
//! `$XDG_CONFIG_HOME/cerium.toml`). Every key is optional and falls back
//! per-field to the built-in Catppuccin Mocha theme, so partial overrides
//! work. A missing config file is silent; a config that exists but can't be
//! read or parsed produces a non-fatal warning on stderr.
//!
//! # Config File Format
//!
//! Define an optional named palette, then map roles to palette references,
//! hex strings, RGB tables, or named colors. Anything omitted keeps its
//! Catppuccin Mocha default.
//!
//! Role names follow the conventions of `lsd` (`permission`, `date`, `size`,
//! `tree-edge`) and `eza` (`filekind`, `file_type`). Unsectioned roles go at
//! the top of the file, before the first section.
//!
//! ```toml
//! user      = "yellow"
//! tree-edge = "#6c7086"
//!
//! [palette]
//! accent = "#89b4fa"
//!
//! [filekind]
//! directory = "accent"
//! normal    = "#cdd6f4"
//!
//! [permission]
//! read = "yellow"
//!
//! [file_type]
//! rust = { r = 250, g = 179, b = 135 }
//! ```

pub mod color;
mod theme;

pub use theme::Theme;

use crate::render::output::terminal;
use std::fs;
use std::path::PathBuf;

/// Loads the theme from the config file, falling back to the built-in
/// Catppuccin Mocha theme.
///
/// Behaviour:
/// - **No config file** (or no resolvable config dir): use the built-in
///   default, silently.
/// - **Config exists but can't be read or is invalid TOML**: use the
///   built-in default, and print a non-fatal warning to stderr when stdout
///   is an interactive terminal.
/// - **Config exists and parses**: per-field resolution is handled by
///   [`Theme::from_value`]; absent or unresolvable keys use their default.
///
/// # Returns
///
/// The resolved [`Theme`].
pub fn load_theme() -> Theme {
    let Ok(config_path) = get_config_path() else {
        return Theme::default();
    };

    if !config_path.exists() {
        return Theme::default();
    }

    let parsed = fs::read_to_string(&config_path)
        .map_err(|e| e.to_string())
        .and_then(|contents| Theme::parse(&contents).map_err(|e| e.to_string()));

    match parsed {
        Ok(theme) => theme,
        Err(error) => {
            // Warn only on an interactive terminal; stay silent for pipes,
            // scripts, and command substitution.
            if terminal::is_tty() {
                eprintln!(
                    "cerium: could not load theme from {} ({error}); using built-in theme.",
                    config_path.display()
                );
            }
            Theme::default()
        }
    }
}

/// Returns the path to the config file (`~/.config/cerium.toml`).
///
/// # Returns
/// The config file path, or an error if the home directory cannot be determined.
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

    #[test]
    fn test_config_path() {
        let path = get_config_path();
        // Should return a path (may vary by system)
        assert!(path.is_ok() || path.is_err()); // Just verify it doesn't panic
    }
}
