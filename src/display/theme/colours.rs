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

use crate::cli::flags::ShowColour;
use crate::display::output::terminal;
use crate::display::theme::config::Theme;
use nu_ansi_term::{Color, Style};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) type Colour = Color;

static COLOURS_ENABLED: AtomicBool = AtomicBool::new(true);
static THEME: OnceLock<Theme> = OnceLock::new();

pub struct ColourSettings;

impl ColourSettings {
    pub(crate) fn enable() {
        COLOURS_ENABLED.store(true, Ordering::SeqCst);
    }

    pub(crate) fn disable() {
        COLOURS_ENABLED.store(false, Ordering::SeqCst);
    }

    pub(crate) fn is_enabled() -> bool {
        COLOURS_ENABLED.load(Ordering::SeqCst)
    }

    /// Setup colours at startup based on CLI flag / terminal detection
    pub fn setup(show_colour: ShowColour) {
        match show_colour {
            ShowColour::Always => Self::enable(),
            ShowColour::Never => Self::disable(),
            ShowColour::Auto => {
                if terminal::colours_enabled() && terminal::is_tty() {
                    Self::enable()
                } else {
                    Self::disable()
                }
            }
        }
    }
}

pub(crate) trait ColourPaint {
    fn apply_to(&self, text: &str) -> String;
    fn apply_to_char(&self, c: char) -> String;
}

impl ColourPaint for Style {
    fn apply_to(&self, text: &str) -> String {
        if ColourSettings::is_enabled() {
            self.paint(text).to_string()
        } else {
            text.to_string()
        }
    }

    fn apply_to_char(&self, c: char) -> String {
        if ColourSettings::is_enabled() {
            self.paint(c.to_string()).to_string()
        } else {
            c.to_string()
        }
    }
}

pub struct RgbColours;

#[rustfmt::skip]
#[allow(dead_code)]
impl RgbColours {
    /// Initialise the theme system (called once at startup)
    pub fn init(theme: Theme) {
        THEME.set(theme).ok();
    }

    /// Get the current theme
    pub(crate) fn theme() -> &'static Theme {
        THEME.get().expect("Theme not initialised - call RgbColours::init() first")
    }

    // Theme-backed colours (size gradients)
    pub(crate) fn pine_glade() -> Colour {
        Self::theme().size_bytes.colour
    }

    pub(crate) fn leaf_green() -> Colour {
        Self::theme().size_kb.colour
    }

    pub(crate) fn fern() -> Colour {
        Self::theme().size_mb.colour
    }

    pub(crate) fn gleaming_mint() -> Colour {
        Self::theme().size_gb.colour
    }

    // Theme-backed colours (date gradients)
    pub(crate) fn frost_glimmer() -> Colour {
        Self::theme().date_recent.colour
    }

    pub(crate) fn crystal_blue() -> Colour {
        Self::theme().date_hours.colour
    }

    pub(crate) fn cerulean() -> Colour {
        Self::theme().date_days.colour
    }

    pub(crate) fn azure_sky() -> Colour {
        Self::theme().date_weeks.colour
    }

    pub(crate) fn royal_blue() -> Colour {
        Self::theme().date_months.colour
    }

    pub(crate) fn ocean_blue() -> Colour {
        Self::theme().date_months.colour  // Reuse months colour
    }

    pub(crate) fn sapphire_shine() -> Colour {
        Self::theme().date_old.colour
    }

    pub(crate) fn midnight_blue() -> Colour {
        Self::theme().date_old.colour
    }

    pub(crate) fn sky_mist() -> Colour {
        Self::theme().date_days.colour  // Reuse days colour
    }

    pub(crate) fn ice_crystal() -> Colour {
        Self::theme().date_hours.colour  // Reuse hours colour
    }

    // Theme-backed colours (user/group)
    pub(crate) fn hen_of_the_day() -> Colour {
        Self::theme().user.colour
    }

    pub(crate) fn hen_of_the_night() -> Colour {
        Self::theme().group.colour
    }

    // Theme-backed colours (file types)
    pub(crate) fn almost_apricot() -> Colour {
        Self::theme().code_rust.colour
    }

    pub(crate) fn mega_blue() -> Colour {
        Self::theme().code_python.colour
    }

    pub(crate) fn thors_thunder() -> Colour {
        Self::theme().code_c.colour
    }

    pub(crate) fn malibu_blue() -> Colour {
        Self::theme().code_go.colour
    }

    pub(crate) fn princeton_orange() -> Colour {
        Self::theme().code_java.colour
    }

    pub(crate) fn scoville_high() -> Colour {
        Self::theme().web_html.colour
    }

    pub(crate) fn cyber_grape() -> Colour {
        Self::theme().web_css.colour
    }

    pub(crate) fn hawaii_morning() -> Colour {
        Self::theme().web_yaml.colour
    }

    pub(crate) fn extraordinary_abundance() -> Colour {
        Self::theme().doc_markdown.colour
    }

    pub(crate) fn sachet_pink() -> Colour {
        Self::theme().media_image.colour
    }

    pub(crate) fn mandarin_sorbet() -> Colour {
        Self::theme().media_video.colour
    }

    pub(crate) fn exhilarating_green() -> Colour {
        Self::theme().media_audio.colour
    }

    pub(crate) fn cobalite() -> Colour {
        Self::theme().entry_directory.colour
    }

    // Non-themed colours for specific file types not in the theme system
    pub(crate) const ZESTY: Color                    = Color::Rgb(248, 248, 148);
    pub(crate) const LILLIPUTIAN_LIME: Color         = Color::Rgb(137, 227, 81);
    pub(crate) const SNOWFLAKE: Color                = Color::Rgb(240, 240, 240);
    pub(crate) const BUMBLEBEE: Color                = Color::Rgb(255, 202, 40);
    pub(crate) const YRIEL_YELLOW: Color             = Color::Rgb(255, 219, 88);
    pub(crate) const WELDED_IRON: Color              = Color::Rgb(110, 110, 110);
    pub(crate) const PUNCHOUT_GLOVE: Color           = Color::Rgb(113, 147, 255);
    pub(crate) const GOLDEN_CARTRIDGE: Color         = Color::Rgb(189, 183, 107);
    pub(crate) const DEEP_COBALT: Color              = Color::Rgb(40, 70, 120);
    pub(crate) const POLAR_SHINE: Color              = Color::Rgb(245, 250, 255);
    pub(crate) const EVERGREEN: Color                = Color::Rgb(60, 110, 65);
    pub(crate) const MINTFIELD: Color                = Color::Rgb(170, 230, 150);
    pub(crate) const SUNFLOWER_YELLOW: Color         = Color::Rgb(240, 200, 60);
    pub(crate) const DAFFODIL_YELLOW: Color          = Color::Rgb(255, 245, 100);
}
