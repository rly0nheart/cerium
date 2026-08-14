// SPDX-License-Identifier: MIT

use std::fs;
use std::path::Path;

/// The arrow used to separate a symlink name from its target: ->
pub const SYMLINK_ARROW: &str = "->";

/// The arrow as a string with spaces for display: " -> "
pub const SYMLINK_ARROW_WITH_SPACES: &str = " -> ";

/// Splits a symlink display string into name and target parts.
///
/// Detection keys off the spaced separator `" -> "` so that filenames merely
/// containing `->` aren't misread as symlinks. Genuine symlink display
/// strings always contain the spaced form because [`format_symlink`] builds
/// them with [`SYMLINK_ARROW_WITH_SPACES`].
///
/// If the text contains the spaced arrow separator, returns
/// `Some((name, target))` with the separator (and its surrounding spaces)
/// removed. Otherwise, returns `None`.
///
/// # Parameters
///
/// - `text`: The symlink display string (e.g., "name -> target").
///
/// # Returns
///
/// `Some((name, target))` if the text is a symlink format, `None` otherwise
///
/// # Examples
///
/// ```text
/// split_symlink("mylink -> /target")  // => Some(("mylink", "/target"))
/// split_symlink("regular_file")       // => None
/// ```
pub fn split_symlink(text: &str) -> Option<(&str, &str)> {
    text.find(SYMLINK_ARROW_WITH_SPACES).map(|index| {
        let (left, right_with_arrow) = text.split_at(index);
        let right = &right_with_arrow[SYMLINK_ARROW_WITH_SPACES.len()..];
        (left, right)
    })
}

/// Formats a symlink display name with the target.
///
/// Creates the standard symlink display format: `"name -> target"`
///
/// # Parameters
///
/// - `name`: The symlink name.
/// - `target`: The symlink target path.
///
/// # Returns
///
/// A formatted string in the format `"name -> target"`
///
/// # Examples
///
/// ```text
/// format_symlink("mylink", "/path/to/target")  // => "mylink -> /path/to/target"
/// ```
pub fn format_symlink(name: &str, target: &str) -> String {
    format!("{}{}{}", name, SYMLINK_ARROW_WITH_SPACES, target)
}

/// Reads the symlink target from the filesystem.
///
/// # Parameters
///
/// - `path`: The path to the symlink.
///
/// # Returns
///
/// The target path as a string, or an empty string if reading fails
///
/// # Examples
///
/// ```text
/// read_symlink_target(Path::new("/path/to/symlink"))  // => "" (non-existent path)
/// ```
pub fn read_symlink_target(path: &Path) -> String {
    fs::read_link(path)
        .ok()
        .and_then(|target| target.to_str().map(String::from))
        .unwrap_or_default()
}
