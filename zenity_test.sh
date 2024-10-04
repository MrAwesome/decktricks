#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"

binary="target/release/decktricks"

result="$("$binary" actions --json)"
program=$(echo "$result" | jq -r '.[].[0]' | zenity --list --title="Select Program" --column="Program")
action=$(echo "$result" | jq -r --arg program "$program" '.[] | select(.[0] == $program) | .[1][]' | zenity --list --title="Select Action" --column="Action")

zenity --info --text="$("$binary" "$action" "$program")"
