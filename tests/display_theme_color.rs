//! Color-value parsing, exercised through the public `Theme` API.
//!
//! Color resolution itself is internal (`pub(crate)`); these integration
//! tests verify the supported behaviour end-to-end via `Theme`.

use cerium::display::theme::config::Theme;
use nu_ansi_term::Color;

/// Parses a config snippet and returns the resolved `filekind.directory` color.
fn filekind_directory(value: &str) -> Color {
    let config = format!("[filekind]\ndirectory = {value}");
    let theme = Theme::parse(&config).unwrap();
    theme.filekind_directory.color
}

#[test]
fn test_parse_rgb_color() {
    assert!(matches!(
        filekind_directory("{ r = 255, g = 128, b = 64 }"),
        Color::Rgb(255, 128, 64)
    ));
}

#[test]
fn test_parse_named_color() {
    assert!(matches!(filekind_directory(r#""red""#), Color::Red));
}

#[test]
fn test_parse_named_color_case_insensitive() {
    assert!(matches!(
        filekind_directory(r#""LightBlue""#),
        Color::LightBlue
    ));
}

#[test]
fn test_invalid_color_falls_back_to_default() {
    // An unresolvable value keeps the built-in Catppuccin Mocha default
    // for that key.
    assert!(matches!(
        filekind_directory(r#""notacolor""#),
        Color::Rgb(137, 180, 250)
    ));
}
