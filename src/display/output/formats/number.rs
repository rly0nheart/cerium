// SPDX-License-Identifier: MIT

use crate::cli::flags::NumberFormat;
use human::HumanNumber;
use std::sync::Arc;

/// Formats a number as human-readable or natural.
///
/// # Parameters
/// - `mode`: The display format to use.
/// - `number`: The value to format.
pub(crate) fn format(mode: NumberFormat, number: u64) -> Arc<str> {
    match mode {
        NumberFormat::Human => HumanNumber::from(number as f64).short().to_string().into(),
        NumberFormat::Natural => number.to_string().into(),
    }
}
