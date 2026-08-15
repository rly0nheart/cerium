use cerium::render::theme::config::Theme;
use nu_ansi_term::Color;

#[test]
fn test_default_theme_creation() {
    let theme = Theme::default();
    // Verify a few key authentic Catppuccin Mocha colors
    assert!(matches!(theme.size_none.color, Color::Rgb(166, 227, 161))); // green
    assert!(matches!(
        theme.permission_read.color,
        Color::Rgb(249, 226, 175)
    )); // yellow
    assert!(matches!(
        theme.filekind_directory.color,
        Color::Rgb(137, 180, 250) // blue
    ));
    assert!(matches!(
        theme.filekind_normal.color,
        Color::Rgb(205, 214, 244) // text
    ));
    assert!(matches!(
        theme.file_type_rust.color,
        Color::Rgb(250, 179, 135)
    )); // peach
}

#[test]
fn test_theme_deserialisation() {
    let toml = r#"
        user = "white"
        group = "white"
        tree-edge = "darkgray"
        header = "white"
        path = "blue"
        numeric = "cyan"
        punctuation = "darkgray"
        summary = "white"
        checksum = "white"
        magic = "white"
        mountpoint = "magenta"

        [size]
        none = { r = 255, g = 0, b = 0 }
        small = "green"
        medium = { r = 0, g = 255, b = 0 }
        large = "blue"

        [date]
        minute-old = "white"
        hour-old = "white"
        day-old = "white"
        week-old = "white"
        month-old = "white"
        older = "white"

        [permission]
        read = "yellow"
        write = "red"
        exec = "green"
        no-access = "darkgray"
        exec-sticky = "magenta"
        filetype = "blue"
        acl = "green"
        context = "magenta"
        attribute = "cyan"

        [filekind]
        normal = "white"
        directory = "blue"
        symlink = "cyan"

        [file_type]
        source = "yellow"
        build = "cyan"
        document = "white"
        crypto = "red"
        image = "magenta"
        video = "red"
        music = "green"
        compressed = "yellow"
        rust = "red"
        python = "blue"
        markdown = "white"
        pdf = "white"
        text = "white"

        [cli_help]
        header = "yellow"
        usage = "green"
        literal = "cyan"
        placeholder = "yellow"
    "#;

    let theme = Theme::parse(toml).unwrap();
    assert!(matches!(theme.size_none.color, Color::Rgb(255, 0, 0)));
    assert!(matches!(theme.size_small.color, Color::Green));
}
