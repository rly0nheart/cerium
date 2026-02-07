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
use crate::fs::symlink;
use std::path::Display;

/// Applies colour styling and formatting to text values based on their content and context.
///
/// `TextStyle` provides centralised styling logic for all text elements in the output,
/// including column values, icons, tree connectors, and headers. It uses ANSI colour
/// codes and terminal styling to create visually distinct and informative displays.
pub(crate) struct TextStyle;

impl TextStyle {
    /// Styles entry size with colours based on magnitude.
    ///
    /// Applies a colour gradient from green (small files) through yellow to red (large files),
    /// making it easy to visually identify file sizes at a glance.
    ///
    /// # Parameters
    ///
    /// * `size` - The formatted size string (e.g., "1.2 MB", "45 KB")
    ///
    /// # Returns
    ///
    /// Bold-styled text with magnitude-appropriate colour
    ///
    /// # Color Mapping
    ///
    /// * Bytes (B) → Pine glade green
    /// * Kilobytes (kB/KiB) → Leaf green
    /// * Megabytes (MB/MiB) → Fern green
    /// * Gigabytes (GB/GiB) → Gleaming mint
    /// * Other/Unknown → Red (fallback)
    pub(crate) fn size(size: &str) -> String {
        let colour = if size.ends_with(" kB") || size.ends_with("KiB") {
            RgbColours::leaf_green()
        } else if size.ends_with(" MB") || size.ends_with("MiB") {
            RgbColours::fern()
        } else if size.ends_with(" GB") || size.ends_with("GiB") {
            RgbColours::gleaming_mint()
        } else {
            RgbColours::pine_glade()
        };

        colour.bold().apply_to(size)
    }

    /// Styles tree connector characters (│, ├──, ╰──) in a subdued colour.
    ///
    /// Tree connectors are displayed in dark gray to make them visually distinct
    /// from entry names while still showing the hierarchical structure clearly.
    ///
    /// # Parameters
    ///
    /// * `connector` - The connector string (box-drawing characters)
    ///
    /// # Returns
    ///
    /// Dark gray styled connector text
    pub(crate) fn tree_connector(connector: &str) -> String {
        Colour::DarkGray.normal().apply_to(connector)
    }

    /// Styles entry names with special handling for symlinks and ignored files.
    ///
    /// # Parameters
    ///
    /// * `name` - The entry name (may contain symlink arrow `⇒`)
    /// * `colour` - The base colour for the entry
    ///
    /// # Returns
    ///
    /// Styled name with appropriate formatting
    ///
    /// # Special Styling
    ///
    /// * **Symlinks**: Link name in italic, arrow in plain text, target in base colour
    /// * **Ignored files**: Strikethrough style if name contains "ignore"
    /// * **Normal files**: Bold text in base colour
    pub(crate) fn name(name: &str, colour: Colour) -> String {
        // Symlink case
        if let Some((link_part, target)) = symlink::split_symlink(name) {
            // Style the link name
            let styled_link = Colour::Blue
                .italic()
                .apply_to(link_part.trim_end())
                .to_string();

            // Style the target name
            let styled_target = colour.bold().apply_to(target.trim_start()).to_string();

            return format!(
                "{}{}{}",
                styled_link,
                symlink::SYMLINK_ARROW_WITH_SPACES,
                styled_target
            );
        }

        // Normal entries
        let styled = if name.contains("ignore") {
            colour.strikethrough()
        } else {
            colour.bold()
        };

        styled.apply_to(name).to_string()
    }

