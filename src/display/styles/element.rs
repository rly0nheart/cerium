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

use crate::display::theme::colours::{Colour, ColourPaint, RgbColours};
use nu_ansi_term::Style;
use std::path::Display;

/// Provides styling for structural UI elements such as tree connectors, table headers,
/// and path titles.
pub(crate) struct ElementStyle;

impl ElementStyle {
    /// Styles tree connector characters (│, ├──, ╰──) in a subdued colour.
    ///
    /// # Parameters
    /// - `connector`: The connector string (box-drawing characters).
    ///
    /// # Returns
    /// Dark grey styled connector text.
    pub(crate) fn tree_connector(connector: &str) -> String {
        Colour::DarkGray.normal().apply_to(connector)
    }

    /// Styles table column headers with bold, underlined default coloured text.
    ///
    /// # Parameters
    /// - `name`: The header text (column name).
    ///
    /// # Returns
    /// Styled header text in fg colour, bold, and underlined.
    pub(crate) fn table_header(name: &str) -> String {
        let style = Style::new();
        style.underline().bold().apply_to(name)
    }

    /// Styles directory path titles for recursive mode output.
    ///
    /// # Parameters
    /// - `path_display`: The path display object (typically from `Path::display()`).
    ///
    /// # Returns
    /// Styled path in blue, underlined.
    pub(crate) fn path_header(path_display: Display) -> String {
        let style = Style::new();
        style
            .underline()
            .apply_to(path_display.to_string().as_str())
    }

    /// Styles a summary string with bold themed numbers and italic themed labels.
    ///
    /// # Parameters
    /// - `text`: The formatted summary string (e.g., "3 directories and 5 files").
    ///
    /// # Returns
    /// Styled text with each numeric segment in bold theme colour and the rest in italic theme colour.
    pub(crate) fn summary(text: &str) -> String {
        let colour = RgbColours::summary();
        Self::text(text, Some(colour))
    }

    /// Styles mixed text by colouring numeric segments as bold cyan and the rest with a given colour.
    ///
    /// # Parameters
    /// - `text`: The text to style, which may contain a mix of numeric and non-numeric segments.
    /// - `colour`: The colour to apply to non-numeric segments.
    ///
    /// # Returns
    /// Styled text with numeric segments in bold cyan and the remainder in the provided colour.
    pub(crate) fn text(text: &str, colour: Option<Colour>) -> String {
        let style = Style::new();
        let colour = colour.unwrap_or_default();

        let mut result = String::new();
        let mut chunk = String::new();
        let mut in_digits = text.starts_with(|character: char| character.is_ascii_digit());

        for character in text.chars() {
            let is_digit = character.is_ascii_digit();
            if is_digit != in_digits && !chunk.is_empty() {
                if in_digits {
                    result.push_str(&Self::numeric(&chunk));
                } else {
                    result.push_str(&style.fg(colour).apply_to(&chunk));
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
                result.push_str(&style.fg(colour).apply_to(&chunk));
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
        Colour::Cyan.bold().apply_to(text)
    }
}
