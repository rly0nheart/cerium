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

//! File-type indicators for `--classify`/`--file-type`/`--slash`.
//!
//! This mirrors GNU coreutils `ls`'s `get_type_indicator()`: the character
//! appended after an entry name depends on the entry's `st_mode` and the
//! selected [`IndicatorStyle`].

use crate::cli::args::Args;
use crate::cli::flags::IndicatorStyle;
use crate::fs::entry::Entry;
use crate::fs::metadata::Metadata;
use libc::{S_IFDIR, S_IFIFO, S_IFLNK, S_IFMT, S_IFREG, S_IFSOCK, S_IXGRP, S_IXOTH, S_IXUSR};

/// Returns the indicator character to append after `entry`'s name, or `None`
/// when no indicator applies (or no indicator flag is set).
///
/// The result is meant to be appended *unstyled* — `ls` never colours the
/// indicator, and neither do we.
///
/// # Parameters
/// - `entry`: The filesystem entry being rendered.
/// - `args`: Parsed command-line arguments (selects the indicator style and
///   whether symlinks are dereferenced).
pub(crate) fn indicator(entry: &Entry, args: &Args) -> Option<char> {
    let style = args.indicator_style();
    if style == IndicatorStyle::None {
        return None;
    }

    match entry {
        // A real directory always gets '/', under every style.
        Entry::Directory(_) => Some('/'),

        Entry::Symlink(_) => {
            // Long mode renders the link as `name -> target`: classify the
            // *target* with no '@' on the link, exactly like `ls -lF`. A
            // target that can't be resolved (broken link) gets no indicator.
            if args.long {
                return Metadata::load(entry.path(), true)
                    .ok()
                    .and_then(|meta| from_mode(meta.mode, style));
            }
            // -L dereferences the link, so classify the target. If the target
            // resolves we use its indicator (even when that's "none", e.g. a
            // plain file); if it can't be resolved, `ls` falls back to
            // treating the entry as a symlink.
            if args.dereference
                && let Ok(meta) = Metadata::load(entry.path(), true)
            {
                return from_mode(meta.mode, style);
            }
            // The link itself: '@' under classify/file-type, nothing under
            // the slash style (which only marks directories).
            if style == IndicatorStyle::Slash {
                None
            } else {
                Some('@')
            }
        }

        // `Entry::File` also covers FIFOs, sockets, and devices, so the mode
        // is needed both to tell those apart and to test the execute bits.
        Entry::File(_) => {
            // The slash style only ever marks directories, so a file never
            // gets an indicator — skip the stat entirely.
            if style == IndicatorStyle::Slash {
                return None;
            }
            let mode = entry
                .metadata()
                .filter(|meta| meta.mode != 0)
                .map(|meta| meta.mode)
                .or_else(|| Metadata::load(entry.path(), false).ok().map(|m| m.mode))?;
            from_mode(mode, style)
        }
    }
}

/// Maps a raw `st_mode` to an indicator character, following coreutils'
/// precedence: directory → (slash style stops here) → executable regular
/// file → symlink → FIFO → socket. Block/char devices yield no indicator,
/// matching `ls`.
///
/// # Parameters
/// - `mode`: The `st_mode` value from a stat call.
/// - `style`: The active indicator style.
fn from_mode(mode: u32, style: IndicatorStyle) -> Option<char> {
    if mode & S_IFMT == S_IFDIR {
        return Some('/');
    }
    // Slash style (`-p`) only ever marks directories.
    if style == IndicatorStyle::Slash {
        return None;
    }
    match mode & S_IFMT {
        S_IFREG => {
            let executable = mode & (S_IXUSR | S_IXGRP | S_IXOTH) != 0;
            if style == IndicatorStyle::Classify && executable {
                Some('*')
            } else {
                None
            }
        }
        S_IFLNK => Some('@'),
        S_IFIFO => Some('|'),
        S_IFSOCK => Some('='),
        _ => None,
    }
}
