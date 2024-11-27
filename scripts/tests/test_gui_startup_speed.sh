#!/bin/bash

set -euxo pipefail
source "$(dirname "$0")"/lib.sh

# If we do not successfully finish startup in under 4 seconds
# on a decently fast system, something is very wrong.
#
# This timeout number should always be lower than the number
# in scripts/tests/test_gui_does_not_exit.sh
export DECKTRICKS_GUI_EXIT_IMMEDIATELY=true
timeout 4 "${DECKTRICKS_TEST_COMMAND[@]}" 2>&1
RETCODE="$?"

if [[ "$RETCODE" == "124" ]]; then
    exit 1
fi
exit 0
