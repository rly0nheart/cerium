use crate::cli::flags::QuoteStyle;
use crate::fs::symlink::{SYMLINK_ARROW_WITH_SPACES, split_symlink};

/// A text quoter that handles shell-safe quoting with symlink support.
///
/// This struct provides methods for quoting text in various styles, with special
/// handling for symlinks (indicated by the ⇒ arrow).
pub(crate) struct Quotes<'a> {
    text: &'a str,
}

impl<'a> Quotes<'a> {
    /// Creates a new Quote instance for the given text.
    ///
    /// # Parameters
    ///
    /// * `text` - The text to be quoted
    ///
    /// # Examples
    ///
    /// ```
    /// let q = Quotes::new("file name");
    /// ```
    pub(crate) fn new(text: &'a str) -> Self {
        Self { text }
    }

    /// Applies the specified quote style to the text with smart alignment handling.
    ///
    /// # Parameters
    ///
    /// * `style` - The quoting style to apply
    /// * `add_alignment_space` - Whether to add a leading space for unquoted entries.
    ///   Only applies to Auto mode when alignment is needed. For Single and Double modes,
    ///   this parameter is ignored since all entries are quoted uniformly.
    ///
    /// # Returns
    ///
    /// A `String` with the appropriate quoting applied
    ///
    /// # Examples
    ///
    /// ```
    /// let q = Quotes::new("file name");
    /// assert_eq!(q.apply(QuoteStyle::Single, false), "'file name'");
    /// assert_eq!(q.apply(QuoteStyle::Single, true), "'file name'");  // Alignment ignored
    ///
    /// let q2 = Quotes::new("normal");
    /// assert_eq!(q2.apply(QuoteStyle::Auto, false), "normal");
    /// assert_eq!(q2.apply(QuoteStyle::Auto, true), " normal");  // Space for alignment
    /// ```
    pub(crate) fn apply(&self, style: QuoteStyle, add_alignment_space: bool) -> String {
        match style {
            QuoteStyle::Single => self.single_quote_always(),
            QuoteStyle::Double => self.double_quote_always(),
            QuoteStyle::Auto => {
                let quoted = self.single_quote_conditional();
                // Auto mode: add alignment space if text wasn't quoted and alignment is needed
                // This ensures unquoted entries align with quoted entries in the same directory
                if add_alignment_space && !quoted.starts_with('\'') {
                    format!(" {}", quoted)
                } else {
                    quoted
                }
            }
            QuoteStyle::Never => self.text.into(),
        }
    }

