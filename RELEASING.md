# Releasing vim-review

This document describes the release process for vim-review.

## Prerequisites

- Maintainer access to the GitHub repository
- Git installed and configured

## Release Process

### 1. Prepare the Release

1. Update version in `Cargo.toml`:
   ```toml
   version = "0.2.0"
   ```

2. Update `CHANGELOG.md`:
   - Move items from `[Unreleased]` to a new version section
   - Add the release date
   - Update the comparison links at the bottom

3. Commit the changes:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "Prepare release v0.2.0"
   git push origin main
   ```

### 2. Create and Push the Tag

```bash
# Create an annotated tag
git tag -a v0.2.0 -m "Release v0.2.0"

# Push the tag to GitHub
git push origin v0.2.0
```

### 3. Automated Build

Once the tag is pushed, GitHub Actions will automatically:

1. Build binaries for all platforms:
   - Linux x86_64
   - Linux ARM64 (aarch64)
   - macOS x86_64 (Intel)
   - macOS ARM64 (Apple Silicon)
   - Windows x86_64

2. Create SHA256 checksums for all binaries

3. Create a GitHub Release with:
   - All binaries attached
   - Checksum file (SHA256SUMS)
   - Auto-generated release notes

### 4. Verify the Release

1. Check the [Actions tab](https://github.com/bobrenjc93/vr/actions) to ensure the workflow completed successfully

2. Visit the [Releases page](https://github.com/bobrenjc93/vr/releases) to verify:
   - All binaries are present
   - SHA256SUMS file is present
   - Release notes look correct

3. Test download and installation on at least one platform:
   ```bash
   curl -LO https://github.com/bobrenjc93/vr/releases/download/v0.2.0/vr-linux-x86_64
   chmod +x vr-linux-x86_64
   ./vr-linux-x86_64
   ```

### 5. Announce

- Update README if needed
- Announce on social media, forums, etc.
- Update any package managers (Homebrew, etc.) if applicable

## Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** version (X.0.0): Incompatible API changes
- **MINOR** version (0.X.0): New functionality in a backwards-compatible manner
- **PATCH** version (0.0.X): Backwards-compatible bug fixes

## Troubleshooting

### Build Fails

- Check the Actions logs for specific errors
- Common issues:
  - Dependency version conflicts
  - Platform-specific compilation errors
  - Missing cross-compilation tools

### Release Not Created

- Ensure the tag follows the `v*` pattern (e.g., `v0.2.0`)
- Check that the GitHub token has sufficient permissions
- Verify the workflow file syntax is correct

## Emergency Rollback

If a release has critical issues:

1. Delete the tag:
   ```bash
   git tag -d v0.2.0
   git push origin :refs/tags/v0.2.0
   ```

2. Delete the GitHub Release from the web interface

3. Fix the issues and create a new patch release (e.g., v0.2.1)
