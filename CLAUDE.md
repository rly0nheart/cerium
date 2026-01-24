# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Cerium (`ce`) is a Rust-based ls-like utility for listing directory contents with rich formatting options. It's explicitly NOT trying to replace ls, but provides a modern alternative with features like tree view, file type detection, checksum calculation status, and customizable display formats.

## Build and Development Commands

### Building
```bash
# Run setup (installs libmagic, generates source map)
make setup

# Build release binary
make build
# or
cargo build --release

# Clean build artifacts
make clean
```

### Running
```bash
# Run with arguments
make run ARGS="--long --all"
# or
cargo run -- --long --all

# Run specific feature
cargo run --feature magic -- --magic
cargo run --feature checksum -- --checksum {md5|crc32|sha224|sha256|sha384|sha512}
cargo run --all-feature
```

### Testing
```bash
# Run all tests
make test
# or
cargo test

# Run a specific test
cargo test <test_name>
```

### Code Quality
```bash
# Format code
make fmt

# Lint with Clippy
make lint
```

### Installation
```bash
# Install globally with all feature
make install
```

## Optional Features

Cerium uses Cargo features for optional functionality:

- **`magic`**: File type detection via libmagic (requires libmagic system library). Use `scripts/install-libmagic.sh` to install the dependency.
- **`checksum`**: Display file checksum

When working with feature-gated code, ensure you're aware of the conditional compilation:
- `fs/feature/magic.rs` - only compiled with `--features magic`
- `fs/feature/checksum.rs` - only compiled with `--features checksum`

## Architecture Overview

### Core Data Flow

1. **Entry Point** (`main.rs`): Parses CLI args, sets up colours/icons, validates path, creates display mode
2. **Directory** (`fs/directory.rs`): Filesystem traversal and entry collection
3. **Entry** (`fs/entry.rs`): Individual file/directory metadata representation
4. **Display Factory** (`output/display/factory.rs`): Selects appropriate output format (Grid, List, or Tree)
5. **Display Mode** (`output/display/*`): Renders entries to stdout

### Module Structure

```
src/
├── cli/              # Command-line argument parsing (clap-based)
├── fs/               # Filesystem operations
│   ├── feature/      # Optional features (magic, checksum)
│   ├── cache.rs      # Entry data caching for performance
│   └── entry.rs      # Core Entry type with metadata
├── output/           # Output rendering system
│   ├── display/      # Display modes and factory (grid, list, tree)
│   ├── formatters/   # Data formatters (dates, sizes, permissions)
│   ├── layout/       # Column layout and width calculation
│   └── theme/        # Colours, icons, and styling
└── lib.rs            # Shared utilities (text width, quoting, TTY detection)
```

### Key Design Patterns

**Factory Pattern**: `DisplayFactory` creates the appropriate display mode (Grid/List/Tree) based on CLI args.

**Strategy Pattern**: All display modes implement the `DisplayMode` trait, allowing polymorphic rendering.

**Trait-based Formatting**: Formatters implement the `Formatter` trait for consistent data transformation (dates, sizes, permissions, etc.).

**Recursive Traversal**: The `RecursiveTraversal` trait provides a common pattern for recursive directory listing across Grid and List modes.

**Caching**: The `Cache` struct (`fs/cache.rs`) stores expensive-to-compute data like user/group names to avoid repeated system calls.

**Theme System**: Global colour and icon settings are set up once at startup via `ColourSettings::setup()` and `IconSettings::setup()` based on CLI flags. Colours are customizable via TOML configuration.

### Theme Customization System

Cerium supports runtime theme customization via a TOML configuration file (`~/.config/cerium.toml`). The theme system provides semantic colour categories that can be customized without modifying code.

#### Architecture

**Module Structure:**
```
src/output/theme/config/
├── mod.rs          # Config loading from ~/.config/cerium.toml
├── theme.rs        # Theme struct with semantic colour categories
└── colour.rs       # Colour type supporting RGB and named ANSI colours
```

