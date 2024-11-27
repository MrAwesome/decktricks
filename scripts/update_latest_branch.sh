#!/bin/bash

set -euxo pipefail

if [[ "${GITHUB_ACTIONS:-}" != "" && "${ACT:-}" != "true" ]]; then
    git branch -f latest
    git push origin latest --force
else
    echo "Skipping latest push because not in GitHub Actions."
fi
