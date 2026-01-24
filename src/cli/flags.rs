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

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum DateFormat {
    Locale,
    Humanly,
    Timestamp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum NumberFormat {
    Humanly,
    Natural,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum OwnershipFormat {
    Name,
    Id,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum ShowIcons {
    Auto,
    Always,
    Never,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum PermissionsFormat {
    Symbolic,
    Octal,
    Hex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum SizeFormat {
    Bytes,
    Binary,
    Decimal,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum SortBy {
    Name,
    Size,
    Created,
    Accessed,
    Modified,
    Extension,
    Inode,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum ShowColour {
    Always,
    Auto,
    Never,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum ShowHyperlink {
    Always,
    Auto,
    Never,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum QuoteStyle {
    Auto,
    Double,
    Single,
    Never,
}

#[cfg(feature = "checksum")]
/// Hash algorithm selection for checksum computation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub(crate) enum HashAlgorithm {
    Crc32,
    Md5,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}
