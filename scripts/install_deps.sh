#!/usr/bin/env bash
#
# Install local build dependencies for VantisOS development.
# Supports Debian/Ubuntu, Arch Linux, Fedora, and macOS.
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
KERNEL_TOOLCHAIN="${KERNEL_TOOLCHAIN:-nightly-2025-10-03}"

ASSUME_YES=false
MINIMAL=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        -y|--yes)
            ASSUME_YES=true
            shift
            ;;
        --minimal)
            MINIMAL=true
            shift
            ;;
        -h|--help)
            cat <<'USAGE'
Usage: ./scripts/install_deps.sh [options]

Options:
  -y, --yes      Non-interactive install where supported
  --minimal      Install only Rust + core build tools
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

info() { echo "[INFO] $*"; }
ok() { echo "[ OK ] $*"; }
warn() { echo "[WARN] $*"; }
fail() { echo "[FAIL] $*" >&2; exit 1; }

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

run_as_root() {
    if [[ "${EUID}" -eq 0 ]]; then
        "$@"
    elif command_exists sudo; then
        sudo "$@"
    else
        fail "sudo is required to install system packages."
    fi
}

install_rust_toolchain() {
    if command_exists rustup; then
        info "Rustup already installed. Updating stable toolchain."
        rustup update stable
        rustup default stable
        ok "Rust toolchain ready"
        return
    fi

    info "Installing rustup (stable toolchain)"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    # shellcheck disable=SC1091
    source "${HOME}/.cargo/env"
    ok "Rustup installed"
}

install_kernel_toolchain() {
    if ! command_exists rustup; then
        fail "rustup is required before installing kernel toolchain"
    fi

    info "Ensuring kernel toolchain ${KERNEL_TOOLCHAIN} with rust-src"
    rustup toolchain install "${KERNEL_TOOLCHAIN}" --component rust-src --profile minimal
    ok "Kernel toolchain ready"
}

install_debian() {
    local pkgs=(
        build-essential
        clang
        cmake
        curl
        dosfstools
        file
        genisoimage
        git
        mtools
        nasm
        pkg-config
        qemu-system-x86
        xorriso
    )

    if [[ "${MINIMAL}" == true ]]; then
        pkgs=(build-essential curl git pkg-config)
    fi

    info "Detected Debian/Ubuntu"
    local apt_flags=()
    if [[ "${ASSUME_YES}" == true ]]; then
        apt_flags=(-y)
    fi
    run_as_root apt-get update
    run_as_root apt-get install "${apt_flags[@]}" "${pkgs[@]}"
    ok "System packages installed (Debian/Ubuntu)"
}

install_arch() {
    local pkgs=(
        base-devel
        clang
        cmake
        curl
        dosfstools
        git
        libisoburn
        mtools
        nasm
        qemu-system-x86
    )

    if [[ "${MINIMAL}" == true ]]; then
        pkgs=(base-devel curl git)
    fi

    info "Detected Arch Linux"
    local pacman_flags=(--needed)
    if [[ "${ASSUME_YES}" == true ]]; then
        pacman_flags+=(-y --noconfirm)
    fi
    run_as_root pacman -S "${pacman_flags[@]}" "${pkgs[@]}"
    ok "System packages installed (Arch Linux)"
}

install_fedora() {
    local pkgs=(
        @development-tools
        clang
        cmake
        curl
        dosfstools
        genisoimage
        git
        libisoburn
        mtools
        nasm
        qemu-system-x86
    )

    if [[ "${MINIMAL}" == true ]]; then
        pkgs=(@development-tools curl git)
    fi

    info "Detected Fedora"
    local dnf_flags=()
    if [[ "${ASSUME_YES}" == true ]]; then
        dnf_flags=(-y)
    fi
    run_as_root dnf install "${dnf_flags[@]}" "${pkgs[@]}"
    ok "System packages installed (Fedora)"
}

install_macos() {
    if ! command_exists brew; then
        fail "Homebrew not found. Install from https://brew.sh and rerun."
    fi

    local pkgs=(cmake curl git nasm qemu xorriso)
    if [[ "${MINIMAL}" == true ]]; then
        pkgs=(curl git)
    fi

    info "Detected macOS"
    brew update
    brew install "${pkgs[@]}"
    ok "System packages installed (macOS)"
}

detect_and_install_system_packages() {
    if [[ "$(uname -s)" == "Darwin" ]]; then
        install_macos
        return
    fi

    if command_exists apt-get; then
        install_debian
    elif command_exists pacman; then
        install_arch
    elif command_exists dnf; then
        install_fedora
    else
        warn "Unsupported package manager. Install dependencies manually."
    fi
}

main() {
    info "Installing VantisOS dependencies"
    info "Repository: ${REPO_ROOT}"

    detect_and_install_system_packages
    install_rust_toolchain
    install_kernel_toolchain

    info "Dependency bootstrap completed."
    info "Recommended next step: ./scripts/check_installability.sh"
}

main "$@"
