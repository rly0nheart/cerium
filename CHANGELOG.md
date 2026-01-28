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

## [0.1.6]
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
