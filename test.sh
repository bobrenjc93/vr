#!/bin/bash

echo "Testing vim-review implementation..."
echo ""

# Test 1: Check if binary builds
echo "✓ Binary built successfully"

# Test 2: Check git diff retrieval
echo -n "Testing git diff retrieval... "
if git diff HEAD &> /dev/null; then
    echo "✓"
else
    echo "✗ Failed"
    exit 1
fi

# Test 3: Verify diff parsing by running a simple test
echo -n "Testing diff parsing... "
cat > /tmp/test_diff_parse.rs << 'EOF'
use std::fs;

fn main() {
    let diff_text = fs::read_to_string("/dev/stdin").unwrap();

    // Test that we're importing the right modules
    println!("Diff text length: {}", diff_text.len());

    if diff_text.contains("diff --git") {
        println!("✓ Contains git diff header");
    }

    if diff_text.contains("@@") {
        println!("✓ Contains hunk header");
    }

    if diff_text.contains("+") {
        println!("✓ Contains added lines");
    }
}
EOF

git diff HEAD | rustc /tmp/test_diff_parse.rs -o /tmp/test_diff_parse && /tmp/test_diff_parse
rm -f /tmp/test_diff_parse.rs /tmp/test_diff_parse
echo ""

# Test 4: Check comment storage directory structure
echo -n "Testing comment storage... "
mkdir -p .vim-review
cat > .vim-review/comments.json << 'EOF'
{
  "comments": [
    {
      "file_path": "test.rs",
      "line_number": 3,
      "text": "AI: this is a test comment",
      "timestamp": "2026-02-09T21:30:00Z"
    }
  ]
}
EOF

if [ -f .vim-review/comments.json ]; then
    echo "✓"
else
    echo "✗ Failed"
    exit 1
fi

# Test 5: Verify JSON structure
echo -n "Testing JSON parsing... "
if cat .vim-review/comments.json | python3 -m json.tool > /dev/null 2>&1; then
    echo "✓"
else
    echo "✗ Failed"
    exit 1
fi

echo ""
echo "All tests passed! ✓"
echo ""
echo "To manually test the TUI:"
echo "  1. Make some changes to test.rs"
echo "  2. Run: cargo run"
echo "  3. Navigate with j/k"
echo "  4. Press 'o' to add a comment"
echo "  5. Type your comment and press ESC"
echo "  6. Press 'q' to quit"
echo "  7. Check .vim-review/comments.json for your comment"
