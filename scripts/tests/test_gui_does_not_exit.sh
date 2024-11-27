#!/bin/bash

set -euxo pipefail
source "$(dirname "$0")"/lib.sh

# If the GUI was still running after 5 seconds, we can be sure that we
# at least finished _ready(), because sripts/tests/test_gui_startup_speed.sh 
# ensures we start up faster than this.
set +e
timeout 5 "${DECKTRICKS_TEST_COMMAND[@]}" 2>&1
RETCODE="$?"
set -e

if [[ "$RETCODE" == "124" ]]; then
    exit 0
fi
exit 1
