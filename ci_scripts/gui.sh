#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"
. lib.sh
cd "$DECKTRICKS_REPO_ROOT"

# [] NOTE: warn on local uncommitted git changes

# This helps godot find the gdextension file correctly, and
# avoids any previous/local builds from corrupting state.
pushd gui/godot/
rm -rf .godot/
popd

# GUI Rust libs {{{
pushd gui/rust/
cargo run --release --bin gui-tool -- build-and-export | tee /tmp/decktricks_gui_build
popd
# }}}

# Put the Godot dylib and the binary into our target build dir
pushd gui/godot/
test -x build/decktricks-gui
test -f build/libdecktricks_godot_gui.so
cp build/decktricks-gui "$REPOBUILD"/inside_tar/bin/
cp build/libdecktricks_godot_gui.so "$REPOBUILD"/inside_tar/bin/
popd
# }}}
