#!/bin/bash
# Distribution build script for Omarchy Kanban

set -e

VERSION="0.1.0"
ARCH="x86_64"
BINARY_NAME="omarchy-kanban"
DIST_DIR="dist"

echo "=== Building Omarchy Kanban v${VERSION} ==="

# Clean previous builds
echo "Cleaning previous builds..."
cargo clean
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Build release binary
echo "Building release binary..."
cargo build --release

# Strip debug symbols
echo "Stripping debug symbols..."
strip target/release/${BINARY_NAME}

# Get binary size
BINARY_SIZE=$(du -h target/release/${BINARY_NAME} | cut -f1)
echo "Binary size: ${BINARY_SIZE}"

# Create tarball
echo "Creating tarball..."
TARBALL="${BINARY_NAME}-${VERSION}-${ARCH}.tar.gz"
tar -czf "${DIST_DIR}/${TARBALL}" \
    -C target/release ${BINARY_NAME} \
    -C ../../ README.md

# Create checksum
echo "Generating checksum..."
cd "$DIST_DIR"
sha256sum "${TARBALL}" > "${TARBALL}.sha256"
cd ..

echo ""
echo "=== Build Complete ==="
echo "Package: ${DIST_DIR}/${TARBALL}"
echo "SHA256: ${DIST_DIR}/${TARBALL}.sha256"
echo ""
echo "Install with:"
echo "  tar -xzf ${TARBALL}"
echo "  sudo install -Dm755 ${BINARY_NAME} /usr/local/bin/${BINARY_NAME}"
echo ""
echo "Upload to GitHub Release:"
echo "  gh release create v${VERSION} ${DIST_DIR}/${TARBALL} --title \"v${VERSION}\" --notes \"Release v${VERSION}\""
