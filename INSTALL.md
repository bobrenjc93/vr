# Installation Instructions for 'vr'

The binary has been built and a `vr` symlink has been created in this directory.

## Quick Option: Use from this directory

You can run it directly from this project directory:

```bash
cd /Users/bobren/projects/vim-review
./vr
```

## Recommended Option: Add to PATH

Add this project to your PATH so you can run `vr` from anywhere:

### For Zsh (macOS default):

```bash
echo 'export PATH="/Users/bobren/projects/vim-review:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### For Bash:

```bash
echo 'export PATH="/Users/bobren/projects/vim-review:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Alternative: Manual Install (requires sudo)

If you prefer to install it system-wide:

```bash
cd /Users/bobren/projects/vim-review
sudo cp target/release/vim-review /usr/local/bin/vr
```

## Verify Installation

After adding to PATH, test it:

```bash
# Navigate to any git repository with changes
cd /path/to/your/git/repo

# Run vr
vr
```

You should see the colorized diff viewer. Press `?` for help or `q` to quit.

## Uninstall

To remove from PATH, edit your ~/.zshrc or ~/.bashrc and remove the export line.

To remove system-wide install:
```bash
sudo rm /usr/local/bin/vr
```
