#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"
. lib.sh
cd "$REPOBUILD"

find .
pushd inside_tar/
XZ_OPT='--x86 --lzma2' tar -cJf decktricks.tar.xz -- *
popd
mv inside_tar/decktricks.tar.xz .

rm -rf inside_tar

echo "[DEBUG] $(pwd; echo; find .)"
