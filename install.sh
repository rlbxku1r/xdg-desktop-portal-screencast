#!/bin/bash

SOURCE_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

if [[ $EUID -ne 0 ]]; then
	echo "$0: This script must be run as root." >&2
	exit 1
fi

# Check for the prerequisite tools
for tool in cargo rustc; do
	if ! command -v "$tool" &>/dev/null; then
		echo "$0: You must install '$tool' to run this script." >&2
		exit 1
	fi
done

TARGET_DIR=$(mktemp -d)

# Build these crates separately to avoid unnecessary library linking
cargo b -r -p 'xdg-desktop-portal-screencast' --manifest-path "$SOURCE_DIR/Cargo.toml" --target-dir "$TARGET_DIR"
cargo b -r -p 'sourceselector-ui' --manifest-path "$SOURCE_DIR/Cargo.toml" --target-dir "$TARGET_DIR"

cp -f -t '/usr/libexec' "$TARGET_DIR/release/xdg-desktop-portal-screencast"
cp -f -t '/usr/libexec' "$TARGET_DIR/release/sourceselector-ui"

rm -r "$TARGET_DIR"

exit 0
