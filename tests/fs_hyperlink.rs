use cerium::fs::hyperlink::wrap_hyperlink;
use std::path::Path;

#[test]
fn test_wrap_hyperlink_absolute_path() {
    let path = Path::new("/home/user/file.txt");
    let result = wrap_hyperlink("file.txt", path);

    // Should contain OSC 8 sequences and the file:// URL
    assert!(result.contains("\x1b]8;;file:///home/user/file.txt\x1b\\"));
    assert!(result.contains("file.txt"));
    assert!(result.ends_with("\x1b]8;;\x1b\\"));
}

#[test]
fn test_wrap_hyperlink_contains_visible_text() {
    let path = Path::new("/tmp/test");
    let result = wrap_hyperlink("visible_text", path);

    // The visible text should be present
    assert!(result.contains("visible_text"));
}

#[test]
fn test_wrap_hyperlink_format() {
    let path = Path::new("/test/path");
    let result = wrap_hyperlink("link", path);

    // Should start with OSC 8 opener
    assert!(result.starts_with("\x1b]8;;"));
    // Should end with OSC 8 closer
    assert!(result.ends_with("\x1b]8;;\x1b\\"));
    // Should contain string terminator
    assert!(result.contains("\x1b\\"));
}
