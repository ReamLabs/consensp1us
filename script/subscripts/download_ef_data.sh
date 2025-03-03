#!/bin/bash

TARGET="mainnet.tar.gz"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARENT_DIR="$(dirname "$SCRIPT_DIR")"
EXTRACT_DIR="$PARENT_DIR/mainnet"
LATEST_RELEASE_URL="https://api.github.com/repos/ethereum/consensus-spec-tests/releases/latest"

download_and_extract() {
    if [ -d "$EXTRACT_DIR" ]; then
        echo "$EXTRACT_DIR already exists. Skipping extraction."
        return 0
    fi

    echo "Fetching the latest release URL for $TARGET..."
    DOWNLOAD_URL=$(curl -s "$LATEST_RELEASE_URL" | jq -r ".assets[] | select(.name == \"$TARGET\") | .browser_download_url")

    if [ -z "$DOWNLOAD_URL" ] || [ "$DOWNLOAD_URL" == "null" ]; then
        echo "Failed to fetch download URL for $TARGET."
        exit 1
    fi

    echo "Downloading $TARGET..."
    echo "URL: $DOWNLOAD_URL"
    curl -L "$DOWNLOAD_URL" -o "$SCRIPT_DIR/$TARGET"

    if [ ! -f "$SCRIPT_DIR/$TARGET" ]; then
        echo "Download failed. Exiting."
        exit 1
    fi

    echo "Extracting $TARGET into $EXTRACT_DIR..."
    mkdir -p "$EXTRACT_DIR"
    tar -xzf "$SCRIPT_DIR/$TARGET" -C "$EXTRACT_DIR"
    rm -f "$SCRIPT_DIR/$TARGET"
    echo "Extraction complete."
}

download_and_extract