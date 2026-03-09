#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

ISO_PATH="${1:-VantisOS-0.5-alpha.iso}"
ISO_NAME="$(basename "${ISO_PATH}")"
ISO_DIR="$(cd "$(dirname "${ISO_PATH}")" && pwd)"

"${SCRIPT_DIR}/package_iso_assets.sh" \
    --iso "${ISO_PATH}" \
    --output-dir "${ISO_DIR}" \
    --name "${ISO_NAME}" \
    --sign
