# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-02-10

### Added
- Mercurial (hg) support - auto-detects git or hg repositories
- VCS abstraction layer for unified diff handling across version control systems
- Support for hg-specific commands:
  - `vr` - review uncommitted changes (`hg diff`)
  - `vr .` - review current changeset (`hg export .`)
  - `vr 123` - review specific changeset (`hg export 123`)
  - `vr rev1::rev2` - review range of changesets (`hg log -p -r rev1::rev2`)

### Changed
- Removed git2 dependency in favor of lighter VCS detection
- Updated description to mention both git and mercurial support

## [0.1.1] - 2026-02-10

### Fixed
- Build Linux binaries with musl for better compatibility across different GLIBC versions
- Fixes "GLIBC_2.39 not found" errors on older Linux systems

## [0.1.0] - 2026-02-09

### Added
- Initial release
- TUI for reviewing git diffs with vim keybindings
- Multi-line comment support
- Save comments to `/tmp/vr/{uuid}.txt`
- Vim-style commands (`:wq`, `:w`, `:q`, `:q!`)
- Navigation with j/k, g/G, Ctrl+d/u
- Search functionality with `/`, n, N
- Help screen with `?`
- Support for reviewing uncommitted changes, specific commits, and branch diffs
- Colorized diff display (green for additions, red for deletions)
- Comment persistence with file path and line number provenance
- Multi-platform support (Linux x86_64/ARM64, macOS Intel/Apple Silicon, Windows)

[Unreleased]: https://github.com/bobrenjc93/vr/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/bobrenjc93/vr/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/bobrenjc93/vr/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/bobrenjc93/vr/releases/tag/v0.1.0
