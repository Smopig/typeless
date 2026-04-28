#!/bin/bash
set -e

echo "==> Building Typeless.app (this takes a few minutes)..."
npm run tauri build

APP_SRC="src-tauri/target/release/bundle/macos/Typeless.app"
APP_DEST="/Applications/Typeless.app"

if [ ! -d "$APP_SRC" ]; then
  echo "ERROR: Build output not found at $APP_SRC"
  exit 1
fi

echo "==> Installing to /Applications..."
rm -rf "$APP_DEST"
cp -R "$APP_SRC" "$APP_DEST"

echo "==> Done! Launching Typeless..."
open "$APP_DEST"

echo ""
echo "Typeless is now installed in /Applications."
echo "Enable 'Launch at login' in Settings to auto-start it on boot."
