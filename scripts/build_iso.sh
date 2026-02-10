#!/usr/bin/env bash
# Build UEFI ISO image for VantisOS.
# Usage:
#   ./scripts/build_iso.sh [output.iso]

set -euo pipefail

OUTPUT_ISO="${1:-VantisOS-0.5-alpha.iso}"
EFI_BIN="BOOTX64.EFI"
KERNEL_BIN="kernel.elf"
INITRD_BIN="initrd.img"
CFG_FILE="vantis.cfg"

for f in "$EFI_BIN" "$KERNEL_BIN" "$INITRD_BIN" "$CFG_FILE"; do
  if [[ ! -f "$f" ]]; then
    echo "Missing required input file: $f" >&2
    exit 1
  fi
done

if ! command -v xorriso >/dev/null 2>&1; then
  echo "xorriso is required but not installed." >&2
  exit 1
fi

STAGE_DIR="$(mktemp -d)"
trap 'rm -rf "$STAGE_DIR"' EXIT

mkdir -p "$STAGE_DIR/EFI/BOOT"
cp "$EFI_BIN" "$STAGE_DIR/EFI/BOOT/BOOTX64.EFI"
cp "$KERNEL_BIN" "$STAGE_DIR/kernel.elf"
cp "$INITRD_BIN" "$STAGE_DIR/initrd.img"
cp "$CFG_FILE" "$STAGE_DIR/vantis.cfg"

xorriso -as mkisofs \
  -efi-boot EFI/BOOT/BOOTX64.EFI \
  -no-emul-boot \
  -o "$OUTPUT_ISO" "$STAGE_DIR"

echo "Built ISO: $OUTPUT_ISO"
