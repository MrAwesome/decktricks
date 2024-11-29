#!/bin/bash

set -euxo pipefail

if [[ "${GITHUB_ACTIONS:-}" != "" && "${ACT:-}" != "true" ]]; then
    git branch -f stable
    git push origin stable --force
else
    echo "Skipping staging push because not in GitHub Actions."
fi
