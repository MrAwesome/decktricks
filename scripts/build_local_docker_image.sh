#!/bin/bash

set -euxo pipefail

cd "$(dirname "$0")"/../misc/act-docker

# To avoid needing sudo to run docker commands, run this and then log out and back in:
# sudo usermod -aG docker $USER

systemctl status docker --no-pager || systemctl start docker

# To install buildx on Arch:
# sudo pacman -S docker-buildx

# NOTE: this uses the github repo, NOT the local repo.
docker buildx build --no-cache-filter="gitclone" -t decktricks-act .
