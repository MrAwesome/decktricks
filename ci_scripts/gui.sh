#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"
. lib.sh
cd "$DECKTRICKS_REPO_ROOT"

# [] TODO: convert this to rust.
# [] TODO: debug builds too?
# [] TODO: warn on local uncommitted git changes

# For now, we don't package up the CLI for deployment, but we can if needed.
# CLI {{{
#cargo build --release
#cp target/release/decktricks "$TMPBUILD"
# }}}

# GUI Rust libs {{{
pushd gui/rust/
cargo build --release
popd
# }}}

# GUI Binary {{{
pushd gui/godot/
rm -rf build/
mkdir -p build/

# This helps godot find the gdextension file correctly:
rm -rf .godot/

cp "$DECKTRICKS_REPO_ROOT"/gui/rust/target/release/libdecktricks_godot_gui.so build/
godot --headless --export-release "Linux" 2>&1 | tee /tmp/godot_output.txt
if grep ERROR /tmp/godot_output.txt; then
    echo 'Errors detected during godot build! Will not continue.'
    exit 1
fi

# Put the Godot dylib and the binary into our target build dir
cp build/* "$REPOBUILD"/inside_tar

popd
# }}}

#pushd "$TMPBUILD"
#tar czf "$TMPDIR"/decktricks.tar.gz ./*
#popd
