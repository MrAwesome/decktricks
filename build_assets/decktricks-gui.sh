#!/bin/bash

set -euxo pipefail
if [ "$(id -u)" -eq 0 ]; then
    echo "[WARN] This script should never be run as root! Exiting now..."
    exit 1
fi

cd "$(dirname "$0")"

LOGS_DIR="$HOME/.local/share/decktricks/logs/"
TMP_INIT_FILE=/tmp/decktricks_only_init

# When first running via Steam through the installer, we don't actually want to launch Decktricks,
# we just want it to be launched through Steam so that it shows up first in the recent apps list.
if [[ -f "$TMP_INIT_FILE" ]] && rm -f "$TMP_INIT_FILE"; then
    echo "Successfully ran Decktricks from Steam!"
    exit 0
fi

( [[ -f "$LOGS_DIR"/decktricks-update.log ]] && mv "$LOGS_DIR/decktricks-update.log"{,.bak} ) || true &
( nice -n 5 -- /bin/bash decktricks-update.sh &> "$LOGS_DIR"/decktricks-update.log ) || true &

( [[ -f "$LOGS_DIR"/decktricks-gui.log ]] && mv "$LOGS_DIR/decktricks-gui.log"{,.bak} ) || true &
./decktricks-gui "$@" &> "$LOGS_DIR/decktricks-gui.log"
