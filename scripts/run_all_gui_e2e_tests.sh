#!/bin/bash

set -euxo pipefail

cd "$(dirname "$0")"/..

for script in scripts/tests/test_*.sh; do
    set +x
    echo
    echo "=================================="
    echo "= Running: $script"
    echo "=================================="
    echo
    set -x
    ./"$script"
done
