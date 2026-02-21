#!/usr/bin/env bash
#
# Populate legacy Redox-style source directories expected by Makefile.
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
BOOTLOADER_LEGACY_COMMIT="${BOOTLOADER_LEGACY_COMMIT:-60b87c44774d799699ca50989f283f833ca54da7}"

REFRESH=false
DRY_RUN=false
DEPTH=1

while [[ $# -gt 0 ]]; do
    case "$1" in
        --refresh)
            REFRESH=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --depth)
            if [[ $# -lt 2 ]]; then
                echo "Missing value for --depth" >&2
                exit 2
            fi
            DEPTH="$2"
            shift 2
            ;;
        -h|--help)
            cat <<'USAGE'
Usage: ./scripts/bootstrap_legacy_tree.sh [options]

Options:
  --refresh      Update already-cloned directories with fast-forward pull
  --dry-run      Print actions without making changes
  --depth N      Clone depth (default: 1)
  -h, --help     Show this help
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
warn() { echo "[WARN] $*"; }
fail() { echo "[FAIL] $*" >&2; exit 1; }

declare -a DIRS=(
    "bootloader"
    "cookbook"
    "installer"
    "isolinux"
    "kernel"
    "rust"
    "redoxfs"
)

repo_url() {
    local dir="$1"
    printf "https://github.com/redox-os/%s.git" "${dir}"
}

run_cmd() {
    if [[ "${DRY_RUN}" == true ]]; then
        echo "[DRY ] $*"
    else
        "$@"
    fi
}

clone_or_refresh() {
    local dir="$1"
    local url
    url="$(repo_url "${dir}")"
    local target="${REPO_ROOT}/${dir}"

    if [[ -d "${target}/.git" ]]; then
        if [[ "${REFRESH}" == true ]]; then
            log "Refreshing ${dir}"
            run_cmd git -C "${target}" fetch --tags origin
            run_cmd git -C "${target}" pull --ff-only origin "$(git -C "${target}" rev-parse --abbrev-ref HEAD)"
            ok "Refreshed ${dir}"
        else
            ok "Already present: ${dir}"
        fi
        if ! run_cmd git -C "${target}" submodule sync --recursive; then
            if [[ "${dir}" == "rust" ]]; then
                warn "Failed to sync rust submodules; continuing without full rust tree"
            else
                fail "Failed to sync submodules for ${dir}"
            fi
        fi
        if ! run_cmd git -C "${target}" submodule update --init --recursive; then
            if [[ "${dir}" == "rust" ]]; then
                warn "Failed to update rust submodules; continuing without full rust tree"
            else
                fail "Failed to initialize submodules for ${dir}"
            fi
        fi
        return
    fi

    if [[ -e "${target}" ]]; then
        warn "Path exists but is not a git checkout, skipping: ${target}"
        return
    fi

    log "Cloning ${dir} from ${url}"
    run_cmd git clone --depth "${DEPTH}" --recursive "${url}" "${target}"
    ok "Cloned ${dir}"
}

ensure_legacy_bootloader_layout() {
    local target="${REPO_ROOT}/bootloader"
    local asm_path="${target}/x86_64/disk.asm"

    if [[ -f "${asm_path}" ]]; then
        ok "Bootloader legacy disk.asm layout available"
        return
    fi

    warn "Bootloader repository is on modern layout; applying legacy compatibility checkout"

    if ! git -C "${target}" rev-parse --verify "${BOOTLOADER_LEGACY_COMMIT}^{commit}" >/dev/null 2>&1; then
        run_cmd git -C "${target}" fetch origin "${BOOTLOADER_LEGACY_COMMIT}"
    fi

    run_cmd git -C "${target}" checkout "${BOOTLOADER_LEGACY_COMMIT}"

    if [[ "${DRY_RUN}" == true ]]; then
        ok "Dry-run: legacy bootloader pin would be applied"
        return
    fi

    if [[ ! -f "${asm_path}" ]]; then
        fail "Legacy bootloader layout still missing after checkout"
    fi

    ok "Pinned bootloader to legacy-compatible commit ${BOOTLOADER_LEGACY_COMMIT}"
}

main() {
    log "Bootstrapping legacy source tree at ${REPO_ROOT}"
    for dir in "${DIRS[@]}"; do
        clone_or_refresh "${dir}"
    done

    ensure_legacy_bootloader_layout

    if [[ "${DRY_RUN}" == true ]]; then
        log "Dry run complete. No files were changed."
        return
    fi

    if [[ -f "${REPO_ROOT}/kernel/linkers/x86_64.ld" ]]; then
        ok "Kernel linker script is available"
    else
        fail "kernel/linkers/x86_64.ld still missing after bootstrap"
    fi

    log "Bootstrap complete. Next steps:"
    echo "  1) ./scripts/check_installability.sh"
    echo "  2) make iso"
}

main "$@"
