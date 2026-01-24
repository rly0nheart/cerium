#!/usr/bin/env python3
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Tuple

PROJECT_ROOT = Path(__file__).resolve().parent.parent
SRC_DIR = PROJECT_ROOT / "src"
OUTPUT_FILE = SRC_DIR / "README.md"

# Descriptions for known files/folders using relative paths to src/
DESCRIPTIONS = {
    "main.rs": "Application entry point; initialises CLI and main runtime.",
    "cli": "Command-line argument parsing and flag structures.",
    "cli/mod.rs": "CLI module exports.",
    "cli/args.rs": "CLI argument parser and configuration.",
    "cli/flags.rs": "Enums for CLI flags (e.g., `--colours=WHEN`, `--date=TYPE`, etc.).",
    "fs": "Filesystem management (entries, directories, metadata).",
    "fs/mod.rs": "Filesystem module exports.",
    "fs/cache.rs": "In-memory caching of entry data for performance.",
    "fs/dir.rs": "Directory traversal and filesystem operations.",
    "fs/entry.rs": "Entry metadata and attributes representation.",
    "fs/feature": "Additional filesystem features (magic, git).",
    "fs/feature/mod.rs": "Features module exports.",
    "fs/feature/checksum.rs": "Entry checksum calculation.",
    "fs/feature/magic.rs": "File magic type detection via libmagic.",
    "fs/metadata.rs": "File metadata extraction and handling.",
    "fs/hyperlink.rs": "Terminal hyperlinks (OSC 8) for filesystem entries.",
    "fs/symlink.rs": "Symlink utilities (reading targets, formats display).",
    "fs/acl.rs": "ACL (Access Control List) detection and handling.",
    "fs/xattr.rs": "Extended attributes (xattr) detection and handling.",
    "fs/mountpoint.rs": "Mountpoint detection for filesystem entries.",
    "fs/glob.rs": "Glob pattern matching for search and hide filtering.",
    "fs/search.rs": "File search functionality using glob patterns.",
    "fs/tree.rs": "Tree structure and builder for hierarchical directory representation.",
    "output": "Output presentation system (layouts, themes, printers and formats).",
    "output/mod.rs": "Output module exports.",
    "output/banner.rs": "ASCII art banner generation with gradient colours.",
    "output/populate.rs": "Populates table rows with formatted entry data.",
    "output/quotes.rs": "Shell-safe text quoting utilities (single, double, auto).",
    "output/terminal.rs": "Terminal capabilities detection and configuration.",
    "output/formats": "Formats for output (dates, sizes, permissions, etc.).",
    "output/formats/mod.rs": "Format module exports.",
    "output/formats/format.rs": "Format trait used as an implementation base for formats.",
    "output/formats/number.rs": "Number formats (human-readable, metric, etc.).",
    "output/formats/date.rs": "Date and time formats.",
    "output/formats/ownership.rs": "User/group name resolution and formats.",
    "output/formats/permissions.rs": "File permissions formats (symbolic/octal).",
    "output/formats/size.rs": "File size formats (bytes, KB, MB, etc.).",
    "output/layout": "Column and row data structures, width calculation, and alignment.",
    "output/layout/mod.rs": "Layout module exports.",
    "output/layout/column.rs": "Column definitions, selectors, and width calculations.",
    "output/layout/row.rs": "Row structure and value resolution for columns.",
    "output/layout/alignment.rs": "Text alignment and padding utilities.",
    "output/layout/width.rs": "Cached width calculator for optimised text measurement.",
    "output/layout/unicode_width.rs": "Unicode character width calculation via libc wcwidth().",
    "output/layout/term_grid": "Terminal grid layout calculator for multi-column display.",
    "output/layout/term_grid/mod.rs": "Term grid module exports.",
    "output/layout/term_grid/cell.rs": "Grid cell structure with content, width, and alignment.",
    "output/layout/term_grid/layout.rs": "Grid layout calculator and display formatter.",
    "output/layout/term_grid/options.rs": "Grid configuration options (direction, column spacing).",
    "output/display": "Display modes and factory for rendering directory contents.",
    "output/display/mod.rs": "Display module exports.",
    "output/display/mode.rs": "DisplayMode trait for different output formats.",
    "output/display/factory.rs": "Factory for creating appropriate display modes based on args.",
    "output/display/grid.rs": "Grid display mode for compact multi-column layout.",
    "output/display/list.rs": "List display mode with column-based table output.",
    "output/display/tree.rs": "Tree display mode for hierarchical directory view.",
    "output/display/traversal.rs": "RecursiveTraversal trait for recursive directory rendering.",
    "output/theme": "UI theme, icons, colours, and styling system.",
    "output/theme/mod.rs": "Theme module exports.",
    "output/theme/colours.rs": "Colour palette and RGB colour definitions.",
    "output/theme/icons.rs": "Icons for file types, folders, and extensions.",
    "output/theme/config": "TOML-based theme configuration system.",
    "output/theme/config/mod.rs": "Config module exports and theme loader.",
    "output/theme/config/colour.rs": "Colour deserialisation (RGB and named colours).",
    "output/theme/config/theme.rs": "Theme struct with semantic colour categories and Gruvbox default.",
    "output/styles": "Styling system for cells, columns, and entries.",
    "output/styles/mod.rs": "Styles module exports.",
    "output/styles/text.rs": "Individual text styles logic.",
    "output/styles/column.rs": "Column-specific styling rules.",
    "output/styles/entry.rs": "Entry-specific styling and colorisation.",
}


