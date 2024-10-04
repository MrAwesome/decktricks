#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"

binary="target/release/decktricks"

result="$("$binary" see-all-available-actions --json)"
program=$(echo "$result" | jq -r '.[].[0]' | zenity --list --title="Select Program" --column="Program")
action=$(echo "$result" | jq -r --arg program "$program" '.[] | select(.[0] == $program) | .[1][]' | zenity --list --title="Select Action" --column="Action")

if [[ "$action" == "info" ]]; then
    zenity --info --text="$("$binary" "$action" "$program")"
else 
    "$binary" "$action" "$program" 
fi
