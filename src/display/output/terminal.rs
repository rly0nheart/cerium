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

/// Checks if coloured output should be enabled based on environment variables and terminal capabilities.
///
/// # Returns
/// `true` if coloured output should be enabled, `false` otherwise.
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

/// Checks if standard output is connected to a TTY.
pub fn is_tty() -> bool {
    {
        let fd = io::stdout().as_raw_fd();
        unsafe { libc::isatty(fd) != 0 }
    }
}
