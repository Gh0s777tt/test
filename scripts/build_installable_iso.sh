#!/usr/bin/env bash
#
# One-command helper to prepare and build a VantisOS live/install image.
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

RUN_INSTALL_DEPS=false
RUN_BOOTSTRAP=false
BOOTSTRAP_REFRESH=false
SKIP_PREFLIGHT=false
TARGET="iso"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --install-deps)
            RUN_INSTALL_DEPS=true
            shift
            ;;
        --bootstrap)
            RUN_BOOTSTRAP=true
            shift
            ;;
        --refresh-bootstrap)
            RUN_BOOTSTRAP=true
            BOOTSTRAP_REFRESH=true
            shift
            ;;
        --skip-preflight)
            SKIP_PREFLIGHT=true
            shift
            ;;
        --target)
            if [[ $# -lt 2 ]]; then
                echo "Missing value for --target" >&2
                exit 2
            fi
            TARGET="$2"
            shift 2
            ;;
        -h|--help)
            cat <<'USAGE'
Usage: ./scripts/build_installable_iso.sh [options]

Options:
  --install-deps       Run scripts/install_deps.sh first
  --bootstrap          Run scripts/bootstrap_legacy_tree.sh first
  --refresh-bootstrap  Run bootstrap and refresh existing clones
  --skip-preflight     Skip scripts/check_installability.sh
  --target <iso|live>  Build target (default: iso)
  -h, --help           Show this help

Examples:
  ./scripts/build_installable_iso.sh --install-deps --bootstrap
  ./scripts/build_installable_iso.sh --bootstrap --target live
USAGE
            exit 0
            ;;
        *)
            echo "Unknown argument: $1" >&2
            exit 2
            ;;
    esac
done

log() { echo "[INFO] $*"; }
ok() { echo "[ OK ] $*"; }
fail() { echo "[FAIL] $*" >&2; exit 1; }

if [[ "${TARGET}" != "iso" && "${TARGET}" != "live" ]]; then
    fail "Unsupported target '${TARGET}'. Use --target iso or --target live."
fi

cd "${REPO_ROOT}"

if [[ ! -f "Makefile" ]]; then
    fail "Makefile not found in repository root: ${REPO_ROOT}"
fi

if [[ "${RUN_INSTALL_DEPS}" == true ]]; then
    log "Installing dependencies"
    "${REPO_ROOT}/scripts/install_deps.sh"
fi

if [[ "${RUN_BOOTSTRAP}" == true ]]; then
    log "Bootstrapping legacy source tree"
    if [[ "${BOOTSTRAP_REFRESH}" == true ]]; then
        "${REPO_ROOT}/scripts/bootstrap_legacy_tree.sh" --refresh
    else
        "${REPO_ROOT}/scripts/bootstrap_legacy_tree.sh"
    fi
fi

if [[ "${SKIP_PREFLIGHT}" == false ]]; then
    log "Running installability preflight"
    "${REPO_ROOT}/scripts/check_installability.sh"
fi

if [[ "${TARGET}" == "iso" ]]; then
    log "Building installable ISO via make iso"
    make iso
    ok "ISO build completed: ${REPO_ROOT}/build/livedisk.iso"
else
    log "Building live disk image via make live"
    make live
    ok "Live disk build completed: ${REPO_ROOT}/build/livedisk.bin"
fi
