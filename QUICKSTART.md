# Quick Start Guide

Get up and running with vim-review in 2 minutes.

## Install

```bash
git clone <repo-url>
cd vim-review
cargo build --release
sudo cp target/release/vim-review /usr/local/bin/  # Optional: install globally
```

## Basic Usage

```bash
# In any git repository with changes
vim-review
```

## Essential Keys

**Navigation**
- `j/k` - Move up/down
- `g/G` - Jump to top/bottom

**Comments**
- `o` - Add comment
- Type your comment
- `ESC` - Save and exit

**Other**
- `?` - Help
- `q` - Quit

## That's It!

Comments are automatically saved to `.vim-review/comments.json` with file paths and line numbers.

Use these comments to:
- Track review feedback
- Feed to AI for automated fixes
- Share with your team
- Document technical debt

For more details, see [README.md](README.md) and [EXAMPLES.md](EXAMPLES.md).
