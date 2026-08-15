// SPDX-License-Identifier: MIT

use crate::render::theme::config::Theme;
use clap::builder::Styles;
use clap::builder::styling::{Color, RgbColor, Style};

/// CLI help styling configuration backed by the active theme.
pub struct HelpStyle<'a> {
    theme: &'a Theme,
}

impl<'a> HelpStyle<'a> {
    /// Creates a new help style from the given theme.
    ///
    /// # Parameters
    /// - `theme`: The active theme to derive help colors from.
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    /// Builds clap [`Styles`] for CLI help output using the active theme colors.
    ///
    /// # Returns
    /// A [`Styles`] instance with header, usage, literal, and placeholder colors applied.
    pub fn get_styles(&self) -> Styles {
        use clap::builder::styling::AnsiColor;
        use nu_ansi_term::Color as ThemeColor;

        // Convert nu_ansi_term::Color to clap::builder::styling::Color
        let to_clap_color = |color: &ThemeColor| -> Color {
            match color {
                ThemeColor::Rgb(r, g, b) => Color::Rgb(RgbColor(*r, *g, *b)),
                ThemeColor::Black => Color::Ansi(AnsiColor::Black),
                ThemeColor::Red => Color::Ansi(AnsiColor::Red),
                ThemeColor::Green => Color::Ansi(AnsiColor::Green),
                ThemeColor::Yellow => Color::Ansi(AnsiColor::Yellow),
                ThemeColor::Blue => Color::Ansi(AnsiColor::Blue),
                ThemeColor::Purple => Color::Ansi(AnsiColor::Magenta),
                ThemeColor::Cyan => Color::Ansi(AnsiColor::Cyan),
                ThemeColor::White => Color::Ansi(AnsiColor::White),
                ThemeColor::DarkGray => Color::Ansi(AnsiColor::BrightBlack),
                ThemeColor::LightRed => Color::Ansi(AnsiColor::BrightRed),
                ThemeColor::LightGreen => Color::Ansi(AnsiColor::BrightGreen),
                ThemeColor::LightYellow => Color::Ansi(AnsiColor::BrightYellow),
                ThemeColor::LightBlue => Color::Ansi(AnsiColor::BrightBlue),
                ThemeColor::LightPurple => Color::Ansi(AnsiColor::BrightMagenta),
                ThemeColor::LightCyan => Color::Ansi(AnsiColor::BrightCyan),
                ThemeColor::LightGray => Color::Ansi(AnsiColor::BrightWhite),
                _ => Color::Ansi(AnsiColor::White),
            }
        };

        let header_style = Style::new()
            .fg_color(Some(to_clap_color(&self.theme.cli_help_header.color)))
            .bold()
            .underline();
        let usage_style =
            Style::new().fg_color(Some(to_clap_color(&self.theme.cli_help_usage.color)));
        let literal_style =
            Style::new().fg_color(Some(to_clap_color(&self.theme.cli_help_literal.color)));
        let placeholder_style =
            Style::new().fg_color(Some(to_clap_color(&self.theme.cli_help_placeholder.color)));

        Styles::styled()
            .header(header_style)
            .usage(usage_style)
            .literal(literal_style)
            .placeholder(placeholder_style)
            .context(usage_style)
            .context_value(placeholder_style)
    }
}
