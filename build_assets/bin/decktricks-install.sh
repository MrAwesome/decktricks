#!/bin/bash

# []: todo: if run in Game Mode, don't do the steam restart
# []: todo: check hashes and redownload if needed? once only, don't loop
# []: tell user if failure happens in writing files (usually because it was run as root) and how to fix ('sudo rm -rf ~/.local/share/decktricks/')

set -euxo pipefail
if [ "$(id -u)" -eq 0 ]; then
    echo "[WARN] This script should never be run as root! Exiting now..."
    exit 1
fi

channel="${DECKTRICKS_CHANNEL:-stable}"
steam_shutdown_wait_secs=60
steam_restart_wait_secs=60
dtdir="$HOME/.local/share/decktricks" 
mkdir -p "$dtdir"
cd "$dtdir"
bin_dir="$dtdir/bin"

restart_to_game_mode_manual() {
    qdbus org.kde.Shutdown /Shutdown org.kde.Shutdown.logout
}

xdotool getwindowfocus windowstate --add ABOVE || true

http_status=$(curl -L -o /dev/null -s -w "%{http_code}\n" https://github.com)
if [[ "$http_status" != "200" ]]; then
    echo "[WARN] Could not connect to GitHub! Are you connected to the Internet? Will attempt to continue anyway..."
fi

curl -f -L --progress-bar --retry 7 --connect-timeout 60 -o "/tmp/decktricks-update.sh" \
    "https://github.com/MrAwesome/decktricks/releases/download/${channel}/decktricks-update.sh"

# Use the update script to actually fetch and untar, since it contains all of our logic for
# checking checksums and retrying.
bash /tmp/decktricks-update.sh

[[ -d "$HOME"/Desktop/ ]] && ln -sf "$bin_dir"/decktricks.desktop "$HOME"/Desktop/

added_to_steam="true"
if ! "$bin_dir"/decktricks add-decktricks-to-steam; then
    added_to_steam="false"
fi

decktricks_full_appid=$(cat /tmp/decktricks_newest_full_steam_appid || echo "")

if [[ "${DECKTRICKS_INSTALL_DO_NOT_RESTART_STEAM:-false}" == "true" ]]; then
    echo "[INFO] Exiting early..."
    exit 0
fi

# TODO: just add a `decktricks restart-steam` and `decktricks run-via-steam` and use those here
if "$added_to_steam" && [[ "$decktricks_full_appid" != "" ]]; then
    if pgrep -x steam > /dev/null; then
        set +x
        echo
        echo
        echo "Shutting down Steam, please wait..."
        echo
        set -x

        steam -shutdown &> /dev/null || true

        set +x
        for ((i=0; i<$steam_shutdown_wait_secs; i++)); do
            echo "($i/$steam_shutdown_wait_secs) Waiting for Steam to shut down..."
            sleep 1
            if ! pgrep -x steam > /dev/null; then
                break
            fi
        done
        set -x
    fi

    nohup steam "steam://rungameid/$decktricks_full_appid" &> /dev/null &

    rm -f /tmp/decktricks_only_init
    touch /tmp/decktricks_only_init

    set +x
    for ((i=0; i<$steam_restart_wait_secs; i++)); do
        if [[ ! -f /tmp/decktricks_only_init ]]; then
            break
        fi
        # We're not actually installing anymore, just waiting on Decktricks to be
        # placed first in the most recent games list
        echo "($i/$steam_restart_wait_secs) Waiting for Steam to finish installing Decktricks..."
        sleep 1
    done
    set -x


fi

# TODO: Check if on SteamOS and improve these messages

set +x
if ! "$added_to_steam"; then
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

if [[ "$decktricks_full_appid" == "" ]]; then
    cat <<EOF

[NOTE]: Did not find an appid for Decktricks. This is a bug, please report it at: https://github.com/MrAwesome/decktricks/issues

EOF
fi

cat << EOF
=====================

Decktricks is installed!

Will restart to Game Mode in 10 seconds unless you close this window.

To return to Game Mode yourself, just double-click
the "Return to Gaming Mode" icon on the desktop.

You should see Decktricks first in your list of games. If not, it will
be under "Non-Steam Games".

=====================
EOF

# TODO: read instead, and say "press A to restart to Game Mode"?
sleep 10

# Try to use the desktop file, and run a (possibly old) return to game mode command if it's not present
set -x
if [[ -f "$HOME"/Desktop/Return.desktop ]]; then
    cmd=$(grep '^Exec=' "$HOME"/Desktop/Return.desktop | head -n 1 | sed 's/^Exec=//')
    $cmd || restart_to_game_mode_manual || true
else
    restart_to_game_mode_manual || true
fi
