#!/usr/bin/env bash
#
# Package ISO release assets: checksum and optional cosign signature.
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

ISO_PATH="${REPO_ROOT}/build/livedisk.iso"
OUTPUT_DIR="${REPO_ROOT}/release/assets"
ASSET_NAME=""
SIGN=false
COSIGN_YES=true

usage() {
    cat <<'USAGE'
Usage: ./scripts/package_iso_assets.sh [options]

Options:
  --iso <path>          Path to ISO file (default: build/livedisk.iso)
  --output-dir <path>   Output directory (default: release/assets)
  --name <filename>     Output ISO filename (default: basename of --iso)
  --sign                Sign ISO with cosign (keyless/blob signing)
  --no-cosign-yes       Do not pass --yes to cosign
  -h, --help            Show this help

Outputs:
  <asset>.iso
  <asset>.iso.sha256
  <asset>.iso.sig        (when --sign)
  <asset>.iso.pem        (when --sign)
USAGE
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --iso)
            ISO_PATH="$2"
            shift 2
            ;;
        --output-dir)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --name)
            ASSET_NAME="$2"
            shift 2
            ;;
        --sign)
            SIGN=true
            shift
            ;;
        --no-cosign-yes)
            COSIGN_YES=false
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown argument: $1" >&2
            usage
            exit 2
            ;;
    esac
done

log() { echo "[INFO] $*"; }
ok() { echo "[ OK ] $*"; }
fail() { echo "[FAIL] $*" >&2; exit 1; }

if [[ ! -f "${ISO_PATH}" ]]; then
    fail "ISO file not found: ${ISO_PATH}"
fi

if [[ -z "${ASSET_NAME}" ]]; then
    ASSET_NAME="$(basename "${ISO_PATH}")"
fi

mkdir -p "${OUTPUT_DIR}"

TARGET_ISO="${OUTPUT_DIR}/${ASSET_NAME}"

SOURCE_ABS="$(cd "$(dirname "${ISO_PATH}")" && pwd)/$(basename "${ISO_PATH}")"
TARGET_ABS="$(cd "$(dirname "${TARGET_ISO}")" && pwd)/$(basename "${TARGET_ISO}")"

if [[ "${SOURCE_ABS}" != "${TARGET_ABS}" ]]; then
    cp "${ISO_PATH}" "${TARGET_ISO}"
    ok "Copied ISO to ${TARGET_ISO}"
else
    ok "Using ISO in place: ${TARGET_ISO}"
fi

SHA_FILE="${TARGET_ISO}.sha256"
sha256sum "${TARGET_ISO}" | awk '{print $1}' > "${SHA_FILE}"
ok "Generated checksum: ${SHA_FILE}"

if [[ "${SIGN}" == true ]]; then
    if ! command -v cosign >/dev/null 2>&1; then
        fail "cosign is required for --sign"
    fi

    local_yes_flag=()
    if [[ "${COSIGN_YES}" == true ]]; then
        local_yes_flag=(--yes)
    fi

    COSIGN_EXPERIMENTAL=1 cosign sign-blob \
        "${local_yes_flag[@]}" \
        "${TARGET_ISO}" \
        --output-signature "${TARGET_ISO}.sig" \
        --output-certificate "${TARGET_ISO}.pem"
    ok "Generated cosign signature and certificate"
fi

log "Assets ready in ${OUTPUT_DIR}:"
ls -1 "${OUTPUT_DIR}" | sed 's/^/  - /'
