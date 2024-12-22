#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"
. lib.sh
cd "$DECKTRICKS_REPO_ROOT"

cargo build --release

cp target/release/decktricks "$REPOBUILD"/inside_tar/bin/
