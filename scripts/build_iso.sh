#!/usr/bin/env bash
# Build a bootable VantisOS live ISO (GRUB + Linux kernel + initramfs shell).
# Usage:
#   ./scripts/build_iso.sh
#   ./scripts/build_iso.sh --output build/VantisOS-live.iso --run-qemu-smoke

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
OUTPUT_ISO="$ROOT/build/VantisOS-live.iso"
KERNEL_IMAGE=""
RUN_QEMU_SMOKE=0
QEMU_TIMEOUT_SECONDS=45

usage() {
  cat <<'USAGE'
Usage: ./scripts/build_iso.sh [options]

Options:
  --output <path>            Output ISO path (default: build/VantisOS-live.iso)
  --kernel <path>            Kernel image path (default: /boot/vmlinuz or latest /boot/vmlinuz-*)
  --run-qemu-smoke           Boot ISO in QEMU and verify interactive shell prompt
  --qemu-timeout <seconds>   Timeout for smoke boot test (default: 45)
  -h, --help                 Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --output)
      OUTPUT_ISO="${2:-}"
      shift 2
      ;;
    --kernel)
      KERNEL_IMAGE="${2:-}"
      shift 2
      ;;
    --run-qemu-smoke)
      RUN_QEMU_SMOKE=1
      shift
      ;;
    --qemu-timeout)
      QEMU_TIMEOUT_SECONDS="${2:-}"
      shift 2
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

if [[ -z "$OUTPUT_ISO" ]]; then
  echo "Error: --output path cannot be empty" >&2
  exit 1
fi

if ! [[ "$QEMU_TIMEOUT_SECONDS" =~ ^[0-9]+$ ]] || (( QEMU_TIMEOUT_SECONDS < 5 )); then
  echo "Error: --qemu-timeout must be an integer >= 5" >&2
  exit 1
fi

require_cmd() {
  local cmd="$1"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Error: required command is missing: $cmd" >&2
    exit 1
  fi
}

require_cmd grub-mkrescue
require_cmd xorriso
require_cmd cpio
require_cmd gzip
require_cmd rustc
require_cmd busybox
if (( RUN_QEMU_SMOKE == 1 )); then
  require_cmd qemu-system-x86_64
  require_cmd timeout
fi

