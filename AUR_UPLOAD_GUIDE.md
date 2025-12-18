# AUR Upload Guide for Omarchy Kanban

This guide will help you publish Omarchy Kanban to the Arch User Repository (AUR).

## Prerequisites

1. **AUR Account**: Create an account at https://aur.archlinux.org/register
2. **SSH Key**: Add your SSH public key to your AUR account settings
3. **Git**: Ensure git is installed
4. **makepkg**: Part of the base-devel package group

## Step 1: Prepare Your Repository

Before uploading to AUR, make sure:

1. **Add a license file** to your repository (MIT, GPL, etc.)
2. **Tag a release** on GitHub:
   ```bash
   git tag -a v0.1.0 -m "Release version 0.1.0"
   git push origin v0.1.0
   ```

3. **Update README.md** with installation instructions

## Step 2: Choose Package Name

You can publish two versions:
- `omarchy-kanban` - Stable release version (uses git tags)
- `omarchy-kanban-git` - Development version (uses latest git commit)

For your first upload, start with `omarchy-kanban-git` since it's easier (no tags required).

## Step 3: Create AUR Package Repository

1. **Clone the AUR repository** (it will be empty initially):
   ```bash
   git clone ssh://aur@aur.archlinux.org/omarchy-kanban-git.git
   cd omarchy-kanban-git
   ```

2. **Copy the PKGBUILD**:
   ```bash
   cp ../Omarchy-Kanban/PKGBUILD-git ./PKGBUILD
   ```

3. **Edit the PKGBUILD**:
   - Update the maintainer line with your name and email
   - Verify the license matches your repository

## Step 4: Generate .SRCINFO

The .SRCINFO file is required by AUR and generated from PKGBUILD:

```bash
makepkg --printsrcinfo > .SRCINFO
```

## Step 5: Test the Package

Before uploading, test that your package builds correctly:

```bash
makepkg -si
```

This will:
- Download the source
- Build the package
- Install it on your system

Test the installed binary:
```bash
omarchy-kanban
```

## Step 6: Upload to AUR

1. **Add files to git**:
   ```bash
   git add PKGBUILD .SRCINFO
   ```

2. **Commit**:
   ```bash
   git commit -m "Initial upload: omarchy-kanban-git 0.1.0"
   ```

3. **Push to AUR**:
   ```bash
   git push origin master
   ```

## Step 7: Verify Upload

Visit your package page:
- https://aur.archlinux.org/packages/omarchy-kanban-git

## Updating the Package

When you make changes to your project:

1. **Update pkgrel** in PKGBUILD (or pkgver if version changed)
2. **Regenerate .SRCINFO**:
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```
3. **Commit and push**:
   ```bash
   git add PKGBUILD .SRCINFO
   git commit -m "Update to version X.Y.Z"
   git push
   ```

## Alternative: Binary Distribution

If you want to distribute without requiring users to build from source:

### Option 1: GitHub Releases
1. Build the release binary:
   ```bash
   cargo build --release
   strip target/release/omarchy-kanban  # Reduce binary size
   ```

2. Create a tarball:
   ```bash
   tar -czf omarchy-kanban-0.1.0-x86_64.tar.gz -C target/release omarchy-kanban
   ```

3. Upload to GitHub Releases with installation instructions:
   ```bash
   tar -xzf omarchy-kanban-0.1.0-x86_64.tar.gz
   sudo install -Dm755 omarchy-kanban /usr/local/bin/omarchy-kanban
   ```

### Option 2: AUR with Pre-built Binary
Create a `-bin` package that downloads pre-built binaries from GitHub releases.

## Best Practices

1. **Keep PKGBUILD simple** - Let cargo handle dependencies
2. **Test on clean system** - Use a clean chroot or container
3. **Respond to comments** - Users may report issues on AUR page
4. **Update regularly** - Keep package in sync with upstream
5. **Follow naming conventions** - Use `-git` suffix for VCS packages

## Resources

- [AUR Submission Guidelines](https://wiki.archlinux.org/title/AUR_submission_guidelines)
- [PKGBUILD Examples](https://wiki.archlinux.org/title/PKGBUILD)
- [Rust Package Guidelines](https://wiki.archlinux.org/title/Rust_package_guidelines)
