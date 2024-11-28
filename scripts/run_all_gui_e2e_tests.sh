#!/bin/bash

set -euxo pipefail

cd "$(dirname "$0")"/..

TARGET_BINARY="${1:-}"

for script in scripts/tests/test_*.sh; do
    set +x
    echo
    echo "=================================="
    echo "= Running: $script"
    echo "=================================="
    echo
    set -x
    if [[ "$1" != "" ]]; then
        ./"$script" "$TARGET_BINARY"
    else
        ./"$script"
    fi
done
