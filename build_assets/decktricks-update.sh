#!/bin/bash

set -euxo pipefail

# [] use rust code
# [] fetch remote hashes file
# [] check if any listed hashes differ using xxh128sum
# [] check that all files are writable
# []    if not, write out an error message and fail out, recommend 'sudo rm -rf ~/.local/share/decktricks' and reinstall
# [] download stable
#   [] download to same filesystem! create a temporary dir in decktricks/tmp_update/ (make sure is writable/removable)
#   [] consider using rsync with continue for downloads over a slow connection?
# [] untar stable
# [] check filesize + hashes
# []    if failed: redownload and repeat (once only, no loop. if failed second time, just warn user and leave it)
# []    if success: link in temporary files, etc
#
# [] have godot gui check for error/success messages and inform user
# [] write changelog in logs

DTDIR="$HOME/.local/share/decktricks" 
TMP_UPDATE="$DTDIR/tmp_update"
mkdir -p "$TMP_UPDATE"
cd "$TMP_UPDATE"

SHOULD_UPDATE=0

curl -f -L -O --progress-bar --retry 7 --connect-timeout 60 \
    --output-dir "$TMP_UPDATE" \
    'https://github.com/MrAwesome/decktricks/releases/download/stable/XXH64SUMS'




curl -f -L -O --progress-bar --retry 7 --connect-timeout 60 \
    --output-dir "$TMP_UPDATE" \
    'https://github.com/MrAwesome/decktricks/releases/download/stable/decktricks.tar.xz'
