use cerium::fs::symlink::{
    SYMLINK_ARROW, SYMLINK_ARROW_WITH_SPACES, format_symlink, split_symlink,
};

#[test]
fn test_format_symlink() {
    assert_eq!(format_symlink("link", "target"), "link ⇒ target");
    assert_eq!(
        format_symlink("mylink", "/path/to/target"),
        "mylink ⇒ /path/to/target"
    );
}

#[test]
fn test_split_symlink() {
    let (name, target) = split_symlink("link ⇒ target").unwrap();
    assert_eq!(name, "link ");
    assert_eq!(target, " target");

    let (name, target) = split_symlink("a⇒b").unwrap();
    assert_eq!(name, "a");
    assert_eq!(target, "b");

    assert!(split_symlink("regular_file").is_none());
}

#[test]
fn test_split_symlink_with_spaces() {
    let (name, target) = split_symlink("my link ⇒ my target").unwrap();
    assert_eq!(name, "my link ");
    assert_eq!(target, " my target");
}

#[test]
fn test_arrow_constant() {
    assert_eq!(SYMLINK_ARROW, '⇒');
    assert_eq!(SYMLINK_ARROW_WITH_SPACES, " ⇒ ");
}
