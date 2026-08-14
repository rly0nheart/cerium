// SPDX-License-Identifier: MIT

use crate::cli::args::Args;
use crate::display::classify;
use crate::display::output::quotes;
use crate::display::styles::value::ValueStyle;
use crate::display::theme::colors::{Color, ColorPaint};
use crate::display::theme::icons;
use crate::fs::entry::Entry;
use crate::fs::hyperlink;

/// Represents the final visual presentation of an entry, ready for display.
pub(crate) struct EntryView {
    pub(crate) name: String,
    pub(crate) color: Color,
}

/// Styling information for a filesystem entry (icon + color)
#[derive(Debug, Clone)]
pub(crate) struct EntryStyle {
    pub(crate) icon: char,
    pub(crate) color: Color,
}

impl EntryStyle {
    /// Resolves the appropriate icon and color for a filesystem entry using PHF maps.
    ///
    /// # Parameters
    /// - `entry`: The filesystem entry to resolve styling for.
    ///
    /// # Returns
    /// An [`EntryStyle`] with the resolved icon and color.
    pub(crate) fn from(entry: &Entry) -> Self {
        let name = entry.name().as_ref();
        let extension = entry.extension();

        let icon = icons::icon_for_entry(
            name,
            extension,
            entry.is_dir(),
            entry.has_children(),
            entry.is_symlink(),
        );
        let color = icons::color_for_entry(name, extension, entry.is_dir(), entry.is_symlink());

        Self { icon, color }
    }
}

/// A filesystem entry paired with its resolved styling information
pub(crate) struct StyledEntry<'a> {
    pub(crate) entry: &'a Entry,
    pub(crate) style: EntryStyle,
}

impl<'a> StyledEntry<'a> {
    /// Creates a new styled entry by resolving the style for the given entry.
    ///
    /// # Parameters
    /// - `entry`: The filesystem entry to style.
    ///
    /// # Returns
    /// A [`StyledEntry`] pairing the entry with its resolved styling.
    pub(crate) fn new(entry: &'a Entry) -> Self {
        let style = EntryStyle::from(entry);
        Self { entry, style }
    }

    /// Renders the entry name with icon and styling.
    ///
    /// # Parameters
    /// - `args`: Command-line arguments controlling display options.
    /// - `add_alignment_space`: Whether to add space for grid/list alignment when entries aren't quoted.
    ///
    /// # Returns
    /// An [`EntryView`] with the styled entry name.
    pub(crate) fn load(&self, args: &Args, add_alignment_space: bool) -> EntryView {
        let mut name = String::new();

        // Add styled icon if enabled
        if icons::enabled() {
            let styled_icon = self.style.color.bold().apply_to_char(self.style.icon);
            name.push_str(&styled_icon);
            name.push(' ');
        }

        let entry_name = if args.tree {
            // Tree mode skips quoting to match traditional `tree` command behavior.
            // Filenames display as-is without quotes, prioritizing clean hierarchical display.
            if hyperlink::enabled() {
                hyperlink::wrap_hyperlink(self.entry.name(), self.entry.path())
            } else {
                self.entry.name().to_string()
            }
        } else {
            // Determine quoting based on the ORIGINAL filename (not hyperlinked)
            let quoted = quotes::apply(self.entry.name(), args.quote_name, add_alignment_space);

            // Then apply hyperlink to just the filename part if enabled
            if hyperlink::enabled() {
                // Hyperlink the original name, then insert it into the quoted result
                let hyperlinked_name =
                    hyperlink::wrap_hyperlink(self.entry.name(), self.entry.path());
                quoted.replace(&**self.entry.name(), &hyperlinked_name)
            } else {
                quoted
            }
        };

        // Apply text style to the entry name (without icon)
        let styled_entry_name = ValueStyle::name(&entry_name, self.style.color);
        name.push_str(&styled_entry_name);

        // Append the `-F`/`--file-type`/`--slash` indicator last and
        // deliberately *unstyled*: `ls` never colors it, and keeping it
        // outside the styled span also lets width measurement count it for
        // grid/column alignment.
        if let Some(symbol) = classify::indicator(self.entry, args) {
            name.push(symbol);
        }

        EntryView {
            name,
            color: self.style.color,
        }
    }
}
