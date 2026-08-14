# Cerium Themes

Pre-made color themes for Cerium. Each file is a complete, ready-to-use configuration.

## Available Themes

| Theme                                          | Description                                           |
|------------------------------------------------|-------------------------------------------------------|
| [catppuccin-mocha.toml](catppuccin-mocha.toml)                   | Soothing pastel theme (default)                       |
| [gruvbox.toml](gruvbox.toml)                            | Warm retro palette by Pavel Pertsev                   |
| [dracula.toml](dracula.toml)                            | Dark theme with vibrant colors by Zeno Rocha         |
| [nord.toml](nord.toml)                         | Arctic, bluish color palette by Arctic Ice Studio    |
| [solarized-dark.toml](solarized-dark.toml)                     | Precision colors by Ethan Schoonover                 |
| [tokyo-night.toml](tokyo-night.toml)                        | Inspired by Downtown Tokyo at night                   |
| [one-dark.toml](one-dark.toml)                           | Atom's iconic dark theme                              |
| [rose-pine.toml](rose-pine.toml)                          | All natural pine with soho vibes                      |

## Installation

1. Choose a theme from the list above
2. Copy the theme file to your config directory:

```bash
# Linux/macOS
cp themes/dracula.toml ~/.config/cerium.toml

# Or manually copy the contents
cat themes/dracula.toml > ~/.config/cerium.toml
```

3. Run `ce` to see the new colors

## Switching Themes

Simply replace `~/.config/cerium.toml` with a different theme file:

```bash
cp themes/tokyo-night.toml ~/.config/cerium.toml
```

## Customising Themes

Every key is **optional**. Anything you don't set keeps its built-in
**Catppuccin Mocha** default, so a config can be as small as a single line.
A config that exists but can't be parsed prints a non-fatal warning and uses
the default; an individual unresolvable key uses its own default.

A color can be written four ways:

- **RGB table**: `{ r = 255, g = 128, b = 0 }`
- **Hex**: `"#ff8000"`, `"#f80"`, or `"#ff8000ff"` (alpha ignored)
- **Named**: `"red"`, `"blue"`, `"cyan"`, … (plus `light*` variants, `darkgray`)
- **Palette reference**: a name defined in the optional `[palette]` table

### Palette layer

Define base colors once in `[palette]`, then reference them by name:

```toml
header = "accent"

[palette]
accent  = "#89b4fa"
surface = "#1e1e2e"
fg      = "#cdd6f4"

[filekind]
directory = "accent"
symlink   = "accent"
normal    = "fg"
# every other role -> Catppuccin Mocha default
```

Unsectioned roles (`user`, `group`, `tree-edge`, `header`, `path`, `numeric`,
`punctuation`, `summary`, `checksum`, `magic`, `mountpoint`) go at the top of
the file, before the first section header.

## Matugen (wallpaper-based colors)

Cerium ships a [matugen](https://github.com/InioX/matugen) template so its
colors can follow your wallpaper. See
[`matugen/README.md`](matugen/README.md) for setup.

## Contributing a Theme

1. Create a new `.toml` file in this directory
2. Use an existing theme as a template
3. Update all color values to match your palette
4. Add a header comment with:
   - Theme name
   - Original author/source
   - Link to the color palette

## Roles

Role names follow the conventions used by [lsd](https://github.com/lsd-rs/lsd)
(`permission`, `date`, `size`, `tree-edge`) and
[eza](https://github.com/eza-community/eza) (`filekind`, `file_type`), so a
palette ported from either tool maps across directly.

- `[size]`: `none`, `small`, `medium`, `large`
- `[date]`: `minute-old`, `hour-old`, `day-old`, `week-old`, `month-old`, `older`
- `[permission]`: `read`, `write`, `exec`, `no-access`, `exec-sticky`,
  `filetype`, `acl`, `context`, `attribute`
- `[filekind]`: `normal`, `directory`, `symlink`
- `[file_type]`: `source`, `build`, `document`, `crypto`, `image`, `video`,
  `music`, `compressed`, plus per-language refinements (`rust`, `python`,
  `javascript`, `c`, `go`, `java`, `ruby`, `php`, `lua`, `html`, `css`,
  `json`, `xml`, `yaml`, `markdown`, `pdf`, `text`)
- Top level: `user`, `group`, `tree-edge`, `header`, `path`, `numeric`,
  `punctuation`, `summary`, `checksum`, `magic`, `mountpoint`
- `[cli_help]`: `header`, `usage`, `literal`, `placeholder`
