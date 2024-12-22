#!/bin/bash

set -euxo pipefail

source "$(dirname "$0")"/tests/lib.sh

if [[ ! -x /bin/time ]]; then
    echo "This benchmark requires GNU time. It can be installed on Arch Linux with:"
    echo "sudo pacman -S time"
    exit 1
fi

export DECKTRICKS_GUI_EXIT_IMMEDIATELY=true
full_output=$(/bin/time --format="%e / %P" "${DECKTRICKS_TEST_COMMAND[@]}" 2>&1)

time_output=$(echo "$full_output" | tail -n 1)

cd "$DECKTRICKS_REPO_ROOT"
if [[ "$DECKTRICKS_TEST_TYPE" == "built_binary" ]]; then
    output_file=misc/gui_benchmarks_built.txt
else
    output_file=misc/gui_benchmarks_debug.txt
fi
echo "$(date +%s) :: $time_output" >> "$output_file"
