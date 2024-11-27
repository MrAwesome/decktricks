#!/bin/bash

set -euxo pipefail

source "$(dirname "$0")"/tests/lib.sh

if [[ ! -x /bin/time ]]; then
    echo "This benchmark requires GNU time. It can be installed on Arch Linux with:"
    echo "sudo pacman -S time"
    exit 1
fi

export DECKTRICKS_GUI_EXIT_IMMEDIATELY=true
FULL_OUTPUT=$(/bin/time --format="%e / %P" "${DECKTRICKS_TEST_COMMAND[@]}" 2>&1)

TIME_OUTPUT=$(echo "$FULL_OUTPUT" | tail -n 1)

cd "$DECKTRICKS_REPO_ROOT"
if [[ "$DECKTRICKS_TEST_TYPE" == "built_binary" ]]; then
    OUTPUT_FILE=misc/gui_benchmarks_built.txt
else
    OUTPUT_FILE=misc/gui_benchmarks_debug.txt
fi
echo "$(date +%s) :: $TIME_OUTPUT" >> "$OUTPUT_FILE"
