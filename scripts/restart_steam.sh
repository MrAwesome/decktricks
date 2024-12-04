# If needed in scripts:
# TODO: just add a `decktricks restart-steam` and `decktricks run-via-steam`
if [[ "$ADDED_TO_STEAM" == "1" && "$DECKTRICKS_TEST_SHOULD_RESTART" == "1" ]]; then
    echo
    echo
    echo "Shutting down Steam, please wait..."
    steam -shutdown &> /dev/null || true
    sleep 15

    DECKTRICKS_FULL_APPID=$(cat /tmp/decktricks_newest_full_steam_appid)

    steam "steam://rungameid/$DECKTRICKS_FULL_APPID"
fi

