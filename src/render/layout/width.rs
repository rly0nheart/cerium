// SPDX-License-Identifier: MIT

use std::cell::Cell;

unsafe extern "C" {
    fn wcwidth(wc: libc::wchar_t) -> libc::c_int;
}

thread_local! {
    /// Whether `setlocale` has run on this thread. Checked once per character,
    /// so it stays a plain `Cell` rather than a `Once`.
    static LOCALE_READY: Cell<bool> = const { Cell::new(false) };
}

/// Initialises the locale for proper UTF-8 character width detection.
///
/// This function uses `setlocale(LC_CTYPE, "")` to inherit the locale
/// from the environment, which is necessary for `wcwidth()` to correctly
/// handle Unicode characters.
fn init_locale() {
    LOCALE_READY.with(|ready| {
        if ready.replace(true) {
            return;
        }
        unsafe {
            // Empty string means inherit from environment (LANG, LC_CTYPE, etc.)
            libc::setlocale(libc::LC_CTYPE, c"".as_ptr());
        }
    });
}

/// Returns the display width of a Unicode character using libc's `wcwidth()`.
///
/// # Parameters
/// - `ch`: The character to measure.
///
/// # Returns
/// The display width (0, 1, or 2), or `1` as fallback for non-printable characters.
pub fn char_width(ch: char) -> usize {
    // ASCII printables are always one column: skip the locale setup and FFI call.
    if ch.is_ascii_graphic() || ch == ' ' {
        return 1;
    }

    init_locale();

    let width = unsafe { wcwidth(ch as libc::wchar_t) };

    // wcwidth returns -1 for non-printable characters; use 1 as fallback
    if width < 0 { 1 } else { width as usize }
}

/// Measures the display width of text, skipping ANSI escape codes and respecting Unicode widths.
///
/// # Parameters
/// - `text`: The text string to measure (may contain ANSI escape codes).
///
/// # Returns
/// The visual width of the text in terminal columns.
pub fn measure(text: &str) -> usize {
    let mut width = 0;
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Check what type of escape sequence
            match chars.peek() {
                Some(&'[') => {
                    // CSI sequence (colors, styling)
                    chars.next(); // consume '['
                    while let Some(&next_ch) = chars.peek() {
                        chars.next();
                        if next_ch.is_ascii_alphabetic() {
                            break;
                        }
                    }
                }
                Some(&']') => {
                    // OSC sequence (hyperlinks, titles, etc.)
                    chars.next(); // consume ']'
                    // Skip until string terminator: either \x1b\\ or \x07
                    while let Some(&next_ch) = chars.peek() {
                        chars.next();
                        if next_ch == '\x1b' {
                            // Check for \x1b\\ terminator
                            if chars.peek() == Some(&'\\') {
                                chars.next(); // consume '\\'
                                break;
                            }
                        } else if next_ch == '\x07' {
                            // BEL terminator (alternative)
                            break;
                        }
                    }
                }
                _ => {
                    // Unknown escape, skip just the escape char
                }
            }
        } else {
            // Regular character - add its display width
            width += char_width(ch);
        }
    }

    width
}