def describe(path: Path) -> str:
    """Get description for a file or directory."""
    rel = path.relative_to(SRC_DIR).as_posix()
    return DESCRIPTIONS.get(rel, "No description available.")


def line_count(path: Path) -> int:
    """Count lines of executable code in a file, ignoring comments and empty lines."""
    try:
        with open(path, "r", encoding="utf-8", errors="ignore") as f:
            count = 0
            for line in f:
                stripped = line.strip()
                # Skip empty lines and lines starting with // or ///
                if stripped and not stripped.startswith(("///", "//")):
                    count += 1
            return count
    except Exception:
        return 0


def is_rust_file(path: Path) -> bool:
    """Check if a file is a Rust source file."""
    return path.suffix == ".rs"


def generate_tree(base: Path) -> Dict:
    """
    Recursively generate a tree structure of the directory.
    Returns dict with 'files' (list of tuples) and 'dirs' (dict of subdirs).
    """
    result = {"files": [], "dirs": {}}

    try:
        entries = sorted(base.iterdir())
    except PermissionError:
        return result

    for entry in entries:
        # Skip hidden files and common ignore patterns
        if entry.name.startswith(".") or entry.name in [
            "target",
            "node_modules",
            "__pycache__",
        ]:
            continue

        if entry.is_file():
            # Only include Rust files and README
            if is_rust_file(entry) or entry.name == "README.md":
                result["files"].append((entry.name, describe(entry), line_count(entry)))
        elif entry.is_dir():
            result["dirs"][entry.name] = generate_tree(entry)

    return result


def build_tree_visual(
    base: Path, tree: Dict, prefix: str = "", is_last: bool = True
) -> List[str]:
    """
    Build a visual tree representation with box-drawing characters.
    Returns a list of strings representing the tree.
    """
    lines = []

    # Combine files and directories for unified sorting
    items = []

    # Add directories
    for dirname in sorted(tree["dirs"].keys()):
        items.append(("dir", dirname, tree["dirs"][dirname]))

    # Add files
    for filename, _, line_cnt in tree["files"]:
        items.append(("file", filename, line_cnt))

    # Process all items
    for idx, (item_type, name, data) in enumerate(items):
        is_last_item = idx == len(items) - 1

        # Choose the correct branch character
        if is_last_item:
            connector = "╰── "
            extension = "    "
        else:
            connector = "├── "
            extension = "│   "

        if item_type == "dir":
            # Directory
            lines.append(f"{prefix}{connector}{name}/")
            # Recurse into subdirectory
            subtree_lines = build_tree_visual(
                base / name, data, prefix + extension, is_last_item
            )
            lines.extend(subtree_lines)
        else:
            # File
            lines.append(f"{prefix}{connector}{name}")

    return lines


