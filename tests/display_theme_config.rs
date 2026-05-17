use cerium::display::theme::config::{Theme, load_theme};
use nu_ansi_term::Color as Colour;

#[test]
fn test_load_theme_does_not_panic() {
    // `load_theme()` reads the real user config if one exists, so it must not
    // assert specific colours (that would be environment-dependent). The
    // deterministic per-field default behaviour is covered by
    // `test_empty_config_is_all_defaults` and `test_default_theme_creation`.
    // This is just a smoke test: it resolves and never panics.
    let theme = load_theme();
    let _ = theme.size_bytes.colour;
}

#[test]
fn test_hex_colours() {
    let theme: Theme = toml::from_str(
        r##"
        entry_directory = "#89b4fa"
        entry_file = "#abc"
        entry_symlink = "#89b4faff"
    "##,
    )
    .unwrap();
    assert!(matches!(theme.entry_directory.colour, Colour::Rgb(137, 180, 250)));
    // #abc expands to #aabbcc
    assert!(matches!(theme.entry_file.colour, Colour::Rgb(170, 187, 204)));
    // 8-digit: alpha ignored
    assert!(matches!(theme.entry_symlink.colour, Colour::Rgb(137, 180, 250)));
}

#[test]
fn test_palette_layer_and_partial_override() {
    let theme: Theme = toml::from_str(
        r##"
        [palette]
        accent = "#ff0000"

        [colors]
        entry_directory = "accent"
        table_header = "#00ff00"
    "##,
    )
    .unwrap();
    // Palette reference resolves.
    assert!(matches!(theme.entry_directory.colour, Colour::Rgb(255, 0, 0)));
    // Direct hex under [colors] resolves.
    assert!(matches!(theme.table_header.colour, Colour::Rgb(0, 255, 0)));
    // Unspecified key keeps the Catppuccin Mocha default (peach).
    assert!(matches!(theme.code_rust.colour, Colour::Rgb(250, 179, 135)));
}

#[test]
fn test_invalid_value_falls_back_per_field() {
    let theme: Theme = toml::from_str(
        r##"
        entry_directory = "not-a-real-colour"
        entry_file = "#zzzzzz"
    "##,
    )
    .unwrap();
    // Each unresolvable value uses its own per-field default.
    assert!(matches!(theme.entry_directory.colour, Colour::Rgb(137, 180, 250)));
    assert!(matches!(theme.entry_file.colour, Colour::Rgb(205, 214, 244)));
}

#[test]
fn test_empty_config_is_all_defaults() {
    let theme: Theme = toml::from_str("").unwrap();
    assert!(matches!(theme.entry_directory.colour, Colour::Rgb(137, 180, 250)));
    assert!(matches!(theme.size_gb.colour, Colour::Rgb(249, 226, 175)));
}

#[test]
fn test_load_config_with_valid_toml() {
    let toml_content = r#"
        size_bytes = { r = 255, g = 0, b = 0 }
        size_kb = "green"
        size_mb = "blue"
        size_gb = "yellow"
        date_recent = "white"
        date_hours = "white"
        date_days = "white"
        date_weeks = "white"
        date_months = "white"
        date_old = "white"
        perm_read = "yellow"
        perm_write = "red"
        perm_execute = "green"
        perm_none = "darkgray"
        perm_special = "magenta"
        perm_filetype = "blue"
        entry_directory = "blue"
        entry_symlink = "cyan"
        entry_file = "white"
        user = "white"
        group = "white"
        code_rust = "red"
        code_python = "blue"
        code_javascript = "yellow"
        code_c = "cyan"
        code_go = "blue"
        code_java = "red"
        code_ruby = "red"
        code_php = "blue"
        code_lua = "blue"
        web_html = "red"
        web_css = "magenta"
        web_json = "magenta"
        web_xml = "white"
        web_yaml = "cyan"
        doc_text = "white"
        doc_markdown = "white"
        doc_pdf = "white"
        media_image = "magenta"
        media_video = "red"
        media_audio = "green"
        archive = "yellow"
        tree_connector = "darkgray"
        table_header = "white"
        path_display = "blue"
        checksum = "white"
        magic = "white"
        xattr = "cyan"
        acl = "green"
        mountpoint = "magenta"
        numeric = "cyan"
        placeholder = "darkgray"
        cli_help_header = "yellow"
        cli_help_usage = "green"
        cli_help_literal = "cyan"
        cli_help_placeholder = "yellow"
        summary = "white"
        banner_gradient_1 = "cyan"
        banner_gradient_2 = "green"
        banner_gradient_3 = "yellow"
        banner_gradient_4 = "red"
        banner_gradient_5 = "red"
        banner_gradient_6 = "magenta"
        banner_gradient_7 = "blue"
    "#;

    let theme: Theme = toml::from_str(toml_content).unwrap();
    assert!(matches!(theme.size_bytes.colour, Colour::Rgb(255, 0, 0)));
}
