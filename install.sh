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

on_exit() {
	rm -r "$TARGET_DIR"
}

trap on_exit EXIT

# Build these crates separately to avoid unnecessary library linking
if ! cargo b -r -p 'xdg-desktop-portal-screencast' --manifest-path "$SOURCE_DIR/Cargo.toml" --target-dir "$TARGET_DIR"; then
	echo "$0: Failed to build the 'xdg-desktop-portal-screencast' crate." >&2
	exit 1
fi
if ! cargo b -r -p 'sourceselector-ui' --manifest-path "$SOURCE_DIR/Cargo.toml" --target-dir "$TARGET_DIR"; then
	echo "$0: Failed to build the 'sourceselector-ui' crate." >&2
	exit 1
fi

cp -f -t '/usr/local/libexec' "$TARGET_DIR/release/xdg-desktop-portal-screencast" "$TARGET_DIR/release/sourceselector-ui"
cp -rf -t '/' "$SOURCE_DIR/files/."

exit 0
