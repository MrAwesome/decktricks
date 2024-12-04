#!/bin/bash

set -euxo pipefail
cd "$(dirname "$0")"

# NOTE: any "critical background updates" code can be spawned off here using the cli, if desired

./decktricks-gui "$@"
