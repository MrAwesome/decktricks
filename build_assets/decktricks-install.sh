#!/bin/bash

# []: todo: check hashes and redownload if needed? once only, don't loop
# []: tell user if failure happens in writing files (usually because it was run as root) and how to fix ('sudo rm -rf ~/.local/share/decktricks/')

set -euxo pipefail
if [ "$(id -u)" -eq 0 ]; then
    echo "[WARN] This script should never be run as root! Exiting now..."
    exit 1
fi

ERROR=0

restart_to_game_mode_manual() {
    qdbus org.kde.Shutdown /Shutdown org.kde.Shutdown.logout
}

xdotool getwindowfocus windowstate --add ABOVE || true

if ! curl -Is https://github.com | head -1 | grep 200 > /dev/null; then
    echo "[ERROR] Could not connect to GitHub! Are you connected to the Internet?"
    read -r
    exit 1
fi

curl -f -L -O --progress-bar --retry 7 --connect-timeout 60 \
    --output-dir "/tmp" \
    'https://github.com/MrAwesome/decktricks/releases/download/stable/decktricks.tar.xz'

DTDIR="$HOME/.local/share/decktricks" 
mkdir -p "$DTDIR"
cd "$DTDIR"

tar xvf /tmp/decktricks.tar.xz
chmod +x ./*
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
    if pgrep -x steam > /dev/null; then
        set +x
        echo
        echo
        echo "Shutting down Steam, please wait..."
        echo
        set -x

        steam -shutdown &> /dev/null || true

        set +x
        for ((i=0; i<60; i++)); do
            echo "($i/60) Waiting for Steam to shut down..."
            sleep 1
            if ! pgrep -x steam > /dev/null; then
                break
            fi
        done
        set -x
    fi

    DECKTRICKS_FULL_APPID=$(cat /tmp/decktricks_newest_full_steam_appid)

    nohup steam "steam://rungameid/$DECKTRICKS_FULL_APPID" &> /dev/null &

    rm -f /tmp/decktricks_only_init
    touch /tmp/decktricks_only_init

    set +x
    for ((i=0; i<60; i++)); do
        if [[ ! -f /tmp/decktricks_only_init ]]; then
            break
        fi
        # We're not actually installing anymore, just waiting on Decktricks to be
        # placed first in the most recent games list
        echo "($i/60) Waiting for Steam to finish installing Decktricks..."
        sleep 1
    done
fi

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

cat << EOF
=====================

Decktricks is installed!

Will restart to Game Mode in 10 seconds unless you close this window.

To return to Game Mode yourself, just double-click
the "Return to Gaming Mode" icon on the desktop.

=====================
EOF

    sleep 10
    # Try to use the desktop file, and run a (possibly old) return to game mode command if it's not present
    # TODO: run the qdbus command also if CMD fails
    set -x
    if [[ -f "$HOME"/Desktop/Return.desktop ]]; then
        CMD=$(grep '^Exec=' "$HOME"/Desktop/Return.desktop | sed 's/^Exec=//')
        $CMD || restart_to_game_mode_manual
    else
        restart_to_game_mode_manual
    fi
fi

set -x
