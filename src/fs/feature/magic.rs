// SPDX-License-Identifier: MIT

//! Content-based file type identification via libmagic.

#![cfg(all(feature = "magic", not(target_os = "android")))]

use crate::fs::cache::Cache;
use filemagic::Magic;
use std::fs::read_link;
use std::path::Path;
use std::sync::Arc;

/// Truncates libmagic output to the first two comma-separated segments.
///
/// Descriptions run long, and the first two parts carry the useful bits.
///
/// # Parameters
/// - `text`: The description to truncate.
///
/// # Returns
/// The text up to the second comma, or all of it when it has fewer commas.
fn clip_2nd_comma(text: String) -> Arc<str> {
    let mut parts = text.splitn(3, ',');
    let first = parts.next().unwrap_or("");
    let second = parts.next().unwrap_or("");

    if second.is_empty() {
        first.into()
    } else {
        format!("{},{}", first, second).into()
    }
}

/// Returns the libmagic description for a path.
///
/// # Parameters
/// - `path`: The file to identify.
///
/// # Returns
/// A truncated description, or an empty string for directories.
pub(crate) fn describe(path: &Path) -> Arc<str> {
    if path.is_dir() {
        return "".into();
    }

    if path.is_symlink() {
        return format!("Symbolic link, to {:?}", read_link(path).unwrap_or_default()).into();
    }

    Cache::magic(path.to_path_buf(), |path| {
        thread_local! {
            static MAGIC: std::cell::RefCell<Option<Magic>> = const { std::cell::RefCell::new(None) };
        }

        MAGIC.with(|cell| {
            let mut maybe_magic = cell.borrow_mut();

            if maybe_magic.is_none()
                && let Ok(magic) = Magic::open(Default::default())
            {
                let _ = magic.load::<String>(&[]);
                *maybe_magic = Some(magic);
            }

            match maybe_magic.as_ref() {
                Some(magic) => clip_2nd_comma(
                    magic
                        .file(path.to_str().unwrap_or_default())
                        .unwrap_or_default(),
                ),
                None => "Magic library unavailable".into(),
            }
        })
    })
}
