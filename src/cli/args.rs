// SPDX-License-Identifier: MIT

use crate::cli::flags::{
    DateFormat, IndicatorStyle, NumberFormat, OwnershipFormat, PermissionFormat, QuoteStyle,
    ShowColor, ShowHyperlink, ShowIcons, SizeFormat, SortBy,
};


use clap::{Parser, ValueHint};
use std::path::PathBuf;

/// Parsed command-line arguments controlling listing behaviour and output formatting.
#[derive(Parser, Debug, Clone)]
#[command(name = crate::NAME, author = crate::AUTHORS, version, about=crate::DESCRIPTION)]
pub struct Args {
    #[arg(default_value = ".", value_hint = ValueHint::AnyPath)]
    pub path: PathBuf,

    /// Display one entry per line
    #[arg(short = '1', long)]
    pub oneline: bool,

    /// Don't ignore entries starting with `.`
    #[arg(short, long)]
    pub all: bool,

    /// This entry's last accessed date
    #[arg(long)]
    pub accessed: bool,

    /// Display ACL indicator (+ if file has ACLs)
    #[arg(long)]
    pub acl: bool,

    /// Display number of blocks (format affected by --number-format)
    #[arg(short, long)]
    pub blocks: bool,

    /// Block size
    #[arg(short = 'B', long)]
    pub block_size: bool,

    /// This entry's creation date
    #[arg(short, long)]
    pub created: bool,

    /// Only show directories
    #[arg(short, long)]
    pub dirs: bool,

    /// Only show files
    #[arg(short, long)]
    pub files: bool,

    /// Find entries that match a query
    #[arg(
        long,
        value_name = "QUERY",
        default_value = "",
        conflicts_with = "tree",
        visible_alias = "search"
    )]
    pub find: String,

    /// Display this entry's group
    #[arg(short = 'g', long)]
    pub group: bool,

    /// Display number of hard links (format affected by --number-format)
    #[arg(long)]
    pub hard_links: bool,

    /// Show column headers, works with metadata flags and options
    #[arg(short = 'H', long)]
    pub headers: bool,

    /// Omit (a comma-separated list of) implied entries from output
    #[arg(long, value_name = "ENTRIES", value_delimiter = ',')]
    pub ignore: Vec<String>,

    /// Hyperlink entry names WHEN
    #[arg(long, value_enum, default_value = "never", value_name = "WHEN")]
    pub hyperlink: ShowHyperlink,

    /// Display inode number
    #[arg(short, long)]
    pub inode: bool,

    /// When viewing symlinks, show metadata for the link target rather than for the link itself
    #[arg(short = 'L', long)]
    pub dereference: bool,

    /// Long listing format, show permissions, user, group, size, and modified date
    #[arg(short, long)]
    pub long: bool,

    /// This entry's last modification datetime
    #[arg(short, long)]
    pub modified: bool,

    /// Display filesystem mount point
    #[arg(long)]
    pub mountpoint: bool,

    /// This entry's permissions
    #[arg(short, long)]
    pub permissions: bool,

    /// Omit empty files and directories from output
    #[arg(long)]
    pub prune: bool,

    /// How to quote entry names
    #[arg(short = 'q', long, value_enum, default_value = "auto")]
    pub quote_name: QuoteStyle,

    /// Reverse order while sorting
    #[arg(short, long)]
    pub reverse: bool,

    /// List subdirectories recursively
    #[arg(short = 'R', long, conflicts_with_all = ["tree"])]
    pub recursive: bool,

    /// Display this entry's size
    #[arg(short, long)]
    pub size: bool,

    /// Sort entries by ...
    #[arg(long, value_enum, value_name = "BY", default_value = "name")]
    pub sort: SortBy,

    /// Show the recursive byte size of directories in the size column instead of the item count
    #[arg(short = 'S', long)]
    pub dir_size: bool,

    /// Display directories hierarchically (tree view)
    #[arg(short, long, conflicts_with = "recursive")]
    pub tree: bool,

    /// Display this entry's user
    #[arg(short, long)]
    pub user: bool,

    /// What the heck happened? (this will only make sense when used with --find)
    #[arg(short, long)]
    pub verbose: bool,

    /// Set output width to COLS (0 = no limit)
    #[arg(short = 'w', long, value_name = "COLS")]
    pub width: Option<usize>,

    /// Display extended attributes (xattr)
    #[arg(short, long)]
    pub xattr: bool,

    /// Display SELinux security context
    #[arg(short = 'Z', long)]
    pub context: bool,

    /// Append indicator (one of */=@|) to entry names
    #[arg(short = 'F', long)]
    pub classify: bool,

    /// Like --classify, but do not append '*' to executables
    #[arg(long)]
    pub file_type: bool,

    /// Append / indicator to directories
    #[arg(long)]
    pub slash: bool,

    /// Enable colors WHEN
    #[arg(short = 'C', long = "color", value_enum, default_value = "auto", value_name = "WHEN", visible_aliases = ["colour", "colors", "colours"], help_heading = "Display")]
    pub colors: ShowColor,

    /// Show icons WHEN
    #[arg(
        short = 'I',
        long,
        value_enum,
        default_value = "never",
        value_name = "WHEN",
        help_heading = "Display"
    )]
    pub icons: ShowIcons,

    #[cfg(all(feature = "magic", not(target_os = "android")))]
    /// File magic type
    #[arg(long, help_heading = "Features")]
    pub magic: bool,

    // Formatting section
    /// How to display dates (affects the output of --created, --modified, and --accessed)
    #[arg(long, value_enum, default_value = "human", help_heading = "Formatting")]
    pub date_format: DateFormat,

    /// How to display numbers (affects the output of --hard-links, and --blocks)
    #[arg(long, value_enum, default_value = "human", help_heading = "Formatting")]
    pub number_format: NumberFormat,

    /// How to display users or groups (affects the output of --user, --group, and --long)
    #[arg(long, value_enum, default_value = "name", help_heading = "Formatting")]
    pub ownership_format: OwnershipFormat,

    /// How to display permissions (affects the output of --permission)
    #[arg(
        long,
        value_enum,
        default_value = "symbolic",
        help_heading = "Formatting"
    )]
    pub permission_format: PermissionFormat,

    /// How to display sizes (affects the output of --block-size, and --size)
    #[arg(
        long,
        value_enum,
        default_value = "decimal",
        help_heading = "Formatting"
    )]
    pub size_format: SizeFormat,
}

