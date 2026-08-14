// SPDX-License-Identifier: MIT

use crate::display::theme::colors::{self, Color, ColorPaint};
use nu_ansi_term::Style;
use std::path::Display;

/// Provides styling for structural UI elements such as tree connectors, table headers,
/// and path titles.
pub(crate) struct ElementStyle;

impl ElementStyle {
    /// Styles tree connector characters (│, ├──, ╰──) in a subdued color.
    ///
    /// # Parameters
    /// - `connector`: The connector string (box-drawing characters).
    ///
    /// # Returns
    /// Dark grey styled connector text.
    pub(crate) fn tree_connector(connector: &str) -> String {
        colors::theme().tree_edge.color.normal().apply_to(connector)
    }

    /// Styles table column headers with bold, underlined default colored text.
    ///
    /// # Parameters
    /// - `name`: The header text (column name).
    ///
    /// # Returns
    /// Styled header text in fg color, bold, and underlined.
    pub(crate) fn table_header(name: &str) -> String {
        Style::new()
            .fg(colors::theme().header.color)
            .underline()
            .bold()
            .apply_to(name)
    }

    /// Styles directory path titles for recursive mode output.
    ///
    /// # Parameters
    /// - `path_display`: The path display object (typically from `Path::display()`).
    ///
    /// # Returns
    /// Styled path in blue, underlined.
    pub(crate) fn path_header(path_display: Display) -> String {
        Style::new()
            .fg(colors::theme().path.color)
            .underline()
            .apply_to(path_display.to_string().as_str())
    }

    /// Styles a summary string with bold themed numbers and italic themed labels.
    ///
    /// # Parameters
    /// - `text`: The formatted summary string (e.g., "3 directories and 5 files").
    ///
    /// # Returns
    /// Styled text with each numeric segment in bold theme color and the rest in italic theme color.
    pub(crate) fn summary(text: &str) -> String {
        let color = colors::theme().summary.color;
        Self::text(text, Some(color))
    }

    /// Styles mixed text by coloring numeric segments as bold cyan and the rest with a given color.
    ///
    /// # Parameters
    /// - `text`: The text to style, which may contain a mix of numeric and non-numeric segments.
    /// - `color`: The color to apply to non-numeric segments.
    ///
    /// # Returns
    /// Styled text with numeric segments in bold cyan and the remainder in the provided color.
    pub(crate) fn text(text: &str, color: Option<Color>) -> String {
        let style = Style::new();
        let color = color.unwrap_or_default();

        let mut result = String::new();
        let mut chunk = String::new();
        let mut in_digits = text.starts_with(|character: char| character.is_ascii_digit());

        for character in text.chars() {
            // Keep '.' and ',' attached to an active digit run so decimals and thousands
            // separators (e.g. "25.9", "1,024") render as a single numeric chunk.
            let is_digit =
                character.is_ascii_digit() || (in_digits && (character == '.' || character == ','));
            if is_digit != in_digits && !chunk.is_empty() {
                if in_digits {
                    result.push_str(&Self::numeric(&chunk));
                } else {
                    result.push_str(&style.fg(color).apply_to(&chunk));
                }
                chunk.clear();
                in_digits = is_digit;
            }
            chunk.push(character);
        }

        if !chunk.is_empty() {
            if in_digits {
                result.push_str(&Self::numeric(&chunk));
            } else {
                result.push_str(&style.fg(color).apply_to(&chunk));
            }
        }

        result
    }

    /// Styles numeric text as bold cyan.
    ///
    /// # Parameters
    /// - `text`: The numeric text to style.
    ///
    /// # Returns
    /// Bold cyan styled text.
    pub(crate) fn numeric(text: &str) -> String {
        colors::theme().numeric.color.bold().apply_to(text)
    }
}
