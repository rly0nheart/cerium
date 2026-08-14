# Contributing to Cerium

## Getting started

You need Rust (edition 2024) with `cargo`. The `magic` feature needs libmagic,
which `make setup` installs. Icons need a terminal with a
[Nerd Font](https://www.nerdfonts.com/).

```sh
git clone https://codeberg.org/rly0nheart/cerium.git
cd cerium
make setup    # installs libmagic
make build    # release build
make test     # run the test suite
```

The Makefile covers the rest:

| Command         | Description                                       |
|-----------------|---------------------------------------------------|
| `make setup`    | Install libmagic for the `magic` feature          |
| `make build`    | Build the release binary                          |
| `make run`      | Run cerium (pass args with `ARGS="..."`)          |
| `make fmt`      | Format code with `cargo fmt`                      |
| `make lint`     | Run Clippy with strict warnings (`-D warnings`)   |
| `make test`     | Run all tests                                     |
| `make install`  | Install the binary to `~/.cargo/bin/ce`           |
| `make clean`    | Remove build artefacts                            |
| `make rebuild`  | Clean and rebuild from scratch                    |

## Architecture

Three top-level modules:

| Module     | Purpose                                            |
|------------|----------------------------------------------------|
| `cli/`     | Command-line argument parsing and flag definitions |
| `display/` | Output formatting, layout, styling, and theming    |
| `fs/`      | Filesystem operations, metadata, and entry types   |

Two feature flags:

| Feature    | What it enables                                     | External dependency |
|------------|-----------------------------------------------------|---------------------|
| `magic`    | Content-based file type identification via libmagic | `libmagic-dev`      |
| `checksum` | File checksums (CRC32, MD5, SHA-224/256/384/512)    | None (pure Rust)    |

Gate code that needs one behind `#[cfg(feature = "...")]`.

## Code style

Give every struct, function, and module one job. If a function outgrows that,
split it. Many small named functions beat one long one.

Cerium renders the same listing twice in some paths and once in others. Before
you add work to a per-entry code path, check whether the value already exists.
The width pass and the render pass share their results on purpose.

### Doc comments

Every function, method, struct, enum, and trait needs a doc comment, public or
private.

Structs, enums, and traits get a one-line summary of what they represent:

```rust
/// Thread-local caching layer for formatted display strings and computed values.
pub struct Cache;
```

Functions get a summary line starting with a verb, then `# Parameters` if they
take any beyond `&self`, then `# Returns` unless the answer is obvious:

```rust
/// Loads metadata for a path using a raw libc stat call.
///
/// # Parameters
/// - `path`: The filesystem path to query.
/// - `dereference`: If `true`, follows symlinks (stat); otherwise uses lstat.
///
/// # Returns
/// The populated [`Metadata`], or an I/O error if the stat call fails.
pub fn load(path: &Path, dereference: bool) -> io::Result<Metadata> {}
```

The rules for those comments: use `///`, and `//!` only for module docs. Say
`# Parameters`, not `# Arguments`. Say `# Returns`, not `# Return Value`. Put
no blank line between the summary and `# Parameters`, and one blank line
before `# Returns`. Use backticks for inline code and [`Type`] links for crate
types. Skip `# Examples`, `# Errors`, and `# Panics`, and fold error
behaviour into `# Returns`. Leave struct fields undocumented. Trivial getters
get the summary line and nothing else.

Do not write comments that argue with the reader or defend a choice. State
what the code does, or why a non-obvious constraint exists, and stop.

Spell in British English, with one exception: write `color`, not `colour`.
That keeps the code matching the theme file keys, which follow lsd and eza.

| American    | British      |
|-------------|--------------|
| behavior    | behaviour    |
| initialize  | initialise   |
| customize   | customise    |
| organize    | organise     |
| optimize    | optimise     |
| gray        | grey         |
| center      | centre       |
| favor       | favour       |

## Commit messages

Conventional commits:

```
<type>(<scope>): <description>
```

| Type       | When to use                                          |
|------------|------------------------------------------------------|
| `feat`     | New feature                                          |
| `fix`      | Bug fix                                              |
| `refactor` | Code restructuring without feature/behaviour changes |
| `docs`     | Documentation only                                   |
| `chore`    | Maintenance, CI, dependency updates                  |
| `test`     | Adding or updating tests                             |
| `style`    | Formatting, whitespace (no logic changes)            |

Add a scope when the change sits in one area:

```
feat(tree): add collapsible node support
fix(metadata): handle broken symlinks in stat call
chore(ci): fix nightly build workflow
docs(readme): update installation instructions
```

Keep descriptions lowercase, imperative, and short.

## Testing

Run `make test`.

Tests live in `tests/`, named `<module>_<topic>.rs`. `fs_entry.rs` covers
`fs::entry`. `display_theme.rs` covers `display::theme`. `common/mod.rs` holds
shared fixtures.

Add a test for any new public function or behaviour change. Use `tempfile` for
filesystem fixtures, already a dev dependency. Name tests after the scenario:
`test_sort_by_extension`, `test_broken_symlink`.

## Submitting changes

Fork the repository on [Codeberg](https://codeberg.org/rly0nheart/cerium) and
branch from `dev`. Write the code, document it, then run:

```sh
make fmt
make lint
make test
```

Open the pull request against `dev` on Codeberg. The
[GitHub mirror](https://github.com/rly0nheart/cerium) is read-only and exists
for crates.io deployments.

## Themes

Themes are TOML files read from `~/.config/cerium.toml`. Bundled ones live in
[`themes/`](themes). Role names follow lsd and eza.
[`themes/README.md`](themes/README.md) lists every role and shows how to write
a new theme.

## Versioning

The project follows [Semantic Versioning](https://semver.org/) and keeps
[`CHANGELOG.md`](CHANGELOG.md) in the
[Keep a Changelog](https://keepachangelog.com/) format. If your change
deserves an entry, add it under `[Unreleased]` in one of Added, Changed,
Deprecated, Removed, Fixed, or Security.
