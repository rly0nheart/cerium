use cerium::display::theme::config::colour::ThemeColour;
use nu_ansi_term::Color as Colour;
use serde::Deserialize;

#[test]
fn test_parse_rgb_colour() {
    let toml = r#"
        colour = { r = 255, g = 128, b = 64 }
    "#;

    #[derive(Deserialize)]
    struct Test {
        colour: ThemeColour,
    }

    let parsed: Test = toml::from_str(toml).unwrap();
    assert!(matches!(parsed.colour.colour, Colour::Rgb(255, 128, 64)));
}

#[test]
fn test_parse_named_colour() {
    let toml = r#"
        colour = "red"
    "#;

    #[derive(Deserialize)]
    struct Test {
        colour: ThemeColour,
    }

    let parsed: Test = toml::from_str(toml).unwrap();
    assert!(matches!(parsed.colour.colour, Colour::Red));
}

#[test]
fn test_parse_named_colour_case_insensitive() {
    let toml = r#"
        colour = "LightBlue"
    "#;

    #[derive(Deserialize)]
    struct Test {
        colour: ThemeColour,
    }

    let parsed: Test = toml::from_str(toml).unwrap();
    assert!(matches!(parsed.colour.colour, Colour::LightBlue));
}

#[test]
fn test_parse_invalid_colour() {
    let toml = r#"
        colour = "notacolour"
    "#;

    #[derive(Deserialize)]
    struct Test {
        #[allow(dead_code)]
        colour: ThemeColour,
    }

    let result: Result<Test, _> = toml::from_str(toml);
    assert!(result.is_err());
}
