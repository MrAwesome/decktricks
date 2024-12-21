#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"
. lib.sh
cd "$DECKTRICKS_REPO_ROOT"

git rev-parse --short HEAD > /tmp/.decktricks_git_hash
git log -1 --pretty=%s > /tmp/.decktricks_git_title

rm -rf "$REPOBUILD"
mkdir -p "$REPOBUILD"
mkdir -p "$REPOBUILD"/inside_tar
