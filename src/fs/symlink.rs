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
use std::fs;
use std::path::Path;

/// The arrow character used to separate symlink name from target: ⇒
pub(crate) const SYMLINK_ARROW: char = '\u{21D2}';

/// The arrow as a string with spaces for display: " ⇒ "
pub(crate) const SYMLINK_ARROW_WITH_SPACES: &str = " \u{21D2} ";

/// Splits a symlink display string into name and target parts.
///
/// If the text contains the arrow separator, returns `Some((name, target))`.
/// Otherwise, returns `None`.
///
/// # Parameters
///
/// * `text` - The symlink display string (e.g., "name ⇒ target")
///
/// # Returns
///
/// `Some((name, target))` if the text is a symlink format, `None` otherwise
///
/// # Examples
///
/// ```
/// let (name, target) = symlink::split_symlink("mylink ⇒ /target").unwrap();
/// assert_eq!(name, "mylink ");
/// assert_eq!(target, " /target");
///
/// assert!(symlink::split_symlink("regular_file").is_none());
/// ```
pub(crate) fn split_symlink(text: &str) -> Option<(&str, &str)> {
    text.find(SYMLINK_ARROW).map(|index| {
        let (left, right_with_arrow) = text.split_at(index);
        let right = &right_with_arrow[SYMLINK_ARROW.len_utf8()..];
        (left, right)
    })
}

/// Formats a symlink display name with the target.
///
/// Creates the standard symlink display format: `"name ⇒ target"`
///
/// # Parameters
///
/// * `name` - The symlink name
/// * `target` - The symlink target path
///
/// # Returns
///
/// A formatted string in the format `"name ⇒ target"`
///
/// # Examples
///
/// ```
/// use cerium::output::symlink::format_symlink;
///
/// let display = format_symlink("mylink", "/path/to/target");
/// assert_eq!(display, "mylink ⇒ /path/to/target");
/// ```
pub(crate) fn format_symlink(name: &str, target: &str) -> String {
    format!("{}{}{}", name, SYMLINK_ARROW_WITH_SPACES, target)
}

/// Reads the symlink target from the filesystem.
///
/// # Parameters
///
/// * `path` - The path to the symlink
///
/// # Returns
///
/// The target path as a string, or an empty string if reading fails
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use cerium::output::symlink::read_symlink_target;
///
/// let target = read_symlink_target(Path::new("/path/to/symlink"));
/// println!("Target: {}", target);
/// ```
pub(crate) fn read_symlink_target(path: &Path) -> String {
    fs::read_link(path)
        .ok()
        .and_then(|target| target.to_str().map(String::from))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_symlink() {
        assert_eq!(format_symlink("link", "target"), "link ⇒ target");
        assert_eq!(
            format_symlink("mylink", "/path/to/target"),
            "mylink ⇒ /path/to/target"
        );
    }

    #[test]
    fn test_split_symlink() {
        let (name, target) = split_symlink("link ⇒ target").unwrap();
        assert_eq!(name, "link ");
        assert_eq!(target, " target");

        let (name, target) = split_symlink("a⇒b").unwrap();
        assert_eq!(name, "a");
        assert_eq!(target, "b");

        assert!(split_symlink("regular_file").is_none());
    }

    #[test]
    fn test_split_symlink_with_spaces() {
        let (name, target) = split_symlink("my link ⇒ my target").unwrap();
        assert_eq!(name, "my link ");
        assert_eq!(target, " my target");
    }

    #[test]
    fn test_arrow_constant() {
        assert_eq!(SYMLINK_ARROW, '⇒');
        assert_eq!(SYMLINK_ARROW_WITH_SPACES, " ⇒ ");
    }
}
