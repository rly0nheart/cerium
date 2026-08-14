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
