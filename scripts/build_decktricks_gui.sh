#!/bin/bash
set -euxo pipefail

# [] TODO: convert this to rust.
# [] TODO: debug builds too?
# [] TODO: warn on local uncommitted git changes

BTYPE="release"
if [[ "${1:-}" == "debug" ]]; then
    BTYPE="debug"
fi

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
if [[ "$BTYPE" == "debug" ]]; then
    cargo build
else
    cargo build --release
fi
cp target/"$BTYPE"/decktricks "$TMPBUILD"
# }}}

# GUI Rust libs {{{
pushd gui/rust/
if [[ "$BTYPE" == "debug" ]]; then
    cargo build
else
    cargo build --release
fi
popd
# }}}

# GUI Binary {{{
pushd gui/godot/
rm -rf build/

mkdir -p build/
cp "$REPOROOT"/gui/rust/target/"$BTYPE"/libdecktricks_godot_gui.so build/

# This helps godot find the gdextension file correctly:
rm -rf .godot/
timeout 15 godot --headless --import

godot --headless "--export-${BTYPE}" "Linux" 2>&1 | tee /tmp/godot_output.txt
if grep ERROR /tmp/godot_output.txt; then
    echo 'Errors detected during godot build! Will not continue.'
    exit 1
fi
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
