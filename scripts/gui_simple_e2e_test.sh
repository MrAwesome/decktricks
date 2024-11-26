#!/bin/bash

set -euxo pipefail

cd "$(dirname "$0")"/..

pushd gui/rust
cargo build
popd

pushd gui/godot

export DECKTRICKS_GUI_EXIT_IMMEDIATELY=true
timeout 10 godot --headless | grep "Decktricks GUI initialization complete!"
