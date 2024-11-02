#!/bin/bash
set -euxo pipefail

# [] TODO: debug builds too?

cd "$(dirname "$0")"

GUI_NAME="Decktricks"
TMPDIR=$(mktemp -d)

cp decktricks.desktop "$TMPDIR"

cargo build --release
cp target/release/decktricks "$TMPDIR"

pushd gui/godot/
godot --headless --export-debug "Linux"
cp build/"$GUI_NAME"* "$TMPDIR"
popd

pushd "$TMPDIR"
tar czf decktricks.tar.gz "$GUI_NAME"* decktricks
popd

cp "$TMPDIR"/decktricks.tar.gz build/
