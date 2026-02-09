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

use std::sync::Once;

unsafe extern "C" {
    fn wcwidth(wc: libc::wchar_t) -> libc::c_int;
}

static LOCALE_INIT: Once = Once::new();

/// Initialises the locale for proper UTF-8 character width detection.
///
/// This function uses `setlocale(LC_CTYPE, "")` to inherit the locale
/// from the environment, which is necessary for `wcwidth()` to correctly
/// handle Unicode characters.
fn init_locale() {
    LOCALE_INIT.call_once(|| {
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
    init_locale();

    let wc = ch as libc::wchar_t;
    let width = unsafe { wcwidth(wc) };

    // wcwidth returns -1 for non-printable characters; use 1 as fallback
    if width < 0 { 1 } else { width as usize }
}
