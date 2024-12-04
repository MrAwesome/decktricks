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

# TODO: just add a `decktricks restart-steam` and `decktricks run-via-steam`
if [[ "$ADDED_TO_STEAM" == "1" ]]; then
    echo
    echo
    echo "Shutting down Steam, please wait..."
    steam -shutdown &> /dev/null || true

    for ((i=0; i<15; i++)); do
        if ! pgrep steam > /dev/null; then
            break
        fi
        echo "($i/15) Waiting for Steam to shut down..."
        sleep 1
    done

    DECKTRICKS_FULL_APPID=$(cat /tmp/decktricks_newest_full_steam_appid)

    steam "steam://rungameid/$DECKTRICKS_FULL_APPID"
fi

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

echo
echo "Will restart to Game Mode in 10 seconds unless you close this window..."
sleep 10
# Try to use the desktop file, and run a (possibly old) return to game mode command if it's not present
if [[ -f "$HOME"/Desktop/Return.desktop ]]; then
    CMD=$(grep '^Exec=' "$HOME"/Desktop/Return.desktop | sed 's/^Exec=//')
    $CMD
else
    qdbus org.kde.Shutdown /Shutdown org.kde.Shutdown.logout
fi

set -x
