#!/bin/bash

set -euxo pipefail

docker tag decktricks-act:latest gleesus/decktricks:latest
docker push gleesus/decktricks:latest
