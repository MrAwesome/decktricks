#!/bin/bash

DECKTRICKS_REPO_ROOT="$(realpath "$(dirname "${BASH_SOURCE[0]}")"/../..)"
REPOBUILD="$DECKTRICKS_REPO_ROOT"/build
REPOTAR="$DECKTRICKS_REPO_ROOT"/inside_tar

export DECKTRICKS_REPO_ROOT
export REPOBUILD
export REPOTAR
