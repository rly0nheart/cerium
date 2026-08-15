// SPDX-License-Identifier: MIT

use crate::cli::flags::ShowColor;
use crate::render::output::terminal;
use crate::render::output::toggle::Toggle;
use crate::render::theme::config::Theme;
use nu_ansi_term::Style;
use std::sync::OnceLock;

pub(crate) use nu_ansi_term::Color;

static COLORS: Toggle = Toggle::new(true);
static THEME: OnceLock<Theme> = OnceLock::new();

/// Checks whether color output is currently enabled.
pub(crate) fn enabled() -> bool {
    COLORS.is_enabled()
}

/// Configures color output at startup from the CLI flag and terminal detection.
///
/// # Parameters
/// - `show_color`: The user's color preference from the CLI.
pub fn setup(show_color: ShowColor) {
    COLORS.set(match show_color {
        ShowColor::Always => true,
        ShowColor::Never => false,
        ShowColor::Auto => terminal::colors_enabled() && terminal::is_tty(),
    });
}

/// Trait for applying color styles to text, respecting the global color toggle.
pub(crate) trait ColorPaint {
    /// Applies this style to a string slice, returning plain text when colors are disabled.
    fn apply_to(&self, text: &str) -> String;
    /// Applies this style to a single character, returning plain text when colors are disabled.
    fn apply_to_char(&self, c: char) -> String;
}

impl ColorPaint for Style {
    /// Applies this style to a string slice, returning plain text when colors are disabled.
    ///
    /// # Parameters
    /// - `text`: The text to style.
    ///
    /// # Returns
    /// The styled string, or the original text if colors are disabled.
    fn apply_to(&self, text: &str) -> String {
        if enabled() {
            self.paint(text).to_string()
        } else {
            text.to_string()
        }
    }

    /// Applies this style to a single character, returning plain text when colors are disabled.
    ///
    /// # Parameters
    /// - `c`: The character to style.
    ///
    /// # Returns
    /// The styled character as a string, or the plain character if colors are disabled.
    fn apply_to_char(&self, c: char) -> String {
        if enabled() {
            self.paint(c.to_string()).to_string()
        } else {
            c.to_string()
        }
    }
}

/// Stores the theme every color is read from (called once at startup).
///
/// # Parameters
/// - `theme`: The resolved theme.
pub fn init(theme: Theme) {
    THEME.set(theme).ok();
}

/// Returns the active theme.
///
/// Every color in the output comes from one of its roles; there is no
/// built-in palette to pick from.
pub(crate) fn theme() -> &'static Theme {
    THEME
        .get()
        .expect("Theme not initialised - call colors::init() first")
}
