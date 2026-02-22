#!/usr/bin/env fish

set SOURCE_DIR (status dirname)

# Check for the prerequisite tools
for tool in cargo rustc
    if ! command -q $tool
        echo "You must install the '$tool' to run this script." >&2
        exit 1
    end
end

set TARGET_DIR (mktemp -d)

function on_exit -e fish_exit
    rm -r $TARGET_DIR
end

# Build these crates separately to avoid unnecessary library linking
for crate in xdg-desktop-portal-screencast sourceselector-ui
    if ! cargo b -r -p $crate --manifest-path $SOURCE_DIR/Cargo.toml --target-dir $TARGET_DIR
        echo "Failed to build the '$crate' crate." >&2
        exit 1
    end
end

sudo cp -f -t /usr/local/libexec $TARGET_DIR/release/xdg-desktop-portal-screencast $TARGET_DIR/release/sourceselector-ui
sudo cp -rf -t / $SOURCE_DIR/files/.

exit 0
