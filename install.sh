#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

REPO="aldevv/katac"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

echo -e "${BLUE}Installing katac...${NC}"

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "darwin";;
        MINGW*|MSYS*|CYGWIN*) echo "windows";;
        *)          echo "unknown";;
    esac
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)   echo "x86_64";;
        aarch64|arm64)  echo "aarch64";;
        armv7l)         echo "armv7";;
        i686|i386)      echo "i686";;
        *)              echo "unknown";;
    esac
}

# Get the target triple based on OS and architecture
get_target() {
    local os=$1
    local arch=$2

    case "$os-$arch" in
        linux-x86_64)   echo "x86_64-unknown-linux-gnu";;
        linux-aarch64)  echo "aarch64-unknown-linux-gnu";;
        linux-armv7)    echo "armv7-unknown-linux-gnueabihf";;
        darwin-x86_64)  echo "x86_64-apple-darwin";;
        darwin-aarch64) echo "aarch64-apple-darwin";;
        windows-x86_64) echo "x86_64-pc-windows-msvc";;
        windows-i686)   echo "i686-pc-windows-msvc";;
        windows-aarch64) echo "aarch64-pc-windows-msvc";;
        *)              echo "unknown";;
    esac
}

# Get the latest release version from GitHub
get_latest_version() {
    curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
        | grep '"tag_name":' \
        | sed -E 's/.*"([^"]+)".*/\1/' \
        || echo ""
}

# Download and install binary from GitHub releases
install_from_binary() {
    local os=$(detect_os)
    local arch=$(detect_arch)
    local target=$(get_target "$os" "$arch")

    if [ "$os" = "unknown" ] || [ "$arch" = "unknown" ] || [ "$target" = "unknown" ]; then
        echo -e "${YELLOW}⚠ Could not detect system type (OS: $os, Arch: $arch)${NC}"
        return 1
    fi

    echo -e "${BLUE}Detected: $os ($arch)${NC}"
    echo -e "${BLUE}Target: $target${NC}"

    # Get latest version
    echo -e "${BLUE}Fetching latest release...${NC}"
    local version=$(get_latest_version)

    if [ -z "$version" ]; then
        echo -e "${YELLOW}⚠ Could not fetch latest release version${NC}"
        return 1
    fi

    echo -e "${GREEN}Latest version: $version${NC}"

    # Construct download URL
    local ext="tar.gz"
    if [ "$os" = "windows" ]; then
        ext="zip"
    fi

    local filename="katac-${target}.${ext}"
    local url="https://github.com/$REPO/releases/download/$version/$filename"

    echo -e "${BLUE}Downloading from: $url${NC}"

    # Create temp directory
    local tmp_dir=$(mktemp -d)
    trap "rm -rf $tmp_dir" EXIT

    # Download
    if ! curl -fsSL "$url" -o "$tmp_dir/$filename"; then
        echo -e "${YELLOW}⚠ Failed to download binary${NC}"
        return 1
    fi

    # Extract
    echo -e "${BLUE}Extracting...${NC}"
    cd "$tmp_dir"
    if [ "$ext" = "zip" ]; then
        if command -v unzip &> /dev/null; then
            unzip -q "$filename"
        else
            echo -e "${RED}✗ unzip not found${NC}"
            return 1
        fi
    else
        tar xzf "$filename"
    fi

    # Find the binary
    local binary="katac"
    if [ "$os" = "windows" ]; then
        binary="katac.exe"
    fi

    if [ ! -f "$binary" ]; then
        echo -e "${RED}✗ Binary not found in archive${NC}"
        return 1
    fi

    # Make executable
    chmod +x "$binary"

    # Install
    echo -e "${BLUE}Installing to $INSTALL_DIR...${NC}"
    mkdir -p "$INSTALL_DIR"

    if mv "$binary" "$INSTALL_DIR/"; then
        echo -e "${GREEN}✓ katac installed successfully to $INSTALL_DIR/$binary${NC}"
        return 0
    else
        echo -e "${RED}✗ Failed to install binary${NC}"
        return 1
    fi
}

# Fallback: Install from source using cargo
install_from_source() {
    echo -e "${BLUE}Installing from source using cargo...${NC}"

    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}✗ Cargo not found.${NC}"
        echo -e "${YELLOW}Please install Rust from: https://rustup.rs${NC}"
        exit 1
    fi

    cargo install --git "https://github.com/$REPO"
    echo -e "${GREEN}✓ katac installed successfully!${NC}"
}

# Check if we should use binary installation
USE_BINARY=1
if [ "$1" = "--source" ]; then
    USE_BINARY=0
    echo -e "${YELLOW}Building from source (--source flag)${NC}"
fi

# Try binary installation first, fall back to source
if [ $USE_BINARY -eq 1 ]; then
    if install_from_binary; then
        echo ""
        echo -e "${GREEN}Installation complete!${NC}"
        echo -e "${BLUE}Make sure $INSTALL_DIR is in your PATH${NC}"
        echo ""
        echo -e "Add this to your shell config (~/.bashrc, ~/.zshrc, etc.):"
        echo -e "${YELLOW}    export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
        echo ""
        echo -e "Run '${GREEN}katac --help${NC}' to get started"
        exit 0
    else
        echo ""
        echo -e "${YELLOW}Binary installation failed, falling back to building from source...${NC}"
        echo ""
        install_from_source
    fi
else
    install_from_source
fi

echo ""
echo -e "${GREEN}Installation complete!${NC}"
echo -e "Run '${GREEN}katac --help${NC}' to get started"
