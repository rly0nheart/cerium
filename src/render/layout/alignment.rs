// SPDX-License-Identifier: MIT

/// Text alignment direction within a column.
#[derive(Debug, Copy, Clone)]
pub enum Alignment {
    Left,
    Right,
}

/// Pads a string to the target width using the given alignment.
///
/// # Parameters
/// - `value`: The string to pad (may contain ANSI codes).
/// - `visible`: The value's display width, ANSI codes excluded.
/// - `width`: The target display width.
/// - `alignment`: Whether to left- or right-align the value.
///
/// # Returns
/// The padded string.
pub fn pad(value: &str, visible: usize, width: usize, alignment: Alignment) -> String {
    let padding = " ".repeat(width.saturating_sub(visible));
    match alignment {
        Alignment::Right => padding + value,
        Alignment::Left => value.to_string() + &padding,
    }
}
