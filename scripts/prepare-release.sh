#!/bin/bash
set -e

VERSION="1.0.0"
echo "Preparing release v$VERSION..."

# Check git status
if [ -n "$(git status --porcelain)" ]; then
    echo "Error: Working directory not clean. Commit or stash changes first."
    exit 1
fi

# Generate icons
echo "Generating icons..."
./scripts/generate-icons.sh

# Build frontend
echo "Building frontend..."
npm run build

# Run tests (if any)
echo "Running tests..."
npm test || true

# Build for current platform
echo "Building application..."
cd src-tauri
cargo tauri build
cd ..

# Create release directory
RELEASE_DIR="releases/v$VERSION"
mkdir -p "$RELEASE_DIR"

# Copy build artifacts
echo "Copying build artifacts..."

# Linux artifacts
if [ -d "src-tauri/target/release/bundle/appimage" ]; then
    cp src-tauri/target/release/bundle/appimage/*.AppImage "$RELEASE_DIR/" 2>/dev/null || true
fi

if [ -d "src-tauri/target/release/bundle/deb" ]; then
    cp src-tauri/target/release/bundle/deb/*.deb "$RELEASE_DIR/" 2>/dev/null || true
fi

# Windows artifacts
if [ -d "src-tauri/target/release/bundle/nsis" ]; then
    cp src-tauri/target/release/bundle/nsis/*-setup.exe "$RELEASE_DIR/" 2>/dev/null || true
fi

if [ -d "src-tauri/target/release/bundle/msi" ]; then
    cp src-tauri/target/release/bundle/msi/*.msi "$RELEASE_DIR/" 2>/dev/null || true
fi

# Copy documentation
echo "Copying documentation..."
cp docs/MANUAL_PT.md "$RELEASE_DIR/"
cp docs/INSTALLATION.md "$RELEASE_DIR/"
cp CHANGELOG.md "$RELEASE_DIR/"
cp LICENSE "$RELEASE_DIR/"

# Create checksums
echo "Creating checksums..."
cd "$RELEASE_DIR"
sha256sum * > SHA256SUMS.txt
cd ../..

# List artifacts
echo ""
echo "Release artifacts prepared in $RELEASE_DIR:"
ls -lh "$RELEASE_DIR"

echo ""
echo "Next steps:"
echo "1. Test all artifacts"
echo "2. Create git tag: git tag -a v$VERSION -m 'Release v$VERSION'"
echo "3. Push tag: git push origin v$VERSION"
echo "4. Create GitHub release with artifacts from $RELEASE_DIR"
