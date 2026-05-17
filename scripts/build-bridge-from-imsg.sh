#!/usr/bin/env bash
set -euo pipefail

IMSG_SRC="${IMSG_SRC:-$(mktemp -d)}"
INSTALL_LIB="${INSTALL_LIB:-/opt/homebrew/lib}"

if [[ ! -f "${IMSG_SRC}/Makefile" ]]; then
  git clone --depth 1 https://github.com/openclaw/imsg.git "${IMSG_SRC}"
fi

cd "${IMSG_SRC}"
make build-dylib

DYLIB="$(find .build -name imsg-bridge-helper.dylib | head -1)"
if [[ -z "${DYLIB}" ]]; then
  echo "dylib not found under .build" >&2
  exit 1
fi

mkdir -p "${INSTALL_LIB}"
cp "${DYLIB}" "${INSTALL_LIB}/imsg-bridge-helper.dylib"
echo "Installed ${INSTALL_LIB}/imsg-bridge-helper.dylib"
echo "Requires SIP disabled: csrutil disable (Recovery mode)"
