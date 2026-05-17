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

//! Colour value parsing for theme configuration.
//!
//! A colour can be expressed in any of four ways, in this resolution order:
//!
//! 1. **RGB table** — `{ r = 255, g = 128, b = 0 }`
//! 2. **Hex string** — `"#ff8000"`, `"#f80"`, or `"#ff8000ff"` (alpha ignored)
//! 3. **Palette reference** — a bare name matching a key in the `[palette]`
//!    table (e.g. `"primary"`)
//! 4. **Named colour** — an ANSI name such as `"red"` or `"lightblue"`

use nu_ansi_term::Color as Colour;
use std::collections::HashMap;

/// A resolved colour, ready for rendering.
///
/// This is the value type stored on every [`super::Theme`] field. It is
/// produced by [`colour_from_value`] or by the built-in default.
#[derive(Debug, Clone)]
pub struct ThemeColour {
    pub colour: Colour,
}

/// Resolves a TOML value into a [`Colour`], using `palette` to look up bare
/// palette references.
///
/// # Parameters
/// - `value`: The raw TOML value (an RGB table or a string token).
/// - `palette`: Resolved `[palette]` entries, keyed by name. Pass an empty
///   map when resolving the palette itself (references don't nest).
///
/// # Returns
/// The resolved [`Colour`], or `None` if the value isn't a recognisable
/// colour (the caller then falls back to the per-field default).
pub(crate) fn colour_from_value(
    value: &toml::Value,
    palette: &HashMap<String, Colour>,
) -> Option<Colour> {
    match value {
        // RGB table: { r = .., g = .., b = .. }
        toml::Value::Table(table) => {
            let channel = |key: &str| {
                table
                    .get(key)
                    .and_then(toml::Value::as_integer)
                    .filter(|n| (0..=255).contains(n))
                    .map(|n| n as u8)
            };
            Some(Colour::Rgb(channel("r")?, channel("g")?, channel("b")?))
        }
        // String token: hex, palette reference, or named colour.
        toml::Value::String(token) => {
            let token = token.trim();
            if token.starts_with('#') {
                parse_hex(token)
            } else if let Some(colour) = palette.get(token) {
                Some(*colour)
            } else {
                parse_named_colour(token).ok()
            }
        }
        _ => None,
    }
}

/// Parses a hex colour string (`#rgb`, `#rrggbb`, or `#rrggbbaa`).
///
/// Shorthand `#rgb` is expanded by nibble duplication (`#abc` → `#aabbcc`).
/// An 8-digit value's alpha channel is parsed but ignored (terminals have no
/// alpha). Returns `None` for any malformed input.
///
/// # Parameters
/// - `hex`: The hex string, including the leading `#`.
fn parse_hex(hex: &str) -> Option<Colour> {
    let digits = hex.strip_prefix('#')?;
    if !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }

    let (r, g, b) = match digits.len() {
        3 => {
            let n = u16::from_str_radix(digits, 16).ok()?;
            let r = ((n >> 8) & 0xf) as u8;
            let g = ((n >> 4) & 0xf) as u8;
            let b = (n & 0xf) as u8;
            // Expand each nibble: 0xF -> 0xFF (×17).
            (r * 17, g * 17, b * 17)
        }
        6 | 8 => (
            u8::from_str_radix(&digits[0..2], 16).ok()?,
            u8::from_str_radix(&digits[2..4], 16).ok()?,
            u8::from_str_radix(&digits[4..6], 16).ok()?,
        ),
        _ => return None,
    };

    Some(Colour::Rgb(r, g, b))
}

/// Parses a named colour string to a [`Colour`].
///
/// # Parameters
/// - `name`: The colour name to parse (case-insensitive).
///
/// # Returns
/// The corresponding [`Colour`], or an error string if the name is
/// unrecognised.
pub(crate) fn parse_named_colour(name: &str) -> Result<Colour, String> {
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

    fn token(s: &str) -> toml::Value {
        toml::Value::String(s.to_string())
    }

    #[test]
    fn rgb_table_resolves() {
        let v: toml::Value = toml::from_str("c = { r = 1, g = 2, b = 3 }").unwrap();
        let c = v.as_table().unwrap().get("c").unwrap();
        assert_eq!(
            colour_from_value(c, &HashMap::new()),
            Some(Colour::Rgb(1, 2, 3))
        );
    }

    #[test]
    fn rgb_out_of_range_is_rejected() {
        let v: toml::Value = toml::from_str("c = { r = 0, g = 0, b = 999 }").unwrap();
        let c = v.as_table().unwrap().get("c").unwrap();
        assert_eq!(colour_from_value(c, &HashMap::new()), None);
    }

    #[test]
    fn hex_long_short_and_alpha() {
        let p = HashMap::new();
        assert_eq!(
            colour_from_value(&token("#89b4fa"), &p),
            Some(Colour::Rgb(137, 180, 250))
        );
        // shorthand #abc -> #aabbcc
        assert_eq!(
            colour_from_value(&token("#abc"), &p),
            Some(Colour::Rgb(170, 187, 204))
        );
        // 8-digit: alpha ignored
        assert_eq!(
            colour_from_value(&token("#89b4fa80"), &p),
            Some(Colour::Rgb(137, 180, 250))
        );
        assert_eq!(colour_from_value(&token("#zz"), &p), None);
    }

    #[test]
    fn named_colour_is_case_insensitive() {
        assert_eq!(
            colour_from_value(&token("LightBlue"), &HashMap::new()),
            Some(Colour::LightBlue)
        );
    }

    #[test]
    fn palette_reference_resolves_and_unknown_is_none() {
        let mut palette = HashMap::new();
        palette.insert("accent".to_string(), Colour::Rgb(9, 9, 9));
        assert_eq!(
            colour_from_value(&token("accent"), &palette),
            Some(Colour::Rgb(9, 9, 9))
        );
        assert_eq!(colour_from_value(&token("nope"), &palette), None);
    }
}
