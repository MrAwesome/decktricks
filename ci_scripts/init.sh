#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"
. lib.sh
cd "$DECKTRICKS_REPO_ROOT"

git -c safe.directory='*' rev-parse --short HEAD > /tmp/.decktricks_git_hash
git -c safe.directory='*' log -1 --pretty=%s > /tmp/.decktricks_git_title

rm -rf "$GUIBUILD"
rm -rf "$REPOBUILD"
mkdir -p "$REPOBUILD"
mkdir -p "$REPOBUILD"/inside_tar/bin
