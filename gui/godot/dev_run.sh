#!/bin/bash

set -euxo pipefail

cd "$(dirname "$0")"

./dev_build.sh

if command -v godot4; then
    godot4
else
    godot
fi
