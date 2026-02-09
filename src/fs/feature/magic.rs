#[cfg(all(feature = "magic", not(target_os = "android")))]
use std::fs::read_link;

#[cfg(all(feature = "magic", not(target_os = "android")))]
use std::path::PathBuf;

#[cfg(all(feature = "magic", not(target_os = "android")))]
use std::sync::Arc;

#[cfg(all(feature = "magic", not(target_os = "android")))]
use crate::fs::cache::Cache;

#[cfg(all(feature = "magic", not(target_os = "android")))]
use filemagic::Magic as FileMagic;

#[cfg(all(feature = "magic", not(target_os = "android")))]
/// Truncates a string to include only content up to and including the second comma.
///
/// Primarily used to simplify libmagic output for display in tables,
/// where file type descriptions can be excessively long but the first two segments
/// usually contain the most relevant information.
///
/// # Parameters
///
/// - `text`: The input string to truncate.
///
/// # Returns
///
/// An `Arc<str>` containing the text up to the second comma (inclusive).
/// If the input has fewer than two commas, the entire string is returned.
///
/// # Examples
///
/// ```
/// let result = output::text::clip_2nd_comma("text/plain, ASCII text, with CRLF line terminators".to_string());
/// // Returns: "text/plain, ASCII text"
///
/// let result = output::text::clip_2nd_comma("application/pdf".to_string());
/// // Returns: "application/pdf"
/// ```
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

#[cfg(all(feature = "magic", not(target_os = "android")))]
/// Detects file types using `libmagic`.
pub(crate) struct Magic;

#[cfg(all(feature = "magic", not(target_os = "android")))]
impl Magic {
    /// Returns the `libmagic` file type description for a path.
    ///
    /// # Parameters
    /// - `path`: The file to identify.
    ///
    /// # Returns
    /// A truncated magic description, or an empty string for directories.
    pub(crate) fn file(path: &PathBuf) -> Arc<str> {
        if path.is_dir() {
            return "".into();
        }

        if path.is_symlink() {
            return format!(
                "Symbolic link, to {:?}",
                read_link(path).unwrap_or_default()
            )
            .into();
        }

        Cache::magic(path, || {
            thread_local! {
                static MAGIC: std::cell::RefCell<Option<FileMagic>> = const { std::cell::RefCell::new(None) };
            }

            MAGIC.with(|cell| {
                let mut maybe_magic = cell.borrow_mut();

                if maybe_magic.is_none()
                    && let Ok(magic) = FileMagic::open(Default::default())
                {
                    let _ = magic.load::<String>(&[]);
                    *maybe_magic = Some(magic);
                }

                if let Some(magic) = maybe_magic.as_ref() {
                    clip_2nd_comma(
                        magic
                            .file(path.to_str().unwrap_or_default())
                            .unwrap_or_default(),
                    )
                } else {
                    "Magic library unavailable".into()
                }
            })
        })
    }
}
