use crate::display::output::terminal::{colours_enabled, is_tty};

/// Generates the ASCII art banner with theme-based gradient colours.
///
/// # Parameters
///
/// * `gradient` - A slice of 7 RGB tuples (r, g, b) for the gradient
///
/// # Returns
///
/// The banner string with ANSI colour codes applied according to the gradient
pub fn get_banner(gradient: &[(u8, u8, u8)]) -> String {
    // Convert RGB tuples to ANSI escape sequences
    let colours: Vec<String> = gradient
        .iter()
        .map(|(r, g, b)| format!("\x1b[38;2;{};{};{}m", r, g, b))
        .collect();
    let reset = "\x1b[0m";

    let lines = [
        "                              ███                            ",
        "                             ░░░                             ",
        "  ██████   ██████  ████████  ████  █████ ████ █████████████  ",
        " ███░░███ ███░░███░░███░░███░░███ ░░███ ░███ ░░███░░███░░███ ",
        "░███ ░░░ ░███████  ░███ ░░░  ░███  ░███ ░███  ░███ ░███ ░███ ",
        "░███  ███░███░░░   ░███      ░███  ░███ ░███  ░███ ░███ ░███ ",
        "░░██████ ░░██████  █████     █████ ░░████████ █████░███ █████",
        " ░░░░░░   ░░░░░░  ░░░░░     ░░░░░   ░░░░░░░░ ░░░░░ ░░░ ░░░░░ ",
    ];
    let show_colour = colours_enabled() && is_tty();

    let mut banner = String::from("\n");
    let total_lines = lines.len();

    for (line_idx, line) in lines.iter().enumerate() {
        if show_colour {
            let colour_idx = (line_idx * colours.len()) / total_lines;
            let colour_idx = colour_idx.min(colours.len() - 1);
            banner.push_str(&format!("{}{}{}\n", colours[colour_idx], line, reset));
        } else {
            banner.push_str(&format!("{}\n", line));
        }
    }
    banner
}
