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

use crate::cli::args::Args;
use crate::display::layout::column::Column;
use crate::display::layout::row::Row;
use crate::display::styles::entry::StyledEntry;
use crate::display::styles::text::TextStyle;
use crate::display::theme::colours::{Colour, ColourPaint, RgbColours};
use crate::fs::entry::Entry;

pub(crate) struct ColumnStyle;

impl ColumnStyle {
    pub(crate) fn get(
        entry: &Entry,
        column: &Column,
        args: &Args,
        add_alignment_space: bool,
    ) -> String {
        let styled_entry = StyledEntry::new(entry);
        let style = styled_entry.load(args, add_alignment_space);

        if *column == Column::Name {
            style.name.to_string()
        } else {
            let row = Row::new(entry, args);
            let row_value = row.value(column);
            Self::column_value(column, row_value.to_string(), style.colour)
        }
    }

    /// Applies appropriate styling to a column value based on the column type and content.
    ///
    /// This is the main entry point for styling column values. It applies context-aware
    /// styling rules that vary based on:
    /// * The column type (size, timestamp, permissions, etc.)
    /// * The content (special values like `-`, numeric values)
    /// * User-provided colour preferences
    ///
    /// # Parameters
    ///
    /// * `column` - The column type, which determines the styling rules
    /// * `value` - The raw text value to style
    /// * `colour` - The base colour to use (typically from entry type)
    ///
    /// # Returns
    ///
    /// A string with ANSI colour codes applied for terminal display
    ///
    /// # Special Cases
    ///
    /// * `"-"` → Dark gray (placeholder value)
    /// * Numeric strings → Cyan bold
    /// * Symlinks (containing `⇒`) → Dual-coloured with arrow
    fn column_value(column: &Column, value: String, colour: Colour) -> String {
        if value == "-" {
            Colour::DarkGray.normal().apply_to(&value) // DarkGray for values that are "-"
        } else if value.parse::<f64>().is_ok() {
            // Cyan for numeric values
            Colour::Cyan.bold().apply_to(&value)
        } else {
            match column {
                Column::Name => TextStyle::name(&value, colour),
                #[cfg(all(feature = "magic", not(target_os = "android")))]
                Column::Magic => colour.bold().apply_to(&value),

                #[cfg(feature = "checksum")]
                Column::Checksum(_) => Colour::White.italic().apply_to(&value),

                Column::Xattr => Colour::Cyan.normal().apply_to(&value),
                Column::Acl => Colour::Green.normal().apply_to(&value),
                Column::Mountpoint => Colour::Magenta.normal().apply_to(&value),
                Column::Permissions => TextStyle::permissions(&value),
                Column::BlockSize | Column::Size => TextStyle::size(&value),
                Column::User => RgbColours::hen_of_the_day().normal().apply_to(&value),
                Column::Group => RgbColours::hen_of_the_night().normal().apply_to(&value),
                Column::Created | Column::Modified | Column::Accessed => {
                    TextStyle::datetime(&value)
                }
                _ => Colour::White.normal().apply_to(&value), // Anything else is white
            }
        }
    }
}
