#!/bin/bash

set -euxo pipefail

# To avoid needing sudo to run docker commands, run this and then log out and back in:
# sudo usermod -aG docker $USER

# To install buildx on Arch:
# sudo pacman -S docker-buildx

cd "$(dirname "$0")"/../misc/act-docker
docker build -t decktricks-act .
