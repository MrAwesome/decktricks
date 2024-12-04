#!/bin/bash

set -euxo pipefail

curl -L -O --progress-bar --output-dir /tmp --connect-timeout 60 "https://github.com/MrAwesome/decktricks/releases/download/stable/decktricks.tar.xz"

DTDIR="$HOME/.local/share/decktricks" 
mkdir -p "$DTDIR"
cd "$DTDIR"

tar xvf /tmp/decktricks.tar.xz
ln -sf "$DTDIR"/decktricks.desktop "$HOME"/Desktop/

set +x 
echo "+ ./decktricks add-decktricks-to-steam"
./decktricks add-decktricks-to-steam || {
    cat <<EOF
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
Failed to add Decktricks to Steam! This is most likely because
you aren't logged in to Steam on this system.
If you are logged in, please report this error here:
https://github.com/MrAwesome/decktricks/issues

In SteamOS (Steam Deck), you can add Decktricks to Steam easily:
1) Right-click the "Decktricks" icon on the desktop
        (use L2/left-trigger on the Deck or controller)
2) Select "Add to Steam"
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
EOF
}

cat << EOF
=====================

Decktricks is installed! You can return to Game Mode now by
double-clicking the "Return to Gaming Mode" icon on the desktop.

You can also run Decktricks now by double-clicking the
"Decktricks" icon on the desktop.

Decktricks will be available in Steam after you return to Game Mode,
restart Steam, or restart your Deck.

Look for "Non-Steam Games" in your Library, press R1/right-bumper
until you see it. Enjoy!

=====================
EOF

set -x
