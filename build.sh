#!/bin/bash
set -euxo pipefail

# [] TODO: debug builds too?

cd "$(dirname "$0")"

TMPDIR=$(mktemp -d)
BUILDDIR="$TMPDIR/build"

mkdir -p "$BUILDDIR"

cp scripts/decktricks_install.desktop "$BUILDDIR"
cp scripts/decktricks_post_install.sh "$BUILDDIR"

cargo build --release

pushd gui/godot/
godot --headless --export-debug "Linux"
cp build/* "$BUILDDIR"
popd

pushd "$TMPDIR"
tar czf decktricks.tar.gz build/*
popd

cp "$TMPDIR"/decktricks.tar.gz build/
