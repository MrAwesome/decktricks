#!/bin/bash

set -euxo pipefail

DTDIR="$HOME/.local/share/decktricks" 
mkdir -p "$DTDIR"
cd "$DTDIR"
curl -L -O --progress-bar --output-dir /tmp --connect-timeout 60 "https://github.com/MrAwesome/decktricks/releases/download/stable/decktricks.tar.xz"
tar xvf /tmp/decktricks.tar.xz
ln -sf "$DTDIR"/decktricks.desktop "$HOME"/Desktop/

# TODO: Add to steam here!
# something like:
#   ./"$DTDIR"/decktricks add-gui-to-steam

echo "Successfully installed Decktricks!"
