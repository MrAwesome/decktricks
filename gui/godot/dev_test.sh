#!/bin/bash

set -euxo pipefail

cd "$(dirname "$0")"

pushd ../rust
cargo test --release
popd
