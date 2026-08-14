// SPDX-License-Identifier: MIT

use crate::display::layout::unicode_width::char_width;
use libc::{TIOCGWINSZ, ioctl, winsize};
use std::os::fd::AsRawFd;
use std::{io, mem};

/// Returns the current terminal width in columns via `TIOCGWINSZ` ioctl.
///
/// # Returns
/// The terminal width, or `80` if the query fails.
pub fn terminal() -> usize {
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

/// Measures the display width of text, skipping ANSI escape codes and respecting Unicode widths.
///
/// # Parameters
/// - `text`: The text string to measure (may contain ANSI escape codes).
///
/// # Returns
/// The visual width of the text in terminal columns.
pub fn measure(text: &str) -> usize {
    let mut width = 0;
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Check what type of escape sequence
            match chars.peek() {
                Some(&'[') => {
                    // CSI sequence (colors, styling)
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