**Flow:**
1. `main.rs` calls `config::load_theme()` at startup
2. Config loader attempts to read `~/.config/cerium.toml`
3. If config exists and is valid, deserialize the active theme
4. If config is missing or invalid, silently fall back to built-in Gruvbox theme
5. Initialize `RgbColours` static theme via `RgbColours::init(theme)`
6. All colour accessors throughout the app use theme-backed colours

**Key Components:**

- **`Theme` struct** (`config/theme.rs`): Contains ~40 semantic colour fields organized into categories:
  - Size gradients (bytes → gigabytes)
  - Date gradients (recent → old)
  - Permission colours (read, write, execute, etc.)
  - Entry type colours (files, directories, symlinks)
  - Code file type colours (Rust, Python, JavaScript, etc.)
  - Web file type colours (HTML, CSS, JSON, YAML)
  - Document type colours (text, markdown, PDF)
  - Media type colours (images, videos, audio)
  - Archive type colours
  - UI element colours (tree connectors, headers, paths, etc.)

- **`ThemeColour` type** (`config/colour.rs`): Deserializes from either:
  - RGB format: `{ r = 255, g = 128, b = 0 }`
  - Named ANSI colour: `"red"`, `"blue"`, `"yellow"`, etc.

- **`RgbColours` refactoring** (`output/theme/colours.rs`):
  - Changed from `const` values to functions returning theme colours
  - Uses `OnceLock<Theme>` for thread-safe lazy initialization
  - Example: `RgbColours::leaf_green()` returns `theme().size_kb.colour`
  - Maintains backward compatibility by keeping function names unchanged

- **Extension colour mapping** (`output/theme/icons.rs`):
  - `colour_for_entry()` checks themed extensions first before falling back to static PHF maps
  - Themed extensions (`.rs`, `.py`, `.js`, etc.) dynamically use theme colours
  - Non-themed extensions still use static PHF map lookups for performance

#### Built-in Themes

**Gruvbox** (default): Warm, retro colour palette extracted from the banner ASCII art colours:
- Aqua: RGB(104, 157, 106)
- Green: RGB(152, 151, 26)
- Yellow: RGB(215, 153, 33)
- Orange: RGB(214, 93, 14)
- Red: RGB(204, 36, 29)
- Purple: RGB(177, 98, 134)
- Blue: RGB(69, 133, 136)

#### Configuration Format

```toml
# ~/.config/cerium.toml

# Size gradients
size_bytes = { r = 80, g = 135, b = 80 }
size_kb = { r = 105, g = 165, b = 95 }
size_mb = { r = 135, g = 195, b = 115 }
size_gb = { r = 215, g = 255, b = 190 }

# Date gradients
date_recent = { r = 220, g = 235, b = 255 }
date_hours = { r = 170, g = 210, b = 245 }
date_days = { r = 130, g = 180, b = 230 }
date_weeks = { r = 100, g = 150, b = 210 }
date_months = { r = 75, g = 120, b = 180 }
date_old = { r = 20, g = 40, b = 90 }

# Permission colours
perm_read = "yellow"
perm_write = "red"
perm_execute = "green"
perm_none = "darkgray"
perm_special = "magenta"
perm_filetype = "blue"

# Entry types
entry_directory = { r = 150, g = 150, b = 255 }
entry_symlink = "blue"
entry_file = "white"

# ... 30+ more colour settings
```

#### Implementation Notes

- **No breaking changes**: Default behavior unchanged (Gruvbox theme)
- **Silent fallback**: Missing or invalid config files don't produce errors
- **Global state**: Theme loaded once at startup, never reloaded
- **Performance**: Themed colour functions have negligible overhead vs constants
- **Static maps preserved**: PHF maps for icons and non-themed colours remain compile-time
- **Backward compatibility**: All existing colour names maintained as functions

