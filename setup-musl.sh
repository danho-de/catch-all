#!/bin/bash
set -e

echo "Installing musl target for static linking..."

# Add the musl target for static compilation
rustup target add x86_64-unknown-linux-musl

# Install musl-tools (needed on Ubuntu/Debian)
if command -v apt-get &> /dev/null; then
    echo "Installing musl-tools via apt..."
    sudo apt-get update
    sudo apt-get install -y musl-tools
elif command -v dnf &> /dev/null; then
    echo "Installing musl-tools via dnf..."
    sudo dnf install -y musl-gcc
elif command -v pacman &> /dev/null; then
    echo "Installing musl via pacman..."
    sudo pacman -S --noconfirm musl
else
    echo "Please manually install musl-tools for your distribution"
fi

echo "Setup complete! You can now run ./build-rust.sh"
