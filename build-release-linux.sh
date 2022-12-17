#!/usr/bin/env bash
# Build release distribution for Linux
# This script is meant to be run by a cofob personally, so it's not very generic
#
# Requirements:
# - Node.js
# - w3 cli (node.js package)
# - Yarn
# - Rust
# - Tauri CLI and Tauri system dependencies
# - Tauri private key (for signing)
# - GPG (and cofob's private key :O)

# Setup signing
export SIGN=1
export SIGN_KEY=5F3D9D3DECE08651DE14D29FACAD4265E193794D
export TAURI_PRIVATE_KEY=~/.tauri/firesquare.key

# Build the app
cargo tauri build

# Reset dist folder
rm -rf dist
mkdir dist

# Copy the app bundles
# AppImage
cp target/release/bundle/appimage/*.AppImage dist/firesquare-linux-x64.AppImage # Main bin
cp target/release/bundle/appimage/*.tar.gz dist/firesquare-linux-x64.AppImage.tar.gz # Updater
cp target/release/bundle/appimage/*.sig dist/firesquare-linux-x64.AppImage.tar.gz.sig # Updater signature

# Debian
cp target/release/bundle/deb/*.deb dist/firesquare-linux-x64.deb # Main bin

# Raw binary
cp target/release/firesquare dist/firesquare-linux-x64 # Main bin

# Get hashes of all files in dist folder
cd dist
sha256sum * >> sha256sums.txt

# Sign the hashes
cat sha256sums.txt | gpg2 --sign --armor --clear-sign --default-key $SIGN_KEY > sha256sums.txt.sig
rm sha256sums.txt
mv sha256sums.txt.sig sha256sums.txt

# Export the public key
gpg2 --armor --export $SIGN_KEY > key.txt
cd ..

# Upload assets to IPFS
w3 put --no-wrap --name "firesquare-launcher dist: $(date)" dist
