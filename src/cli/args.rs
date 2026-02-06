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

use crate::cli::flags::{
    DateFormat, NumberFormat, OwnershipFormat, PermissionFormat, QuoteStyle, ShowColour,
    ShowHyperlink, ShowIcons, SizeFormat, SortBy,
};

#[cfg(feature = "checksum")]
use crate::cli::flags::HashAlgorithm;

use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(name = cerium::NAME, author = cerium::AUTHORS, version)]
pub(crate) struct Args {
    #[arg(default_value = ".", value_hint = ValueHint::AnyPath)]
    pub(crate) path: PathBuf,

    /// Display one entry per line
    #[arg(short = '1', long)]
    pub(crate) oneline: bool,

    /// Don't ignore entries starting with `.`
    #[arg(short, long)]
    pub(crate) all: bool,

    /// This entry's last accessed date
    #[arg(long)]
    pub(crate) accessed: bool,

    /// Display ACL indicator (+ if file has ACLs)
    #[arg(long)]
    pub(crate) acl: bool,

    /// Display number of blocks (format affected by --number-format)
    #[arg(short, long)]
    pub(crate) blocks: bool,

    /// Block size
    #[arg(short = 'B', long)]
    pub(crate) block_size: bool,

    /// This entry's creation date
    #[arg(short, long)]
    pub(crate) created: bool,

    /// Only show directories
    #[arg(short, long)]
    pub(crate) dirs: bool,

    /// Only show files
    #[arg(short, long)]
    pub(crate) files: bool,

    /// Find entries that match a query
    #[arg(
        long,
        value_name = "QUERY",
        default_value = "",
        conflicts_with = "tree",
        visible_alias = "search"
    )]
    pub(crate) find: String,

    /// Display this entry's group
    #[arg(short = 'g', long)]
    pub(crate) group: bool,

    /// Display number of hard links (format affected by --number-format)
    #[arg(long)]
    pub(crate) hard_links: bool,

    /// Show column headers, works with metadata flags and options
    #[arg(short = 'H', long)]
    pub(crate) headers: bool,

    /// Omit (a comma-separated list of) implied entries from output
    #[arg(long, value_name = "ENTRIES", value_delimiter = ',')]
    pub(crate) hide: Vec<String>,

    /// Hyperlink entry names WHEN
    #[arg(long, value_enum, default_value = "never", value_name = "WHEN")]
    pub(crate) hyperlink: ShowHyperlink,

    /// Display inode number
    #[arg(short, long)]
    pub(crate) inode: bool,

    /// Long listing format, show permissions, user, group, size, and modified date
    #[arg(short, long)]
    pub(crate) long: bool,

    /// This entry's last modification datetime
    #[arg(short, long)]
    pub(crate) modified: bool,

    /// Display filesystem mount point
    #[arg(long)]
    pub(crate) mountpoint: bool,

    /// This entry's permissions
    #[arg(short, long)]
    pub(crate) permission: bool,

    /// Omit empty files and directories from output
    #[arg(long)]
    pub(crate) prune: bool,

    /// How to quote entry names
    #[arg(short = 'Q', long, value_enum, default_value = "auto")]
    pub(crate) quote_name: QuoteStyle,

    /// Reverse order while sorting
    #[arg(short, long)]
    pub(crate) reverse: bool,

    /// List subdirectories recursively
    #[arg(short = 'R', long, conflicts_with_all = ["tree"])]
    pub(crate) recursive: bool,

    /// Display this entry's size
    #[arg(short, long)]
    pub(crate) size: bool,

    /// Sort entries by ...
    #[arg(long, value_enum, value_name = "BY", default_value = "name")]
    pub(crate) sort: SortBy,

    /// Display true size of directories based on their contents
    #[arg(short = 'S', long)]
    pub(crate) true_size: bool,

    /// Display directories hierarchically (tree view)
    #[arg(short, long, conflicts_with = "recursive")]
    pub(crate) tree: bool,

    /// Display this entry's user
    #[arg(short, long)]
    pub(crate) user: bool,

    /// What the heck happened? (this will only make sense when used with --find)
    #[arg(short, long)]
    pub(crate) verbose: bool,

    /// Set output width to COLS (0 = no limit)
    #[arg(short = 'w', long, value_name = "COLS")]
    pub(crate) width: Option<usize>,

    /// Display extended attributes (xattr)
    #[arg(short, long)]
    pub(crate) xattr: bool,

    /// Display SELinux security context
    #[arg(short = 'Z', long)]
    pub(crate) context: bool,

    /// Enable colours WHEN
    #[arg(short = 'C', long, value_enum, default_value = "auto", value_name = "WHEN", visible_aliases = ["colors"], help_heading = "Display")]
    pub(crate) colours: ShowColour,

    /// Show icons WHEN
    #[arg(
        short = 'I',
        long,
        value_enum,
        default_value = "never",
        value_name = "WHEN",
        help_heading = "Display"
    )]
    pub(crate) icons: ShowIcons,

    #[cfg(feature = "checksum")]
    /// Checksum!
    #[arg(long, value_name = "ALGORITHM", help_heading = "Features")]
    pub(crate) checksum: Option<HashAlgorithm>,

    #[cfg(all(feature = "magic", not(target_os = "android")))]
    /// File magic type
    #[arg(long, help_heading = "Features")]
    pub(crate) magic: bool,

    // Formatting section
    /// How to display dates (affects the output of --created, --modified, and --accessed)
    #[arg(
        long,
        value_enum,
        default_value = "locale",
        help_heading = "Formatting"
    )]
    pub(crate) date_format: DateFormat,

    /// How to display numbers (affects the output of --hard-links, and --blocks)
    #[arg(
        long,
        value_enum,
        default_value = "humanly",
        help_heading = "Formatting"
    )]
    pub(crate) number_format: NumberFormat,

    /// How to display users or groups (affects the output of --user, --group, and --long)
    #[arg(long, value_enum, default_value = "name", help_heading = "Formatting")]
    pub(crate) ownership_format: OwnershipFormat,

    /// How to display permissions (affects the output of --permission)
    #[arg(
        long,
        value_enum,
        default_value = "symbolic",
        help_heading = "Formatting"
    )]
    pub(crate) permission_format: PermissionFormat,

    /// How to display sizes (affects the output of --block-size, and --size)
    #[arg(
        long,
        value_enum,
        default_value = "decimal",
        help_heading = "Formatting"
    )]
    pub(crate) size_format: SizeFormat,
}

/// Determines whether specified args requests entry metadata
///
/// # Parameters
///
/// * `args` Parsed command-line args
/// # Returns
///
/// True if any of the passed args request metadata, otherwise False.
pub(crate) fn is_metadata_args(args: &Args) -> bool {
    // 1. Anything that reads size
    if args.size || args.long {
        return true;
    }

    // 2. Any date information
    if args.created || args.modified || args.accessed || args.long {
        return true;
    }

    // 3. Permissions / owner / group / hard_links / blocks / inode
    if args.permission
        || args.hard_links
        || args.blocks
        || args.block_size
        || args.user
        || args.group
        || args.long
        || args.inode
    {
        return true;
    }
    false
}
