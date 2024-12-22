#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"
. lib.sh 
cd "$DECKTRICKS_REPO_ROOT"

# Artifacts {{{
# Place our .desktop files and install script in the tar
cp -r build_assets/bin/* "$REPOBUILD"/inside_tar/bin/

# Installer is separate for ease of download
cp build_assets/bin/decktricks-install.desktop "$REPOBUILD"
cp build_assets/bin/decktricks-install.sh "$REPOBUILD"
# }}}
