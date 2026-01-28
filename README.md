![intro](https://codeberg.org/rly0nheart/cerium/raw/branch/master/img/intro.png)

Yet, another ls-like util that is **not** trying to replace `ls`.

Cerium gets inspiration from similar tools such as lsd and eza, but with a small difference: It aims to stay small by avoiding features that introduce heavy dependencies while doing what a tool of its kind is supposed to do... list files and directories.

## Development
Development happens on [Codeberg](https://codeberg.org/rly0nheart/cerium). The GitHub repository is a read-only mirror used solely for crates.io deployments. Please open issues and pull requests on Codeberg instead.

## Table of Contents
- [Installation](#installation-)
  - [With Cargo](#quick-install-with-cargo)
  - [Build from Source](#build-from-source)
- [Features (optional)](#features-optional)
  - [Magic](#magic)
  - [Checksum](#checksum)
- [Usage](#usage)
  - [Display Options](#display-options)
  - [Filtering](#filtering)
  - [Metadata Display](#metadata-display)
  - [Sorting & Traversal](#sorting--traversal)
  - [Formatting](#formatting)
  - [Appearance](#appearance)
- [Examples](#examples)
  - [Basic Operation](#basic-operations)
  - [Metadata Inspection](#metadata-inspection)
  - [Advanced Usage](#advanced-usage)
  - [Combined Operations](#combined-operations)
- [Themes](#themes)
  - [Quick Start](#quick-start)
  - [Available Themes](#available-themes)
- [License](#licence)

## Installation 

### Quick Install (with Cargo)

```shell
# Standard installation with all features
cargo install cerium --all-features

# Minimal installation
cargo install cerium

# Specific features
cargo install cerium --features magic
cargo install cerium --features checksum
```

### Build from Source

```bash
# Clone the repo
git clone https://codeberg.org/rly0nheart/cerium.git

# Move to cerium directory
cd cerium

# Build and install: This will build cerium with all its features
make install
```

## Features (optional)

### Magic

Content-based file type identification using libmagic. Shows actual file types regardless of extension.

**Requirements:** libmagic library (`scripts/install-libmagic.sh`)

```bash
ce --magic
```

### Checksum

Calculate file checksums with multiple algorithms.

**Supported:** `crc32`, `md5`, `sha224`, `sha256`, `sha384`, `sha512`

```bash
ce --checksum sha256
```

## Usage

```
ce [OPTIONS] [PATH]
```

### Display Options

```bash
-1, --oneline          One entry per line
-l, --long             Long format (permissions, user, group, size, modified)
-t, --tree             Tree view
-H, --column-headers   Show column headers
```

### Filtering

```bash
-a, --all              Include hidden entries
-d, --dirs             Directories only
-f, --files            Files only
--find <QUERY>         Search for entries that match a query
--hide <ENTRIES>       Exclude specific entries (comma-separated)
--prune                Omit empty directories
```

### Metadata Display

```bash
-p, --permission       File permissions
-u, --user             Owner
-g, --group            Group
-s, --size             File size
-m, --modified         Modification time
-c, --created          Creation time
--accessed             Access time
-i, --inode            Inode number
-b, --blocks           Block count
--hard-links           Hard link count
--acl                  ACL indicator
-x, --xattr            Extended attributes
--mountpoint           Mount point
```

### Sorting & Traversal

```bash
--sort <BY>            name, size, created, accessed, modified, extension, inode
-r, --reverse          Reverse order
-R, --recursive        Recurse into subdirectories
-S, --true-size        Calculate actual directory sizes
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
-C, --colo[u]rs <WHEN>   always, auto, never
-I, --icons <WHEN>       always, auto, never
-Q, --quote-name         auto, double, single, never
```

## Examples

### Basic Operations

```bash
ce -la                                    # Long format, all files
ce -t                                     # Tree view
ce -lt --icons=always                     # Tree with metadata and icons
ce --find=*.rs --sort=size -r              # Find Rust files, sort by size
```

### Metadata Inspection

```bash
ce -pugm --date-format=humanly            # Permissions, ownership, modified date, human dates
ce -i --hard-links --sort=inode           # Inodes and hard links
ce --acl -x                               # ACLs and extended attributes
ce -lb --block-size                       # Block usage
```

### Advanced Usage

```bash
ce --magic --checksum sha256              # Type detection + checksums
ce -RS --true-size                        # Recursive with actual directory sizes
ce --hide=target,node_modules -t          # Tree excluding build artifacts
ce --permission-format=octal -p           # Octal permissions
ce --ownership-format=id -ug              # Numeric UIDs/GIDs
```

### Combined Operations

```bash
ce -laH --date-format=humanly --size-format=binary
ce --find=.pdf --checksum md5 --sort=modified -r
ce -t --prune --hide=.git,target --icons=always
```

## Themes

Cerium supports customisable themes via a TOML configuration file. By default, it uses the Gruvbox colour palette.

### Quick Start

```bash
# Apply a pre-made theme
cp themes/dracula.toml ~/.config/cerium.toml
```

### Available Themes

See [`themes/README.md`](themes/README.md) for the full list of pre-made themes, installation instructions, and customisation guide.

## Licence
MIT Licence. [Read more](https://choosealicense.com/licenses/mit/) for details.