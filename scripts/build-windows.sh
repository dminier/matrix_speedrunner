#!/usr/bin/env bash
# Cross-compile a Windows .exe from Linux/macOS using the GNU toolchain
# and package it into dist/.
#
# Pré-requis :
#   - Linux : sudo apt install mingw-w64
#   - macOS : brew install mingw-w64
#   - rustup target add x86_64-pc-windows-gnu
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

NAME="matrix_speedrunner"
VERSION="$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)"
TARGET="x86_64-pc-windows-gnu"
DIST="dist"
PKG="${NAME}-${VERSION}-windows-x86_64"

if ! command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1; then
    echo "ERROR: mingw-w64 cross compiler not found." >&2
    echo "  Linux: sudo apt install mingw-w64" >&2
    echo "  macOS: brew install mingw-w64" >&2
    exit 1
fi

if ! rustup target list --installed | grep -q "^${TARGET}$"; then
    echo "==> Installing Rust target ${TARGET}"
    rustup target add "${TARGET}"
fi

echo "==> Building ${NAME} ${VERSION} for ${TARGET}"
cargo build --release --target "${TARGET}"

BIN="target/${TARGET}/release/${NAME}.exe"
if [[ ! -f "${BIN}" ]]; then
    echo "ERROR: binary not found at ${BIN}" >&2
    exit 1
fi

echo "==> Packaging into ${DIST}/${PKG}.zip"
mkdir -p "${DIST}"
STAGE="${DIST}/${PKG}"
rm -rf "${STAGE}"
mkdir -p "${STAGE}"
cp "${BIN}"     "${STAGE}/${NAME}.exe"
cp README.md   "${STAGE}/"

# Petit batch d'aide pour les utilisateurs Windows qui double-cliquent.
cat > "${STAGE}/run.bat" <<'EOF'
@echo off
REM Lance Matrix Speedrunner. Conseillé d'ouvrir Windows Terminal pour la
REM pluie en truecolor.
"%~dp0matrix_speedrunner.exe"
pause
EOF

if command -v zip >/dev/null 2>&1; then
    ( cd "${DIST}" && zip -qr "${PKG}.zip" "${PKG}" )
else
    echo "ERROR: 'zip' not found, install it first." >&2
    exit 1
fi

( cd "${DIST}" && sha256sum "${PKG}.zip" > "${PKG}.zip.sha256" )

rm -rf "${STAGE}"

echo
echo "==> Done"
ls -lh "${DIST}/${PKG}.zip" "${DIST}/${PKG}.zip.sha256"
