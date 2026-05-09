#!/usr/bin/env bash
# Launch Obsidian on the .vault folder of this project.
set -euo pipefail
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VAULT="$DIR/.vault"
mkdir -p "$VAULT"
URI="obsidian://open?path=$(python3 -c 'import sys, urllib.parse; print(urllib.parse.quote(sys.argv[1]))' "$VAULT")"
exec obsidian "$URI" >/dev/null 2>&1 &
