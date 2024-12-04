#!/bin/bash

set -euxo pipefail

ERROR=0

curl -L -O --progress-bar --output-dir /tmp --connect-timeout 60 "https://github.com/MrAwesome/decktricks/releases/download/stable/decktricks.tar.xz"

DTDIR="$HOME/.local/share/decktricks" 
mkdir -p "$DTDIR"
cd "$DTDIR"

tar xvf /tmp/decktricks.tar.xz
ln -sf "$DTDIR"/decktricks.desktop "$HOME"/Desktop/

set +x 
ADDED_TO_STEAM=1
echo "+ ./decktricks add-decktricks-to-steam"
./decktricks add-decktricks-to-steam || {
    ADDED_TO_STEAM=0
    ERROR=1
}
set -x

# TODO: just add a `decktricks restart-steam` and `decktricks run-via-steam` and use those here
if [[ "$ADDED_TO_STEAM" == "1" ]]; then
    echo
    echo
    echo "Shutting down Steam, please wait..."
    steam -shutdown &> /dev/null || true

    set +x
    for ((i=0; i<30; i++)); do
        if ! pgrep steam > /dev/null; then
            break
        fi
        echo "($i/30) Waiting for Steam to shut down..."
        sleep 1
    done
    set -x

    DECKTRICKS_FULL_APPID=$(cat /tmp/decktricks_newest_full_steam_appid)

    nohup steam "steam://rungameid/$DECKTRICKS_FULL_APPID" &> /dev/null &

    # TODO: wait for steam to launch, wait for decktricks to launch, then restart
    rm -f /tmp/decktricks_only_init
    touch /tmp/decktricks_only_init

    set +x
    for ((i=0; i<30; i++)); do
        if [[ ! -f /tmp/decktricks_only_init ]]; then
            break
        fi
        # We're not actually installing anymore, just waiting on Decktricks to be
        # placed first in the most recent games list
        echo "($i/30) Waiting for Steam to finish installing Decktricks..."
        sleep 1
    done
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

if [[ "$ERROR" == "1" ]]; then
    if [[ "$ADDED_TO_STEAM" != "1" ]]; then
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
    fi
else
    echo
    echo "Will restart to Game Mode in 10 seconds unless you close this window..."
    sleep 10
    # Try to use the desktop file, and run a (possibly old) return to game mode command if it's not present
    set -x
    if [[ -f "$HOME"/Desktop/Return.desktop ]]; then
        CMD=$(grep '^Exec=' "$HOME"/Desktop/Return.desktop | sed 's/^Exec=//')
        $CMD
    else
        qdbus org.kde.Shutdown /Shutdown org.kde.Shutdown.logout
    fi
fi

set -x
