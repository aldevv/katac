#!/bin/bash
set -e

echo "Installing katac..."

# Check if cargo is available
if command -v cargo &> /dev/null; then
    echo "Installing katac using cargo..."
    cargo install --git https://github.com/aldevv/katac
    echo "✓ katac installed successfully!"
    echo "Run 'katac --help' to get started"
else
    echo "Error: Cargo not found."
    echo "Please install Rust from: https://rustup.rs"
    exit 1
fi
