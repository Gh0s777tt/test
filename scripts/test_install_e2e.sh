#!/usr/bin/env bash
#
# Automated VM smoke test for installable ISO artifacts.
# Default behavior validates that the ISO boots in QEMU for a fixed window.
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

ISO_PATH="${REPO_ROOT}/build/livedisk.iso"
DISK_PATH="${REPO_ROOT}/build/e2e-install.qcow2"
LOG_DIR="${REPO_ROOT}/build/e2e"
BOOT_TIMEOUT=60
DISK_TIMEOUT=45
INSTALL_TIMEOUT=600
MEMORY_MB=2048
CPUS=2
DISK_SIZE="20G"
DISK_FORMAT="qcow2"
INSTALL_CONFIG="${REPO_ROOT}/filesystem.toml"
INSTALLER_MANIFEST="${REPO_ROOT}/installer/Cargo.toml"

AUTO_BUILD=false
EXPECT_DISK_BOOT=false
KEEP_DISK=false
DISABLE_KVM=false
OVMF_BIOS=""
PROVISION_DISK=false
INSTALLER_NO_MOUNT=true

usage() {
    cat <<'USAGE'
Usage: ./scripts/test_install_e2e.sh [options]

Options:
  --iso <path>            ISO path (default: build/livedisk.iso)
  --disk <path>           QCOW2 disk path (default: build/e2e-install.qcow2)
  --log-dir <path>        Directory for QEMU logs (default: build/e2e)
  --boot-timeout <sec>    Live ISO boot smoke timeout (default: 60)
  --disk-timeout <sec>    Installed disk boot timeout (default: 45)
  --install-timeout <sec> Installer provisioning timeout (default: 600)
  --memory <mb>           VM memory in MB (default: 2048)
  --cpus <count>          VM vCPU count (default: 2)
  --disk-size <size>      Disk size for qemu-img create (default: 20G)
  --disk-format <fmt>     Disk format: qcow2|raw (default: qcow2)
  --ovmf-bios <path>      Optional OVMF BIOS file
  --provision-disk        Provision disk with redox_installer before disk boot test
  --install-config <path> Installer config TOML (default: filesystem.toml)
  --installer-manifest    Path to installer Cargo.toml
  --installer-mount       Use installer mount mode (default: --no-mount)
  --auto-build            Build ISO before running tests
  --expect-disk-boot      Run phase 2 boot check from disk (after install)
  --keep-disk             Do not recreate test disk
  --disable-kvm           Disable KVM acceleration even if available
  -h, --help              Show help

Examples:
  ./scripts/test_install_e2e.sh
  ./scripts/test_install_e2e.sh --auto-build --boot-timeout 90
  ./scripts/test_install_e2e.sh --disk-format raw --provision-disk --expect-disk-boot
  ./scripts/test_install_e2e.sh --expect-disk-boot --disk build/installed.qcow2
USAGE
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --iso)
            ISO_PATH="$2"
            shift 2
            ;;
        --disk)
            DISK_PATH="$2"
            shift 2
            ;;
        --log-dir)
            LOG_DIR="$2"
            shift 2
            ;;
        --boot-timeout)
            BOOT_TIMEOUT="$2"
            shift 2
            ;;
        --disk-timeout)
            DISK_TIMEOUT="$2"
            shift 2
            ;;
        --install-timeout)
            INSTALL_TIMEOUT="$2"
            shift 2
            ;;
        --memory)
            MEMORY_MB="$2"
            shift 2
            ;;
        --cpus)
            CPUS="$2"
            shift 2
            ;;
        --disk-size)
            DISK_SIZE="$2"
            shift 2
            ;;
        --disk-format)
            DISK_FORMAT="$2"
            shift 2
            ;;
        --ovmf-bios)
            OVMF_BIOS="$2"
            shift 2
            ;;
        --provision-disk)
            PROVISION_DISK=true
            shift
            ;;
        --install-config)
            INSTALL_CONFIG="$2"
            shift 2
            ;;
        --installer-manifest)
            INSTALLER_MANIFEST="$2"
            shift 2
            ;;
        --installer-mount)
            INSTALLER_NO_MOUNT=false
            shift
            ;;
        --auto-build)
            AUTO_BUILD=true
            shift
            ;;
        --expect-disk-boot)
            EXPECT_DISK_BOOT=true
            shift
            ;;
        --keep-disk)
            KEEP_DISK=true
            shift
            ;;
        --disable-kvm)
            DISABLE_KVM=true
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
warn() { echo "[WARN] $*"; }
fail() { echo "[FAIL] $*" >&2; exit 1; }

require_cmd() {
    if ! command -v "$1" >/dev/null 2>&1; then
        fail "Required command not found: $1"
    fi
}

detect_qemu() {
    if command -v qemu-system-x86_64 >/dev/null 2>&1; then
        echo "qemu-system-x86_64"
    elif command -v qemu-system-x86 >/dev/null 2>&1; then
        echo "qemu-system-x86"
    else
        fail "QEMU system emulator not found (qemu-system-x86_64 or qemu-system-x86)"
    fi
}

detect_timeout_bin() {
    if command -v timeout >/dev/null 2>&1; then
        echo "timeout"
    elif command -v gtimeout >/dev/null 2>&1; then
        echo "gtimeout"
    else
        fail "timeout command not found (install coreutils)"
    fi
}

