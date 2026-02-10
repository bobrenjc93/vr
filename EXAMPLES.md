# Example Usage

This document demonstrates a complete workflow using vim-review.

## Setup

First, let's create a simple project with some changes:

```bash
# Initialize a new project
mkdir my-project && cd my-project
git init

# Create initial file
cat > calculator.rs << 'EOF'
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Result: {}", add(2, 3));
}
EOF

git add calculator.rs
git commit -m "Initial calculator implementation"
```

## Making Changes

Now let's add some new functionality and make changes:

```bash
# Modify the file
cat > calculator.rs << 'EOF'
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    println!("Add: {}", add(2, 3));
    println!("Subtract: {}", subtract(5, 2));
    println!("Multiply: {}", multiply(4, 3));
}
EOF
```

## Reviewing Changes

Run vim-review to see the diff:

```bash
vim-review
```

You'll see a colorized diff like this:

```diff
diff --git a/calculator.rs b/calculator.rs
--- a/calculator.rs
+++ b/calculator.rs
@@ -1,6 +1,14 @@
 fn add(a: i32, b: i32) -> i32 {
     a + b
 }
+
+fn subtract(a: i32, b: i32) -> i32 {
+    a - b
+}
+
+fn multiply(a: i32, b: i32) -> i32 {
+    a * b
+}

 fn main() {
-    println!("Result: {}", add(2, 3));
+    println!("Add: {}", add(2, 3));
+    println!("Subtract: {}", subtract(5, 2));
+    println!("Multiply: {}", multiply(4, 3));
 }
```

## Adding Comments

Navigate to line 5 (the new `subtract` function) and add a comment:

1. Press `j` to move down to the line you want to comment on
2. Press `o` to enter insert mode
3. Type: `AI: Please add input validation for divide by zero`
4. Press `ESC` or `Enter` to save

The line will now be highlighted in yellow with your comment displayed below it.

## Managing Comments

Navigate to a line with a comment and press `d` to delete it.

## Searching

Press `/` to search for text:

1. Press `/`
2. Type `multiply`
3. Press `Enter`
4. Press `n` to jump to next occurrence
5. Press `N` to jump to previous occurrence

## Viewing Comments

Comments are saved to `.vim-review/comments.json`:

```bash
cat .vim-review/comments.json
```

```json
{
  "comments": [
    {
      "file_path": "calculator.rs",
      "line_number": 5,
      "text": "AI: Please add input validation for divide by zero",
      "timestamp": "2026-02-09T21:30:00Z"
    }
  ]
}
```

## AI Integration

The comment format is designed for AI consumption. You can:

1. Send the JSON to an LLM along with your code
2. Ask the LLM to implement the suggestions
3. Review the changes with vim-review again
4. Iterate until satisfied

## Tips

- Use `?` to show the help screen anytime
- Use `Ctrl+d` and `Ctrl+u` for faster navigation
- Use `g` and `G` to jump to top/bottom
- Comments persist across sessions - you can quit and resume later
- Review specific commits: `vim-review HEAD~3` to review the 3rd commit back
- Review branch differences: `vim-review main..feature-branch`
