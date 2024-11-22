#!/bin/bash
set -euxo pipefail

# [] TODO: debug builds too?

REPOROOT="$(dirname "$0")"
cd "$REPOROOT"

TMPDIR=$(mktemp -d)
BUILDDIR="$TMPDIR/build"

mkdir -p "$BUILDDIR"

cp scripts/decktricks_install.desktop "$BUILDDIR"
cp scripts/decktricks_post_install.sh "$BUILDDIR"

cargo build --release

pushd gui/godot/
rm build/*
godot --headless --export-debug "Linux"
cp build/* "$BUILDDIR"
popd

pushd "$BUILDDIR"
tar czf "$TMPDIR"/decktricks.tar.gz ./*
popd

cp "$TMPDIR"/decktricks.tar.gz build/
cp "$REPOROOT"/scripts/decktricks_install.desktop "$BUILDDIR"
