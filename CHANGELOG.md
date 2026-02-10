# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.5] - 2026-02-10

### Changed
- Increase context to 5 lines before the commented line (plus commented line = 6 lines total)
- Provides better surrounding context for understanding changes

## [0.3.4] - 2026-02-10

### Changed
- Reduce context to just 2 lines total: 1 line before + the commented line
- Cleaner, more focused output without excessive whitespace

## [0.3.3] - 2026-02-10

### Changed
- Use `=` prefix for unchanged lines (instead of space) for better visual clarity
- Add separator lines (dashes) around context blocks
- Remove extra blank lines for cleaner output
- Much clearer visual distinction between added (+), removed (-), and unchanged (=) lines

## [0.3.2] - 2026-02-10

### Fixed
- Fix double prefix issue (++ instead of +) in context lines
- Context lines now display correctly with single prefix

## [0.3.1] - 2026-02-10

### Changed
- Show only lines before the comment (not after) to reduce confusion
- Context now includes the commented line plus 3 lines before it

## [0.3.0] - 2026-02-10

### Added
- Include 3 lines of context around each comment in output
- Add +/- prefixes to context lines to show if they are added, removed, or unchanged lines
- Better code provenance for AI consumption with full context

## [0.2.1] - 2026-02-10

### Fixed
- Walk up directory tree to find .git or .hg directory (fixes detection in subdirectories of large monorepos)

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

[Unreleased]: https://github.com/bobrenjc93/vr/compare/v0.3.5...HEAD
[0.3.5]: https://github.com/bobrenjc93/vr/compare/v0.3.4...v0.3.5
[0.3.4]: https://github.com/bobrenjc93/vr/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/bobrenjc93/vr/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/bobrenjc93/vr/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/bobrenjc93/vr/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/bobrenjc93/vr/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/bobrenjc93/vr/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/bobrenjc93/vr/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/bobrenjc93/vr/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/bobrenjc93/vr/releases/tag/v0.1.0
