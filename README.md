![logo](https://codeberg.org/rly0nheart/cerium/raw/branch/master/img/logo.svg)

**A light `ls` alternative.**

Cerium lists files and directories. It borrows ideas from lsd and eza, and
stays small: eight crates, plus four more only if you turn on the optional
features.

## Table of Contents

- [Availability](#availability)
- [Development](#development)
- [Installation](#installation)
  - [Prebuilt binary](#prebuilt-binary)
  - [#With Cargo](#with-cargo)
- [Optional features](#optional-features)
- [Themes](#themes)
- [Licence](#licence)

## Availability

|        OS        | Tested |        Features         |
|:----------------:|:------:|:-----------------------:|
| Android (Termux) | `yes`  |       `checksum`        |
|      Fedora      | `yes`  | `checksum`, `filemagic` |
|      Ubuntu      | `yes`  | `checksum`, `filemagic` |
|      MacOS       |  `no`  |      `not tested`       |

## Development

Development happens on [Codeberg](https://codeberg.org/rly0nheart/cerium). The
[GitHub repository](https://github.com/rly0nheart/cerium) mirrors it read-only
for crates.io deployments. Open issues on either host. Open pull requests on
Codeberg.

## Installation

### Prebuilt binary

```bash
curl -fsSL https://codeberg.org/rly0nheart/cerium/raw/branch/master/scripts/install.sh | sh
```

The installer takes three options:

|        Option        |                      Description                       |
|:--------------------:|:------------------------------------------------------:|
|     `--nightly`      |   Install the latest nightly build instead of stable   |
|    `--dir <path>`    |   Installation directory (default: `/usr/local/bin`)   |
| `--features <value>` | Feature variant to install: `checksum`, `magic`, `all` |

> [!NOTE]
> The `magic` and `all` variants need libmagic at runtime. The installer tries
> to install it for you. Without `--features` you get a binary with no
> external dependencies.

```bash
# Nightly build
curl -fsSL .../install.sh | bash -s -- --nightly

# Custom install directory
curl -fsSL .../install.sh | bash -s -- --dir ~/.local/bin

# With checksum support
curl -fsSL .../install.sh | bash -s -- --features checksum

# With every feature (needs libmagic)
curl -fsSL .../install.sh | bash -s -- --features all
```

### With Cargo

```shell
# Everything
cargo install cerium --all-features

# Nothing optional
cargo install cerium

# Pick one
cargo install cerium --features magic
cargo install cerium --features checksum
```

## Optional features

`magic` reads the start of each file and names its type, so a mislabelled
`.txt` still shows up as a PNG. It needs the libmagic library, which
`scripts/libmagic.sh` installs.

```bash
ce --magic
```

`checksum` hashes each file. It supports `crc32`, `md5`, `sha224`, `sha256`,
`sha384`, and `sha512`, and pulls in no C libraries.

```bash
ce --checksum sha256
```

## Themes

Cerium reads a TOML theme from `~/.config/cerium.toml`. Without one it uses
Catppuccin Mocha.

Every role is optional. Anything you leave out keeps its default, so a config
can be one line. Values can be RGB tables, hex strings, named colors, or
references into a `[palette]` table you define. A
[matugen](https://github.com/InioX/matugen) template ships with the repo if you
want colors to track your wallpaper.

Role names follow lsd (`permission`, `date`, `size`, `tree-edge`) and eza
(`filekind`, `file_type`), so a palette from either tool maps across.

```bash
# Use a theme as-is
cp themes/dracula.toml ~/.config/cerium.toml

# Or change one role
printf '[filekind]\ndirectory = "#89b4fa"\n' > ~/.config/cerium.toml
```

[`themes/README.md`](themes/README.md) lists the bundled themes and every role
you can set.

## Licence

MIT Licence. See [choosealicense](https://choosealicense.com/licenses/mit/).
