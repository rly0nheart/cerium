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

/// Global colour toggle controlling whether ANSI colour codes are emitted.
pub struct ColourSettings;

impl ColourSettings {
    /// Enables colour output globally.
    pub(crate) fn enable() {
        COLOURS_ENABLED.store(true, Ordering::SeqCst);
    }

    /// Disables colour output globally.
    pub(crate) fn disable() {
        COLOURS_ENABLED.store(false, Ordering::SeqCst);
    }

    /// Checks whether colour output is currently enabled.
    pub(crate) fn is_enabled() -> bool {
        COLOURS_ENABLED.load(Ordering::SeqCst)
    }

    /// Configures colour output at startup based on the CLI flag and terminal detection.
    ///
    /// # Parameters
    /// - `show_colour`: The user's colour preference from the CLI.
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

/// Trait for applying colour styles to text, respecting the global colour toggle.
pub(crate) trait ColourPaint {
    /// Applies this style to a string slice, returning plain text when colours are disabled.
    fn apply_to(&self, text: &str) -> String;
    /// Applies this style to a single character, returning plain text when colours are disabled.
    fn apply_to_char(&self, c: char) -> String;
}

impl ColourPaint for Style {
    /// Applies this style to a string slice, returning plain text when colours are disabled.
    ///
    /// # Parameters
    /// - `text`: The text to style.
    ///
    /// # Returns
    /// The styled string, or the original text if colours are disabled.
    fn apply_to(&self, text: &str) -> String {
        if ColourSettings::is_enabled() {
            self.paint(text).to_string()
        } else {
            text.to_string()
        }
    }

    /// Applies this style to a single character, returning plain text when colours are disabled.
    ///
    /// # Parameters
    /// - `c`: The character to style.
    ///
    /// # Returns
    /// The styled character as a string, or the plain character if colours are disabled.
    fn apply_to_char(&self, c: char) -> String {
        if ColourSettings::is_enabled() {
            self.paint(c.to_string()).to_string()
        } else {
            c.to_string()
        }
    }
}

/// Theme-backed colour palette providing named colour accessors.
pub struct RgbColours;

#[rustfmt::skip]
#[allow(dead_code)]
impl RgbColours {
    /// Initialises the theme system (called once at startup).
    ///
    /// # Parameters
    /// - `theme`: The theme to store globally.
    pub fn init(theme: Theme) {
        THEME.set(theme).ok();
    }

    /// Returns the current theme.
    pub(crate) fn theme() -> &'static Theme {
        THEME.get().expect("Theme not initialised - call RgbColours::init() first")
    }

    /// Returns the theme colour for byte-sized files.
    pub(crate) fn pine_glade() -> Colour {
        Self::theme().size_bytes.colour
    }

    /// Returns the theme colour for kilobyte-sized files.
    pub(crate) fn leaf_green() -> Colour {
        Self::theme().size_kb.colour
    }

    /// Returns the theme colour for megabyte-sized files.
    pub(crate) fn fern() -> Colour {
        Self::theme().size_mb.colour
    }

    /// Returns the theme colour for gigabyte-sized files.
    pub(crate) fn gleaming_mint() -> Colour {
        Self::theme().size_gb.colour
    }

    /// Returns the theme colour for recent timestamps.
    pub(crate) fn frost_glimmer() -> Colour {
        Self::theme().date_recent.colour
    }

    /// Returns the theme colour for hour-old timestamps.
    pub(crate) fn crystal_blue() -> Colour {
        Self::theme().date_hours.colour
    }

    /// Returns the theme colour for day-old timestamps.
    pub(crate) fn cerulean() -> Colour {
        Self::theme().date_days.colour
    }

    /// Returns the theme colour for week-old timestamps.
    pub(crate) fn azure_sky() -> Colour {
        Self::theme().date_weeks.colour
    }

    /// Returns the theme colour for month-old timestamps.
    pub(crate) fn royal_blue() -> Colour {
        Self::theme().date_months.colour
    }

    /// Returns the theme colour for month-old timestamps (alternate).
    pub(crate) fn ocean_blue() -> Colour {
        Self::theme().date_months.colour  // Reuse months colour
    }

    /// Returns the theme colour for old timestamps.
    pub(crate) fn sapphire_shine() -> Colour {
        Self::theme().date_old.colour
    }

    /// Returns the theme colour for old timestamps (alternate).
    pub(crate) fn midnight_blue() -> Colour {
        Self::theme().date_old.colour
    }

    /// Returns the theme colour for day-old timestamps (alternate).
    pub(crate) fn sky_mist() -> Colour {
        Self::theme().date_days.colour  // Reuse days colour
    }

    /// Returns the theme colour for hour-old timestamps (alternate).
    pub(crate) fn ice_crystal() -> Colour {
        Self::theme().date_hours.colour  // Reuse hours colour
    }

    /// Returns the theme colour for user ownership.
    pub(crate) fn hen_of_the_day() -> Colour {
        Self::theme().user.colour
    }

    /// Returns the theme colour for group ownership.
    pub(crate) fn hen_of_the_night() -> Colour {
        Self::theme().group.colour
    }

    /// Returns the theme colour for Rust files.
    pub(crate) fn almost_apricot() -> Colour {
        Self::theme().code_rust.colour
    }

    /// Returns the theme colour for Python files.
    pub(crate) fn mega_blue() -> Colour {
        Self::theme().code_python.colour
    }

    /// Returns the theme colour for C/C++ files.
    pub(crate) fn thors_thunder() -> Colour {
        Self::theme().code_c.colour
    }

    /// Returns the theme colour for Go files.
    pub(crate) fn malibu_blue() -> Colour {
        Self::theme().code_go.colour
    }

    /// Returns the theme colour for Java files.
    pub(crate) fn princeton_orange() -> Colour {
        Self::theme().code_java.colour
    }

    /// Returns the theme colour for HTML files.
    pub(crate) fn scoville_high() -> Colour {
        Self::theme().web_html.colour
    }

    /// Returns the theme colour for CSS files.
    pub(crate) fn cyber_grape() -> Colour {
        Self::theme().web_css.colour
    }

    /// Returns the theme colour for YAML files.
    pub(crate) fn hawaii_morning() -> Colour {
        Self::theme().web_yaml.colour
    }

    /// Returns the theme colour for Markdown files.
    pub(crate) fn extraordinary_abundance() -> Colour {
        Self::theme().doc_markdown.colour
    }

    /// Returns the theme colour for image files.
    pub(crate) fn sachet_pink() -> Colour {
        Self::theme().media_image.colour
    }

    /// Returns the theme colour for video files.
    pub(crate) fn mandarin_sorbet() -> Colour {
        Self::theme().media_video.colour
    }

    /// Returns the theme colour for audio files.
    pub(crate) fn exhilarating_green() -> Colour {
        Self::theme().media_audio.colour
    }

    /// Returns the theme colour for directories.
    pub(crate) fn cobalite() -> Colour {
        Self::theme().entry_directory.colour
    }

    /// Returns the theme colour for summary text.
    pub(crate) fn summary() -> Colour {
        Self::theme().summary.colour
    }
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
