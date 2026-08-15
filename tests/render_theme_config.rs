use cerium::render::theme::config::{Theme, load_theme};
use nu_ansi_term::Color;

#[test]
fn test_load_theme_does_not_panic() {
    // `load_theme()` reads the real user config if one exists, so it must not
    // assert specific colors (that would be environment-dependent). The
    // deterministic per-field default behaviour is covered by
    // `test_empty_config_is_all_defaults` and `test_default_theme_creation`.
    // This is just a smoke test: it resolves and never panics.
    let theme = load_theme();
    let _ = theme.size_none.color;
}

#[test]
fn test_hex_colors() {
    let theme = Theme::parse(
        r##"
        [filekind]
        directory = "#89b4fa"
        normal = "#abc"
        symlink = "#89b4faff"
    "##,
    )
    .unwrap();
    assert!(matches!(
        theme.filekind_directory.color,
        Color::Rgb(137, 180, 250)
    ));
    // #abc expands to #aabbcc
    assert!(matches!(
        theme.filekind_normal.color,
        Color::Rgb(170, 187, 204)
    ));
    // 8-digit: alpha ignored
    assert!(matches!(
        theme.filekind_symlink.color,
        Color::Rgb(137, 180, 250)
    ));
}

#[test]
fn test_palette_layer_and_partial_override() {
    let theme = Theme::parse(
        r##"
        header = "#00ff00"

        [palette]
        accent = "#ff0000"

        [filekind]
        directory = "accent"
    "##,
    )
    .unwrap();
    // Palette reference resolves.
    assert!(matches!(
        theme.filekind_directory.color,
        Color::Rgb(255, 0, 0)
    ));
    // Direct hex at the top level resolves.
    assert!(matches!(theme.header.color, Color::Rgb(0, 255, 0)));
    // Unspecified key keeps the Catppuccin Mocha default (peach).
    assert!(matches!(
        theme.file_type_rust.color,
        Color::Rgb(250, 179, 135)
    ));
}

#[test]
fn test_invalid_value_falls_back_per_field() {
    let theme = Theme::parse(
        r##"
        [filekind]
        directory = "not-a-real-color"
        normal = "#zzzzzz"
    "##,
    )
    .unwrap();
    // Each unresolvable value uses its own per-field default.
    assert!(matches!(
        theme.filekind_directory.color,
        Color::Rgb(137, 180, 250)
    ));
    assert!(matches!(
        theme.filekind_normal.color,
        Color::Rgb(205, 214, 244)
    ));
}

#[test]
fn test_empty_config_is_all_defaults() {
    let theme = Theme::parse("").unwrap();
    assert!(matches!(
        theme.filekind_directory.color,
        Color::Rgb(137, 180, 250)
    ));
    assert!(matches!(theme.size_large.color, Color::Rgb(249, 226, 175)));
}

#[test]
fn test_unknown_keys_are_ignored() {
    // A key that isn't a known role is skipped rather than rejected, so a
    // theme file written for a newer version still loads.
    let theme = Theme::parse(
        r#"
        not_a_role = "cyan"

        [size]
        none = { r = 255, g = 0, b = 0 }
        nonsense = "green"
    "#,
    )
    .unwrap();
    assert!(matches!(theme.size_none.color, Color::Rgb(255, 0, 0)));
}
