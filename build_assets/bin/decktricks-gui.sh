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

# NOTE: these files are created in src/actions/specific.rs
trap 'rm -f /tmp/decktricks-install-*' EXIT

# Backup our previous update logs, and spawn off a background process to update
# TODO: make sure that this nohup won't be detected/killed by SteamOS when exiting in Game Mode
( [[ -f "$logs_dir/decktricks-update.log" ]] && mv "$logs_dir/decktricks-update.log"{,.bak} ) || true
nohup nice -n 10 -- /bin/bash "$bin_dir/decktricks-update.sh" 2>&1 | grep -v 'gameoverlayrenderer.so' &> "$logs_dir/decktricks-update.log" &

( [[ -f "$logs_dir/decktricks-gui.log" ]] && mv "$logs_dir/decktricks-gui.log"{,.bak} ) || true
"$bin_dir/decktricks-gui" "$@" &> "$logs_dir/decktricks-gui.log"
