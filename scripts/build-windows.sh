#!/bin/bash
set -e

echo "Building Windows installer..."
echo "Note: This should be run on Windows or with cross-compilation setup"

# Ensure icons are generated
if [ ! -f "src-tauri/icons/icon.ico" ]; then
    echo "Generating icons first..."
    ./scripts/generate-icons.sh
fi

# Build frontend
npm run build

# Build Tauri Windows bundle
cd src-tauri
cargo tauri build --target nsis
cd ..

echo ""
echo "Build complete! Installer location:"
echo "src-tauri/target/release/bundle/nsis/Gestor do Clube_1.0.0_x64-setup.exe"
