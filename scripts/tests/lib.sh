set -euxo pipefail

# If a binary name (of a built GUI) is given, use that.
# Otherwise, build the rust libs and run dynamically.
GIVEN_BINARY="${1:-}"
COMMAND_LOCATION=$(realpath "$(dirname "$GIVEN_BINARY")")
COMMAND_BASENAME=$(basename "$GIVEN_BINARY")

# Go to repo root
DECKTRICKS_REPO_ROOT="$(realpath "$(dirname "${BASH_SOURCE[0]}")"/../..)"
cd "$DECKTRICKS_REPO_ROOT"

if [[ "$GIVEN_BINARY" == "" ]]; then
    DECKTRICKS_TEST_TYPE=debug
    DECKTRICKS_TEST_COMMAND=(godot --headless)
    pushd gui/rust
    cargo build
    popd

    pushd gui/godot
else
    DECKTRICKS_TEST_TYPE=built_binary
    DECKTRICKS_TEST_COMMAND=("$COMMAND_LOCATION"/"$COMMAND_BASENAME" --headless)

    pushd .
fi

export DECKTRICKS_TEST_COMMAND
export DECKTRICKS_REPO_ROOT
export DECKTRICKS_TEST_TYPE
