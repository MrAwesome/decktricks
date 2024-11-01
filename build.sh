#!/bin/bash

cd "$(dirname "$0")"

set -euxo pipefail

# TODO: release builds too

TMPDIR=$(mktemp -d)
cargo build
cp target/debug/decktricks "$TMPDIR"

GUI_NAME="GUI Test"
pushd godot_gui/godot/
godot --headless --export-debug "Linux"
cp "$GUI_NAME"* "$TMPDIR"
popd

pushd "$TMPDIR"
tar czf decktricks.tar.gz "$GUI_NAME"* decktricks
popd

cp "$TMPDIR"/decktricks.tar.gz build/
