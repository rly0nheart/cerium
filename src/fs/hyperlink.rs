// SPDX-License-Identifier: MIT

use crate::cli::flags::ShowHyperlink;
use crate::display::output::terminal::is_tty;
use crate::display::output::toggle::Toggle;
use std::path::Path;

static HYPERLINKS: Toggle = Toggle::new(false);

/// Returns whether hyperlinks are currently enabled.
pub(crate) fn enabled() -> bool {
    HYPERLINKS.is_enabled()
}

/// Configures hyperlinks at startup from the CLI flag and terminal detection.
///
/// # Parameters
/// - `show_hyperlink`: The user-selected hyperlink mode (always, never, or auto).
pub fn setup(show_hyperlink: ShowHyperlink) {
    HYPERLINKS.set(match show_hyperlink {
        ShowHyperlink::Always => true,
        ShowHyperlink::Never => false,
        ShowHyperlink::Auto => is_tty(),
    });
}

/// Wraps text in an OSC 8 terminal hyperlink.
///
/// # Parameters
/// - `text`: The visible text to make clickable.
/// - `path`: The file path to link to.
///
/// # Returns
/// A string containing `text` wrapped with OSC 8 hyperlink escape sequences.
pub fn wrap_hyperlink(text: &str, path: &Path) -> String {
    // Convert to absolute path if relative
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        // Try to canonicalise (resolve symlinks and make absolute)
        std::env::current_dir()
            .ok()
            .and_then(|cwd| cwd.join(path).canonicalize().ok())
            .unwrap_or_else(|| path.to_path_buf())
    };

    // Generate file:// URL
    // Note: On Unix, file:// URLs should start with three slashes (file:/// not file://)
    let url = format!("file://{}", absolute_path.display());

    // OSC 8 format: \x1b]8;;URL\x1b\\text\x1b]8;;\x1b\\
    format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url, text)
}
