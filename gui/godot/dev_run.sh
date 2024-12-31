#!/bin/bash

set -euxo pipefail

cd "$(dirname "$0")"
pushd ../rust

cargo build --release
popd

tmpdir=$(mktemp -d)
cp ../rust/target/release/libdecktricks_godot_gui.so "$tmpdir"/libdecktricks_godot_gui.so
# mv instead of copy to get atomic switch without overwriting a running file
mv "$tmpdir"/libdecktricks_godot_gui.so build/

godot
