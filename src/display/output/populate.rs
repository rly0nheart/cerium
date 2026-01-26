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
use crate::fs::acl::Acl;
use crate::fs::cache::Cache;
use crate::fs::context::Context;
use crate::fs::dir::DirReader;
use crate::fs::entry::Entry;

#[cfg(feature = "checksum")]
use crate::fs::feature::checksum::Checksum;

#[cfg(feature = "magic")]
use crate::fs::feature::magic::Magic;

use crate::display::layout::column::Column;
use crate::display::output::formats::date::Date;
use crate::display::output::formats::format::Format;
use crate::display::output::formats::number::Number;
use crate::display::output::formats::ownership::Ownership;
use crate::display::output::formats::permissions::Permissions;
use crate::display::output::formats::size::Size;
use crate::fs::mountpoint::Mountpoint;
use crate::fs::xattr::Xattr;
use std::sync::Arc;
use std::time;

pub(crate) struct Populate<'a> {
    entry: &'a Entry,
    column: &'a Column,
    args: &'a Args,
}

impl<'a> Populate<'a> {
    pub(crate) fn new(entry: &'a Entry, column: &'a Column, args: &'a Args) -> Self {
        Self {
            entry,
            column,
            args,
        }
    }

    pub(crate) fn value(&self) -> Arc<str> {
        let path = self.entry.path();

        let date_formatter = Date::new(self.args.date_format);
        let permissions_formatter = Permissions::new(self.args.permission_format, path.to_owned());
        let number_formatter = Number::new(self.args.number_format);
        let size_formatter = Size::new(self.args.size_format);
        let ownership_formatter = Ownership::new(self.args.ownership_format);
        let metadata = self.entry.metadata();

        match self.column {
            Column::Name => self.entry.name().clone(),

            #[cfg(feature = "magic")]
            Column::Magic => Magic::file(path),

            #[cfg(feature = "checksum")]
            Column::Checksum(algo) => Checksum::new(path, *algo).compute(),

            Column::Xattr => Xattr::list(path),
            Column::Acl => Acl::check(path),
            Column::Context => Context::get(path),
            Column::Mountpoint => Mountpoint::get(path),
            Column::Inode => metadata
                .map(|meta| meta.ino.to_string())
                .unwrap_or_default()
                .into(),
            Column::Permissions => {
                Cache::permissions(metadata.map(|meta| meta.mode).unwrap_or_default(), |meta| {
                    permissions_formatter.format(meta)
                })
            }
            Column::HardLinks => {
                Cache::number(metadata.map(|meta| meta.nlink).unwrap_or_default(), |n| {
                    number_formatter.format(n)
                })
            }
            Column::User => {
                Cache::owner(metadata.map(|meta| meta.uid).unwrap_or_default(), |uid| {
                    ownership_formatter.format_user(uid)
                })
            }
            Column::Group => {
                Cache::group(metadata.map(|meta| meta.gid).unwrap_or_default(), |gid| {
                    ownership_formatter.format_group(gid)
                })
            }
            Column::Blocks => {
                Cache::number(metadata.map(|meta| meta.blocks).unwrap_or_default(), |b| {
                    number_formatter.format(b)
                })
            }
            Column::BlockSize => {
                Cache::size(metadata.map(|meta| meta.blksize).unwrap_or_default(), |b| {
                    size_formatter.format(b)
                })
            }
            Column::Size => {
                let size_bytes = if self.entry.is_dir() && self.args.true_size {
                    Cache::true_size(self.entry.path(), self.args.all, || {
                        DirReader::from(path.to_owned()).true_size(self.args.all)
                    })
                } else {
                    metadata.map(|meta| meta.size).unwrap_or_default()
                };
                Cache::size(size_bytes, |s| size_formatter.format(s))
            }
            Column::Created => Cache::date(
                metadata
                    .map(|meta| time::UNIX_EPOCH + time::Duration::from_secs(meta.ctime as u64)),
                |ts| date_formatter.format(ts),
            ),
            Column::Accessed => Cache::date(
                metadata
                    .map(|meta| time::UNIX_EPOCH + time::Duration::from_secs(meta.atime as u64)),
                |ts| date_formatter.format(ts),
            ),
            Column::Modified => Cache::date(
                metadata
                    .map(|meta| time::UNIX_EPOCH + time::Duration::from_secs(meta.mtime as u64)),
                |ts| date_formatter.format(ts),
            ),
        }
    }
}
