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

//! Glob pattern matching using POSIX regex.
//!
//! Supports wildcard patterns:
//! - `*` matches any sequence of characters
//! - `?` matches any single character

use std::ffi::CString;
use std::mem::MaybeUninit;

/// A compiled glob pattern for matching filenames.
///
/// Wraps a POSIX `regex_t` compiled with case-insensitive, anchored matching.
/// The compiled regex is freed on drop.
pub struct Glob {
    inner: libc::regex_t,
}

impl Glob {
    /// Compiles a glob pattern into a matcher.
    ///
    /// # Parameters
    /// - `pattern`: A glob string where `*` matches any sequence and `?` matches any single character.
    ///
    /// # Returns
    /// A compiled [`Glob`] or an error message if the pattern is invalid.
    pub fn new(pattern: &str) -> Result<Self, String> {
        let regex_pattern = Self::to_regex(pattern);

        let c_pattern =
            CString::new(regex_pattern).map_err(|_| "Invalid pattern: contains null byte")?;

        let mut regex = MaybeUninit::<libc::regex_t>::uninit();
        let flags = libc::REG_EXTENDED | libc::REG_ICASE | libc::REG_NOSUB;

        let result = unsafe { libc::regcomp(regex.as_mut_ptr(), c_pattern.as_ptr(), flags) };

        if result == 0 {
            Ok(Self {
                inner: unsafe { regex.assume_init() },
            })
        } else {
            let initialized = unsafe { regex.assume_init() };
            Err(Self::error_message(&initialized, result))
        }
    }

    /// Tests if the compiled pattern matches the given text.
    ///
    /// # Parameters
    /// - `text`: The string to match against. Returns `false` if it contains a null byte.
    pub fn is_match(&self, text: &str) -> bool {
        let Ok(c_text) = CString::new(text) else {
            return false;
        };

        let result =
            unsafe { libc::regexec(&self.inner, c_text.as_ptr(), 0, std::ptr::null_mut(), 0) };

        result == 0
    }

    /// Converts a glob pattern to an anchored POSIX extended regex string.
    ///
    /// # Parameters
    /// - `pattern`: The glob pattern to convert. Metacharacters are escaped; `*` and `?` are translated.
    fn to_regex(pattern: &str) -> String {
        let mut result = String::with_capacity(pattern.len() * 2 + 2);
        result.push('^');

        for c in pattern.chars() {
            match c {
                '*' => result.push_str(".*"),
                '?' => result.push('.'),
                '.' | '+' | '(' | ')' | '[' | ']' | '{' | '}' | '^' | '$' | '|' | '\\' => {
                    result.push('\\');
                    result.push(c);
                }
                _ => result.push(c),
            }
        }

        result.push('$');
        result
    }

    /// Extracts a human-readable error message from a failed `regcomp` call.
    ///
    /// # Parameters
    /// - `regex`: The (initialised) regex that failed compilation.
    /// - `error_code`: The non-zero error code returned by `regcomp`.
    fn error_message(regex: &libc::regex_t, error_code: i32) -> String {
        // c_char is i8 on most platforms but u8 on Android
        let mut buffer = [0 as libc::c_char; 256];
        unsafe {
            libc::regerror(error_code, regex, buffer.as_mut_ptr(), buffer.len());
        }

        let c_str = unsafe { std::ffi::CStr::from_ptr(buffer.as_ptr()) };
        c_str.to_string_lossy().into_owned()
    }
}

impl Drop for Glob {
    /// Frees the underlying POSIX regex resources.
    fn drop(&mut self) {
        unsafe {
            libc::regfree(&mut self.inner);
        }
    }
}

/// # Safety
/// The POSIX `regex_t` is self-contained after compilation and safe to send across threads.
unsafe impl Send for Glob {}
