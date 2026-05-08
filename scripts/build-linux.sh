#!/bin/bash
set -e

echo "Building Linux AppImage..."

# Ensure icons are generated
if [ ! -f "src-tauri/icons/icon.png" ]; then
    echo "Generating icons first..."
    ./scripts/generate-icons.sh
fi

# Install dependencies if needed
echo "Checking for required dependencies..."
if ! command -v linuxdeploy &> /dev/null; then
    echo "Warning: linuxdeploy not found"
    echo "AppImage bundling requires linuxdeploy to be installed"
    echo "Install with: sudo apt-get install linuxdeploy (Debian/Ubuntu)"
    echo "Or: sudo dnf install linuxdeploy (Fedora/RHEL)"
fi

# Build frontend
echo "Building frontend..."
npm run build

# Build Tauri Linux bundle
echo "Building Tauri bundle..."
npx tauri build --bundles appimage

echo ""
echo "Build complete! AppImage location:"
find src-tauri/target/release/bundle -name "*.AppImage" -type f