resolve_kernel() {
  if [[ -n "$KERNEL_IMAGE" ]]; then
    if [[ ! -f "$KERNEL_IMAGE" ]]; then
      echo "Error: kernel image not found: $KERNEL_IMAGE" >&2
      exit 1
    fi
    printf '%s\n' "$KERNEL_IMAGE"
    return 0
  fi

  if [[ -f "/boot/vmlinuz" ]]; then
    printf '%s\n' "/boot/vmlinuz"
    return 0
  fi

  shopt -s nullglob
  local candidates=(/boot/vmlinuz-*)
  shopt -u nullglob
  if (( ${#candidates[@]} == 0 )); then
    echo "Error: no kernel image found in /boot (expected /boot/vmlinuz*)" >&2
    echo "Hint: install linux-image-generic and rerun this script." >&2
    exit 1
  fi
  printf '%s\n' "${candidates[${#candidates[@]}-1]}"
}

KERNEL_PATH="$(resolve_kernel)"

if ! rustup target list --installed | rg -q '^x86_64-unknown-linux-musl$'; then
  echo "Installing Rust target x86_64-unknown-linux-musl..."
  rustup target add x86_64-unknown-linux-musl
fi

WORK_DIR="$ROOT/build/live_iso_work"
INITRAMFS_ROOT="$WORK_DIR/initramfs"
ISO_ROOT="$WORK_DIR/iso_root"
INITRD_PATH="$WORK_DIR/initrd.img"
GRUB_CFG_PATH="$ISO_ROOT/boot/grub/grub.cfg"

rm -rf "$WORK_DIR"
mkdir -p "$INITRAMFS_ROOT/bin" "$INITRAMFS_ROOT/dev" "$INITRAMFS_ROOT/proc" "$INITRAMFS_ROOT/sys"
mkdir -p "$ISO_ROOT/boot/grub"
mkdir -p "$(dirname "$OUTPUT_ISO")"
mkdir -p "$ANALYSIS_DIR"

echo "Building static userspace binaries..."
rustc --target x86_64-unknown-linux-musl -O "$ROOT/userspace/vantis.rs" -o "$INITRAMFS_ROOT/bin/vantis"
rustc --target x86_64-unknown-linux-musl -O "$ROOT/userspace/init.rs" -o "$INITRAMFS_ROOT/bin/vantis-init"

echo "Preparing initramfs root..."
cp "$(command -v busybox)" "$INITRAMFS_ROOT/bin/busybox"
for tool in sh mount swapoff echo cat ls mkdir mknod sleep uname dmesg; do
  ln -sf busybox "$INITRAMFS_ROOT/bin/$tool"
done

cat >"$INITRAMFS_ROOT/init" <<'INIT'
#!/bin/sh
set -eu

export PATH=/bin:/sbin

mount -t devtmpfs devtmpfs /dev 2>/dev/null || {
  mount -t tmpfs tmpfs /dev
  [ -c /dev/console ] || mknod -m 600 /dev/console c 5 1
  [ -c /dev/null ] || mknod -m 666 /dev/null c 1 3
}
mount -t proc proc /proc
mount -t sysfs sysfs /sys

echo "[VANTIS] initramfs boot sequence started"
exec /bin/vantis-init
INIT
chmod +x "$INITRAMFS_ROOT/init"

echo "Packing initrd..."
(
  cd "$INITRAMFS_ROOT"
  find . -print0 | cpio --null -o --format=newc | gzip -9 >"$INITRD_PATH"
)

if [[ -r "$KERNEL_PATH" ]]; then
  cp "$KERNEL_PATH" "$ISO_ROOT/boot/vmlinuz"
elif command -v sudo >/dev/null 2>&1 && sudo -n test -r "$KERNEL_PATH"; then
  sudo cp "$KERNEL_PATH" "$ISO_ROOT/boot/vmlinuz"
  sudo chown "$(id -u):$(id -g)" "$ISO_ROOT/boot/vmlinuz"
else
  echo "Error: kernel image is not readable: $KERNEL_PATH" >&2
  exit 1
fi

cp "$INITRD_PATH" "$ISO_ROOT/boot/initrd.img"

cat >"$GRUB_CFG_PATH" <<'GRUBCFG'
set timeout=0
set default=0
terminal_output console

menuentry "VantisOS Live" {
    linux /boot/vmlinuz console=ttyS0 loglevel=3 rdinit=/init
    initrd /boot/initrd.img
}
GRUBCFG

echo "Creating ISO image..."
grub-mkrescue -o "$OUTPUT_ISO" "$ISO_ROOT" >/dev/null
echo "Built ISO: $OUTPUT_ISO"

if (( RUN_QEMU_SMOKE == 1 )); then
  echo "Running QEMU smoke boot test..."
  LOG_PATH="$ANALYSIS_DIR/iso_smoke_boot_$(date -u +%Y%m%dT%H%M%SZ).log"
  OVMF_CODE=""
  OVMF_VARS=""
  for candidate in /usr/share/OVMF/OVMF_CODE.fd /usr/share/OVMF/OVMF_CODE_4M.fd; do
    if [[ -f "$candidate" ]]; then
      OVMF_CODE="$candidate"
      break
    fi
  done
  for candidate in /usr/share/OVMF/OVMF_VARS.fd /usr/share/OVMF/OVMF_VARS_4M.fd; do
    if [[ -f "$candidate" ]]; then
      OVMF_VARS="$candidate"
      break
    fi
  done

  QEMU_RC=0
  if [[ -n "$OVMF_CODE" && -n "$OVMF_VARS" ]]; then
    OVMF_VARS_COPY="$WORK_DIR/OVMF_VARS.fd"
    cp "$OVMF_VARS" "$OVMF_VARS_COPY"
    timeout "${QEMU_TIMEOUT_SECONDS}s" qemu-system-x86_64 \
      -machine q35 \
      -m 1024 \
      -serial mon:stdio \
      -display none \
      -no-reboot \
      -drive if=pflash,format=raw,readonly=on,file="$OVMF_CODE" \
      -drive if=pflash,format=raw,file="$OVMF_VARS_COPY" \
      -cdrom "$OUTPUT_ISO" >"$LOG_PATH" 2>&1 || QEMU_RC=$?
  else
    timeout "${QEMU_TIMEOUT_SECONDS}s" qemu-system-x86_64 \
      -m 1024 \
      -serial mon:stdio \
      -display none \
      -no-reboot \
      -cdrom "$OUTPUT_ISO" >"$LOG_PATH" 2>&1 || QEMU_RC=$?
  fi

  if [[ "$QEMU_RC" != "0" && "$QEMU_RC" != "124" ]]; then
    echo "Error: QEMU exited unexpectedly with code $QEMU_RC" >&2
    echo "Boot log: $LOG_PATH" >&2
    exit 1
  fi

  if rg -q '\[VANTIS\] WRAITH MODE ACTIVE|vantis> ' "$LOG_PATH"; then
    echo "QEMU smoke boot passed: interactive prompt detected"
    echo "Boot log: $LOG_PATH"
  else
    echo "Error: boot prompt was not detected in QEMU output" >&2
    echo "Boot log: $LOG_PATH" >&2
    exit 1
  fi
fi
