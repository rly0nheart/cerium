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

use clap::ValueEnum;

/// Controls how dates are formatted in output.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum DateFormat {
    Locale,
    Humanly,
    Timestamp,
}

/// Controls how numeric values (hard links, blocks) are formatted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum NumberFormat {
    Humanly,
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

/// Controls when ANSI colours are used in output.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ShowColour {
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

#[cfg(feature = "checksum")]
/// Hash algorithm selection for checksum computation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum HashAlgorithm {
    Crc32,
    Md5,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}
