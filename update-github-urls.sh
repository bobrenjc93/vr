#!/bin/bash

# Quick script to replace YOUR_USERNAME with actual GitHub username
# Usage: ./update-github-urls.sh your-github-username

if [ -z "$1" ]; then
    echo "Usage: $0 <github-username>"
    echo "Example: $0 octocat"
    exit 1
fi

USERNAME="$1"

echo "Updating GitHub URLs with username: $USERNAME"
echo ""

# Files to update
FILES=(
    "README.md"
    "CHANGELOG.md"
    "RELEASING.md"
    "Cargo.toml"
)

for file in "${FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "Updating $file..."
        sed -i.bak "s/YOUR_USERNAME/$USERNAME/g" "$file"
        rm "${file}.bak"
    else
        echo "Warning: $file not found"
    fi
done

echo ""
echo "✓ URLs updated successfully!"
echo ""
echo "Next steps:"
echo "  1. Review the changes: git diff"
echo "  2. Commit: git add -A && git commit -m 'Update GitHub URLs'"
echo "  3. Push: git push origin main"
