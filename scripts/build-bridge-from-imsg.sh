#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
INSTALL_LIB="${INSTALL_LIB:-}"

make -C "${ROOT}/helper" all

if [[ -n "${INSTALL_LIB}" ]]; then
  mkdir -p "${INSTALL_LIB}"
  cp "${ROOT}/lib/imsg-bridge-helper.dylib" "${INSTALL_LIB}/"
  echo "Installed ${INSTALL_LIB}/imsg-bridge-helper.dylib"
fi

echo "Built ${ROOT}/lib/imsg-bridge-helper.dylib"
echo "Requires SIP disabled: csrutil disable (Recovery mode)"
