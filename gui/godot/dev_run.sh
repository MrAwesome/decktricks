#!/bin/bash

set -euxo pipefail

cd "$(dirname "$0")"
pushd ../rust

cargo build --release
popd

cp ../rust/target/release/libdecktricks_godot_gui.so build/

godot
