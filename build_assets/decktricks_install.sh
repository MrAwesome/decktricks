#!/bin/bash

set -euxo pipefail

curl -L -O --progress-bar --output-dir /tmp --connect-timeout 60 "https://github.com/MrAwesome/decktricks/releases/download/stable/decktricks.tar.xz"

DTDIR="$HOME/.local/share/decktricks" 
mkdir -p "$DTDIR"
cd "$DTDIR"

tar xvf /tmp/decktricks.tar.xz
ln -sf "$DTDIR"/decktricks.desktop "$HOME"/Desktop/
./decktricks add-decktricks-to-steam

set +x
echo
echo
echo
echo "====================="
echo 
echo "Successfully installed Decktricks! You can return to Game Mode now by double-clicking the \"Return to Gaming Mode\" icon on the desktop."
echo
echo "====================="
echo
set -x
