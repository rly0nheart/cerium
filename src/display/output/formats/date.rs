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

use crate::cli::flags::DateFormat;
use crate::display::output::formats::format::Format;
use chrono::{DateTime, Local};
use humanly::HumanDuration;
use std::sync::Arc;
use std::time::SystemTime;

impl Format<Option<SystemTime>> for Date {
    /// Formats an optional [`SystemTime`] according to the configured date format.
    fn format(&self, input: Option<SystemTime>) -> Arc<str> {
        self.format_date(input)
    }
}

/// Formats timestamps according to the selected [`DateFormat`].
pub(crate) struct Date {
    date_format: DateFormat,
}

impl Date {
    /// Creates a new [`Date`] formatter.
    ///
    /// # Parameters
    /// - `date_format`: The display format to use.
    pub(crate) fn new(date_format: DateFormat) -> Self {
        Self { date_format }
    }

    /// Dispatches to the appropriate date formatting method.
    ///
    /// # Parameters
    /// - `system_time`: The timestamp to format, or `None` for a placeholder.
    fn format_date(&self, system_time: Option<SystemTime>) -> Arc<str> {
        match self.date_format {
            DateFormat::Humanly => self.humanised(system_time),
            DateFormat::Locale => Self::locale(system_time),
            DateFormat::Timestamp => match system_time {
                Some(st) => match st.duration_since(SystemTime::UNIX_EPOCH) {
                    Ok(dur) => dur.as_secs().to_string().into(),
                    Err(_) => "-".into(),
                },
                None => "-".into(),
            },
        }
    }

    /// Formats the timestamp as a human-readable relative duration.
    ///
    /// # Parameters
    /// - `system_time`: The timestamp to format.
    fn humanised(&self, system_time: Option<SystemTime>) -> Arc<str> {
        Arc::from(HumanDuration::from(system_time).to_string())
    }

    /// Formats the timestamp using the locale date format.
    ///
    /// # Parameters
    /// - `system_time`: The timestamp to format, or `None` for `"-"`.
    fn locale(system_time: Option<SystemTime>) -> Arc<str> {
        match system_time {
            Some(st) => {
                let datetime: DateTime<Local> = st.into();
                datetime.format("%b %d %H:%M").to_string().into()
            }
            None => "-".into(),
        }
    }
}
