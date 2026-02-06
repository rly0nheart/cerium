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

#[cfg(feature = "checksum")]
use crate::cli::flags::HashAlgorithm;

use crate::display::layout::alignment::{Align, Alignment};
use crate::display::layout::width::Width;
use crate::display::styles::text::TextStyle;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Column {
    #[cfg(all(feature = "magic", not(target_os = "android")))]
    Magic,

    #[cfg(feature = "checksum")]
    Checksum(HashAlgorithm),

    Xattr,
    Acl,
    Context,
    Mountpoint,
    Permissions,
    HardLinks,
    User,
    Group,
    Blocks,
    BlockSize,
    Created,
    Accessed,
    Modified,
    Size,
    Name,
    Inode,
}

impl Column {
    pub(crate) fn header(&self) -> &str {
        match self {
            Self::Name => "Name",

            #[cfg(all(feature = "magic", not(target_os = "android")))]
            Self::Magic => "Magic",

            #[cfg(feature = "checksum")]
            Column::Checksum(algo) => match algo {
                HashAlgorithm::Md5 => "MD5",
                HashAlgorithm::Crc32 => "CRC32",
                HashAlgorithm::Sha224 => "SHA-224",
                HashAlgorithm::Sha256 => "SHA-256",
                HashAlgorithm::Sha384 => "SHA-384",
                HashAlgorithm::Sha512 => "SHA-512",
            },

            Self::Xattr => "Xattr",
            Self::Acl => "ACL",
            Self::Context => "Context",
            Self::Mountpoint => "Mountpoint",
            Self::Inode => "inode",
            Self::Permissions => "Permissions",
            Self::HardLinks => "HardLinks",
            Self::User => "User",
            Self::Group => "Group",
            Self::Blocks => "Blocks",
            Self::BlockSize => "Block Size",
            Self::Size => "Size",
            Self::Created => "Created",
            Self::Accessed => "Accessed",
            Self::Modified => "Modified",
        }
    }

    pub(crate) fn alignment(&self) -> Alignment {
        match self {
            Self::Size
            | Self::Modified
            | Self::Created
            | Self::Accessed
            | Self::Inode
            | Self::HardLinks
            | Self::Blocks
            | Self::BlockSize => Alignment::Right,
            _ => Alignment::Left,
        }
    }

    pub(crate) fn headers(widths: &HashMap<Column, usize>, args: &Args) {
        if !args.headers {
            return;
        }
        let columns = Selector::select(args);

        let parts: Vec<String> = columns
            .iter()
            .map(|column| {
                let style = TextStyle::table_header(column.header());
                let width = *widths
                    .get(column)
                    .unwrap_or(&Width::measure_ansi_text(column.header()));
                Align::pad(&style, width, column.alignment())
            })
            .collect();

        println!("{}", parts.join(" "));
    }
}

pub(crate) struct Selector;

impl Selector {
    pub(crate) fn select(args: &Args) -> Vec<Column> {
        let mut columns = Vec::new();

        if args.long {
            for column in [
                Column::Permissions,
                Column::Size,
                Column::User,
                Column::Modified,
            ] {
                if !columns.contains(&column) {
                    columns.push(column);
                }
            }
        }

        if args.size && !columns.contains(&Column::Size) {
            columns.push(Column::Size);
        }
        if args.permission && !columns.contains(&Column::Permissions) {
            columns.push(Column::Permissions);
        }
        if args.user && !columns.contains(&Column::User) {
            columns.push(Column::User);
        }
        if args.group && !columns.contains(&Column::Group) {
            columns.push(Column::Group);
        }

        #[cfg(all(feature = "magic", not(target_os = "android")))]
        if args.magic && !columns.contains(&Column::Magic) {
            columns.push(Column::Magic);
        }

        #[cfg(feature = "checksum")]
        if let Some(algo) = args.checksum {
            let checksum_column = Column::Checksum(algo);
            if !columns.contains(&checksum_column) {
                columns.push(checksum_column);
            }
        }

        if args.xattr && !columns.contains(&Column::Xattr) {
            columns.push(Column::Xattr);
        }
        if args.acl && !columns.contains(&Column::Acl) {
            columns.push(Column::Acl);
        }
        if args.context && !columns.contains(&Column::Context) {
            columns.push(Column::Context);
        }
        if args.mountpoint && !columns.contains(&Column::Mountpoint) {
            columns.push(Column::Mountpoint);
        }
        if args.inode && !columns.contains(&Column::Inode) {
            columns.push(Column::Inode);
        }
        if args.blocks && !columns.contains(&Column::Blocks) {
            columns.push(Column::Blocks);
        }
        if args.hard_links && !columns.contains(&Column::HardLinks) {
            columns.push(Column::HardLinks);
        }
        if args.block_size && !columns.contains(&Column::BlockSize) {
            columns.push(Column::BlockSize);
        }
        if args.created && !columns.contains(&Column::Created) {
            columns.push(Column::Created);
        }
        if args.modified && !columns.contains(&Column::Modified) {
            columns.push(Column::Modified);
        }
        if args.accessed && !columns.contains(&Column::Accessed) {
            columns.push(Column::Accessed);
        }
        // Name and Separator are always last if not tree
        if !args.tree && !columns.contains(&Column::Name) {
            columns.push(Column::Name);
        }
        columns
    }
}
