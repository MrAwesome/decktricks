#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"
. lib.sh 
cd "$DECKTRICKS_REPO_ROOT"

# Artifacts {{{
# Place our .desktop files and install script in the tar
cp build_assets/* "$REPOBUILD"/inside_tar

# Installer is separate for ease of download
cp build_assets/decktricks-install.desktop "$REPOBUILD"
cp build_assets/decktricks-install.sh "$REPOBUILD"
# }}}
