use cerium::display::layout::alignment::Alignment;
use cerium::display::layout::term_grid::{Cell, Direction, TermGrid};

fn make_cell(contents: &str) -> Cell {
    Cell {
        width: contents.len(),
        contents: contents.to_string(),
        alignment: Alignment::Left,
    }
}

#[test]
fn test_empty_grid() {
    let grid = TermGrid::new(Direction::TopToBottom);

    let display = grid.fit_into_width(80);
    assert_eq!(display.to_string(), "");
}

#[test]
fn test_single_cell() {
    let mut grid = TermGrid::new(Direction::TopToBottom);

    grid.add(make_cell("hello"));

    let display = grid.fit_into_width(80);
    assert_eq!(display.to_string(), "hello\n");
}

#[test]
fn test_multiple_cells_top_to_bottom() {
    let mut grid = TermGrid::new(Direction::TopToBottom);

    grid.add(make_cell("a"));
    grid.add(make_cell("b"));
    grid.add(make_cell("c"));
    grid.add(make_cell("d"));

    let display = grid.fit_into_columns(2);
    let output = display.to_string();

    // With TopToBottom and 2 columns, 4 cells:
    // Layout: col0=[a,b], col1=[c,d]
    // Row 0: a, c
    // Row 1: b, d
    assert!(output.contains("a"));
    assert!(output.contains("b"));
    assert!(output.contains("c"));
    assert!(output.contains("d"));
}

#[test]
fn test_fit_into_width() {
    let mut grid = TermGrid::new(Direction::LeftToRight);

    // Add cells with varying widths
    grid.add(make_cell("short"));
    grid.add(make_cell("medium_len"));
    grid.add(make_cell("x"));

    // With width 80, all three fit on one line
    assert_eq!(grid.fit_into_width(80).to_string().lines().count(), 1);

    // With width 5, should only fit 1 column
    let display = grid.fit_into_width(5);
    let output = display.to_string();
    // Each cell should be on its own line
    assert_eq!(output.lines().count(), 3);
}
