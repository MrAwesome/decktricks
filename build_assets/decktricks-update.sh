#!/bin/bash

# NOTE: curl, tar, sed, rsync, and xxh64sum should always be present on SteamOS.
# NOTE: this file prints directly to stdout, as we will redirect output to a log file when running it.

set -euxo pipefail
if [ "$(id -u)" -eq 0 ]; then
    echo "[WARN] This script should never be run as root! Exiting now..."
    exit 1
fi

echo "[INFO] Decktricks update starting at $(date)..."

# TODOs: 
# [] have godot gui check for error/success messages and inform user
# [] write changelog in logs

dtdir="$HOME/.local/share/decktricks" 
mkdir -p "$dtdir"
cd "$dtdir"
final_message=""

# NOTE: to future maintainers, be *EXTREMELY* careful about setting this value anywhere.
# In fact, just don't do it. This is meant as an override for local testing or power users.
#
# If you ever somehow set it to "true" somewhere by default, all updates for all users
# will be paused forever.
#
# If you want to pause updates temporarily, just add a non-empty file named UPDATES_PAUSED
# (containing a user-friendly message indicating why they are paused) to the latest
# stable release, and delete that file once updates should be enabled again.
if [[ "${XX_DECKTRICKS_UPDATES_FORCE_DISABLED_XX:-false}" == "true" ]]; then
    echo "[WARN] Updates have been force-disabled, will not continue."
    exit 1
fi

repo_link="https://github.com/MrAwesome/decktricks"
issues_link="$repo_link/issues"
news_link="$repo_link"
releases_link="$repo_link/releases/download/stable"
remote_updates_paused_link="$releases_link/UPDATES_PAUSED"

# Clean up any old tmp_update dirs we may have left behind, just in case:
find . -maxdepth 1 -type d -name 'tmp_update_*' -exec rm -rf {} +

# This *MUST* be in the same filesystem as our decktricks dir, so we just make it a subdir.
tmp_update="$dtdir/tmp_update_$(date +%s)_$$"
mkdir -p "$tmp_update"
trap 'rm -rf "$tmp_update"
set +x
if [[ "$final_message" != "" ]]; then
    echo -e "\n\n!!!!!!!!!!!!!!!!"
    echo "$final_message"
fi' EXIT

hash_filename_only="DECKTRICKS_TARBALL_XXH64SUM"
installed_hash_filename="$dtdir/$hash_filename_only"
temp_hash_filename="$tmp_update/$hash_filename_only"
remote_hash_filename="$releases_link/$hash_filename_only"

tar_filename_only="decktricks.tar.xz"
tar_full_filename="$tmp_update/$tar_filename_only"
remote_tar_filename="$releases_link/$tar_filename_only"
tar_output_dir="$tmp_update/extracted"
mkdir -p "$tar_output_dir"

# Simple connectivity check, borrowed from Decky:
http_status=$(curl -L -o /dev/null -s -w "%{http_code}\n" https://github.com)
if [[ "$http_status" != "200" ]]; then
    connectivity_message="[WARN] Could not connect to github! Are you connected to the Internet? Will attempt to continue anyway..."
    final_message="${final_message}
${connectivity_message}"
fi

updates_paused_msg=$(curl -f -L --retry 7 --connect-timeout 60 "$remote_updates_paused_link" || echo "")

if [[ "$updates_paused_msg" != "" ]]; then
    echo "[WARN] Updates are paused! Check here for more info: $news_link"
    echo "Pause message: \"$updates_paused_msg\""
    exit 1
fi

checksums_enabled=true
if ! curl -f -L --retry 7 --connect-timeout 60 -o "$temp_hash_filename" "$remote_hash_filename"; then
    checksums_enabled=false
fi

if [[ ! -s "$temp_hash_filename" || ! -s "$installed_hash_filename" ]]; then
    empty_checksum_file_warning="[WARN] Checksum file was empty/missing! This is a serious bug, please report it at $issues_link

Downloaded hash file:

$(cat "$temp_hash_filename" || echo "Not found.")

Installed hash file:

$(cat "$installed_hash_filename" || echo "Not found.")

"
    echo "$empty_checksum_file_warning"
    final_message="${final_message}
${empty_checksum_file_warning}"
    checksums_enabled=false
fi



if "$checksums_enabled"; then
    # This is where we actually check "should we even update?", assuming everything has gone well
    if cmp "$installed_hash_filename" "$temp_hash_filename"; then
        echo "[INFO] Local version of Decktricks is up-to-date, will not attempt an update..."
        exit 0
    fi
else
    checksum_warning="[WARN] Did not get a valid hash from the server! Will disable hash checking. This means decktricks will attempt to update every time it is run. This is likely a bug, please report it at: $issues_link"
    echo "$checksum_warning"
    final_message="${final_message}
${checksum_warning}"
fi

failed_hash_check=true
num_retries=2
for i in $(seq "$num_retries" -1 0); do
    # If checksums are enabled, we will ignore any curl errors and try again
    curl -f -L --retry 7 --connect-timeout 60 -o "$tar_full_filename" "$remote_tar_filename" \
        || "$checksums_enabled"

    pushd "$tmp_update"
    if xxh64sum -q -c "$temp_hash_filename"; then
        echo "[INFO] Downloaded updated decktricks successfully! Continuing with update..."
        failed_hash_check=false
        break
    fi
    popd

    echo "[WARN] Hash mismatch! Retries remaining: $i"
done

if "$checksums_enabled" && "$failed_hash_check"; then
    hash_mismatch_warning="[WARN] !!! Hashes were mismatched multiple times for Decktricks update tar! Either you have a very very bad Internet connection, or this is a serious error that should be reported at: ${issues_link}
[WARN] Will continue with update, but there may be breakages."
    echo "$hash_mismatch_warning"
    final_message="${final_message}
${hash_mismatch_warning}"
fi

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
echo "[INFO] Beginning extraction..."
tar -xJf "$tar_full_filename" -C "$tar_output_dir"

echo "[INFO] Sanity testing core files..."

echo "[INFO] Extraction complete, swapping in files..."
rsync -a --delay-updates "${tar_output_dir}/" "${dtdir}/"

echo "[INFO] All files updated! Cleaning up..."

# This is fine - all files in the root directory should be executable.
# Anything we don't want to be executable will live in a subdir.
chmod +x "$dtdir"/*

# If we've made it to this point, thanks to -e we can be quite sure we're safe
# to mark this update as completed and update our hashfile
if "$checksums_enabled" && ! "$failed_hash_check"; then
    cp "$temp_hash_filename" "$installed_hash_filename"
fi

set +x
echo -e "\n\n\n"
echo "[INFO] Decktricks has been updated successfully! Version info:
$("$dtdir"/decktricks version --verbose || echo "UNKNOWN")"
