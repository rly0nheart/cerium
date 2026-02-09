# Cerium Themes

Pre-made colour themes for Cerium. Each file is a complete, ready-to-use configuration.

## Available Themes

| Theme                                          | Description                                        |
|------------------------------------------------|----------------------------------------------------|
| [gruvbox.toml](gruvbox.toml)                            | Warm retro palette by Pavel Pertsev (default)      |
| [dracula.toml](dracula.toml)                            | Dark theme with vibrant colours by Zeno Rocha      |
| [nord.toml](nord.toml)                         | Arctic, bluish colour palette by Arctic Ice Studio |
| [solarized-dark.toml](solarized-dark.toml)                     | Precision colours by Ethan Schoonover              |
| [catppuccin-mocha.toml](catppuccin-mocha.toml)                   | Soothing pastel theme                              |
| [tokyo-night.toml](tokyo-night.toml)                        | Inspired by Downtown Tokyo at night                |
| [one-dark.toml](one-dark.toml)                           | Atom's iconic dark theme                           |
| [rose-pine.toml](rose-pine.toml)                          | All natural pine with soho vibes                   |

## Installation

1. Choose a theme from the list above
2. Copy the theme file to your config directory:

```bash
# Linux/macOS
cp themes/dracula.toml ~/.config/cerium.toml

# Or manually copy the contents
cat themes/dracula.toml > ~/.config/cerium.toml
```

3. Run `ce` to see the new colours

## Switching Themes

Simply replace `~/.config/cerium.toml` with a different theme file:

```bash
cp themes/tokyo-night.toml ~/.config/cerium.toml
```

## Customizing Themes

Each theme file is a complete configuration. You can:

1. Copy a theme as a starting point
2. Modify any colour values
3. Colours support two formats:
   - RGB: `{ r = 255, g = 128, b = 0 }`
   - Named: `"red"`, `"blue"`, `"cyan"`, etc.

## Contributing a Theme

1. Create a new `.toml` file in this directory
2. Use an existing theme as a template
3. Update all colour values to match your palette
4. Add a header comment with:
   - Theme name
   - Original author/source
   - Link to the colour palette

## Color Categories

Each theme defines colours for:

- **Size gradients**: `size_bytes`, `size_kb`, `size_mb`, `size_gb`
- **Date gradients**: `date_recent` through `date_old`
- **Permissions**: `perm_read`, `perm_write`, `perm_execute`, etc.
- **Entry types**: `entry_directory`, `entry_symlink`, `entry_file`
- **Code files**: `code_rust`, `code_python`, `code_javascript`, etc.
- **Web files**: `web_html`, `web_css`, `web_json`, `web_yaml`, `web_xml`
- **Documents**: `doc_text`, `doc_markdown`, `doc_pdf`
- **Media**: `media_image`, `media_video`, `media_audio`
- **UI elements**: `tree_connector`, `table_header`, `path_display`, etc.
- **Summary**: `summary_number`, `summary_text`
- **CLI help**: `cli_help_header`, `cli_help_usage`, etc.
