#!/bin/bash

set -euxo pipefail
if [ "$(id -u)" -eq 0 ]; then
    echo "[WARN] This script should never be run as root! Exiting now..."
    exit 1
fi

cd "$(dirname "$0")"

bin_dir="$HOME/.local/share/decktricks/bin"
logs_dir="$HOME/.local/share/decktricks/logs"
tmp_init_file=/tmp/decktricks_only_init

# When first running via Steam through the installer, we don't actually want to launch Decktricks,
# we just want it to be launched through Steam so that it shows up first in the recent apps list.
if [[ -f "$tmp_init_file" ]] && rm -f "$tmp_init_file"; then
    echo "Successfully ran Decktricks from Steam!"
    exit 0
fi

( [[ -f "$logs_dir/decktricks-update.log" ]] && mv "$logs_dir/decktricks-update.log"{,.bak} ) || true &
( nice -n 5 -- /bin/bash "$bin_dir/decktricks-update.sh" &> "$logs_dir/decktricks-update.log" ) || true &

( [[ -f "$logs_dir/decktricks-gui.log" ]] && mv "$logs_dir/decktricks-gui.log"{,.bak} ) || true &
"$bin_dir/decktricks-gui" "$@" &> "$logs_dir/decktricks-gui.log"
