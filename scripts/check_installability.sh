#!/usr/bin/env bash
#
# Preflight checks for producing an installable VantisOS image.
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "${REPO_ROOT}"

FAILURES=0
WARNINGS=0
CHECKS=0
LEGACY_GRAPH_FAILURES=0

pass() {
    echo "[PASS] $1"
    CHECKS=$((CHECKS + 1))
}

warn() {
    echo "[WARN] $1"
    WARNINGS=$((WARNINGS + 1))
}

fail() {
    echo "[FAIL] $1"
    FAILURES=$((FAILURES + 1))
}

check_cmd() {
    local cmd="$1"
    local label="$2"
    if command -v "${cmd}" >/dev/null 2>&1; then
        pass "${label} (${cmd})"
    else
        fail "${label} (${cmd}) is missing"
    fi
}

check_path_exists() {
    local path="$1"
    local label="$2"
    if [[ -e "${path}" ]]; then
        pass "${label}: ${path}"
    else
        fail "${label}: ${path} is missing"
    fi
}

check_legacy_path() {
    local path="$1"
    local label="$2"
    if [[ -e "${path}" ]]; then
        pass "${label}: ${path}"
    else
        fail "${label}: ${path} is missing"
        LEGACY_GRAPH_FAILURES=$((LEGACY_GRAPH_FAILURES + 1))
    fi
}

check_path_optional() {
    local path="$1"
    local label="$2"
    if [[ -e "${path}" ]]; then
        pass "${label}: ${path}"
    else
        warn "${label}: ${path} not found"
    fi
}

check_iso_config() {
    if [[ -f "vantis.cfg" ]]; then
        pass "ISO menu config found at repo root (vantis.cfg)"
    elif [[ -f "release/iso/vantis.cfg" ]]; then
        pass "ISO menu config found at release/iso/vantis.cfg"
    else
        fail "ISO menu config missing (expected vantis.cfg or release/iso/vantis.cfg)"
    fi
}

echo "VantisOS installability preflight"
echo "Repository: ${REPO_ROOT}"
echo

# Core toolchain
check_cmd git "Git"
check_cmd make "Make"
check_cmd rustc "Rust compiler"
check_cmd cargo "Cargo"
check_cmd nasm "NASM assembler"
check_cmd ld "Linker"
check_cmd objcopy "Objcopy"
check_cmd xorriso "ISO builder (xorriso)"
check_cmd genisoimage "ISO builder compatibility (genisoimage)"
check_cmd qemu-system-x86_64 "QEMU runtime"

echo
echo "Legacy build graph checks"

# Build scripts and make graph roots
check_path_exists "Makefile" "Makefile"
check_path_exists "mk/kernel.mk" "Kernel make rules"
check_path_exists "mk/disk.mk" "Disk make rules"
check_path_exists "scripts/build_iso.sh" "ISO script"
check_path_exists "boot/bootloader.toml" "Boot config"

# Critical directories/files expected by Makefile targets
check_legacy_path "kernel/linkers/x86_64.ld" "Kernel linker script"
check_legacy_path "bootloader" "Bootloader directory"
check_legacy_path "isolinux" "ISOLINUX directory"

echo
echo "Boot artifact checks for scripts/build_iso.sh"
check_path_optional "BOOTX64.EFI" "UEFI bootloader binary"
check_path_optional "kernel.elf" "Kernel ELF"
check_path_optional "initrd.img" "Initramfs image"
check_iso_config

echo
echo "Summary"
echo "  checks:   ${CHECKS}"
echo "  warnings: ${WARNINGS}"
echo "  failures: ${FAILURES}"

if [[ "${FAILURES}" -gt 0 ]]; then
    echo
    echo "Installability check failed."
    if [[ "${LEGACY_GRAPH_FAILURES}" -gt 0 ]]; then
        echo "Detected missing legacy build tree components."
        echo "Try: ./scripts/bootstrap_legacy_tree.sh"
    fi
    echo "Fix failures first, then rerun ./scripts/check_installability.sh"
    exit 1
fi

echo
echo "Installability check passed."
