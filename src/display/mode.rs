// SPDX-License-Identifier: MIT

/// Trait implemented by all output renderers (grid, list, tree).
pub trait DisplayMode {
    /// Prints the formatted output to stdout.
    fn print(&self);
}
