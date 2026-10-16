#!/usr/bin/env fish

set source_dir (status dirname)

# Check for the prerequisite tools
for tool in cargo rustc
    if ! command -q $tool
        echo "You must install the '$tool' to run this script." >&2
        exit 1
    end
end

set target_dir (mktemp -d)

function on_exit -e fish_exit
    rm -r $target_dir
end

if ! cargo b -r --target-dir $target_dir
    echo "Failed to build the Cargo workspace." >&2
    exit 1
end

sudo cp -f -t /usr/local/libexec $target_dir/release/xdg-desktop-portal-screencast $target_dir/release/sourceselector-ui
sudo cp -rf -t / $source_dir/files/.

exit 0