    /// Wraps text in single quotes if it contains special characters or whitespace.
    ///
    /// For symlinks (indicated by the ⇒ arrow), quotes are applied to each side
    /// independently, leaving the arrow unquoted.
    ///
    /// # Returns
    ///
    /// A `String` with the text quoted if necessary. Unquoted text is returned as-is.
    ///
    /// # Special Characters
    ///
    /// The following characters trigger quoting:
    /// - Whitespace (spaces, tabs, newlines)
    /// - Shell metacharacters: `\`, `'`, `"`, `` ` ``, `$`, `&`, `|`, `;`
    /// - Glob characters: `*`, `?`
    /// - Other special characters: `<`, `>`, `(`, `)`, `[`, `]`, `{`, `}`, `!`, `#`, `~`, `%`, `^`
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Quotes::new("normal").single_quote_conditional(), "normal");
    /// assert_eq!(Quotes::new("file name").single_quote_conditional(), "'file name'");
    /// assert_eq!(Quotes::new("link ⇒ target").single_quote_conditional(), "link ⇒ target");
    /// assert_eq!(Quotes::new("my link ⇒ my target").single_quote_conditional(), "'my link' ⇒ 'my target'");
    /// assert_eq!(Quotes::new("file$name").single_quote_conditional(), "'file$name'");
    /// ```
    fn single_quote_conditional(&self) -> String {
        if let Some((left, right)) = split_symlink(self.text) {
            let quoted_left = Self::quote_if_quotable(left.trim_end());
            let quoted_right = Self::quote_if_quotable(right.trim_start());

            format!(
                "{}{}{}",
                quoted_left, SYMLINK_ARROW_WITH_SPACES, quoted_right
            )
        } else {
            Self::quote_if_quotable(self.text)
        }
    }

    /// Wraps text in single quotes unconditionally.
    ///
    /// Always adds single quotes around the input text. For symlinks (indicated by the ⇒ arrow),
    /// quotes are applied to each side independently, leaving the arrow unquoted.
    ///
    /// # Returns
    ///
    /// A `String` with the text wrapped in single quotes. For symlinks, both the
    /// link name and target are quoted separately with the arrow between them.
    /// Single quotes within the text are escaped as `\'`.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Quotes::new("file").single_quote_always(), "'file'");
    /// assert_eq!(Quotes::new("file name").single_quote_always(), "'file name'");
    /// assert_eq!(Quotes::new("link ⇒ target").single_quote_always(), "'link' ⇒ 'target'");
    /// assert_eq!(Quotes::new("my link ⇒ my target").single_quote_always(), "'my link' ⇒ 'my target'");
    /// ```
    fn single_quote_always(&self) -> String {
        if let Some((left, right)) = split_symlink(self.text) {
            let quoted_left = Self::add_single_quotes(left.trim_end());
            let quoted_right = Self::add_single_quotes(right.trim_start());

            format!(
                "{}{}{}",
                quoted_left, SYMLINK_ARROW_WITH_SPACES, quoted_right
            )
        } else {
            Self::add_single_quotes(self.text)
        }
    }

    /// Wraps text in double quotes unconditionally.
    ///
    /// Always adds double quotes around the input text. For symlinks (indicated by the ⇒ arrow),
    /// quotes are applied to each side independently.
    ///
    /// # Returns
    ///
    /// A `String` with the text wrapped in double quotes. For symlinks, both the
    /// link name and target are quoted separately with the arrow between them.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Quotes::new("file").double_quote_always(), "\"file\"");
    /// assert_eq!(Quotes::new("file name").double_quote_always(), "\"file name\"");
    /// assert_eq!(Quotes::new("link ⇒ target").double_quote_always(), "\"link\" ⇒ \"target\"");
    /// assert_eq!(Quotes::new("my link ⇒ my target").double_quote_always(), "\"my link\" ⇒ \"my target\"");
    /// ```
    fn double_quote_always(&self) -> String {
        if let Some((left, right)) = split_symlink(self.text) {
            let quoted_left = Self::add_double_quotes(left.trim_end());
            let quoted_right = Self::add_double_quotes(right.trim_start());

            format!(
                "{}{}{}",
                quoted_left, SYMLINK_ARROW_WITH_SPACES, quoted_right
            )
        } else {
            Self::add_double_quotes(self.text)
        }
    }

    /// Checks if text needs shell quoting (contains special characters or whitespace).
    ///
    /// For symlinks (indicated by the ⇒ arrow), each part is checked separately,
    /// matching the behaviour of `single_quote_conditional()`.
    ///
    /// # Parameters
    ///
    /// * `text` - The text to check
    ///
    /// # Returns
    ///
    /// `true` if the text needs quoting, `false` otherwise
    pub(crate) fn is_quotable(text: &str) -> bool {
        // Handle symlinks by checking each part separately
        if let Some((left, right)) = split_symlink(text) {
            Self::has_special_chars(left.trim_end()) || Self::has_special_chars(right.trim_start())
        } else {
            Self::has_special_chars(text)
        }
    }

    /// Checks if a string contains characters that require shell quoting.
    fn has_special_chars(text: &str) -> bool {
        text.chars().any(|c| {
            c.is_whitespace()
                || matches!(
                    c,
                    '\\' | '\''
                        | '"'
                        | '`'
                        | '$'
                        | '&'
                        | '|'
                        | ';'
                        | '<'
                        | '>'
                        | '('
                        | ')'
                        | '['
                        | ']'
                        | '{'
                        | '}'
                        | '*'
                        | '?'
                        | '!'
                        | '#'
                        | '~'
                        | '%'
                        | '^'
                )
        })
    }

    /// Helper function that quotes a single text segment with single quotes if needed.
    ///
    /// # Parameters
    ///
    /// * `text` - The text segment to quote
    ///
    /// # Returns
    ///
    /// The text wrapped in single quotes if it contains special characters,
    /// or the original text if no quoting is necessary. Single quotes within
    /// the text are escaped as `\'`.
    fn quote_if_quotable(text: &str) -> String {
        if Self::has_special_chars(text) {
            Self::add_single_quotes(text)
        } else {
            text.to_string()
        }
    }

    /// Adds single quotes around text, escaping any single quotes within.
    ///
    /// # Parameters
    ///
    /// * `text` - The text to wrap in single quotes
    ///
    /// # Returns
    ///
    /// The text wrapped in single quotes with internal single quotes escaped as `\'`
    fn add_single_quotes(text: &str) -> String {
        let mut quoted = String::with_capacity(text.len() + 2);
        quoted.push('\'');

        for character in text.chars() {
            if character == '\'' {
                quoted.push('\\');
                quoted.push('\'');
            } else {
                quoted.push(character);
            }
        }

        quoted.push('\'');
        quoted
    }

    /// Adds double quotes around text, escaping any double quotes within.
    ///
    /// # Parameters
    ///
    /// * `text` - The text to wrap in double quotes
    ///
    /// # Returns
    ///
    /// The text wrapped in double quotes with internal double quotes escaped as `\"`
    fn add_double_quotes(text: &str) -> String {
        let mut quoted = String::with_capacity(text.len() + 2);
        quoted.push('"');

        for character in text.chars() {
            if character == '"' {
                quoted.push('\\');
                quoted.push('"');
            } else {
                quoted.push(character);
            }
        }

        quoted.push('"');
        quoted
    }
}
