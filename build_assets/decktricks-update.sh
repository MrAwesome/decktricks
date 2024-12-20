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

dtdir="$HOME/.local/share/decktricks" 
cd "$dtdir"

decktricks_xz_hash_filename="$dtdir/decktricks_tar_xz_hash"
tmp_update="$dtdir/tmp_update"
tar_filename_only="$tmp_update/decktricks.tar.gz"
tar_full_filename="$tmp_update/decktricks.tar.gz"
tar_output_dir="$tmp_update/extracted"

mkdir -p "$dtdir/logs"
logfile="$dtdir/logs/decktricks-update.log"

[[ -f "$logfile" ]] && mv "$logfile"{,.bak}
: > "$logfile"
echo "[INFO] Decktricks update starting at $(date)..." &>> "$logfile"

if [[ -f "$decktricks_xz_hash_filename" ]]; then
    local_decktricks_xz_hash=$(cat "$decktricks_xz_hash_filename" | sed -E 's/(^\s*)//' | sed -E 's/\s*$//')
else
    local_decktricks_xz_hash="xXx_NO_LOCAL_HASH_FOUND_xXx"
fi

desired_decktricks_hash=$(curl -f -L --progress-bar --retry 8 --connect-timeout 60 \
    'https://github.com/MrAwesome/decktricks/releases/download/stable/decktricks.tar.xz.xxh64sum' \
        2>> "$logfile" \
        | sed -E 's/(^\s*)//' | sed -E 's/\s*$//')

if [[ "$desired_decktricks_hash" == "$local_decktricks_xz_hash" ]]; then
    echo "[INFO] Local version of Decktricks is up-to-date, will not attempt an update..." &>> "$logfile"
    exit 0
fi

if [[ "$desired_decktricks_hash" == "updates_paused" ]]; then
    echo "[WARN] Updates are paused from the server side! Not continuing..." &>> "$logfile"
    exit 0
fi

hash_retried=false
while true; do
    # Plop our decktricks.tar.xz into "$tmp_update"
    curl -f -L -O --progress-bar --retry 7 --connect-timeout 60 \
        --output-dir "$tmp_update" \
        "https://github.com/MrAwesome/decktricks/releases/download/stable/$tar_filename_only" \
        &>> "$logfile"


    # Grab the hash of the newly-downloaded file
    downloaded_hash=$(xxh64sum "$tar_full_filename" | awk -F'  ' '{ print $1 }')

    # Check the hash, and retry once if it doesn't match
    if [[ "$downloaded_hash" != "$desired_decktricks_hash" ]]; then
        if [[ "$hash_retried" == false ]]; then
            echo "[WARN] Hash mismatch! Will download again." \
                &>> "$logfile"
            hash_retried=true
            continue
        else
            echo "[ERROR] !!! Hashes were mismatched multiple times for Decktricks update tar! Either you have a very very bad Internet connection, or this is a serious error that should be reported at: https://github.com/MrAwesome/decktricks/issues" &>> "$logfile"
            exit 0
        fi
    fi

    break
done

################################################################################
# Some notes:
#  1) --delay-updates helps us preserve atomicity
#       (see the rsync man page)
#  2) The trailing slashes are extremely important, otherwise
#       we would copy the dir named "extracted" into .local/share/decktricks
#  3) We use rsync here (particularly with --delay-updates) because we
#       MUST unlink/mv/rm/ln in some way that does not *overwrite* the existing
#       binaries/libs! The running decktricks instance will continue to see
#       the open inode for the old version, and only pick up the new binaries
#       after a restart. Otherwise, using cp or tar --overwrite, we would be
#       modifying the actual running binary. (And we don't use tar directly
#       here, because it unlinks and *then* does extraction, meaning we would
#       have an incomplete version of the libs/binaries on disk during
#       the extraction process)
#   4) Using rsync instead of our own hand-rolled bespoke find/mv logic would
#       avoid a copy of the files (from "extracted" into our data dir), but
#       at the cost of relying on our own code logic vs. the program designed
#       and tested to do exactly what we want (move all files from A to B)
#   5) rsync should always be present in SteamOS. When supporting other
#       platforms, you may need to check for it and do something else
#       if it is not present
################################################################################
echo "[INFO] Beginning extraction..." &>> "$logfile"
tar -xJf "$tar_full_filename" -C "$tar_output_dir" &>> "$logfile"
    

echo "[INFO] Extraction complete, swapping in files..." &>> "$logfile"
rsync -a --delay-updates "${tar_output_dir}/" "${dtdir}/" &>> "$logfile"

echo "[INFO] All files updated! Cleaning up..." &>> "$logfile"
rm -rf "$tmp_update"

echo "[INFO] Decktricks has been updated! Version:" &>> "$logfile"
"$dtdir"/decktricks --version
