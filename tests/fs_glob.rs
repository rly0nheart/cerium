use cerium::fs::glob::Glob;

#[test]
fn test_literal_match() {
    let g = Glob::new("hello").unwrap();
    assert!(g.is_match("hello"));
    assert!(g.is_match("HELLO"));
    assert!(!g.is_match("hello world"));
    assert!(!g.is_match("say hello"));
}

#[test]
fn test_star_wildcard() {
    let g = Glob::new("*.txt").unwrap();
    assert!(g.is_match("file.txt"));
    assert!(g.is_match("document.txt"));
    assert!(!g.is_match("file.rs"));

    let g = Glob::new("file*").unwrap();
    assert!(g.is_match("file.txt"));
    assert!(g.is_match("file123"));
    assert!(g.is_match("file"));
    assert!(!g.is_match("myfile"));

    let g = Glob::new("*file*").unwrap();
    assert!(g.is_match("file"));
    assert!(g.is_match("myfile.txt"));
    assert!(g.is_match("the_file_name"));
}

#[test]
fn test_question_wildcard() {
    let g = Glob::new("file?.txt").unwrap();
    assert!(g.is_match("file1.txt"));
    assert!(g.is_match("fileA.txt"));
    assert!(!g.is_match("file12.txt"));
    assert!(!g.is_match("file.txt"));
}

#[test]
fn test_literal_dot() {
    let g = Glob::new("foo.bar").unwrap();
    assert!(g.is_match("foo.bar"));
    assert!(!g.is_match("fooXbar"));
}

#[test]
fn test_empty_pattern() {
    let g = Glob::new("").unwrap();
    assert!(g.is_match(""));
    assert!(!g.is_match("anything"));
}
