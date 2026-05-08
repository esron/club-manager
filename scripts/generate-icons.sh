#!/bin/bash
set -e

ICON_SRC="src-tauri/icons/app-icon.png"
ICON_DIR="src-tauri/icons"

if [ ! -f "$ICON_SRC" ]; then
    echo "Error: Source icon not found at $ICON_SRC"
    exit 1
fi

echo "Generating icons from $ICON_SRC..."

# PNG icons for Linux (with alpha channel for Tauri - use PNG32 format)
magick "$ICON_SRC" -resize 32x32 PNG32:"$ICON_DIR/32x32.png"
magick "$ICON_SRC" -resize 128x128 PNG32:"$ICON_DIR/128x128.png"
magick "$ICON_SRC" -resize 256x256 PNG32:"$ICON_DIR/128x128@2x.png"
magick "$ICON_SRC" -resize 256x256 PNG32:"$ICON_DIR/icon.png"
magick "$ICON_SRC" -resize 512x512 PNG32:"$ICON_DIR/512x512.png"

# ICO for Windows (multiple sizes in one file)
magick "$ICON_SRC" \
    \( -clone 0 -resize 16x16 \) \
    \( -clone 0 -resize 32x32 \) \
    \( -clone 0 -resize 48x48 \) \
    \( -clone 0 -resize 64x64 \) \
    \( -clone 0 -resize 128x128 \) \
    \( -clone 0 -resize 256x256 \) \
    -delete 0 "$ICON_DIR/icon.ico"

# ICNS for macOS (if png2icns is available)
if command -v png2icns &> /dev/null; then
    png2icns "$ICON_DIR/icon.icns" "$ICON_SRC"
else
    echo "Warning: png2icns not found, skipping .icns generation"
    echo "Install with: npm install -g png2icons"
fi

echo "Icons generated successfully!"
ls -lh "$ICON_DIR"
