/*
MIT License

Copyright (c) 2025 Ritchie Mwewa

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use nu_ansi_term::Color as Colour;
use serde::{Deserialize, Deserializer, de};

/// A colour value that can be deserialised from either RGB components or a named colour string.
///
/// # TOML Formats
///
/// RGB object:
/// ```toml
/// colour = { r = 255, g = 128, b = 0 }
/// ```
///
/// Named colour:
/// ```toml
/// colour = "red"
/// ```
///
/// Supported named colours match nu_ansi_term::Colour variants:
/// - Basic: black, red, green, yellow, blue, purple/magenta, cyan, white
/// - Light: lightred, lightgreen, lightyellow, lightblue, lightpurple/lightmagenta, lightcyan, lightgray
/// - System: darkgray
#[derive(Debug, Clone)]
pub(crate) struct ThemeColour {
    pub(crate) colour: Colour,
}

impl<'de> Deserialize<'de> for ThemeColour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum ColourValue {
            Rgb { r: u8, g: u8, b: u8 },
            Named(String),
        }

        let value = ColourValue::deserialize(deserializer)?;

        let colour = match value {
            ColourValue::Rgb { r, g, b } => Colour::Rgb(r, g, b),
            ColourValue::Named(name) => parse_named_colour(&name).map_err(de::Error::custom)?,
        };

        Ok(ThemeColour { colour })
    }
}

/// Parses a named colour string to a nu_ansi_term::Colour
fn parse_named_colour(name: &str) -> Result<Colour, String> {
    match name.to_lowercase().as_str() {
        // Basic colours
        "black" => Ok(Colour::Black),
        "red" => Ok(Colour::Red),
        "green" => Ok(Colour::Green),
        "yellow" => Ok(Colour::Yellow),
        "blue" => Ok(Colour::Blue),
        "purple" | "magenta" => Ok(Colour::Purple),
        "cyan" => Ok(Colour::Cyan),
        "white" => Ok(Colour::White),

        // Light variants
        "lightblack" | "darkgray" | "darkgrey" => Ok(Colour::DarkGray),
        "lightred" => Ok(Colour::LightRed),
        "lightgreen" => Ok(Colour::LightGreen),
        "lightyellow" => Ok(Colour::LightYellow),
        "lightblue" => Ok(Colour::LightBlue),
        "lightpurple" | "lightmagenta" => Ok(Colour::LightPurple),
        "lightcyan" => Ok(Colour::LightCyan),
        "lightgray" | "lightgrey" => Ok(Colour::LightGray),

        _ => Err(format!(
            "Unknown colour name: '{}'. Supported colours: black, red, green, yellow, blue, purple, cyan, white, and their light variants (e.g., lightred), plus darkgray.",
            name
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
