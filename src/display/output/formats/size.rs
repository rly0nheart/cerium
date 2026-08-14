// SPDX-License-Identifier: MIT

use crate::cli::flags::SizeFormat;
use human::HumanSize;
use std::sync::Arc;

/// Formats a byte count as human-readable or raw.
///
/// # Parameters
/// - `mode`: The display format (bytes, binary, or decimal).
/// - `bytes`: The byte count to format.
pub(crate) fn format(mode: SizeFormat, bytes: u64) -> Arc<str> {
    match mode {
        SizeFormat::Binary => HumanSize::from(bytes).binary().short().to_string().into(),
        SizeFormat::Decimal => HumanSize::from(bytes).decimal().short().to_string().into(),
        SizeFormat::Bytes => bytes.to_string().into(),
    }
}

/// Formats a directory item count, e.g. `"0 items"`, `"1 item"`, `"3 items"`.
///
/// # Parameters
/// - `count`: How many entries the directory holds.
pub(crate) fn item_count(count: usize) -> Arc<str> {
    if count == 1 {
        "1 item".into()
    } else {
        format!("{} items", count).into()
    }
}
