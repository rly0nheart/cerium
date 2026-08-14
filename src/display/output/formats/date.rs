// SPDX-License-Identifier: MIT

use crate::cli::flags::DateFormat;
use human::HumanRelative;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Formats a timestamp according to the selected [`DateFormat`].
///
/// # Parameters
/// - `mode`: The display format to use.
/// - `system_time`: The timestamp to format, or `None` when unavailable.
pub(crate) fn format(mode: DateFormat, system_time: Option<SystemTime>) -> Arc<str> {
    let Some(time) = system_time else {
        return "-".into();
    };

    match mode {
        DateFormat::Human => HumanRelative::new(time).to_string().into(),
        DateFormat::Locale => locale(time),
        DateFormat::Timestamp => match time.duration_since(UNIX_EPOCH) {
            Ok(elapsed) => elapsed.as_secs().to_string().into(),
            Err(_) => "-".into(),
        },
    }
}

/// Formats a timestamp as local `%b %d %H:%M` via libc.
///
/// # Parameters
/// - `time`: The timestamp to format.
///
/// # Returns
/// The formatted date, or `"-"` if the time predates the epoch or libc rejects it.
fn locale(time: SystemTime) -> Arc<str> {
    let Ok(elapsed) = time.duration_since(UNIX_EPOCH) else {
        return "-".into();
    };

    let seconds = elapsed.as_secs() as libc::time_t;
    let mut parts: libc::tm = unsafe { std::mem::zeroed() };
    let mut buffer = [0u8; 32];

    let written = unsafe {
        if libc::localtime_r(&seconds, &mut parts).is_null() {
            return "-".into();
        }
        libc::strftime(
            buffer.as_mut_ptr() as *mut libc::c_char,
            buffer.len(),
            c"%b %d %H:%M".as_ptr(),
            &parts,
        )
    };

    match std::str::from_utf8(&buffer[..written]) {
        Ok(formatted) => formatted.into(),
        Err(_) => "-".into(),
    }
}
