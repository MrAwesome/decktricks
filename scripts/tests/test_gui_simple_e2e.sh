#!/bin/bash

set -euxo pipefail
source "$(dirname "$0")"/lib.sh

export DECKTRICKS_GUI_EXIT_IMMEDIATELY=true
export DECKTRICKS_GUI_TEST_COMMAND_ONLY="run-system-command|DELIM|--|DELIM|echo|DELIM|THISISMYTESTSTRINGYES"
OUTPUT=$(timeout 10 "${DECKTRICKS_TEST_COMMAND[@]}" 2>&1)

popd

ERR=false
if [[ "$OUTPUT" == *'ERROR'* ]]; then
    echo '[ERROR] Detected an error when running the Godot GUI!'
    ERR=true
fi

if [[ "$OUTPUT" != *'THISISMYTESTSTRINGYES'* ]]; then
    echo '[ERROR] DecktricksDispatcher did not successfully run our command!'
    ERR=true
fi

if [[ "$OUTPUT" != *'Decktricks GUI initialization complete!' ]]; then
    echo '[ERROR] Initilization message not found or was not last output!'
    ERR=true
fi

if [[ "$ERR" == "false" ]]; then
    exit 0
fi
exit 1
