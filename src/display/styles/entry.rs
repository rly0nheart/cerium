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
use crate::display::output::quotes::Quotes;
use crate::display::styles::text::TextStyle;
use crate::display::theme::colours::Colour;
use crate::display::theme::icons::{self, IconSettings};
use crate::fs::entry::Entry;
use crate::fs::hyperlink::{self, HyperlinkSettings};
use std::sync::Arc;

/// Represents the final visual presentation of an entry, ready for display.
pub(crate) struct EntryView {
    pub(crate) name: Arc<str>,
    pub(crate) colour: Colour,
}

/// Styling information for a filesystem entry (icon + colour)
#[derive(Debug, Clone)]
pub(crate) struct EntryStyle {
    pub(crate) icon: char,
    pub(crate) colour: Colour,
}

impl EntryStyle {
    /// Resolves the appropriate icon and colour for a filesystem entry using PHF maps
    pub(crate) fn from(entry: &Entry) -> Self {
        let name = entry.name().as_ref();
        let extension = entry.extension().as_ref();

        let icon = icons::icon_for_entry(
            name,
            extension,
            entry.is_dir(),
            entry.has_children(),
            entry.is_symlink(),
        );
        let colour = icons::colour_for_entry(name, extension, entry.is_dir(), entry.is_symlink());

        Self { icon, colour }
    }
}

/// A filesystem entry paired with its resolved styling information
pub(crate) struct StyledEntry<'a> {
    pub(crate) entry: &'a Entry,
    pub(crate) style: EntryStyle,
}

impl<'a> StyledEntry<'a> {
    pub(crate) fn new(entry: &'a Entry) -> Self {
        let style = EntryStyle::from(entry);
        Self { entry, style }
    }

    /// Renders the entry name with icon and styling
    ///
    /// # Parameters
    ///
    /// * `args` - Command-line arguments controlling display options
    /// * `add_alignment_space` - Whether to add space for grid/list alignment when entries aren't quoted
    /// # Returns
    ///
    /// A `EntryView` with the styled entry name
    pub(crate) fn load(&self, args: &Args, add_alignment_space: bool) -> EntryView {
        let mut name = String::new();

        // Add icon if enabled
        if IconSettings::enabled() {
            name.push(self.style.icon);
            name.push(' ');
        }

        let entry_name = if args.tree {
            // Tree mode skips quoting to match traditional `tree` command behavior.
            // Filenames display as-is without quotes, prioritizing clean hierarchical display.
            if HyperlinkSettings::is_enabled() {
                hyperlink::wrap_hyperlink(self.entry.name(), self.entry.path())
            } else {
                self.entry.name().to_string()
            }
        } else {
            // Determine quoting based on the ORIGINAL filename (not hyperlinked)
            let quotes = Quotes::new(self.entry.name());
            let quoted = quotes.apply(args.quote_name, add_alignment_space);

            // Then apply hyperlink to just the filename part if enabled
            if HyperlinkSettings::is_enabled() {
                // Hyperlink the original name, then insert it into the quoted result
                let hyperlinked_name =
                    hyperlink::wrap_hyperlink(self.entry.name(), self.entry.path());
                quoted.replace(self.entry.name().to_string().as_str(), &hyperlinked_name)
            } else {
                quoted
            }
        };

        name.push_str(&entry_name);

        // Apply text style with colour
        let styled_name = TextStyle::name(&name, self.style.colour);

        EntryView {
            name: Arc::from(styled_name.as_str()),
            colour: self.style.colour,
        }
    }
}
