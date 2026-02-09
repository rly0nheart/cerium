# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [0.1.19] - YYYY-MM-DD
### Changed
- Simplify is_args_requesting_metadata into single expression
- Remove unnecessary abstractions in code
- Updated code doc strings

## [0.1.18] - 2026-02-08
### Changed
- Symlink icon from f0337 to f1177
- Showing Rust icons for `.cargo` and `.rustup` directories.

### Added
- Icon for `.android` directory
- Icon for `CLAUDE.md` file

### Fixed
- Nightly builds workflow file

## [0.1.17] - 2026-02-07
### Added
- `-L/--dereference` option to show metadata for the link target rather than the link itself
- Package description in help message

### Changed
- Styling in help message for `context` and `context_value`
- Extracted tests to a `tests/` directory 

### Removed
- Windows column from the availability table in [README](README.md)

## [0.1.16] - 2026-02-06
### Added
- Icon for directories created by coding agents: `.claude`, `.cursor`, `.codex`, `.aider`, `.autogpt`, `.devin`, `.copilot`, `openai`
- `lib.rs` file for package info constants

### Removed
- `banner.rs` file
- Cerium banner from CLI help message
- "empty directory" message when listing an empty directory

### Changed
- Extracted CLI help styling to a separate module: [display/styles/help.rs](src/display/styles/help.rs)
- `--prune` option also omits empty files from output

## [0.1.15] - 2026-02-05
### Changed
- Simplified nightly build workflow (Linux only)
- Fixed skip_tests output not being captured in nightly workflow
- Fixed publish workflow to skip tag creation silently if tag already exists

## [0.1.14] - 2026-02-05
### Changed
- No changelog (released in error)

## [0.1.13] - 2026-02-04
### Added
- `nightly.yml` file for nightly builds/releases

## [0.1.12] - 2026-01-31
### Changed
- chore(icons): add icon for jsonl files
- fix(linting): fix clippy errors and warnings

## [0.1.11] - 2026-01-28
### Changed
- chore(readme): okay, now i've finally found the perfect mockup

## [0.1.10] - 2026-01-28
### Changed
- docs(readme): add GitHub mirror link
- chore(readme): new mockup image in README.md

## [0.1.9] - 2026-01-28
### Changed
- docs(readme): Add an availability table showcasing which platforms cerium is available and tested on
- docs(readme): Update development section

## [0.1.8] - 2026-01-28
### Changed
- fix: Uneven styling for symlinks
- feat: Unconditionally disable filemagic on Android/Termux
- docs(readme): Added mockup image of Cerium output
- docs(crates.io): Disable generation of docs.rs page
- chore(version): Bump version to 0.1.8

## [0.1.7] - 2026-01-27
### Changed
- docs(development): Add a development section to README.md. This urges new issues, and pull requests to be opened on Codeberg
- chore(version): Bump version to 0.1.7

## [0.1.6] - 2026-01-27
### Changed
- Migrated back to Codeberg as the source of truth (GitHub was used temporarily during a Codeberg outage)
- docs(intro): Update intro image url to point to the Codeberg repo
- docs(source_map): Regenerated source map: src/README.md
- chore: Bump version to 0.1.6

## [0.1.5] - 2026-01-27
### Changed
- refactor: Extract unix permissions logic into separate module 
- chore: Bump version to 0.1.5
- docs: Minor update in README.md
- docs: Regenerated source map: src/README.md


## [0.1.4] - 2026-01-27

### Changed
- Updated CHANGELOG.md based on the [Keep a Changelog](https://keepachangelog.com/) format
- Updated publish.yml file for automatic release notes based on CHANGELOG.md
- Bump version: 0.1.3 -> 0.1.4

## [0.1.3] - 2026-01-27

### Fixed

- Android/Termux compilation errors with libc pointer types

## [0.1.2] - 2026-01-26

### Fixed

- Release workflow issues with immutable tags

## [0.1.1] - 2026-01-26

### Added

- Publishing to crates.io

### Changed

- Renamed `src/output` to `src/display` for clearer module organisation
- Moved icons to constants for better maintainability

### Fixed

- Removed wildcard version for libc dependency

## [0.1.0] - 2026-01-24

### Added

- Initial release
