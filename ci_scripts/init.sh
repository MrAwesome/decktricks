#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"
. lib.sh

rm -rf "$REPOBUILD"
mkdir -p "$REPOBUILD"
mkdir -p "$REPOBUILD"/inside_tar
