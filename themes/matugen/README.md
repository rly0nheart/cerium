# Matugen integration

[matugen](https://github.com/InioX/matugen) generates a Material You colour
scheme from your wallpaper. With the template in this directory, Cerium's
colours can follow that scheme automatically.

## How it works

`cerium.toml` here is a **matugen template**, not a config file. matugen
substitutes the `{{colors.*}}` placeholders with real hex values and writes the
result to `~/.config/cerium.toml`, which Cerium reads on every run.

## Setup

1. Copy the template into your matugen templates directory:

   ```bash
   mkdir -p ~/.config/matugen/templates
   cp themes/matugen/cerium.toml ~/.config/matugen/templates/cerium.toml
   ```

2. Register it in `~/.config/matugen/config.toml`:

   ```toml
   [templates.cerium]
   input_path  = '~/.config/matugen/templates/cerium.toml'
   output_path = '~/.config/cerium.toml'
   ```

3. Generate the scheme from an image:

   ```bash
   matugen image ~/Pictures/wallpaper.png
   ```

   Cerium picks up the new `~/.config/cerium.toml` immediately — just run `ce`.

Re-running matugen (e.g. from a wallpaper hook) keeps Cerium in sync.

## Customising

The generated config uses Cerium's `[palette]` + `[colors]` layers:

- **`[palette]`** maps Material You roles (`primary`, `surface`, `tertiary`,
  …) to the colours matugen produced.
- **`[colors]`** maps Cerium's semantic keys (`entry_directory`,
  `code_rust`, …) to palette names.

Edit the `[colors]` mappings in the template to taste — they only reference
palette names, so they survive re-generation. Any key you remove falls back to
the built-in **Catppuccin Mocha** colour for that key, so a partial template is
fine.

You can also point a key straight at a hex value or a named colour instead of a
palette reference:

```toml
[colors]
entry_directory = "{{colors.primary.default.hex}}"  # direct hex
table_header    = "yellow"                            # named colour
code_rust       = { r = 250, g = 179, b = 135 }       # RGB table
```
