#!/bin/bash
# Installation script for vim-review

set -e

echo "Installing vim-review as 'vr'..."
echo ""

# Build release binary
echo "Building release binary..."
cargo build --release

# Check if /usr/local/bin exists and is in PATH
if [ -d "/usr/local/bin" ]; then
    echo "Installing to /usr/local/bin/vr"
    sudo cp target/release/vim-review /usr/local/bin/vr
    sudo chmod +x /usr/local/bin/vr
    echo "✓ Installed to /usr/local/bin/vr"
elif [ -d "$HOME/.local/bin" ]; then
    echo "Installing to ~/.local/bin/vr"
    cp target/release/vim-review "$HOME/.local/bin/vr"
    chmod +x "$HOME/.local/bin/vr"
    echo "✓ Installed to ~/.local/bin/vr"
    echo ""
    echo "Make sure ~/.local/bin is in your PATH by adding this to your ~/.zshrc or ~/.bashrc:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
elif [ -d "$HOME/bin" ]; then
    echo "Installing to ~/bin/vr"
    cp target/release/vim-review "$HOME/bin/vr"
    chmod +x "$HOME/bin/vr"
    echo "✓ Installed to ~/bin/vr"
    echo ""
    echo "Make sure ~/bin is in your PATH by adding this to your ~/.zshrc or ~/.bashrc:"
    echo "  export PATH=\"\$HOME/bin:\$PATH\""
else
    echo "Creating ~/bin directory and installing there..."
    mkdir -p "$HOME/bin"
    cp target/release/vim-review "$HOME/bin/vr"
    chmod +x "$HOME/bin/vr"
    echo "✓ Installed to ~/bin/vr"
    echo ""
    echo "Add this to your ~/.zshrc or ~/.bashrc:"
    echo "  export PATH=\"\$HOME/bin:\$PATH\""
    echo ""
    echo "Then run: source ~/.zshrc (or source ~/.bashrc)"
fi

echo ""
echo "Installation complete! ✓"
echo ""
echo "Test it by running: vr"
echo ""