run_timed_qemu() {
    local timeout_bin="$1"
    local timeout_sec="$2"
    local log_file="$3"
    shift 3

    set +e
    "${timeout_bin}" --signal=TERM "${timeout_sec}" "$@" >"${log_file}" 2>&1
    local rc=$?
    set -e

    # timeout exits 124 when process did not terminate itself.
    if [[ "${rc}" -eq 124 || "${rc}" -eq 0 ]]; then
        return 0
    fi

    echo
    echo "QEMU command failed. Last log lines:"
    tail -n 40 "${log_file}" || true
    return "${rc}"
}

main() {
    cd "${REPO_ROOT}"
    require_cmd qemu-img
    require_cmd cargo
    local qemu_bin timeout_bin
    qemu_bin="$(detect_qemu)"
    timeout_bin="$(detect_timeout_bin)"

    if [[ "${DISK_FORMAT}" != "qcow2" && "${DISK_FORMAT}" != "raw" ]]; then
        fail "Unsupported disk format: ${DISK_FORMAT}. Use qcow2 or raw."
    fi

    if [[ "${AUTO_BUILD}" == true ]]; then
        log "Building ISO before test"
        "${REPO_ROOT}/scripts/build_installable_iso.sh" --bootstrap
    fi

    if [[ ! -f "${ISO_PATH}" ]]; then
        fail "ISO not found at ${ISO_PATH}. Build it first with ./scripts/build_installable_iso.sh --bootstrap"
    fi

    if [[ "${KEEP_DISK}" == false ]]; then
        rm -f "${DISK_PATH}"
    fi

    mkdir -p "${LOG_DIR}"
    mkdir -p "$(dirname "${DISK_PATH}")"

    if [[ ! -f "${DISK_PATH}" ]]; then
        qemu-img create -f "${DISK_FORMAT}" "${DISK_PATH}" "${DISK_SIZE}" >/dev/null
        ok "Created test disk: ${DISK_PATH}"
    else
        warn "Reusing existing test disk: ${DISK_PATH}"
    fi

    if [[ "${PROVISION_DISK}" == true ]]; then
        if [[ ! -f "${INSTALLER_MANIFEST}" ]]; then
            fail "Installer manifest not found: ${INSTALLER_MANIFEST}"
        fi
        if [[ ! -f "${INSTALL_CONFIG}" ]]; then
            fail "Installer config not found: ${INSTALL_CONFIG}"
        fi
        if [[ "${DISK_FORMAT}" != "raw" ]]; then
            fail "--provision-disk currently requires --disk-format raw"
        fi

        local installer_log="${LOG_DIR}/installer_provision.log"
        local -a install_cmd=(
            cargo
            run
            --manifest-path "${INSTALLER_MANIFEST}"
            --
            "${DISK_PATH}"
            "--config=${INSTALL_CONFIG}"
        )
        if [[ "${INSTALLER_NO_MOUNT}" == true ]]; then
            install_cmd+=(--no-mount)
        fi

        log "Provisioning disk via redox_installer (${INSTALL_TIMEOUT}s)"
        if ! "${timeout_bin}" --signal=TERM "${INSTALL_TIMEOUT}" "${install_cmd[@]}" >"${installer_log}" 2>&1; then
            echo
            echo "Installer provisioning failed. Last log lines:"
            tail -n 60 "${installer_log}" || true
            if grep -q 'Package PackageName("bootloader") not found' "${installer_log}"; then
                echo "Hint: remote package index does not currently expose 'bootloader' for this target."
                echo "      Disk provisioning may require a local cookbook with prebuilt bootloader artifacts."
            fi
            fail "Disk provisioning failed"
        fi
        ok "Disk provisioning completed (log: ${installer_log})"
    fi

    local -a common_flags=(
        -display none
        -no-reboot
        -serial mon:stdio
        -net none
        -m "${MEMORY_MB}"
        -smp "${CPUS}"
        -drive "file=${DISK_PATH},format=qcow2,if=virtio"
    )

    if [[ "${DISABLE_KVM}" == false && -e /dev/kvm ]]; then
        common_flags+=(-enable-kvm -cpu host)
    fi

    if [[ -n "${OVMF_BIOS}" ]]; then
        if [[ ! -f "${OVMF_BIOS}" ]]; then
            fail "OVMF BIOS file not found: ${OVMF_BIOS}"
        fi
        common_flags+=(-bios "${OVMF_BIOS}")
    fi

    local iso_log="${LOG_DIR}/iso_boot.log"
    log "Phase 1: ISO boot smoke test (${BOOT_TIMEOUT}s)"
    if ! run_timed_qemu "${timeout_bin}" "${BOOT_TIMEOUT}" "${iso_log}" \
        "${qemu_bin}" \
        "${common_flags[@]}" \
        -boot d \
        -cdrom "${ISO_PATH}"; then
        fail "ISO smoke test failed"
    fi
    ok "ISO smoke test completed (log: ${iso_log})"

    if [[ "${EXPECT_DISK_BOOT}" == true ]]; then
        local disk_log="${LOG_DIR}/disk_boot.log"
        log "Phase 2: disk boot check (${DISK_TIMEOUT}s)"
        if ! run_timed_qemu "${timeout_bin}" "${DISK_TIMEOUT}" "${disk_log}" \
            "${qemu_bin}" \
            "${common_flags[@]}" \
            -boot c; then
            fail "Disk boot check failed"
        fi
        ok "Disk boot check completed (log: ${disk_log})"
    else
        warn "Disk boot phase skipped. Use --expect-disk-boot after running installer."
    fi

    ok "E2E smoke workflow completed"
}

main "$@"
