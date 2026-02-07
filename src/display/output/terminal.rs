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

use std::os::unix::io::AsRawFd;
use std::{env, io};

/// Checks if coloured output should be enabled based on environment variables
/// and terminal capabilities.
///
/// Follows common conventions for colour detection:
/// 1. NO_COLOR environment variable disables colours (universal override)
/// 2. FORCE_COLOR or CLICOLOR_FORCE explicitly enables colours
/// 3. CLICOLOR=0 disables colours
/// 4. TERM environment variable indicates terminal capabilities
/// 5. COLORTERM indicates modern colour support
/// 6. Falls back to TTY detection
///
/// # Returns
///
/// `true` if coloured output should be enabled, `false` otherwise
///
/// # Environment Variables
///
/// * `NO_COLOR` - If set (any value), disables colours
/// * `FORCE_COLOR` - If set to non-zero, enables colours
/// * `CLICOLOR_FORCE` - If set to non-zero, enables colours
/// * `CLICOLOR` - If set to "0", disables colours
/// * `TERM` - Terminal type (e.g., "xterm-256color", "dumb")
/// * `COLORTERM` - Indicates colour terminal support
///
/// # Examples
///
/// ```no_run
/// use cerium::display::output::terminal::colours_enabled;
///
/// if colours_enabled() {
///     println!("\x1b[31mRed text\x1b[0m");
/// } else {
///     println!("Plain text");
/// }
/// ```
pub fn colours_enabled() -> bool {
    // Check if NO_COLOR is set (universal override to disable colours)
    if env::var("NO_COLOR").is_ok() {
        return false;
    }

    // Check if colours are explicitly forced
    if let Ok(force_color) = env::var("FORCE_COLOR")
        && !force_color.is_empty()
        && force_color != "0"
    {
        return true;
    }

    // Check CLICOLOR_FORCE
    if let Ok(val) = env::var("CLICOLOR_FORCE")
        && val != "0"
    {
        return true;
    }

    // Check if CLICOLOR is set to 0 (disable colours)
    if let Ok(val) = env::var("CLICOLOR")
        && val == "0"
    {
        return false;
    }

    // Check TERM environment variable
    if let Ok(term) = env::var("TERM") {
        // Dumb terminals don't support colours
        if term == "dumb" {
            return false;
        }
        // Common colour-supporting terminals
        if term.contains("color")
            || term.contains("xterm")
            || term.contains("screen")
            || term.contains("tmux")
            || term.contains("rxvt")
            || term.contains("linux")
        {
            return is_tty();
        }
    }

    // Check COLORTERM (modern standard)
    if env::var("COLORTERM").is_ok() {
        return is_tty();
    }

    // Default: check if stdout is a TTY
    is_tty()
}

/// Checks if standard output is connected to a TTY (terminal).
///
/// This is useful for determining whether the program is running interactively
/// in a terminal or if its output is being redirected to a file or pipe.
///
/// # Returns
///
/// `true` if stdout is connected to a TTY, `false` otherwise
///
/// # Platform Support
///
/// This function uses Unix-specific system calls (isatty) and is available
/// on Unix-like systems.
///
/// # Examples
///
/// ```no_run
/// use cerium::display::output::terminal::is_tty;
///
/// if is_tty() {
///     println!("Running interactively");
/// } else {
///     println!("Output is redirected");
/// }
/// ```
pub fn is_tty() -> bool {
    {
        let fd = io::stdout().as_raw_fd();
        unsafe { libc::isatty(fd) != 0 }
    }
}
