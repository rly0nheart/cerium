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
use crate::display::layout::unicode_width::char_width;
use crate::fs::entry::Entry;
use libc::{TIOCGWINSZ, ioctl, winsize};
use std::collections::HashMap;
use std::os::fd::AsRawFd;
use std::sync::Arc;
use std::{io, mem};

/// Centralised width calculator with caching for ANSI-aware text measurement.
pub struct Width {
    width_cache: HashMap<Arc<str>, usize>,
}

impl Width {
    /// Creates a new [`Width`] with an empty cache.
    pub fn new() -> Self {
        Self {
            width_cache: HashMap::new(),
        }
    }

    /// Calculates optimal column widths for all entries.
    ///
    /// # Parameters
    /// - `entries`: The filesystem entries to measure.
    /// - `columns`: The columns to calculate widths for.
    /// - `args`: Command-line arguments controlling display options.
    ///
    /// # Returns
    /// A `HashMap` mapping each column to its maximum required width.
    pub fn calculate(
        &mut self,
        entries: &[Entry],
        columns: &[Column],
        args: &Args,
    ) -> HashMap<Column, usize> {
        let mut widths: HashMap<Column, usize> = HashMap::new();

        // Initialise with header widths if enabled
        if args.headers {
            for column in columns {
                let header_width = self.measure_text_cached(column.header());
                widths.insert(*column, header_width);
            }
        } else {
            for column in columns {
                widths.insert(*column, 0);
            }
        }

        // Single pass over all entries
        for entry in entries {
            let row = Row::new(entry, args);

            for column in columns {
                let value = row.value(column);
                let width = self.measure_text_cached(&value);

                let current = *widths.get(column).unwrap_or(&0);
                if width > current {
                    widths.insert(*column, width);
                }
            }
        }

        widths
    }

    /// Returns the current terminal width in columns via `TIOCGWINSZ` ioctl.
    ///
    /// # Returns
    /// The terminal width, or `80` if the query fails.
    pub fn terminal_width() -> usize {
        {
            let fd = io::stdout().as_raw_fd();
            let mut winsize: winsize = unsafe { mem::zeroed() };

            let result = unsafe { ioctl(fd, TIOCGWINSZ, &mut winsize as *mut _) };

            if result == 0 && winsize.ws_col > 0 {
                winsize.ws_col as usize
            } else {
                // Fallback to 80 columns if ioctl fails
                80
            }
        }
    }

    /// Measures the display width of text with caching.
    ///
    /// # Parameters
    /// - `text`: The text to measure (may contain ANSI escape codes).
    ///
    /// # Returns
    /// The display width in characters (excluding ANSI codes).
    pub fn measure_text_cached(&mut self, text: &str) -> usize {
        // Try to get from cache first
        let text_arc = Arc::<str>::from(text);

        if let Some(&width) = self.width_cache.get(&text_arc) {
            return width;
        }

        // Measure and cache
        let width = Self::measure_ansi_text(text);
        self.width_cache.insert(text_arc, width);
        width
    }

    /// Measures the display width of text, skipping ANSI escape codes and respecting Unicode widths.
    ///
    /// # Parameters
    /// - `text`: The text string to measure (may contain ANSI escape codes).
    ///
    /// # Returns
    /// The visual width of the text in terminal columns.
    pub fn measure_ansi_text(text: &str) -> usize {
        let mut width = 0;
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // Check what type of escape sequence
                match chars.peek() {
                    Some(&'[') => {
                        // CSI sequence (colours, styling)
                        chars.next(); // consume '['
                        while let Some(&next_ch) = chars.peek() {
                            chars.next();
                            if next_ch.is_ascii_alphabetic() {
                                break;
                            }
                        }
                    }
                    Some(&']') => {
                        // OSC sequence (hyperlinks, titles, etc.)
                        chars.next(); // consume ']'
                        // Skip until string terminator: either \x1b\\ or \x07
                        while let Some(&next_ch) = chars.peek() {
                            chars.next();
                            if next_ch == '\x1b' {
                                // Check for \x1b\\ terminator
                                if chars.peek() == Some(&'\\') {
                                    chars.next(); // consume '\\'
                                    break;
                                }
                            } else if next_ch == '\x07' {
                                // BEL terminator (alternative)
                                break;
                            }
                        }
                    }
                    _ => {
                        // Unknown escape, skip just the escape char
                    }
                }
            } else {
                // Regular character - add its display width
                width += char_width(ch);
            }
        }

        width
    }

    /// Returns the number of cached measurements.
    ///
    /// Useful for debugging and performance analysis.
    #[allow(dead_code)]
    pub fn cache_size(&self) -> usize {
        self.width_cache.len()
    }

    /// Clears the measurement cache.
    ///
    /// This can be useful between rendering different directories
    /// to free memory, though in practice the cache is usually small.
    #[allow(dead_code)]
    pub fn clear_cache(&mut self) {
        self.width_cache.clear();
    }
}

impl Default for Width {
    fn default() -> Self {
        Self::new()
    }
}
