#!/bin/bash

set -euxo pipefail
if [ "$(id -u)" -eq 0 ]; then
    echo "[WARN] This script should never be run as root! Exiting now..."
    exit 1
fi

cd "$(dirname "$0")"

bin_dir="$HOME/.local/share/decktricks/bin"
logs_dir="$HOME/.local/share/decktricks/logs"
tmp_init_file="/tmp/decktricks_only_init"
is_updating_file="/tmp/decktricks_is_updating"

# When first running via Steam through the installer, we don't actually want to launch Decktricks,
# we just want it to be launched through Steam so that it shows up first in the recent apps list.
if [[ -f "$tmp_init_file" ]] && rm -f "$tmp_init_file"; then
    echo "Successfully ran Decktricks from Steam!"
    exit 0
fi

# NOTE: these files are created in src/actions/specific.rs
trap 'rm -f /tmp/decktricks-install-*' EXIT


startup_message=""

while :; do
    should_update="true"
    if [[ -f "$is_updating_file" ]]; then
        startup_message="$startup_message
[UPDATE] Found live update!"
        update_pid=$(cat "$is_updating_file")
        # If an update file is older than 4 hours, ignore it
        if find "$is_updating_file" -mmin +240 | grep -q .; then
            startup_message="$startup_message
[UPDATE] But it was outdated, so we will force update anyway."
        elif ! ps -p "$update_pid" > /dev/null; then
            startup_message="$startup_message
[UPDATE] But no corresponding process was found, so we will force update anyway."
        else
            startup_message="$startup_message
[UPDATE] Will not update."
            should_update="false"
        fi
    fi

    if [[ "$should_update" == "true" ]]; then
        # Backup our previous update logs, and spawn off a background process to update
        # If we've just updated, we expect this to just start, clear out notification files, and exit
        # TODO: make sure that this nohup won't be detected/killed by SteamOS when exiting in Game Mode
        ( [[ -f "$logs_dir/decktricks-update.log" ]] && mv "$logs_dir/decktricks-update.log"{,.bak} ) || true
        echo "$startup_message" >> "$logs_dir/decktricks-update.log"
        nohup nice -n 10 -- /bin/bash "$bin_dir/decktricks-update.sh" 2>&1 | grep -v 'gameoverlayrenderer.so' &> "$logs_dir/decktricks-update.log" &
    else
        # Write messages about ongoing updates to prior update's logs, to avoid issues with writing to open files
        echo "$startup_message" >> "$logs_dir/decktricks-update.log.bak"
    fi

    ( [[ -f "$logs_dir/decktricks-gui.log" ]] && mv "$logs_dir/decktricks-gui.log"{,.bak} ) || true
    exit_code=0
    "$bin_dir/decktricks-gui" "$@" &> "$logs_dir/decktricks-gui.log" || exit_code=$?
    [ $exit_code -eq 100 ] || break
done
