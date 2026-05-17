//! Colour-value parsing, exercised through the public `Theme` API.
//!
//! Colour resolution itself is internal (`pub(crate)`); these integration
//! tests verify the supported behaviour end-to-end via `Theme`.

use cerium::display::theme::config::Theme;
use nu_ansi_term::Color as Colour;

/// Parses a config snippet and returns the resolved `entry_directory` colour.
fn entry_directory(config: &str) -> Colour {
    let theme: Theme = toml::from_str(config).unwrap();
    theme.entry_directory.colour
}

#[test]
fn test_parse_rgb_colour() {
    assert!(matches!(
        entry_directory("entry_directory = { r = 255, g = 128, b = 64 }"),
        Colour::Rgb(255, 128, 64)
    ));
}

#[test]
fn test_parse_named_colour() {
    assert!(matches!(
        entry_directory(r#"entry_directory = "red""#),
        Colour::Red
    ));
}

#[test]
fn test_parse_named_colour_case_insensitive() {
    assert!(matches!(
        entry_directory(r#"entry_directory = "LightBlue""#),
        Colour::LightBlue
    ));
}

#[test]
fn test_invalid_colour_falls_back_to_default() {
    // An unresolvable value keeps the built-in Catppuccin Mocha default
    // for that key.
    assert!(matches!(
        entry_directory(r#"entry_directory = "notacolour""#),
        Colour::Rgb(137, 180, 250)
    ));
}