def write_section(md: List[str], title: str):
    """Write a section header to the markdown list."""
    md.append("")
    md.append(title)
    md.append("")


def write_file_list(md: List[str], folder_path: Path, tree: Dict):
    """Write a list of files in a directory."""
    for filename, desc, lines in tree["files"]:
        file_path = folder_path / filename
        rel = file_path.relative_to(SRC_DIR)

        # Format with proper styling
        if desc != "No description available.":
            md.append(
                f"- **[{filename}]({rel.as_posix()})** – `{lines} LoEC (Lines of Executable Code)` – {desc}"
            )
        else:
            md.append(
                f"- **[{filename}]({rel.as_posix()})** – `{lines} LoEC (Lines of Executable Code)`"
            )


def write_directory(md: List[str], base: Path, tree: Dict, level: int = 0):
    """Recursively write directory structure to markdown."""
    for dirname, subtree in sorted(tree["dirs"].items()):
        full = base / dirname
        desc = describe(full)

        # Create heading with description
        heading = f"{'#' * (3 + level)} {dirname}/"
        if desc != "No description available.":
            heading += f" – {desc}"

        write_section(md, heading)

        # Write files in this directory
        if subtree["files"]:
            write_file_list(md, full, subtree)
        else:
            md.append("*No files in this directory.*")

        # Recurse into subdirectories
        if subtree["dirs"]:
            write_directory(md, full, subtree, level + 1)


def count_total_lines(tree: Dict) -> int:
    """Recursively count total lines in all files."""
    total = sum(lines for _, _, lines in tree["files"])
    for subtree in tree["dirs"].values():
        total += count_total_lines(subtree)
    return total


def count_total_files(tree: Dict) -> int:
    """Recursively count total number of files."""
    total = len(tree["files"])
    for subtree in tree["dirs"].values():
        total += count_total_files(subtree)
    return total


def generate_source_map():
    """Generate the complete source map documentation."""
    now = datetime.now().strftime("%x %X")

    # Generate tree
    src_tree = generate_tree(SRC_DIR)
    total_lines = count_total_lines(src_tree)
    total_files = count_total_files(src_tree)

    # Build dynamic tree visual
    tree_lines = ["src/"]
    tree_lines.extend(build_tree_visual(SRC_DIR, src_tree))
    tree_visual = "\n".join(tree_lines)

    md = [
        "# Cerium: Source Map (Auto-Generated)",
        "",
        "> [!WARNING]",
        "> Do not edit manually. This file is generated by `scripts/generate_source_map.py`.",
        "",
        "## Overview",
        "",
        f"- **Total Files**: {total_files}",
        f"- **Total Lines**: {total_lines:,}",
        f"- **Language**: Rust, Python, Shell",
        "",
        "## Project Structure",
        "",
        "```",
        tree_visual,
        "```",
    ]

    # Entry point
    write_section(md, "## Entry Point")
    main_path = SRC_DIR / "main.rs"
    if main_path.exists():
        main_lines = line_count(main_path)
        md.append(
            f"- **[main.rs](main.rs)** – `{main_lines} lines` – {describe(main_path)}"
        )

    # Recursively write all directories under src
    write_section(md, "## Modules")
    write_directory(md, SRC_DIR, src_tree)

    # Footer
    md.extend(
        [
            "",
            "---",
            "",
            f"*Generated by [generate_source_map.py](../scripts/generate_source_map.py) on {now}*",
        ]
    )

    # Write output
    OUTPUT_FILE.write_text("\n".join(md), encoding="utf-8")
    print(f"   \033[1;32mGenerated\033[0m source map saved at {OUTPUT_FILE}")
    print(
        f"       \033[1;36mStats\033[0m {total_files} files, {total_lines:,} LoEC (Lines of Executable Code)"
    )


if __name__ == "__main__":
    print(f"  \033[1;36mGenerating\033[0m source map to {OUTPUT_FILE}")
    generate_source_map()
