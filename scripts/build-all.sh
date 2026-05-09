#!/usr/bin/env bash
# Build both Linux and Windows packages.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"
./build-linux.sh
./build-windows.sh
echo
echo "==> All done"
ls -lh ../dist/
