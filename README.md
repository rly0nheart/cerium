![logo](https://codeberg.org/rly0nheart/cerium/raw/branch/master/img/logo-word.svg)

**A light `ls` alternative.**

Cerium lists files and directories. It borrows ideas from lsd and eza, and
stays small: eight crates, plus four more only if you turn on the optional
features.

## Table of Contents

- [Availability](#availability)
- [Development](#development)
- [Installation](#installation)
- [Optional features](#optional-features)
- [Usage](#usage)
- [Examples](#examples)
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

### From source

```bash
git clone https://codeberg.org/rly0nheart/cerium.git
cd cerium
make install
```

`make install` builds with all features.

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

## Usage

```
ce [OPTIONS] [PATH]
```

### Display

```bash
-1, --oneline          One entry per line
-l, --long             Long format (permissions, user, group, size, modified)
-t, --tree             Tree view
-H, --headers          Show column headers
-w, --width <COLS>     Output width (0 removes the limit)
-F, --classify         Append an indicator (one of */=@|) to names
--file-type            Like --classify, but no '*' on executables
--slash                Append / to directories
```

### Filtering

```bash
-a, --all              Include hidden entries
-d, --dirs             Directories only
-f, --files            Files only
--find <QUERY>         Show entries matching a glob
--hide <ENTRIES>       Drop entries matching these globs (comma-separated)
--prune                Drop empty files and directories
```

### Metadata

```bash
-p, --permissions      File permissions
-u, --user             Owner
-g, --group            Group
-s, --size             File size
-m, --modified         Modification time
-c, --created          Creation time
--accessed             Access time
-i, --inode            Inode number
-b, --blocks           Block count
-B, --block-size       Block size
--hard-links           Hard link count
--acl                  ACL indicator
-x, --xattr            Extended attributes
-Z, --context          SELinux context
--mountpoint           Mount point
-L, --dereference      Report on a symlink's target, not the link
```

### Sorting and traversal

```bash
--sort <BY>            name, size, created, accessed, modified, extension, inode
-r, --reverse          Reverse order
-R, --recursive        Recurse into subdirectories
-S, --dir-size         Size column shows recursive bytes, not item count
```

### Formatting

```bash
--date-format <FMT>            locale, human, timestamp
--number-format <FMT>          human, natural
--ownership-format <FMT>       name, id
--permission-format <FMT>      symbolic, octal, hex
--size-format <FMT>            bytes, binary, decimal
```

### Appearance

```bash
-C, --color <WHEN>       always, auto, never
-I, --icons <WHEN>       always, auto, never
--hyperlink <WHEN>       always, auto, never
-q, --quote-name <HOW>   auto, double, single, never
```

`--color` also answers to `--colour`, `--colors`, and `--colours`.

## Examples

Everyday listings:

```bash
ce -la                                    # Long format, all files
ce -t                                     # Tree view
ce -lt --icons=always                     # Tree with metadata and icons
ce --find='*.rs' --sort=size -r           # Rust files, largest first
```

Looking at metadata:

```bash
ce -pugm --date-format=human              # Permissions, owner, group, relative dates
ce -i --hard-links --sort=inode           # Inodes and hard links
ce --acl -x                               # ACLs and extended attributes
ce -lb --block-size                       # Block usage
```

Less common flags:

```bash
ce --magic --checksum sha256              # Type detection and checksums
ce -RS                                    # Recurse, with directory byte totals
ce --hide=target,node_modules -t          # Tree without build output
ce --permission-format=octal -p           # Octal permissions
ce --ownership-format=id -ug              # Numeric UIDs and GIDs
```

Several at once:

```bash
ce -laH --date-format=human --size-format=binary
ce --find='*.pdf' --checksum md5 --sort=modified -r
ce -t --prune --hide=.git,target --icons=always
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
