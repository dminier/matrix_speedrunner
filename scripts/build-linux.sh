#!/usr/bin/env bash
# Build a Linux release binary and package it into dist/.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

NAME="matrix_speedrunner"
VERSION="$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)"
TARGET="x86_64-unknown-linux-gnu"
DIST="dist"
PKG="${NAME}-${VERSION}-linux-x86_64"

echo "==> Building ${NAME} ${VERSION} for ${TARGET}"
cargo build --release --target "${TARGET}"

BIN="target/${TARGET}/release/${NAME}"
if [[ ! -x "${BIN}" ]]; then
    echo "ERROR: binary not found at ${BIN}" >&2
    exit 1
fi

echo "==> Packaging into ${DIST}/${PKG}.tar.gz"
mkdir -p "${DIST}"
STAGE="${DIST}/${PKG}"
rm -rf "${STAGE}"
mkdir -p "${STAGE}"
cp "${BIN}"      "${STAGE}/${NAME}"
cp README.md    "${STAGE}/"
cp .gitignore   "${STAGE}/" 2>/dev/null || true

# Strip pour réduire la taille du binaire (déjà strippé via profile.release,
# mais on garantit ici).
strip "${STAGE}/${NAME}" 2>/dev/null || true

tar -C "${DIST}" -czf "${DIST}/${PKG}.tar.gz" "${PKG}"
( cd "${DIST}" && sha256sum "${PKG}.tar.gz" > "${PKG}.tar.gz.sha256" )

rm -rf "${STAGE}"

echo
echo "==> Done"
ls -lh "${DIST}/${PKG}.tar.gz" "${DIST}/${PKG}.tar.gz.sha256"
