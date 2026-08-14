// SPDX-License-Identifier: MIT

use crate::cli::args::Args;
use crate::display::layout::column::Column;
use crate::display::output::column_value;
use crate::display::styles::element::ElementStyle;
use crate::display::styles::entry::{EntryStyle, StyledEntry};
use crate::display::styles::value::ValueStyle;
use crate::display::theme::colors::{self, Color, ColorPaint};
use crate::fs::entry::Entry;

/// Provides styling logic for individual columns in the output display.
pub(crate) struct ColumnStyle;

impl ColumnStyle {
    /// Resolves the styled string value for a given column and entry.
    ///
    /// # Parameters
    /// - `entry`: The filesystem entry to display.
    /// - `column`: The column type to render.
    /// - `args`: Command-line arguments controlling display options.
    /// - `add_alignment_space`: Whether to add alignment spacing for the name column.
    ///
    /// # Returns
    /// The styled column value as a string with ANSI color codes.
    pub(crate) fn get(
        entry: &Entry,
        column: &Column,
        args: &Args,
        add_alignment_space: bool,
    ) -> String {
        if *column == Column::Name {
            return StyledEntry::new(entry).load(args, add_alignment_space).name;
        }

        let value = column_value::of(entry, column, args);
        Self::column_value(column, value.to_string(), EntryStyle::from(entry).color)
    }

    /// Applies appropriate styling to a column value based on the column type and content.
    ///
    /// # Parameters
    /// - `column`: The column type, which determines the styling rules.
    /// - `value`: The raw text value to style.
    /// - `color`: The base color to use (typically from entry type).
    ///
    /// # Returns
    /// A string with ANSI color codes applied for terminal display.
    fn column_value(column: &Column, value: String, color: Color) -> String {
        let theme = colors::theme();

        if value == "-" {
            theme.punctuation.color.normal().apply_to(&value)
        } else if value.parse::<f64>().is_ok() {
            ElementStyle::numeric(&value)
        } else {
            match column {
                Column::Name => ValueStyle::name(&value, color),
                #[cfg(all(feature = "magic", not(target_os = "android")))]
                Column::Magic => theme.magic.color.bold().apply_to(&value),

                #[cfg(feature = "checksum")]
                Column::Checksum(_) => theme.checksum.color.italic().apply_to(&value),

                Column::Xattr => theme.permission_attribute.color.normal().apply_to(&value),
                Column::Acl => theme.permission_acl.color.normal().apply_to(&value),
                Column::Mountpoint => theme.mountpoint.color.normal().apply_to(&value),
                Column::Context => theme.permission_context.color.normal().apply_to(&value),
                Column::Permissions => ValueStyle::permissions(&value),
                Column::BlockSize | Column::Size => ValueStyle::size(&value),
                Column::User => theme.user.color.normal().apply_to(&value),
                Column::Group => theme.group.color.normal().apply_to(&value),
                Column::Created | Column::Modified | Column::Accessed => {
                    ValueStyle::datetime(&value)
                }
                _ => ElementStyle::text(&value, None),
            }
        }
    }
}