    /// Styles dates with colours indicating recency.
    ///
    /// Uses a colour gradient from light blue (recent) to darker blues (older),
    /// making it easy to identify when files were last modified at a glance.
    ///
    /// # Parameters
    ///
    /// * `datetime` - The formatted timestamp string (e.g., "2 hours ago", "Jan 15")
    ///
    /// # Returns
    ///
    /// Bold-styled text with recency-appropriate colour
    ///
    /// # Color Mapping (Recent to Old)
    ///
    /// * Seconds/Minutes → Frost glimmer (lightest blue)
    /// * Hours → Crystal blue
    /// * Days → Cerulean
    /// * Weeks → Azure sky
    /// * Months → Royal blue / Ocean blue
    /// * Jan-Dec → Various blues matching seasonal progression
    pub(crate) fn datetime(datetime: &str) -> String {
        let colour = if datetime.contains("second") {
            RgbColours::frost_glimmer()
        } else if datetime.contains("minute") {
            RgbColours::crystal_blue()
        } else if datetime.contains("hour") {
            RgbColours::cerulean()
        } else if datetime.contains("day") {
            RgbColours::azure_sky()
        } else if datetime.contains("week") {
            RgbColours::royal_blue()
        } else if datetime.contains("month") {
            RgbColours::ocean_blue()
        } else if datetime.contains("Jan") {
            RgbColours::frost_glimmer()
        } else if datetime.contains("Feb") {
            RgbColours::crystal_blue()
        } else if datetime.contains("Mar") {
            RgbColours::cerulean()
        } else if datetime.contains("Apr") {
            RgbColours::azure_sky()
        } else if datetime.contains("May") {
            RgbColours::royal_blue()
        } else if datetime.contains("Jun") {
            RgbColours::ocean_blue()
        } else if datetime.contains("Jul") {
            RgbColours::sapphire_shine()
        } else if datetime.contains("Aug") {
            RgbColours::sky_mist()
        } else if datetime.contains("Sep") {
            RgbColours::ice_crystal()
        } else if datetime.contains("Oct") {
            RgbColours::midnight_blue()
        } else if datetime.contains("Nov") {
            RgbColours::sapphire_shine()
        } else if datetime.contains("Dec") {
            RgbColours::ice_crystal()
        } else {
            RgbColours::frost_glimmer()
        };

        colour.bold().apply_to(datetime)
    }

    /// Styles Unix permission strings with character-by-character colour coding.
    ///
    /// Each character in the permission string receives its own colour based on
    /// its meaning, making it easy to quickly parse permission information.
    ///
    /// # Parameters
    ///
    /// * `permissions` - The permission string (e.g., "rwxr-xr-x", "drwxr-xr-x", ".rwxr-xr-x")
    ///
    /// # Returns
    ///
    /// String with each character individually styled
    ///
    /// # Character Color Mapping
    ///
    /// * `.` (dot prefix) → White bold
    /// * `r` (read) → Yellow bold
    /// * `w` (write) → Red bold
    /// * `x` (execute) → Green bold
    /// * `-` (no permission) → Dark gray
    /// * `d`, `l`, `b`, `c`, `p`, `s` (file types) → Blue bold
    /// * `S`, `T`, `t` (special bits) → Magenta bold
    /// * `0-9` (octal/hex) → Cyan bold
    /// * Other → White bold (fallback)
    pub(crate) fn permissions(permissions: &str) -> String {
        permissions
            .chars()
            .map(|character| match character.to_ascii_lowercase() {
                // File type or "dot" prefix
                '.' => Colour::White.bold().apply_to("."),

                // Standard permissions
                'r' => Colour::Yellow.bold().apply_to("r"),
                'w' => Colour::Red.bold().apply_to("w"),
                'x' => Colour::Green.bold().apply_to("x"),
                '-' => Colour::DarkGray.normal().apply_to("-"),

                // File type indicators
                'd' | 'l' | 'b' | 'c' | 'p' | 's' => {
                    Colour::Blue.bold().apply_to(&character.to_string())
                }

                // Special permission bits
                'S' | 'T' | 't' => Colour::Magenta.bold().apply_to(&character.to_string()),

                // Numeric characters (for octal/hex)
                '0'..='9' => Colour::Cyan.bold().apply_to(&character.to_string()),

                // Anything else (just in case)
                other => Colour::White.bold().apply_to(&other.to_string()),
            })
            .map(|s| s.to_string())
            .collect::<String>()
    }

    /// Styles table column headers with bold, underlined white text.
    ///
    /// # Parameters
    ///
    /// * `name` - The header text (column name)
    ///
    /// # Returns
    ///
    /// Styled header text in white, bold, and underlined
    pub(crate) fn table_header(name: &str) -> String {
        Colour::White.underline().bold().apply_to(name)
    }

    /// Styles directory path titles for recursive mode output.
    ///
    /// Used when displaying section headers in recursive directory listings,
    /// providing clear visual separation between different directory levels.
    ///
    /// # Parameters
    ///
    /// * `path_display` - The path display object (typically from `Path::display()`)
    ///
    /// # Returns
    ///
    /// Styled path in blue, underlined, with appropriate quoting applied
    ///
    /// # Formatting
    ///
    /// * Applies shell-style quoting if path contains special characters
    /// * Uses blue colour with underline for prominence
    /// * Normal weight (not bold) for readability
    pub(crate) fn path_display(path_display: Display) -> String {
        Colour::Blue
            .underline()
            .apply_to(path_display.to_string().as_str())
    }
}
