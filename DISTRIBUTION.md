# Distribution Guide for Omarchy Kanban

This document outlines different methods to distribute your kanban application to users.

## Quick Summary

Your application is **1.2 MB** and only depends on **glibc** (standard on all Linux systems), making it very easy to distribute!

## Distribution Methods (Ranked by Ease)

### 1. Pre-built Binary (Easiest for Users)

**Pros:**
- Users don't need Rust/Cargo installed
- Instant installation
- Minimal dependencies

**Steps:**

1. Run the build script:
   ```bash
   ./build-dist.sh
   ```

2. Upload `dist/omarchy-kanban-0.1.0-x86_64.tar.gz` to GitHub Releases

3. Users install with:
   ```bash
   wget https://github.com/xRipzch/Omarchy-Kanban/releases/download/v0.1.0/omarchy-kanban-0.1.0-x86_64.tar.gz
   tar -xzf omarchy-kanban-0.1.0-x86_64.tar.gz
   sudo install -Dm755 omarchy-kanban /usr/local/bin/omarchy-kanban
   ```

### 2. AUR Package (Best for Arch Users)

**Pros:**
- Official Arch distribution method
- Automatic updates via AUR helpers
- Integrates with pacman
- Community-friendly

**Steps:**

See `AUR_UPLOAD_GUIDE.md` for detailed instructions.

**Quick start:**
```bash
# Clone AUR repository
git clone ssh://aur@aur.archlinux.org/omarchy-kanban-git.git
cd omarchy-kanban-git

# Copy and edit PKGBUILD
cp ../PKGBUILD-git ./PKGBUILD
# Edit PKGBUILD: update maintainer info

# Generate .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# Test build
makepkg -si

# Upload to AUR
git add PKGBUILD .SRCINFO
git commit -m "Initial upload"
git push origin master
```

### 3. Cargo Install (For Rust Users)

**Pros:**
- Simple for Rust developers
- Can publish to crates.io

**Steps:**

1. Add metadata to `Cargo.toml`:
   ```toml
   [package]
   name = "omarchy-kanban"
   version = "0.1.0"
   edition = "2021"
   authors = ["Your Name <your.email@example.com>"]
   description = "A simple terminal-based kanban board"
   license = "MIT"
   repository = "https://github.com/xRipzch/Omarchy-Kanban"
   keywords = ["kanban", "tui", "terminal", "productivity"]
   categories = ["command-line-utilities"]
   ```

2. Publish to crates.io:
   ```bash
   cargo login
   cargo publish
   ```

3. Users install with:
   ```bash
   cargo install omarchy-kanban
   ```

### 4. Direct from Source

**Pros:**
- Always up-to-date
- Full control

**Cons:**
- Requires Rust toolchain

Users clone and build:
```bash
git clone https://github.com/xRipzch/Omarchy-Kanban.git
cd Omarchy-Kanban
cargo build --release
sudo install -Dm755 target/release/omarchy-kanban /usr/local/bin/omarchy-kanban
```

## Recommended Distribution Strategy

For the **Omarchy Arch Community**, I recommend:

### Phase 1: Quick Start (Now)
1. **Create a GitHub Release** with pre-built binary
   - Run `./build-dist.sh`
   - Create release v0.1.0 on GitHub
   - Upload the tarball
   - Add installation instructions from README

### Phase 2: Arch Integration (This Week)
2. **Publish to AUR** (`omarchy-kanban-git`)
   - Follow `AUR_UPLOAD_GUIDE.md`
   - Announce in Omarchy community

### Phase 3: Official Release (When Stable)
3. **Tag stable release** (v1.0.0)
   - Publish `omarchy-kanban` (non-git) to AUR
   - Optionally publish to crates.io

## File Checklist

Before distributing:

- [ ] Add LICENSE file (MIT, GPL-3.0, etc.)
- [ ] Update Cargo.toml metadata
- [ ] Test on clean Arch system
- [ ] Update README with all installation methods
- [ ] Create GitHub release with binary
- [ ] Test installation from tarball
- [ ] Upload to AUR
- [ ] Announce to community

## License Recommendation

For an open-source community tool, consider:

- **MIT** - Most permissive, widely compatible
- **GPL-3.0** - Ensures derivatives stay open-source
- **Apache-2.0** - Patent protection, enterprise-friendly

Create LICENSE file:
```bash
# For MIT
cat > LICENSE << 'EOF'
MIT License

Copyright (c) 2024 [Your Name]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
EOF
```

## Commands Summary

```bash
# Build distribution package
./build-dist.sh

# Create and push git tag
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0

# Create GitHub release (requires gh CLI)
gh release create v0.1.0 dist/omarchy-kanban-0.1.0-x86_64.tar.gz \
  --title "v0.1.0" \
  --notes "Initial release of Omarchy Kanban"

# Upload to AUR (after initial setup)
cd ../omarchy-kanban-git
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Update to v0.1.0"
git push
```

## Support & Maintenance

After distribution:

1. **Monitor AUR comments** for issues
2. **Respond to GitHub issues**
3. **Update package** when you push new features
4. **Keep AUR PKGBUILD in sync** with releases
5. **Announce updates** to community

## Testing Installation

Before announcing, test all installation methods on a **clean Arch system** or container:

```bash
# Using Docker/Podman
podman run -it archlinux:latest
pacman -Sy base-devel git

# Test each installation method
```
