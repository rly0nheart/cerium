use cerium::display::layout::unicode_width::char_width;

#[test]
fn test_ascii_width() {
    assert_eq!(char_width('a'), 1);
    assert_eq!(char_width('Z'), 1);
    assert_eq!(char_width('0'), 1);
    assert_eq!(char_width(' '), 1);
}

#[test]
fn test_wide_cjk_width() {
    // CJK characters should be width 2
    assert_eq!(char_width('日'), 2);
    assert_eq!(char_width('本'), 2);
    assert_eq!(char_width('語'), 2);
}

#[test]
fn test_combining_marks() {
    // Combining marks should be width 0
    // U+0301 is COMBINING ACUTE ACCENT
    assert_eq!(char_width('\u{0301}'), 0);
}
