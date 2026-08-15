// SPDX-License-Identifier: MIT

use crate::render::style::element::ElementStyle;
use crate::render::theme::colors::{self, Color, ColorPaint};
use crate::fs::symlink;

/// Applies color styling and formatting to data values based on their content.
///
/// `ValueStyle` provides centralised styling logic for column data values such as
/// sizes, dates, permissions, names, and numeric fields. Each method maps a raw
/// value to its appropriately colored terminal representation.
pub(crate) struct ValueStyle;

impl ValueStyle {
    /// Styles entry size with colors based on magnitude.
    ///
    /// # Parameters
    /// - `size`: The formatted size string (e.g., "1.2 MB", "45 KB").
    ///
    /// # Returns
    /// Bold-styled text with magnitude-appropriate color.
    pub(crate) fn size(size: &str) -> String {
        let color = if size.ends_with(" kB") || size.ends_with("KiB") {
            colors::theme().size_small.color
        } else if size.ends_with(" MB") || size.ends_with("MiB") {
            colors::theme().size_medium.color
        } else if size.ends_with(" GB") || size.ends_with("GiB") {
            colors::theme().size_large.color
        } else {
            colors::theme().size_none.color
        };

        ElementStyle::text(size, Some(color))
    }

    /// Styles entry names with special handling for symlinks and ignored files.
    ///
    /// # Parameters
    /// - `name`: The entry name (may contain symlink arrow `->`).
    /// - `color`: The base color for the entry.
    ///
    /// # Returns
    /// Styled name with appropriate formatting.
    pub(crate) fn name(name: &str, color: Color) -> String {
        // Symlink case
        if let Some((link_part, target)) = symlink::split_symlink(name) {
            // Style the link name
            let styled_link = colors::theme()
                .filekind_symlink
                .color
                .italic()
                .apply_to(link_part.trim_end());

            // Style the target name
            let styled_target = color.bold().apply_to(target.trim_start()).to_string();

            return format!(
                "{}{}{}",
                styled_link,
                symlink::SYMLINK_ARROW_WITH_SPACES,
                styled_target
            );
        }

        // Normal entries
        let styled = if name.contains("ignore") {
            color.strikethrough()
        } else {
            color.bold()
        };

        styled.apply_to(name).to_string()
    }

    /// Styles dates with colors indicating recency.
    ///
    /// # Parameters
    /// - `datetime`: The formatted timestamp string (e.g., "2 hours ago", "Jan 15").
    ///
    /// # Returns
    /// Bold-styled text with recency-appropriate color.
    pub(crate) fn datetime(datetime: &str) -> String {
        let theme = colors::theme();

        // Relative timestamps name their own age bucket. Absolute ones
        // (`--date-format=locale`/`timestamp`) carry no age in the string, so
        // they all share one colour rather than keying off the month name.
        let color = if datetime.contains("second") {
            theme.date_minute_old.color
        } else if datetime.contains("minute") {
            theme.date_hour_old.color
        } else if datetime.contains("hour") {
            theme.date_day_old.color
        } else if datetime.contains("day") {
            theme.date_week_old.color
        } else if datetime.contains("week") || datetime.contains("month") {
            theme.date_month_old.color
        } else {
            theme.date_older.color
        };

        ElementStyle::text(datetime, Some(color))
    }

    /// Styles Unix permission strings with character-by-character color coding.
    ///
    /// # Parameters
    /// - `permissions`: The permission string (e.g., "rwxr-xr-x", "drwxr-xr-x", ".rwxr-xr-x").
    ///
    /// # Returns
    /// String with each character individually styled.
    pub(crate) fn permissions(permissions: &str) -> String {
        let theme = colors::theme();

        permissions
            .chars()
            .map(|character| match character {
                // File type or "dot" prefix
                '.' => theme.permission_filetype.color.bold().apply_to("."),

                // Standard permissions
                'r' => theme.permission_read.color.bold().apply_to("r"),
                'w' => theme.permission_write.color.bold().apply_to("w"),
                'x' => theme.permission_exec.color.bold().apply_to("x"),
                '-' => theme.permission_no_access.color.normal().apply_to("-"),

                // File type indicators
                'd' | 'l' | 'b' | 'c' | 'p' | 's' => theme
                    .permission_filetype
                    .color
                    .bold()
                    .apply_to_char(character),

                // Special permission bits
                'S' | 'T' | 't' => theme
                    .permission_exec_sticky
                    .color
                    .bold()
                    .apply_to_char(character),

                // Numeric characters (for octal/hex)
                '0'..='9' => ElementStyle::numeric(&character.to_string()),

                // Anything else (just in case)
                other => theme.permission_filetype.color.bold().apply_to_char(other),
            })
            .collect()
    }
}
