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

use crate::display::theme::config::Theme;
use clap::builder::Styles;
use clap::builder::styling::{Color, RgbColor, Style};

pub(crate) struct HelpStyle<'a> {
    theme: &'a Theme,
}

impl <'a>HelpStyle<'a> {
    pub(crate) fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    pub(crate) fn get_styles(&self) -> Styles {
        use nu_ansi_term::Color as Colour;

        // Convert nu_ansi_term::Color to clap::builder::styling::Color
        let to_clap_color = |colour: &Colour| -> Color {
            match colour {
                Colour::Rgb(r, g, b) => Color::Rgb(RgbColor(*r, *g, *b)),
                Colour::Black => Color::Ansi(clap::builder::styling::AnsiColor::Black),
                Colour::Red => Color::Ansi(clap::builder::styling::AnsiColor::Red),
                Colour::Green => Color::Ansi(clap::builder::styling::AnsiColor::Green),
                Colour::Yellow => Color::Ansi(clap::builder::styling::AnsiColor::Yellow),
                Colour::Blue => Color::Ansi(clap::builder::styling::AnsiColor::Blue),
                Colour::Purple => Color::Ansi(clap::builder::styling::AnsiColor::Magenta),
                Colour::Cyan => Color::Ansi(clap::builder::styling::AnsiColor::Cyan),
                Colour::White => Color::Ansi(clap::builder::styling::AnsiColor::White),
                Colour::DarkGray => Color::Ansi(clap::builder::styling::AnsiColor::BrightBlack),
                Colour::LightRed => Color::Ansi(clap::builder::styling::AnsiColor::BrightRed),
                Colour::LightGreen => Color::Ansi(clap::builder::styling::AnsiColor::BrightGreen),
                Colour::LightYellow => Color::Ansi(clap::builder::styling::AnsiColor::BrightYellow),
                Colour::LightBlue => Color::Ansi(clap::builder::styling::AnsiColor::BrightBlue),
                Colour::LightPurple => {
                    Color::Ansi(clap::builder::styling::AnsiColor::BrightMagenta)
                }
                Colour::LightCyan => Color::Ansi(clap::builder::styling::AnsiColor::BrightCyan),
                Colour::LightGray => Color::Ansi(clap::builder::styling::AnsiColor::BrightWhite),
                _ => Color::Ansi(clap::builder::styling::AnsiColor::White),
            }
        };

        Styles::styled()
            .header(
                Style::new()
                    .fg_color(Some(to_clap_color(&self.theme.cli_help_header.colour)))
                    .bold()
                    .underline(),
            )
            .usage(Style::new().fg_color(Some(to_clap_color(&self.theme.cli_help_usage.colour))))
            .literal(
                Style::new().fg_color(Some(to_clap_color(&self.theme.cli_help_literal.colour))),
            )
            .placeholder(
                Style::new().fg_color(Some(to_clap_color(&self.theme.cli_help_placeholder.colour))),
            )
    }
}
