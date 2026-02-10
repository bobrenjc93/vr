# Vim Review

A terminal-based UI (TUI) for reviewing git and mercurial changes with vim keybindings. Navigate diffs like vim, add comments inline, and persist them with code provenance for AI consumption.

## Features

- 🔄 Works with both Git and Mercurial (auto-detects)
- 🎨 Colorized diff view (green for additions, red for deletions)
- ⌨️ Vim-style keybindings for navigation
- 💬 Inline commenting with persistence
- 🔍 Search functionality
- 📝 Comments stored with file path and line number context

## Installation

### Pre-built Binaries (Recommended)

Download the latest release for your platform from [GitHub Releases](https://github.com/bobrenjc93/vr/releases):

#### Linux (x86_64)
```bash
# Download and install
curl -LO https://github.com/bobrenjc93/vr/releases/latest/download/vr-linux-x86_64
chmod +x vr-linux-x86_64
sudo mv vr-linux-x86_64 /usr/local/bin/vr

# Or install to user directory
mkdir -p ~/.local/bin
mv vr-linux-x86_64 ~/.local/bin/vr
# Add ~/.local/bin to PATH if not already there
```

#### Linux (ARM64/aarch64)
```bash
curl -LO https://github.com/bobrenjc93/vr/releases/latest/download/vr-linux-aarch64
chmod +x vr-linux-aarch64
sudo mv vr-linux-aarch64 /usr/local/bin/vr
```

#### macOS (Intel)
```bash
curl -LO https://github.com/bobrenjc93/vr/releases/latest/download/vr-macos-x86_64
chmod +x vr-macos-x86_64
sudo mv vr-macos-x86_64 /usr/local/bin/vr
```

#### macOS (Apple Silicon)
```bash
curl -LO https://github.com/bobrenjc93/vr/releases/latest/download/vr-macos-aarch64
chmod +x vr-macos-aarch64
sudo mv vr-macos-aarch64 /usr/local/bin/vr
```

#### Windows
```powershell
# Download vr-windows-x86_64.exe from releases page
# Rename to vr.exe and add to PATH
```

### Homebrew (macOS and Linux)

```bash
# Coming soon
brew install vr
```

### Build from Source

Requires [Rust](https://rustup.rs/) 1.70 or later.

```bash
# Clone the repository
git clone https://github.com/bobrenjc93/vr.git
cd vr

# Build and install
cargo install --path .

# Or build without installing
cargo build --release
# Binary will be at target/release/vr
```

### Verify Installation

```bash
vr --version  # Should work once we add --version flag
# Or just run vr in any git or mercurial repository
cd /path/to/repo
vr
```

## Usage

The tool auto-detects whether you're in a git or mercurial repository.

### Git Examples

```bash
# Review uncommitted changes
vr

# Review last commit
vr HEAD

# Review specific commit
vr abc123

# Review branch diff
vr main..feature
```

### Mercurial Examples

```bash
# Review uncommitted changes
vr

# Review current changeset
vr .

# Review specific changeset
vr 123

# Review range of changesets
vr 100::110
```

## Keybindings

### Normal Mode

| Key | Action |
|-----|--------|
| `j` / `↓` | Move cursor down |
| `k` / `↑` | Move cursor up |
| `g` | Jump to top |
| `G` | Jump to bottom |
| `Ctrl+d` | Page down |
| `Ctrl+u` | Page up |
| `o` / `O` | Enter insert mode (add comment) |
| `d` | Delete comment at cursor |
| `/` | Enter search mode |
| `n` | Next search match |
| `N` | Previous search match |
| `?` | Show help screen |
| `:wq` | Save comments and quit |
| `:w` | Save comments |
| `:q` | Save and quit |
| `:q!` | Quit without saving |
| `Ctrl+c` | Force quit |

### Insert Mode

| Key | Action |
|-----|--------|
| `ESC` | Save comment and exit to normal mode |
| `Enter` | Add new line (multi-line comments) |
| `Backspace` | Delete character |
| Any character | Type comment text |

### Search Mode

| Key | Action |
|-----|--------|
| `Enter` | Execute search |
| `ESC` | Cancel search |
| `Backspace` | Delete character |
| Any character | Type search query |

## Comment Storage

Comments are saved to `/tmp/vr/{uuid}.txt` in plain text format:

```
src/main.rs:42
This function needs better error handling.
Consider using Result<T, E> instead of unwrap().

src/utils.rs:15
Add input validation here

```

Each comment includes:
- File path and line number on the first line (e.g., `src/main.rs:42`)
- Comment text (can be multi-line)
- Blank line separator between comments

The file path is printed when you save with `:wq`.

## Example Workflow

1. Make some changes to your code
2. Run `vr` to see the diff
3. Navigate to a line you want to comment on using `j`/`k`
4. Press `o` to enter insert mode
5. Type your comment (e.g., "AI: please add null check here")
6. Press `Enter` to add more lines if needed (multi-line comments supported!)
7. Press `ESC` to exit insert mode
8. Navigate to other lines and add more comments as needed
9. Press `:wq` and Enter to save and quit
10. The file path will be printed (e.g., `/tmp/vr/abc123.txt`)
11. Comments are saved in plain text format with file paths and line numbers

## Architecture

The application is built with:

- **ratatui**: Terminal UI framework
- **crossterm**: Terminal input handling
- **git2**: Git repository operations
- **serde**: JSON serialization for comments
- **uuid**: Unique file naming

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development

```bash
# Clone and build
git clone https://github.com/bobrenjc93/vr.git
cd vr
cargo build

# Run tests
cargo test

# Run with cargo
cargo run

# Format code
cargo fmt

# Lint
cargo clippy
```

## Releasing

To create a new release:

1. Update version in `Cargo.toml`
2. Create and push a git tag:
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin v0.2.0
   ```
3. GitHub Actions will automatically build binaries for all platforms and create a release

## Roadmap

- [ ] Add `--version` flag
- [ ] Homebrew formula
- [ ] Support for custom output directory
- [ ] Comment editing (not just add/delete)
- [ ] Integration with GitHub PR comments
- [ ] Syntax highlighting for code (not just diff markers)
- [ ] Export to Markdown format

## License

MIT
