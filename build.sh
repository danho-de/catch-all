#!/bin/bash
set -e

echo "Building statically-linked Rust binary for Linux..."

# Format code
cargo fmt

# Build the release binary with musl for static linking
cargo build --release --target x86_64-unknown-linux-musl

# Copy the binary to the root directory
cp target/x86_64-unknown-linux-musl/release/catch-all ./catch-all

# Make the binary executable
chmod +x ./catch-all

echo "Build complete! Statically-linked binary created: ./catch-all"
