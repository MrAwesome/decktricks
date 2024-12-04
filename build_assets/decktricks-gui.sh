#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"

TMP_INIT_FILE=/tmp/decktricks_only_init

# When first running via Steam through the installer, we don't actually want to launch Decktricks,
# we just want it to be launched through Steam so that it shows up first in the recent apps list.
if [[ -f "$TMP_INIT_FILE" ]] && rm -f "$TMP_INIT_FILE"; then
    echo "Successfully ran Decktricks from Steam!"
    exit 0
fi

# NOTE: any "critical background updates" code can be spawned off here using the cli, if desired

./decktricks-gui "$@"
