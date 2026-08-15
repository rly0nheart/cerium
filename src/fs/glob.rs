// SPDX-License-Identifier: MIT

//! Glob pattern matching using POSIX `fnmatch`.
//!
//! Supports wildcard patterns:
//! - `*` matches any sequence of characters
//! - `?` matches any single character
//! - `[abc]` matches any one character in the set

use std::ffi::CString;

/// A glob pattern for matching filenames, compared case-insensitively.
pub struct Glob {
    pattern: CString,
}

impl Glob {
    /// Prepares a glob pattern for matching.
    ///
    /// # Parameters
    /// - `pattern`: A glob string where `*` matches any sequence and `?` matches any single character.
    ///
    /// # Returns
    /// A [`Glob`], or an error message if the pattern contains a null byte.
    pub fn new(pattern: &str) -> Result<Self, String> {
        CString::new(pattern)
            .map(|pattern| Self { pattern })
            .map_err(|_| "Invalid pattern: contains null byte".to_string())
    }

    /// Tests if the pattern matches the given text.
    ///
    /// # Parameters
    /// - `text`: The string to match against. Returns `false` if it contains a null byte.
    pub fn is_match(&self, text: &str) -> bool {
        let Ok(text) = CString::new(text) else {
            return false;
        };

        unsafe { libc::fnmatch(self.pattern.as_ptr(), text.as_ptr(), libc::FNM_CASEFOLD) == 0 }
    }
}
