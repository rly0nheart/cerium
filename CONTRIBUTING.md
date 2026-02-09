# Contributing to Cerium

Thank you for your interest in contributing to Cerium! This guide will help you get started.

---

## Getting Started

### Prerequisites

- **Rust** (edition 2024) with `cargo`
- **libmagic** (optional, for the `magic` feature) -- installed automatically by `make setup`
- A terminal with [Nerd Font](https://www.nerdfonts.com/) support (for icon rendering)

### Setup

```sh
git clone https://codeberg.org/rly0nheart/cerium.git
cd cerium
make setup    # installs libmagic and generates the source map
make build    # release build
make test     # run the test suite
```

### Useful Make Targets

| Command         | Description                                      |
|-----------------|--------------------------------------------------|
| `make setup`    | Install dependencies and generate the source map |
| `make build`    | Build the release binary                         |
| `make run`      | Run cerium (pass args with `ARGS="..."`)         |
| `make fmt`      | Format code with `cargo fmt`                     |
| `make lint`     | Run Clippy with strict warnings (`-D warnings`)  |
| `make test`     | Run all tests                                    |
| `make install`  | Install the binary to `~/.cargo/bin/ce`          |
| `make clean`    | Remove build artefacts                           |
| `make rebuild`  | Clean and rebuild from scratch                   |

---

## Project Architecture

Cerium is organised into three top-level modules. Refer to the
[source map](src/README.md) for a full file-by-file breakdown.

| Module     | Purpose                                            |
|------------|----------------------------------------------------|
| `cli/`     | Command-line argument parsing and flag definitions |
| `display/` | Output formatting, layout, styling, and theming    |
| `fs/`      | Filesystem operations, metadata, and entry types   |

### Feature Flags

| Feature    | What it enables                                              | External dependency |
|------------|--------------------------------------------------------------|---------------------|
| `magic`    | Content-based file type identification via libmagic          | `libmagic-dev`      |
| `checksum` | File checksums (CRC32, MD5, SHA-224/256/384/512)             | None (pure Rust)    |

When adding code that depends on a feature flag, gate it with `#[cfg(feature = "...")]`.

---

## Code Style

### Single Responsibility Principle

Every struct, function, and module should have **one clear responsibility**. Before writing new code, ask:

- Does this function do exactly one thing?
- Could this struct be split into smaller, more focused types?
- Does this module mix unrelated concerns?

If a function grows beyond a single responsibility, refactor it into smaller pieces. Prefer many small, well-named functions over fewer large ones.

### Documentation (Doc Strings)

**Every** public and private function, method, struct, enum, and trait **must** have doc comments. Here is a quick summary:

#### Structs / Enums / Traits

One-line summary describing what it represents:

```rust
/// Thread-safe caching layer for formatted display strings and computed values.
pub struct Cache;
```

#### Functions / Methods

Every function gets:

1. **Summary line** -- one sentence starting with a verb (e.g., "Loads", "Builds", "Returns").
2. **`# Parameters`** section -- if the method takes any parameters (excluding `&self`).
   Use `- \`param\`: description` format.
3. **`# Returns`** section -- unless the return value is trivially obvious (e.g., simple getters).

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

#### Formatting Rules

- Use `///` (not `//!` except for module-level docs).
- Use `# Parameters` (not `# Arguments`).
- Use `# Returns` (not `# Return Value`).
- No blank line between the summary and `# Parameters`.
- One blank line between `# Parameters` and `# Returns`.
- Use backticks for inline code references and [`Type`] link syntax for crate types.
- **Do not** add `# Examples`, `# Errors`, or `# Panics` sections -- fold error info into `# Returns`.
- **Do not** add doc comments to individual struct fields.
- **Trivial getters** get a one-line summary only, no `# Parameters` or `# Returns`.

#### British English

Use British English spellings in all doc comments:

| American    | British      |
|-------------|--------------|
| color       | colour       |
| behavior    | behaviour    |
| initialize  | initialise   |
| customize   | customise    |
| organize    | organise     |
| optimize    | optimise     |
| gray        | grey         |
| center      | centre       |
| favor       | favour       |

---

## Commit Messages

This project uses **conventional commits**:

```
<type>(<scope>): <description>
```

### Types

| Type       | When to use                                          |
|------------|------------------------------------------------------|
| `feat`     | New feature                                          |
| `fix`      | Bug fix                                              |
| `refactor` | Code restructuring without feature/behaviour changes |
| `docs`     | Documentation only                                   |
| `chore`    | Maintenance, CI, dependency updates                  |
| `test`     | Adding or updating tests                             |
| `style`    | Formatting, whitespace (no logic changes)            |

### Scopes (optional)

Use a scope when the change is limited to a specific area, e.g.:

```
feat(tree): add collapsible node support
fix(metadata): handle broken symlinks in stat call
chore(ci): fix nightly build workflow
docs(readme): update installation instructions
```

Keep descriptions in lowercase, imperative mood, and concise.

---

## Testing

### Running Tests

```sh
make test
```

### Test Organisation

Tests live in the `tests/` directory and follow the naming convention `<module>_<topic>.rs`:

- `fs_entry.rs` -- tests for `fs::entry`
- `display_theme.rs` -- tests for `display::theme`
- `common/mod.rs` -- shared test helpers and fixtures

### Writing Tests

- Add tests for any new public function or behaviour change.
- Use `tempfile` for temporary filesystem fixtures (already a dev dependency).
- Test names should describe the scenario: `test_sort_by_extension`, `test_broken_symlink`.

---

## Submitting Changes

1. **Fork** the repository on [Codeberg](https://codeberg.org/rly0nheart/cerium).
2. **Create a branch** from `dev` for your changes.
3. **Write code** following the style guidelines above.
4. **Add doc strings** to all new functions, structs, and traits.
5. **Run the checks** before pushing:
   ```sh
   make fmt
   make lint
   make test
   ```
6. **Open a pull request** against the `dev` branch on Codeberg.

> **Note:** The [GitHub mirror](https://github.com/rly0nheart/cerium) is read-only and used for crates.io deployments. Please submit all contributions on Codeberg.

---

## Themes

Cerium supports TOML-based themes in `~/.config/cerium.toml`. Pre-made themes are in the
[`themes/`](themes) directory. See [`themes/README.md`](themes/README.md) for details on
creating new themes.

---

## Versioning

This project follows [Semantic Versioning](https://semver.org/) and uses the
[Keep a Changelog](https://keepachangelog.com/) format for [`CHANGELOG.md`](CHANGELOG.md).

When your change warrants a changelog entry, add it under the `[Unreleased]` section in the
appropriate category: Added, Changed, Deprecated, Removed, Fixed, or Security.
