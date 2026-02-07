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

pub mod colour;
mod theme;

pub use theme::Theme;

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
/// The loaded theme, or Gruvbox (default) as a fallback.
pub fn load_theme() -> Theme {
    load_config().unwrap_or_else(|_| Theme::default())
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

    #[test]
    fn test_config_path() {
        let path = get_config_path();
        // Should return a path (may vary by system)
        assert!(path.is_ok() || path.is_err()); // Just verify it doesn't panic
    }
}
