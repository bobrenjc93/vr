# Contributing to vim-review

Thank you for your interest in contributing to vim-review! This document provides guidelines and information for contributors.

## Code of Conduct

Be respectful, inclusive, and constructive in all interactions.

## How to Contribute

### Reporting Bugs

When filing a bug report, please include:

1. Your operating system and version
2. Rust version (`rustc --version`)
3. Steps to reproduce the issue
4. Expected behavior
5. Actual behavior
6. Any error messages or logs

### Suggesting Features

Feature suggestions are welcome! Please:

1. Check if the feature has already been requested
2. Describe the use case and motivation
3. Explain how it would work
4. Consider implementation complexity

### Pull Requests

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/vim-review.git
   cd vim-review
   ```

2. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make Changes**
   - Write clean, readable code
   - Follow existing code style
   - Add comments for complex logic
   - Update documentation as needed

4. **Test Your Changes**
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

5. **Commit**
   ```bash
   git add .
   git commit -m "Add feature: your feature description"
   ```

   Commit message format:
   - Use present tense ("Add feature" not "Added feature")
   - Keep first line under 72 characters
   - Add detailed description if needed

6. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```

   Then create a pull request on GitHub.

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or later
- Git

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run
cargo run

# Run with arguments
cargo run -- HEAD
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check

# Run linter
cargo clippy

# Run clippy with stricter checks
cargo clippy -- -D warnings
```

## Project Structure

```
src/
├── main.rs       # Entry point, CLI handling
├── app.rs        # Application state and logic
├── git.rs        # Git operations
├── diff.rs       # Diff parsing
├── comment.rs    # Comment persistence
├── vim.rs        # Vim keybinding handlers
└── tui.rs        # Terminal UI rendering
```

## Code Style

- Use 4 spaces for indentation
- Follow Rust naming conventions
- Keep functions focused and small
- Add doc comments for public APIs
- Run `cargo fmt` before committing

## Adding Dependencies

- Minimize new dependencies when possible
- Prefer well-maintained crates
- Update `Cargo.toml` with appropriate version constraints
- Document why the dependency is needed in your PR

## Documentation

- Update README.md for user-facing changes
- Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/)
- Add doc comments for new public functions
- Update examples if behavior changes

## Questions?

- Open an issue for questions
- Check existing issues and PRs
- Read the code - it's well-commented!

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
