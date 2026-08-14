// SPDX-License-Identifier: MIT

use std::os::unix::io::AsRawFd;
use std::{env, io};

/// Checks if colored output should be enabled based on environment variables and terminal capabilities.
///
/// # Returns
/// `true` if colored output should be enabled, `false` otherwise.
pub fn colors_enabled() -> bool {
    // Check if NO_COLOR is set (universal override to disable colors)
    if env::var("NO_COLOR").is_ok() {
        return false;
    }

    // Check if colors are explicitly forced
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

    // Check if CLICOLOR is set to 0 (disable colors)
    if let Ok(val) = env::var("CLICOLOR")
        && val == "0"
    {
        return false;
    }

    // Check TERM environment variable
    if let Ok(term) = env::var("TERM") {
        // Dumb terminals don't support colors
        if term == "dumb" {
            return false;
        }
        // Common color-supporting terminals
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
