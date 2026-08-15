// SPDX-License-Identifier: MIT

use crate::cli::args::Args;

use crate::render::layout::alignment::{self, Alignment};
use crate::render::layout::width;
use crate::render::style::column::ColumnStyle;
use crate::render::style::element::ElementStyle;
use crate::fs::entry::Entry;

/// Identifies a data column in the tabular output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Column {
    #[cfg(all(feature = "magic", not(target_os = "android")))]
    Magic,

    Xattr,
    Acl,
    Context,
    Mountpoint,
    Permissions,
    HardLinks,
    User,
    Group,
    Blocks,
    BlockSize,
    Created,
    Accessed,
    Modified,
    Size,
    Name,
    Inode,
}

impl Column {
    /// Returns the display header label for this column.
    pub(crate) fn header(&self) -> &str {
        match self {
            Self::Name => "Name",

            #[cfg(all(feature = "magic", not(target_os = "android")))]
            Self::Magic => "Magic",

            Self::Xattr => "Xattr",
            Self::Acl => "ACL",
            Self::Context => "Context",
            Self::Mountpoint => "Mountpoint",
            Self::Inode => "inode",
            Self::Permissions => "Permissions",
            Self::HardLinks => "HardLinks",
            Self::User => "User",
            Self::Group => "Group",
            Self::Blocks => "Blocks",
            Self::BlockSize => "Block Size",
            Self::Size => "Size",
            Self::Created => "Created",
            Self::Accessed => "Accessed",
            Self::Modified => "Modified",
        }
    }

    /// Returns the text alignment for this column.
    pub(crate) fn alignment(&self) -> Alignment {
        match self {
            Self::Size
            | Self::Modified
            | Self::Created
            | Self::Accessed
            | Self::Inode
            | Self::HardLinks
            | Self::Blocks
            | Self::BlockSize => Alignment::Right,
            _ => Alignment::Left,
        }
    }
}

/// A rendered cell: the styled text plus its display width, measured once.
pub(crate) struct Cell {
    text: String,
    width: usize,
}

impl Cell {
    /// Measures a styled string, pairing it with its display width.
    ///
    /// # Parameters
    /// - `text`: The styled cell text (may contain ANSI codes).
    fn new(text: String) -> Self {
        let width = width::measure(&text);
        Self { text, width }
    }
}

/// Renders every column of one entry.
///
/// # Parameters
/// - `entry`: The entry to render.
/// - `columns`: The columns to display.
/// - `args`: Command-line arguments controlling display options.
/// - `add_alignment_space`: Whether to add a space for quote-alignment.
pub(crate) fn render_row(
    entry: &Entry,
    columns: &[Column],
    args: &Args,
    add_alignment_space: bool,
) -> Vec<Cell> {
    columns
        .iter()
        .map(|column| Cell::new(ColumnStyle::get(entry, column, args, add_alignment_space)))
        .collect()
}

/// Computes the width of each column from the rendered rows.
///
/// # Parameters
/// - `columns`: The columns being displayed.
/// - `rows`: The rendered rows to measure.
/// - `headers`: Whether header labels must also fit.
pub(crate) fn widths<'a>(
    columns: &[Column],
    rows: impl IntoIterator<Item = &'a [Cell]>,
    headers: bool,
) -> Vec<usize> {
    let mut widths: Vec<usize> = columns
        .iter()
        .map(|column| {
            if headers {
                width::measure(column.header())
            } else {
                0
            }
        })
        .collect();

    for row in rows {
        for (width, cell) in widths.iter_mut().zip(row) {
            *width = (*width).max(cell.width);
        }
    }

    widths
}

/// Renders one entry and measures it in a single pass, ready for [`widths`].
///
/// # Parameters
/// - `entries`: The entries to render.
/// - `columns`: The columns to display.
/// - `args`: Command-line arguments controlling display options.
/// - `add_alignment_space`: Whether to add a space for quote-alignment.
pub(crate) fn render_rows(
    entries: &[Entry],
    columns: &[Column],
    args: &Args,
    add_alignment_space: bool,
) -> Vec<Vec<Cell>> {
    entries
        .iter()
        .map(|entry| render_row(entry, columns, args, add_alignment_space))
        .collect()
}

/// Appends the styled, aligned header line.
///
/// # Parameters
/// - `out`: The buffer to append to.
/// - `columns`: The columns being displayed.
/// - `widths`: Pre-calculated column widths.
pub(crate) fn write_headers(out: &mut String, columns: &[Column], widths: &[usize]) {
    let cells: Vec<Cell> = columns
        .iter()
        .map(|column| Cell::new(ElementStyle::table_header(column.header())))
        .collect();

    write_row(out, &cells, columns, widths);
}

/// Appends one aligned row, trimmed of trailing padding.
///
/// # Parameters
/// - `out`: The buffer to append to.
/// - `cells`: The rendered cells of this row.
/// - `columns`: The columns being displayed (supplies alignment).
/// - `widths`: Pre-calculated column widths.
pub(crate) fn write_row(out: &mut String, cells: &[Cell], columns: &[Column], widths: &[usize]) {
    out.push_str(row_text(cells, columns, widths).trim_end());
    out.push('\n');
}

/// Joins one row's cells, each padded to its column width.
///
/// # Parameters
/// - `cells`: The rendered cells of this row.
/// - `columns`: The columns being displayed (supplies alignment).
/// - `widths`: Pre-calculated column widths.
pub(crate) fn row_text(cells: &[Cell], columns: &[Column], widths: &[usize]) -> String {
    cells
        .iter()
        .zip(columns)
        .zip(widths)
        .map(|((cell, column), width)| {
            alignment::pad(&cell.text, cell.width, *width, column.alignment())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Builds the ordered list of columns to display based on CLI arguments.
pub(crate) struct Selector;

impl Selector {
    /// Selects the columns to display based on the given arguments.
    ///
    /// # Parameters
    /// - `args`: Parsed command-line arguments.
    ///
    /// # Returns
    /// An ordered vector of [`Column`] variants to display.
    pub(crate) fn select(args: &Args) -> Vec<Column> {
        let mut columns = Vec::new();
        let mut push = |wanted: bool, column: Column| {
            if wanted && !columns.contains(&column) {
                columns.push(column);
            }
        };

        // `--long` implies its own set, in this order, before any explicit flags.
        push(args.long, Column::Permissions);
        push(args.long, Column::User);
        push(args.long, Column::Group);
        push(args.long, Column::Size);
        push(args.long, Column::Modified);

        for (wanted, column) in [
            (args.size, Column::Size),
            (args.permissions, Column::Permissions),
            (args.user, Column::User),
            (args.group, Column::Group),
        ] {
            push(wanted, column);
        }

        #[cfg(all(feature = "magic", not(target_os = "android")))]
        push(args.magic, Column::Magic);

        for (wanted, column) in [
            (args.xattr, Column::Xattr),
            (args.acl, Column::Acl),
            (args.context, Column::Context),
            (args.mountpoint, Column::Mountpoint),
            (args.inode, Column::Inode),
            (args.blocks, Column::Blocks),
            (args.hard_links, Column::HardLinks),
            (args.block_size, Column::BlockSize),
            (args.created, Column::Created),
            (args.modified, Column::Modified),
            (args.accessed, Column::Accessed),
        ] {
            push(wanted, column);
        }

        // Name always comes last, and never in tree mode (the tree draws it itself).
        push(!args.tree, Column::Name);

        columns
    }
}
