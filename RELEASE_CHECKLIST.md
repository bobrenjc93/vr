# Release Checklist

Use this checklist when creating a new release.

## Pre-Release

- [ ] All tests pass: `cargo test`
- [ ] Code is formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`:
  - [ ] Move unreleased changes to new version section
  - [ ] Add release date
  - [ ] Update comparison links
- [ ] Update README.md if needed
- [ ] Commit version bump: `git commit -am "Bump version to vX.Y.Z"`
- [ ] Push to main: `git push origin main`

## Release

- [ ] Create tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
- [ ] Push tag: `git push origin vX.Y.Z`
- [ ] Wait for GitHub Actions to complete
- [ ] Verify all binaries are built (5 platforms)
- [ ] Verify SHA256SUMS file is present

## Post-Release

- [ ] Test download and installation on at least one platform
- [ ] Verify release notes on GitHub
- [ ] Update social media / announcements if applicable
- [ ] Close related GitHub issues/milestones
- [ ] Update project board if applicable

## Rollback (if needed)

If critical issues are found:

- [ ] Delete tag: `git tag -d vX.Y.Z && git push origin :refs/tags/vX.Y.Z`
- [ ] Delete GitHub Release from web interface
- [ ] Fix issues
- [ ] Create patch release vX.Y.Z+1
