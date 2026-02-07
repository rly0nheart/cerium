use cerium::display::theme::config::Theme;
use nu_ansi_term::Color as Colour;

#[test]
fn test_gruvbox_theme_creation() {
    let theme = Theme::default();
    // Verify a few key authentic Gruvbox colours
    assert!(matches!(theme.size_bytes.colour, Colour::Rgb(152, 151, 26))); // green
    assert!(matches!(theme.perm_read.colour, Colour::Rgb(215, 153, 33))); // yellow
    assert!(matches!(
        theme.entry_directory.colour,
        Colour::Rgb(131, 165, 152) // bright_blue
    ));
    assert!(matches!(
        theme.entry_file.colour,
        Colour::Rgb(235, 219, 178)
    )); // fg
    assert!(matches!(theme.code_rust.colour, Colour::Rgb(214, 93, 14))); // orange
}

#[test]
fn test_theme_deserialisation() {
    let toml = r#"
        size_bytes = { r = 255, g = 0, b = 0 }
        size_kb = "green"
        size_mb = { r = 0, g = 255, b = 0 }
        size_gb = "blue"
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
        web_css = "purple"
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
    "#;

    let theme: Theme = toml::from_str(toml).unwrap();
    assert!(matches!(theme.size_bytes.colour, Colour::Rgb(255, 0, 0)));
    assert!(matches!(theme.size_kb.colour, Colour::Green));
}