impl Args {
    /// Checks if any table-specific columns are requested.
    ///
    /// # Returns
    /// `true` if any table-only columns (magic, xattr, acl, context, mountpoint, or oneline) are requested.
    pub(crate) fn wants_table_column(&self) -> bool {
        #[cfg(all(feature = "magic", not(target_os = "android")))]
        let magic = self.magic;
        #[cfg(any(not(feature = "magic"), target_os = "android"))]
        let magic = false;

        magic
            || self.xattr
            || self.acl
            || self.context
            || self.mountpoint
            || self.oneline
    }

    /// Resolves which file-type indicator style is active.
    ///
    /// When several of `--classify`, `--file-type`, and `--slash` are given,
    /// the most informative one wins (classify > file-type > slash), matching
    /// the spirit of GNU `ls` where the broadest indicator set takes effect.
    ///
    /// # Returns
    /// The effective [`IndicatorStyle`] (`None` if no indicator flag is set).
    pub fn indicator_style(&self) -> IndicatorStyle {
        if self.classify {
            IndicatorStyle::Classify
        } else if self.file_type {
            IndicatorStyle::FileType
        } else if self.slash {
            IndicatorStyle::Slash
        } else {
            IndicatorStyle::None
        }
    }

    /// Checks whether the arguments request entry metadata.
    ///
    /// # Returns
    /// `true` if any metadata-displaying flag (long, size, dates, permissions, etc.) is set.
    pub fn wants_metadata(&self) -> bool {
        self.long
            || self.size
            || self.created
            || self.modified
            || self.accessed
            || self.permissions
            || self.hard_links
            || self.blocks
            || self.block_size
            || self.user
            || self.group
            || self.inode
    }
}
