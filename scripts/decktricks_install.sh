#!/bin/bash

set -euxo pipefail

DTDIR="$HOME/.local/share/decktricks" 
mkdir -p "$DTDIR"
cd "$DTDIR"
curl -L -O --progress-bar --output-dir /tmp --connect-timeout 60 "https://github.com/MrAwesome/decktricks/releases/download/stable/decktricks.tar.gz"
tar xvf /tmp/decktricks.tar.gz
ln -sf "$DTDIR"/decktricks.desktop "$HOME"/Desktop/

# TODO: Add to steam here!


echo "Successfully installed Decktricks!"
