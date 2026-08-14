// SPDX-License-Identifier: MIT

//! Shell-safe quoting of entry names.
//!
//! A symlink renders as `link -> target`, so each side is quoted on its own
//! and the arrow is left bare.

use crate::cli::flags::QuoteStyle;
use crate::fs::symlink::{SYMLINK_ARROW_WITH_SPACES, split_symlink};

/// Characters that force quoting: shell metacharacters, globs, and brackets.
const SPECIAL: &[char] = &[
    '\\', '\'', '"', '`', '$', '&', '|', ';', '<', '>', '(', ')', '[', ']', '{', '}', '*', '?',
    '!', '#', '~', '%', '^',
];

/// Quotes `text` in the given style.
///
/// # Parameters
/// - `text`: The name to quote, possibly a `link -> target` pair.
/// - `style`: The quoting style to apply.
/// - `add_alignment_space`: Under [`QuoteStyle::Auto`], pads unquoted names by
///   one space so they line up with quoted ones in the same listing.
pub fn apply(text: &str, style: QuoteStyle, add_alignment_space: bool) -> String {
    match style {
        QuoteStyle::Never => text.to_string(),
        QuoteStyle::Single => each_side(text, |part| wrap(part, '\'')),
        QuoteStyle::Double => each_side(text, |part| wrap(part, '"')),
        QuoteStyle::Auto => {
            let quoted = each_side(text, |part| {
                if has_special_chars(part) {
                    wrap(part, '\'')
                } else {
                    part.to_string()
                }
            });

            if add_alignment_space && !quoted.starts_with('\'') {
                format!(" {}", quoted)
            } else {
                quoted
            }
        }
    }
}

/// Reports whether `text` needs shell quoting.
///
/// # Parameters
/// - `text`: The name to inspect, possibly a `link -> target` pair.
pub(crate) fn is_quotable(text: &str) -> bool {
    match split_symlink(text) {
        Some((left, right)) => {
            has_special_chars(left.trim_end()) || has_special_chars(right.trim_start())
        }
        None => has_special_chars(text),
    }
}

/// Applies `quote` to each side of a symlink, or to the whole name.
///
/// # Parameters
/// - `text`: The name to process.
/// - `quote`: Called with each part that needs quoting.
fn each_side(text: &str, quote: impl Fn(&str) -> String) -> String {
    match split_symlink(text) {
        Some((left, right)) => format!(
            "{}{}{}",
            quote(left.trim_end()),
            SYMLINK_ARROW_WITH_SPACES,
            quote(right.trim_start())
        ),
        None => quote(text),
    }
}

/// Reports whether a string holds whitespace or a shell metacharacter.
///
/// # Parameters
/// - `text`: The text to inspect.
fn has_special_chars(text: &str) -> bool {
    text.chars()
        .any(|character| character.is_whitespace() || SPECIAL.contains(&character))
}

/// Wraps text in `quote`, escaping any occurrence of it inside.
///
/// # Parameters
/// - `text`: The text to wrap.
/// - `quote`: The quote character to surround it with.
fn wrap(text: &str, quote: char) -> String {
    let mut quoted = String::with_capacity(text.len() + 2);
    quoted.push(quote);

    for character in text.chars() {
        if character == quote {
            quoted.push('\\');
        }
        quoted.push(character);
    }

    quoted.push(quote);
    quoted
}
