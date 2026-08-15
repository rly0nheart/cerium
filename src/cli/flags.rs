// SPDX-License-Identifier: MIT

use clap::ValueEnum;

/// Controls how dates are formatted in output.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum DateFormat {
    Locale,
    Human,
    Timestamp,
}

/// Controls how numeric values (hard links, blocks) are formatted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum NumberFormat {
    Human,
    Natural,
}

/// Controls how user and group ownership is displayed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum OwnershipFormat {
    Name,
    Id,
}

/// Controls when file-type icons are shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum ShowIcons {
    Auto,
    Always,
    Never,
}

/// Controls which file-type indicator (if any) is appended to entry names,
/// mirroring GNU `ls`'s `-F`/`--file-type`/`-p` family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndicatorStyle {
    /// No indicator is appended (default).
    None,
    /// Append `/` to directories only (like `ls -p`).
    Slash,
    /// Like [`IndicatorStyle::Classify`], but never append `*` to executables.
    FileType,
    /// Append one of `*/=@|` (like `ls -F`).
    Classify,
}

/// Controls how file permissions are formatted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum PermissionFormat {
    Symbolic,
    Octal,
    Hex,
}

/// Controls how file sizes are formatted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum SizeFormat {
    Bytes,
    Binary,
    Decimal,
}

/// Determines the field used to sort directory entries.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum SortBy {
    Name,
    Size,
    Created,
    Accessed,
    Modified,
    Extension,
    Inode,
}

/// Controls when ANSI colors are used in output.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ShowColor {
    Always,
    Auto,
    Never,
}

/// Controls when OSC 8 hyperlinks wrap entry names.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ShowHyperlink {
    Always,
    Auto,
    Never,
}

/// Controls how entry names are quoted in output.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum QuoteStyle {
    Auto,
    Double,
    Single,
    Never,
}

