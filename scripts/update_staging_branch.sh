#!/bin/bash

set -euxo pipefail

if [[ "${GITHUB_ACTIONS:-}" != "" && "${ACT:-}" != "true" ]]; then
    git branch -f staging
    git push origin staging --force
else
    echo "Skipping staging push because not in GitHub Actions."
fi
