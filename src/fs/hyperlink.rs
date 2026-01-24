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

use crate::cli::flags::ShowHyperlink;
use crate::output::terminal::is_tty;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

static HYPERLINKS_ENABLED: AtomicBool = AtomicBool::new(false);

pub(crate) struct HyperlinkSettings;

impl HyperlinkSettings {
    pub(crate) fn enable() {
        HYPERLINKS_ENABLED.store(true, Ordering::SeqCst);
    }

    pub(crate) fn disable() {
        HYPERLINKS_ENABLED.store(false, Ordering::SeqCst);
    }

    pub(crate) fn is_enabled() -> bool {
        HYPERLINKS_ENABLED.load(Ordering::SeqCst)
    }

    /// Setup hyperlinks at startup based on CLI flag / terminal detection
    pub(crate) fn setup(show_hyperlink: ShowHyperlink) {
        match show_hyperlink {
            ShowHyperlink::Always => Self::enable(),
            ShowHyperlink::Never => Self::disable(),
            ShowHyperlink::Auto => {
                if is_tty() {
                    Self::enable()
                } else {
                    Self::disable()
                }
            }
        }
    }
}

/// Wraps text in an OSC 8 terminal hyperlink.
///
/// Terminal hyperlinks use Operating System Command (OSC) 8 escape sequences
/// to create clickable links in supported terminals. The escape sequences are
/// zero-width (invisible) and don't affect layout calculations.
///
/// # Format
///
/// ```text
/// \x1b]8;;file:///path/to/file\x1b\\text\x1b]8;;\x1b\\
/// ```
///
/// Format breakdown:
/// - `\x1b]8;;` - Start hyperlink (OSC 8 introducer + empty params)
/// - `file:///absolute/path` - The URL (file:// protocol for local files)
/// - `\x1b\\` - String terminator (ST)
/// - `text` - The visible text that is clickable
/// - `\x1b]8;;\x1b\\` - End hyperlink (OSC 8 with empty URL)
///
/// # Parameters
///
/// * `text` - The visible text to make clickable
/// * `path` - The file path to link to
///
/// # Returns
///
/// A string containing the text wrapped with OSC 8 hyperlink escape sequences.
/// In terminals that don't support OSC 8, the escape sequences are ignored
/// and only the text is displayed.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use cerium::output::theme::hyperlink::wrap_hyperlink;
///
/// let path = Path::new("/home/user/file.txt");
/// let hyperlinked = wrap_hyperlink("file.txt", path);
/// // Displays as "file.txt" but is clickable in supported terminals
/// ```
///
/// # Terminal Compatibility
///
/// Supported terminals (as of 2025):
/// - iTerm2 (macOS)
/// - VSCode integrated terminal
/// - Kitty
/// - WezTerm
/// - foot
/// - GNOME Terminal (recent versions)
/// - Windows Terminal
///
/// Unsupported terminals gracefully degrade by ignoring the escape sequences.
pub(crate) fn wrap_hyperlink(text: &str, path: &Path) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_hyperlink_absolute_path() {
        let path = Path::new("/home/user/file.txt");
        let result = wrap_hyperlink("file.txt", path);

        // Should contain OSC 8 sequences and the file:// URL
        assert!(result.contains("\x1b]8;;file:///home/user/file.txt\x1b\\"));
        assert!(result.contains("file.txt"));
        assert!(result.ends_with("\x1b]8;;\x1b\\"));
    }

    #[test]
    fn test_wrap_hyperlink_contains_visible_text() {
        let path = Path::new("/tmp/test");
        let result = wrap_hyperlink("visible_text", path);

        // The visible text should be present
        assert!(result.contains("visible_text"));
    }

    #[test]
    fn test_wrap_hyperlink_format() {
        let path = Path::new("/test/path");
        let result = wrap_hyperlink("link", path);

        // Should start with OSC 8 opener
        assert!(result.starts_with("\x1b]8;;"));
        // Should end with OSC 8 closer
        assert!(result.ends_with("\x1b]8;;\x1b\\"));
        // Should contain string terminator
        assert!(result.contains("\x1b\\"));
    }
}
