use cerium::display::theme::config::{Theme, load_theme};
use nu_ansi_term::Color as Colour;

#[test]
fn test_load_theme_fallback() {
    // Should return gruvbox when no config exists
    let theme = load_theme();
    // Just verify it doesn't panic and returns a valid theme with authentic Gruvbox colours
    assert!(matches!(theme.size_bytes.colour, Colour::Rgb(152, 151, 26))); // green
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
        summary_number = "cyan"
        summary_text = "white"
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
