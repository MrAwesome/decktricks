#!/bin/bash
set -euxo pipefail

# [] TODO: convert this to rust.
# [] TODO: debug builds too?
# [] TODO: warn on local uncommitted git changes

REPOROOT=$(realpath "$(dirname "$0")"/..)
cd "$REPOROOT"

TMPDIR=$(mktemp -d)
TMPBUILD="$TMPDIR/build"

mkdir -p "$TMPBUILD"

# Install scripts {{{
cp scripts/decktricks_install.desktop "$TMPBUILD"
cp scripts/decktricks_install.sh "$TMPBUILD"
# }}}

# CLI {{{
#cargo build --release
#cp target/release/decktricks "$TMPBUILD"
# }}}

# GUI Rust libs {{{
pushd gui/rust/
# Build both, because --import uses dev
cargo build
cargo build --release
popd
# }}}

# GUI Binary {{{
pushd gui/godot/
rm -rf build/

mkdir -p build/

ls

cp "$REPOROOT"/gui/rust/target/release/libdecktricks_godot_gui.so build/
cp "$REPOROOT"/gui/rust/target/release/libdecktricks_godot_gui.so ext/release/

# This helps godot find the gdextension file correctly:
rm -rf .godot/

godot --headless --export-release "Linux" 2>&1 | tee /tmp/godot_output.txt
if grep ERROR /tmp/godot_output.txt; then
    echo 'Errors detected during godot build! Will not continue.'
    exit 1
fi

# Put the dylib and the binary into our target build dir
cp build/* "$TMPBUILD"

popd
# }}}

pushd "$TMPBUILD"
tar czf "$TMPDIR"/decktricks.tar.gz ./*
popd

# Artifacts {{{
rm -rf "$REPOROOT"/build
mkdir -p "$REPOROOT"/build
cp "$TMPDIR"/decktricks.tar.gz "$REPOROOT"/build/
# Installer is separate for ease of download
cp "$REPOROOT"/scripts/decktricks_install.desktop "$REPOROOT"/build/
# }}}
