#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"
. lib.sh
cd "$REPOBUILD"

cd inside_tar/
tar cJf decktricks.tar.xz ./*
cd ..

cp inside_tar/decktricks.tar.xz .

rm -rf inside_tar

echo "[DEBUG] $REPOBUILD: $(echo; find .)"
