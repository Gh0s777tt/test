#!/usr/bin/env bash
# Install host dependencies required for ISO build and optional QEMU smoke tests.
# Usage:
#   ./scripts/install_iso_build_deps_ubuntu.sh
#   ./scripts/install_iso_build_deps_ubuntu.sh --with-qemu
#   ./scripts/install_iso_build_deps_ubuntu.sh --with-qemu --dry-run

set -euo pipefail

WITH_QEMU=0
DRY_RUN=0

usage() {
  cat <<'USAGE'
Usage: ./scripts/install_iso_build_deps_ubuntu.sh [options]

Options:
  --with-qemu     Include QEMU + OVMF packages for smoke/installer tests
  --dry-run       Print package plan without installing
  -h, --help      Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --with-qemu)
      WITH_QEMU=1
      shift
      ;;
    --dry-run)
      DRY_RUN=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      usage
      exit 1
      ;;
  esac
done

if ! command -v apt-get >/dev/null 2>&1; then
  echo "Error: apt-get not found. This installer currently supports Ubuntu/Debian hosts." >&2
  exit 1
fi

SUDO=()
if [[ "${EUID:-$(id -u)}" -ne 0 ]]; then
  if command -v sudo >/dev/null 2>&1; then
    SUDO=(sudo)
  else
    echo "Error: sudo is required to install packages as non-root user." >&2
    exit 1
  fi
fi

PACKAGES=(
  grub-common
  grub-pc-bin
  grub-efi-amd64-bin
  xorriso
  cpio
  gzip
  dosfstools
  e2fsprogs
  mtools
  parted
  busybox-static
  ripgrep
  python3
)

if (( WITH_QEMU == 1 )); then
  PACKAGES+=(
    qemu-system-x86
    qemu-utils
    ovmf
  )
fi

if (( DRY_RUN == 1 )); then
  echo "Dry run: apt package plan"
  printf '  - %s\n' "${PACKAGES[@]}"
  exit 0
fi

echo "Installing ISO build dependencies..."
"${SUDO[@]}" apt-get update
"${SUDO[@]}" env DEBIAN_FRONTEND=noninteractive \
  apt-get install -y --no-install-recommends "${PACKAGES[@]}"
echo "Dependencies installed."
