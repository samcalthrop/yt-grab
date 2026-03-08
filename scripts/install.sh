#!/usr/bin/env bash

set -e

OS=$(uname)

if [ "$OS" = "Linux" ]; then
    FILE=yt-grab-linux
elif [ "$OS" = "Darwin" ]; then
    FILE=yt-grab-macos
else
    echo "Unsupported OS"
    exit 1
fi

curl -L https://github.com/samcalthrop/yt-grab/releases/latest/download/$FILE -o yt-grab

chmod +x yt-grab

sudo mv yt-grab /usr/local/bin/
