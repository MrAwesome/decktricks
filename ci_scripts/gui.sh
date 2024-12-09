#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"
. lib.sh
cd "$DECKTRICKS_REPO_ROOT"

# [] TODO: warn on local uncommitted git changes

# This helps godot find the gdextension file correctly, and
# avoids any previous/local builds from corrupting state.
pushd gui/godot/
rm -rf .godot/
popd

# GUI Rust libs {{{
pushd gui/rust/

# `cargo test --release` contains our canonical build flow,
# and builds/places/tests the production versions of
# both the .so and the binary:
cargo test --release | tee /tmp/decktricks_tests_ran

# Ensure we see the output of at least one expected test (to make sure
# the test run was not a no-op)
grep -q test_dispatcher_e2e /tmp/decktricks_tests_ran

popd
# }}}

# Put the Godot dylib and the binary into our target build dir
pushd gui/godot/
test -x build/decktricks-gui
test -f build/libdecktricks_godot_gui.so
cp build/decktricks-gui "$REPOBUILD"/inside_tar
cp build/libdecktricks_godot_gui.so "$REPOBUILD"/inside_tar
popd
# }}}
