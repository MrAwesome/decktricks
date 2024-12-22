#!/bin/bash

# []: todo: check hashes and redownload if needed? once only, don't loop
# []: tell user if failure happens in writing files (usually because it was run as root) and how to fix ('sudo rm -rf ~/.local/share/decktricks/')

set -euxo pipefail
if [ "$(id -u)" -eq 0 ]; then
    echo "[WARN] This script should never be run as root! Exiting now..."
    exit 1
fi

error=0

restart_to_game_mode_manual() {
    qdbus org.kde.Shutdown /Shutdown org.kde.Shutdown.logout
}

xdotool getwindowfocus windowstate --add ABOVE || true

http_status=$(curl -L -o /dev/null -s -w "%{http_code}\n" https://github.com)
if [[ "$http_status" != "200" ]]; then
    echo "[WARN] Could not connect to GitHub! Are you connected to the Internet? Will attempt to continue anyway..."
fi

curl -f -L -O --progress-bar --retry 7 --connect-timeout 60 \
    --output-dir "/tmp" \
    'https://github.com/MrAwesome/decktricks/releases/download/stable/decktricks.tar.xz'

dtdir="$HOME/.local/share/decktricks" 
mkdir -p "$dtdir"
cd "$dtdir"
bin_dir="$dtdir/bin"

tar xvf /tmp/decktricks.tar.xz

pushd /tmp
xxh64sum decktricks.tar.xz | tee "$dtdir"/DECKTRICKS_TARBALL_XXH64SUM
popd

chmod +x "$bin_dir"/*

[[ -d "$HOME"/Desktop/ ]] && ln -sf "$bin_dir"/decktricks.desktop "$HOME"/Desktop/

if [[ "${DECKTRICKS_INSTALL_EXIT_EARLY_INTERNAL:-false}" == "true" ]]; then
    echo "[INFO] Exiting early..."
    exit 0
fi

set +x 
added_to_steam=1
echo "+ ./bin/decktricks add-decktricks-to-steam"
"$bin_dir"/decktricks add-decktricks-to-steam || {
    added_to_steam=0
    error=1
}
set -x

# TODO: just add a `decktricks restart-steam` and `decktricks run-via-steam` and use those here
if [[ "$added_to_steam" == "1" ]]; then
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

    decktricks_full_appid=$(cat /tmp/decktricks_newest_full_steam_appid)

    nohup steam "steam://rungameid/$decktricks_full_appid" &> /dev/null &

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

if [[ "$error" == "1" ]]; then
    if [[ "$added_to_steam" != "1" ]]; then
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
        cmd=$(grep '^Exec=' "$HOME"/Desktop/Return.desktop | sed 's/^Exec=//')
        $cmd || restart_to_game_mode_manual
    else
        restart_to_game_mode_manual
    fi
fi

set -x
