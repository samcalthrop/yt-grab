#!/usr/bin/env bash

set -e

REPO="samcalthrop/yt-grab"

LATEST=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep tag_name | cut -d '"' -f4)

OS="$(uname -s)"
ARCH="$(uname -m)"

if [[ "$OS" == "Linux" ]]; then

    if [[ "$ARCH" == "x86_64" ]]; then
        FILE="yt-grab-linux-x64"
    elif [[ "$ARCH" == "aarch64" ]]; then
        FILE="yt-grab-linux-arm64"
    else
        echo "Unsupported arch: $ARCH"
        exit 1
    fi

elif [[ "$OS" == "Darwin" ]]; then

    if [[ "$ARCH" == "x86_64" ]]; then
        FILE="yt-grab-macos-x64"
    else
        FILE="yt-grab-macos-arm64"
    fi

else
    echo "Unsupported OS"
    exit 1
fi

URL="https://github.com/$REPO/releases/download/$LATEST/$FILE"

echo "Downloading {$FILE} from {https://github.com/$REPO/releases/download/$LATEST/$FILE}..."

curl -L "$URL" -o yt-grab

chmod +x yt-grab

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

mv yt-grab "$INSTALL_DIR/yt-grab"

echo
echo "Installed to $INSTALL_DIR/yt-grab"
echo "Add ~/.local/bin to PATH if needed"
echo "Run with: 'yt-grab'"
