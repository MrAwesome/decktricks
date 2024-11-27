#!/bin/bash

set -euxo pipefail

# To avoid needing sudo to run docker commands, run this and then log out and back in:
# sudo usermod -aG docker $USER

cd "$(dirname "$0")"/../misc/act-docker
docker build -t decktricks-act .
