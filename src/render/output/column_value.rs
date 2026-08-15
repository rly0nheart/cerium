// SPDX-License-Identifier: MIT

//! The text that goes in one column for one entry, before styling.

use crate::cli::args::Args;
use crate::render::layout::column::Column;
use crate::render::output::format::{date, number, ownership, permission, size};
use crate::fs::cache::Cache;
use crate::fs::dir::DirReader;
use crate::fs::entry::Entry;
use crate::fs::permissions::Permissions;
use crate::fs::{acl, context, mountpoint, xattr};
use std::sync::Arc;
use std::time;

#[cfg(all(feature = "magic", not(target_os = "android")))]
use crate::fs::feature::magic;

/// Returns the unstyled text for one column of one entry.
///
/// # Parameters
/// - `entry`: The filesystem entry to query.
/// - `column`: The column whose value to produce.
/// - `args`: Command-line arguments controlling formatting.
pub(crate) fn of(entry: &Entry, column: &Column, args: &Args) -> Arc<str> {
    let path = entry.path();
    let metadata = entry.metadata();

    match column {
        Column::Name => entry.name().clone(),

        #[cfg(all(feature = "magic", not(target_os = "android")))]
        Column::Magic => magic::describe(path),

        Column::Xattr => xattr::list(path),
        Column::Acl => acl::indicator(path),
        Column::Context => context::get(path),
        Column::Mountpoint => mountpoint::of(path),
        Column::Inode => metadata
            .map(|meta| meta.ino.to_string())
            .unwrap_or_default()
            .into(),
        Column::Permissions => {
            let mode = metadata.map(|meta| meta.mode).unwrap_or_default();
            // The `@` suffix is per-file, so xattr presence belongs in the key.
            let key = (mode, Permissions::check_xattr(path));
            Cache::permissions(key, |&(mode, has_xattr)| {
                permission::format(mode, args.permission_format, has_xattr)
            })
        }
        Column::HardLinks => Cache::number(
            metadata.map(|meta| meta.nlink).unwrap_or_default(),
            |&links| number::format(args.number_format, links),
        ),
        Column::User => Cache::owner(metadata.map(|meta| meta.uid).unwrap_or_default(), |&uid| {
            ownership::user(args.ownership_format, uid)
        }),
        Column::Group => Cache::group(metadata.map(|meta| meta.gid).unwrap_or_default(), |&gid| {
            ownership::group(args.ownership_format, gid)
        }),
        Column::Blocks => Cache::number(
            metadata.map(|meta| meta.blocks).unwrap_or_default(),
            |&blocks| number::format(args.number_format, blocks),
        ),
        Column::BlockSize => Cache::size(
            metadata.map(|meta| meta.blksize).unwrap_or_default(),
            |&bytes| size::format(args.size_format, bytes),
        ),
        Column::Size => {
            if entry.is_dir() && !args.dir_size {
                let count = Cache::item_count((path.to_owned(), args.all), |(path, all)| {
                    DirReader::from(path.clone()).item_count(*all)
                });
                return size::item_count(count);
            }

            let bytes = if entry.is_dir() {
                Cache::dir_size((path.to_owned(), args.all), |(path, all)| {
                    DirReader::from(path.clone()).dir_size(*all)
                })
            } else {
                metadata.map(|meta| meta.size).unwrap_or_default()
            };

            Cache::size(bytes, |&bytes| size::format(args.size_format, bytes))
        }
        Column::Created => date_at(metadata.map(|meta| meta.ctime), args),
        Column::Accessed => date_at(metadata.map(|meta| meta.atime), args),
        Column::Modified => date_at(metadata.map(|meta| meta.mtime), args),
    }
}

/// Formats a raw stat timestamp for display.
///
/// # Parameters
/// - `seconds`: Seconds since the Unix epoch, or `None` when unavailable.
/// - `args`: Command-line arguments controlling the date format.
fn date_at(seconds: Option<i64>, args: &Args) -> Arc<str> {
    let timestamp = seconds.map(|secs| time::UNIX_EPOCH + time::Duration::from_secs(secs as u64));
    Cache::date(timestamp, |&time| date::format(args.date_format, time))
}
