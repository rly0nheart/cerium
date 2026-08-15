// SPDX-License-Identifier: MIT

//! Color value parsing for theme configuration.
//!
//! A color can be expressed in any of four ways, in this resolution order:
//!
//! 1. **RGB table** — `{ r = 255, g = 128, b = 0 }`
//! 2. **Hex string** — `"#ff8000"`, `"#f80"`, or `"#ff8000ff"` (alpha ignored)
//! 3. **Palette reference** — a bare name matching a key in the `[palette]`
//!    table (e.g. `"primary"`)
//! 4. **Named color** — an ANSI name such as `"red"` or `"lightblue"`

use nu_ansi_term::Color;
use std::collections::HashMap;

/// A resolved color, ready for rendering.
///
/// This is the value type stored on every [`super::Theme`] field. It is
/// produced by [`color_from_value`] or by the built-in default.
#[derive(Debug, Clone)]
pub struct ThemeColor {
    pub color: Color,
}

/// Resolves a TOML value into a [`Color`], using `palette` to look up bare
/// palette references.
///
/// # Parameters
/// - `value`: The raw TOML value (an RGB table or a string token).
/// - `palette`: Resolved `[palette]` entries, keyed by name. Pass an empty
///   map when resolving the palette itself (references don't nest).
///
/// # Returns
/// The resolved [`Color`], or `None` if the value isn't a recognisable
/// color (the caller then falls back to the per-field default).
pub(crate) fn color_from_value(
    value: &toml::Value,
    palette: &HashMap<String, Color>,
) -> Option<Color> {
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
            Some(Color::Rgb(channel("r")?, channel("g")?, channel("b")?))
        }
        // String token: hex, palette reference, or named color.
        toml::Value::String(token) => {
            let token = token.trim();
            if token.starts_with('#') {
                parse_hex(token)
            } else if let Some(color) = palette.get(token) {
                Some(*color)
            } else {
                parse_named_color(token).ok()
            }
        }
        _ => None,
    }
}

/// Parses a hex color string (`#rgb`, `#rrggbb`, or `#rrggbbaa`).
///
/// Shorthand `#rgb` is expanded by nibble duplication (`#abc` → `#aabbcc`).
/// An 8-digit value's alpha channel is parsed but ignored (terminals have no
/// alpha). Returns `None` for any malformed input.
///
/// # Parameters
/// - `hex`: The hex string, including the leading `#`.
fn parse_hex(hex: &str) -> Option<Color> {
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

    Some(Color::Rgb(r, g, b))
}

/// Parses a named color string to a [`Color`].
///
/// # Parameters
/// - `name`: The color name to parse (case-insensitive).
///
/// # Returns
/// The corresponding [`Color`], or an error string if the name is
/// unrecognised.
pub(crate) fn parse_named_color(name: &str) -> Result<Color, String> {
    match name.to_lowercase().as_str() {
        // Basic colors
        "black" => Ok(Color::Black),
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "yellow" => Ok(Color::Yellow),
        "blue" => Ok(Color::Blue),
        "purple" | "magenta" => Ok(Color::Purple),
        "cyan" => Ok(Color::Cyan),
        "white" => Ok(Color::White),

        // Light variants
        "lightblack" | "darkgray" | "darkgrey" => Ok(Color::DarkGray),
        "lightred" => Ok(Color::LightRed),
        "lightgreen" => Ok(Color::LightGreen),
        "lightyellow" => Ok(Color::LightYellow),
        "lightblue" => Ok(Color::LightBlue),
        "lightpurple" | "lightmagenta" => Ok(Color::LightPurple),
        "lightcyan" => Ok(Color::LightCyan),
        "lightgray" | "lightgrey" => Ok(Color::LightGray),

        _ => Err(format!(
            "Unknown color name: '{}'. Supported colors: black, red, green, yellow, blue, purple, cyan, white, and their light variants (e.g., lightred), plus darkgray.",
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
            color_from_value(c, &HashMap::new()),
            Some(Color::Rgb(1, 2, 3))
        );
    }

    #[test]
    fn rgb_out_of_range_is_rejected() {
        let v: toml::Value = toml::from_str("c = { r = 0, g = 0, b = 999 }").unwrap();
        let c = v.as_table().unwrap().get("c").unwrap();
        assert_eq!(color_from_value(c, &HashMap::new()), None);
    }

    #[test]
    fn hex_long_short_and_alpha() {
        let p = HashMap::new();
        assert_eq!(
            color_from_value(&token("#89b4fa"), &p),
            Some(Color::Rgb(137, 180, 250))
        );
        // shorthand #abc -> #aabbcc
        assert_eq!(
            color_from_value(&token("#abc"), &p),
            Some(Color::Rgb(170, 187, 204))
        );
        // 8-digit: alpha ignored
        assert_eq!(
            color_from_value(&token("#89b4fa80"), &p),
            Some(Color::Rgb(137, 180, 250))
        );
        assert_eq!(color_from_value(&token("#zz"), &p), None);
    }

    #[test]
    fn named_color_is_case_insensitive() {
        assert_eq!(
            color_from_value(&token("LightBlue"), &HashMap::new()),
            Some(Color::LightBlue)
        );
    }

    #[test]
    fn palette_reference_resolves_and_unknown_is_none() {
        let mut palette = HashMap::new();
        palette.insert("accent".to_string(), Color::Rgb(9, 9, 9));
        assert_eq!(
            color_from_value(&token("accent"), &palette),
            Some(Color::Rgb(9, 9, 9))
        );
        assert_eq!(color_from_value(&token("nope"), &palette), None);
    }
}