#### Adding New Themed Colours

1. Add field to `Theme` struct in `config/theme.rs`
2. Add default value in `Theme::gruvbox()`
3. Create accessor function in `RgbColours` (e.g., `pub(crate) fn NEW_COLOR() -> Colour`)
4. Update usage sites to call the function: `RgbColours::NEW_COLOR()`
5. Add the field to all theme files in `themes/`

#### Pre-made Themes

Theme files are available in the `themes/` directory:
- `gruvbox.toml` - Warm retro (default)
- `dracula.toml` - Vibrant dark
- `nord.toml` - Arctic blue
- `solarized-dark.toml` - Precision colours
- `catppuccin-mocha.toml` - Soothing pastels
- `tokyo-night.toml` - Downtown Tokyo vibes
- `one-dark.toml` - Atom's iconic theme
- `rose-pine.toml` - Natural pine with soho vibes

#### Testing Theme Changes

```bash
# Test with no config (should use Gruvbox)
cargo run -- --long

# Apply a theme
cp themes/dracula.toml ~/.config/cerium.toml

# Test with the theme
cargo run -- --long
```

### Entry Metadata System

Entries lazily load metadata via `conditional_metadata()` which only fetches data when needed based on CLI flags. This is a critical performance optimization - don't eagerly load metadata unless the user requested columns that need it.

The `args_need_metadata()` function determines when metadata is required. See `cli/args.rs` for the logic.

## Working with Display Modes

### Grid Display (`output/display/grid.rs`)
- Compact, multi-column output
- Uses `term_grid` for terminal-aware column layout
- Selected when no metadata columns are requested
- Implements `RecursiveTraversal` for recursive directory listing

### List Display (`output/display/list.rs`)
- Single-entry-per-line with optional columns
- Table-based layout with aligned columns
- Selected when `--long` or metadata columns are requested
- Implements `RecursiveTraversal` for recursive directory listing

### Tree Display (`output/display/tree.rs`)
- Hierarchical directory view
- Recursive by nature
- Selected when `--tree` flag is used
- Handles both compact and detailed (long) modes

## Common Patterns

### Adding a New Column
1. Add the column enum variant to `output/layout/column.rs`
2. Implement width calculation logic
3. Add formatter in `output/formatters/`
4. Update `output/populate.rs` to populate the column value
5. Add styling in `output/theme/styles/column.rs`

### Adding a New CLI Flag
1. Add the argument to `cli/args.rs`
2. If it's an enum type, define it in `cli/flags.rs`
3. Update `args_need_metadata()` if the flag requires file metadata
4. Handle the flag in the relevant display mode or formatter

### Adding a New Display Mode
1. Create a new file in `output/display/`
2. Implement the `DisplayMode` trait for your mode
3. If it supports recursion, implement `RecursiveTraversal`
4. Update `DisplayFactory::create()` to handle selection of your new mode
5. Export the module in `output/display/mod.rs`

### Working with Symlinks
Symlinks are formatted as "name ⇒ target" when `show_link_target` is true (controlled by `args.long`). The quoting logic in `lib.rs` (`quote_text_single`, `quote_text_double`) handles symlinks specially to quote each side independently.

## Terminal Detection and Colours

Cerium respects standard terminal colour environment variables:
- `NO_COLOR` - disables colours
- `FORCE_COLOR` / `CLICOLOR_FORCE` - forces colours
- `TERM` - terminal type detection
- `COLORTERM` - modern colour support

The `colours_enabled()` function in `lib.rs` implements this logic. The system is global - colours are either on or off for the entire program based on startup detection.

## Source Map Generation

The project auto-generates a source map (`src/README.md`) via `scripts/generate_source_map.py`. This is run automatically during `make setup` and `make build`. Don't edit `src/README.md` manually.

## Testing Notes

Tests use `tempfile` for temporary file system fixtures. When adding tests that interact with the file system, follow existing patterns in the test modules.
