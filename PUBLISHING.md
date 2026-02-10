## Before Publishing

Before you publish this to GitHub:

1. **Update GitHub URLs** - Replace `YOUR_USERNAME` in these files:
   - README.md (multiple locations)
   - CHANGELOG.md
   - RELEASING.md
   - Cargo.toml

2. **Create GitHub Repository**
   ```bash
   # On GitHub, create a new repository named "vim-review"
   # Then:
   git remote add origin https://github.com/YOUR_USERNAME/vim-review.git
   git push -u origin main
   ```

3. **First Release**
   ```bash
   git tag -a v0.1.0 -m "Initial release"
   git push origin v0.1.0
   ```

   This will trigger the GitHub Actions workflow to build binaries for all platforms.

4. **Verify the Build**
   - Go to the Actions tab on GitHub
   - Wait for the build to complete
   - Check the Releases page for the new release with binaries

5. **Update Installation URLs**
   - After the first release, the download links in README.md will work
   - Users can download pre-built binaries

## Optional: Publish to crates.io

```bash
cargo login
cargo publish
```

Then users can install with:
```bash
cargo install vim-review
```
