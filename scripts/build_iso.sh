#!/usr/bin/env bash
# Build a bootable VantisOS ISO with:
# - live mode
# - installer mode (writes bootable EFI+persist layout to a target disk)
# - onboarding telemetry summary + rolling rollup artifacts
#
# Usage:
#   ./scripts/build_iso.sh
#   ./scripts/build_iso.sh --output build/VantisOS-live.iso --run-qemu-smoke
#   ./scripts/build_iso.sh --run-installer-smoke

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
OUTPUT_ISO="$ROOT/build/VantisOS-live.iso"
KERNEL_IMAGE=""
RUN_QEMU_SMOKE=0
RUN_INSTALLER_SMOKE=0
QEMU_TIMEOUT_SECONDS=45
INSTALLER_TIMEOUT_SECONDS=120
ONBOARDING_ROLLUP_WINDOW=30
ONBOARDING_ROLLUP_MAX_LOCKOUT_RATIO=1.0
ONBOARDING_ROLLUP_MAX_MEAN_FAILURES=3.0
ONBOARDING_ROLLUP_REQUIRE_FINAL_SOURCE="import_encrypted"
ONBOARDING_ROLLUP_REQUIRE_FINAL_LAST_EVENT="guard_cleared"
ONBOARDING_ROLLUP_MIN_GUARD_CLEARED_RATIO=1.0
ENFORCE_ONBOARDING_ROLLUP_THRESHOLDS=0

usage() {
  cat <<'USAGE'
Usage: ./scripts/build_iso.sh [options]

Options:
  --output <path>              Output ISO path (default: build/VantisOS-live.iso)
  --kernel <path>              Kernel image path (default: /boot/vmlinuz or latest /boot/vmlinuz-*)
  --run-qemu-smoke             Boot ISO in QEMU and verify interactive shell prompt
  --run-installer-smoke        Run installer flow in QEMU and verify installed-disk boot + reboot persistence
                               Also generates onboarding telemetry summary + rollup artifacts
  --qemu-timeout <seconds>     Timeout for smoke boot test (default: 45)
  --installer-timeout <sec>    Timeout for installer session (default: 120)
  --onboarding-rollup-window <n>
                               Number of onboarding summary runs in trend rollup (default: 30)
  --onboarding-rollup-max-lockout-ratio <n>
                               Rollup threshold for lockout_run_ratio 0.0..1.0 (default: 1.0)
  --onboarding-rollup-max-mean-failures <n>
                               Rollup threshold for max_failures_mean (default: 3.0)
  --onboarding-rollup-require-final-source <name>
                               Rollup threshold for latest final source (default: import_encrypted)
  --onboarding-rollup-require-final-last-event <name>
                               Rollup threshold for latest last_event (default: guard_cleared)
  --onboarding-rollup-min-guard-cleared-ratio <n>
                               Rollup threshold for minimum guard_cleared run ratio 0.0..1.0 (default: 1.0)
  --enforce-onboarding-rollup-thresholds
                               Fail build if rollup threshold evaluation reports breaches
  -h, --help                   Show this help
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
    --run-installer-smoke)
      RUN_INSTALLER_SMOKE=1
      shift
      ;;
    --qemu-timeout)
      QEMU_TIMEOUT_SECONDS="${2:-}"
      shift 2
      ;;
    --installer-timeout)
      INSTALLER_TIMEOUT_SECONDS="${2:-}"
      shift 2
      ;;
    --onboarding-rollup-window)
      ONBOARDING_ROLLUP_WINDOW="${2:-}"
      shift 2
      ;;
    --onboarding-rollup-max-lockout-ratio)
      ONBOARDING_ROLLUP_MAX_LOCKOUT_RATIO="${2:-}"
      shift 2
      ;;
    --onboarding-rollup-max-mean-failures)
      ONBOARDING_ROLLUP_MAX_MEAN_FAILURES="${2:-}"
      shift 2
      ;;
    --onboarding-rollup-require-final-source)
      ONBOARDING_ROLLUP_REQUIRE_FINAL_SOURCE="${2:-}"
      shift 2
      ;;
    --onboarding-rollup-require-final-last-event)
      ONBOARDING_ROLLUP_REQUIRE_FINAL_LAST_EVENT="${2:-}"
      shift 2
      ;;
    --onboarding-rollup-min-guard-cleared-ratio)
      ONBOARDING_ROLLUP_MIN_GUARD_CLEARED_RATIO="${2:-}"
      shift 2
      ;;
    --enforce-onboarding-rollup-thresholds)
      ENFORCE_ONBOARDING_ROLLUP_THRESHOLDS=1
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

if [[ -z "$OUTPUT_ISO" ]]; then
  echo "Error: --output path cannot be empty" >&2
  exit 1
fi

if ! [[ "$QEMU_TIMEOUT_SECONDS" =~ ^[0-9]+$ ]] || (( QEMU_TIMEOUT_SECONDS < 5 )); then
  echo "Error: --qemu-timeout must be an integer >= 5" >&2
  exit 1
fi

if ! [[ "$INSTALLER_TIMEOUT_SECONDS" =~ ^[0-9]+$ ]] || (( INSTALLER_TIMEOUT_SECONDS < 20 )); then
  echo "Error: --installer-timeout must be an integer >= 20" >&2
  exit 1
fi

if ! [[ "$ONBOARDING_ROLLUP_WINDOW" =~ ^[0-9]+$ ]] || (( ONBOARDING_ROLLUP_WINDOW < 1 )); then
  echo "Error: --onboarding-rollup-window must be an integer >= 1" >&2
  exit 1
fi

require_cmd() {
  local cmd="$1"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Error: required command is missing: $cmd" >&2
    echo "Hint: install host deps via ./scripts/install_iso_build_deps_ubuntu.sh --with-qemu" >&2
    exit 1
  fi
}

require_cmd grub-mkrescue
require_cmd grub-mkstandalone
require_cmd xorriso
require_cmd cpio
require_cmd gzip
require_cmd mkfs.vfat
require_cmd mkfs.ext4
require_cmd mmd
require_cmd mcopy
require_cmd parted
require_cmd rustc
require_cmd busybox
require_cmd rg
require_cmd python3
if (( RUN_QEMU_SMOKE == 1 || RUN_INSTALLER_SMOKE == 1 )); then
  require_cmd qemu-system-x86_64
  require_cmd timeout
fi
if (( RUN_INSTALLER_SMOKE == 1 )); then
  require_cmd qemu-img
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

copy_readable_file() {
  local src="$1"
  local dst="$2"
  if [[ -r "$src" ]]; then
    cp "$src" "$dst"
    return 0
  fi
  if command -v sudo >/dev/null 2>&1 && sudo -n test -r "$src"; then
    sudo cp "$src" "$dst"
    sudo chown "$(id -u):$(id -g)" "$dst"
    return 0
  fi
  echo "Error: file is not readable: $src" >&2
  exit 1
}

pack_initrd() {
  local source_root="$1"
  local out_path="$2"
  (
    cd "$source_root"
    find . -print0 | cpio --null -o --format=newc | gzip -9 >"$out_path"
  )
}

build_installed_system_image() {
  local raw_img="$1"
  local gz_img="$2"
  local stamp_file="$WORK_DIR/install_stamp.txt"
  local esp_img="$WORK_DIR/esp_partition.img"
  local data_img="$WORK_DIR/data_partition.img"
  local disk_bytes=$((512 * 1024 * 1024))
  local sector_size=512
  local esp_start=2048
  local esp_sectors=262144
  local data_start=$((esp_start + esp_sectors))
  local data_sectors=$(((disk_bytes / sector_size) - data_start))
  local esp_bytes=$((esp_sectors * sector_size))
  local data_bytes=$((data_sectors * sector_size))

  rm -f "$raw_img" "$gz_img" "$esp_img" "$data_img"
  truncate -s "$disk_bytes" "$raw_img"

  parted -s "$raw_img" unit s mklabel msdos
  parted -s "$raw_img" unit s mkpart primary fat32 "${esp_start}s" "$((esp_start + esp_sectors - 1))s"
  parted -s "$raw_img" set 1 boot on
  parted -s "$raw_img" unit s mkpart primary ext4 "${data_start}s" "$((data_start + data_sectors - 1))s"

  truncate -s "$esp_bytes" "$esp_img"
  mkfs.vfat -F 32 -n VANTIS_BOOT "$esp_img" >/dev/null

  mmd -i "$esp_img" ::/EFI
  mmd -i "$esp_img" ::/EFI/BOOT
  mcopy -i "$esp_img" "$INSTALLED_EFI_PATH" ::/EFI/BOOT/BOOTX64.EFI
  mcopy -i "$esp_img" "$KERNEL_PAYLOAD_PATH" ::/vmlinuz
  mcopy -i "$esp_img" "$STAGE1_INITRD_PATH" ::/initrd.img
  mcopy -i "$esp_img" "$INSTALLED_RUNTIME_GRUB_CFG_PATH" ::/EFI/BOOT/grub.cfg

  printf 'VantisOS install payload generated on %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >"$stamp_file"
  mcopy -i "$esp_img" "$stamp_file" ::/install_stamp.txt

  truncate -s "$data_bytes" "$data_img"
  mkfs.ext4 -F -L VANTIS_DATA -O ^64bit,^metadata_csum "$data_img" >/dev/null

  dd if="$esp_img" of="$raw_img" bs="$sector_size" seek="$esp_start" conv=notrunc status=none
  dd if="$data_img" of="$raw_img" bs="$sector_size" seek="$data_start" conv=notrunc status=none

  gzip -9 -c "$raw_img" >"$gz_img"
}

KERNEL_PATH="$(resolve_kernel)"

if ! rustup target list --installed | rg -q '^x86_64-unknown-linux-musl$'; then
  echo "Installing Rust target x86_64-unknown-linux-musl..."
  rustup target add x86_64-unknown-linux-musl
fi

WORK_DIR="$ROOT/build/live_iso_work"
INITRAMFS_ROOT="$WORK_DIR/initramfs"
ISO_ROOT="$WORK_DIR/iso_root"
STAGE1_INITRD_PATH="$WORK_DIR/initrd_installed.img"
FINAL_INITRD_PATH="$WORK_DIR/initrd_live.img"
KERNEL_PAYLOAD_PATH="$WORK_DIR/vmlinuz_payload"
SYSTEM_DISK_RAW_PATH="$WORK_DIR/vantis_system_disk.img"
SYSTEM_DISK_GZ_PATH="$WORK_DIR/vantis_system_disk.img.gz"
GRUB_CFG_PATH="$ISO_ROOT/boot/grub/grub.cfg"
INSTALLED_GRUB_CFG_PATH="$WORK_DIR/grub_installed.cfg"
INSTALLED_RUNTIME_GRUB_CFG_PATH="$WORK_DIR/grub_installed_runtime.cfg"
INSTALLED_EFI_PATH="$WORK_DIR/BOOTX64.EFI"

rm -rf "$WORK_DIR"
mkdir -p "$INITRAMFS_ROOT/bin" "$INITRAMFS_ROOT/dev" "$INITRAMFS_ROOT/proc" "$INITRAMFS_ROOT/sys" "$INITRAMFS_ROOT/mnt"
mkdir -p "$ISO_ROOT/boot/grub"
mkdir -p "$(dirname "$OUTPUT_ISO")"
mkdir -p "$ANALYSIS_DIR"

copy_readable_file "$KERNEL_PATH" "$KERNEL_PAYLOAD_PATH"

echo "Building static userspace binaries..."
rustc --target x86_64-unknown-linux-musl -O "$ROOT/userspace/vantis.rs" -o "$INITRAMFS_ROOT/bin/vantis"
rustc --target x86_64-unknown-linux-musl -O "$ROOT/userspace/init.rs" -o "$INITRAMFS_ROOT/bin/vantis-init"

echo "Preparing initramfs root..."
cp "$(command -v busybox)" "$INITRAMFS_ROOT/bin/busybox"
for tool in sh mount umount swapoff echo cat ls mkdir mknod sleep uname dmesg dd fdisk mkdosfs mke2fs partprobe blockdev sync findfs cp chmod rm reboot mdev; do
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

MODE="live"
for token in $(cat /proc/cmdline); do
  case "$token" in
    vantis.mode=*)
      MODE="${token#vantis.mode=}"
      ;;
  esac
done

echo "[VANTIS] initramfs boot sequence started (mode=$MODE)"
exec /bin/vantis-init
INIT
chmod +x "$INITRAMFS_ROOT/init"

cat >"$INITRAMFS_ROOT/bin/vantis-installer" <<'INSTALLER'
#!/bin/sh
set -eu

TARGET="${1:-/dev/vda}"
CONFIRM="${2:-}"

if [ "$CONFIRM" != "--yes" ]; then
  echo "Usage: install <target-device> --yes"
  echo "Example: install /dev/vda --yes"
  echo "Warning: this destroys all data on the target device."
  exit 1
fi

if [ ! -b "$TARGET" ]; then
  echo "[VANTIS] target block device not found: $TARGET"
  exit 1
fi

if [ ! -f /boot_payload/system_disk.img.gz ]; then
  echo "[VANTIS] installer payload missing: /boot_payload/system_disk.img.gz"
  exit 1
fi

echo "[VANTIS] writing prebuilt system image to $TARGET"
busybox gzip -dc /boot_payload/system_disk.img.gz | dd of="$TARGET" bs=4M conv=fsync
sync
echo "[VANTIS] Installation complete. Reboot and boot from target disk."
INSTALLER
chmod +x "$INITRAMFS_ROOT/bin/vantis-installer"

echo "Packing installed-system initrd..."
pack_initrd "$INITRAMFS_ROOT" "$STAGE1_INITRD_PATH"

echo "Generating standalone UEFI bootloader payload..."
cat >"$INSTALLED_GRUB_CFG_PATH" <<'INSTGRUB'
set timeout=0
set default=0
terminal_output console
insmod part_msdos
insmod fat
set root=(hd0,msdos1)

menuentry "VantisOS Installed" {
    linux /vmlinuz console=ttyS0 loglevel=3 rdinit=/init vantis.mode=installed vantis.persist=LABEL=VANTIS_DATA
    initrd /initrd.img
}
INSTGRUB
cp "$INSTALLED_GRUB_CFG_PATH" "$INSTALLED_RUNTIME_GRUB_CFG_PATH"
grub-mkstandalone \
  -O x86_64-efi \
  --modules="part_msdos fat normal linux search" \
  -o "$INSTALLED_EFI_PATH" \
  "boot/grub/grub.cfg=$INSTALLED_GRUB_CFG_PATH" >/dev/null

echo "Building preinstalled disk image payload..."
build_installed_system_image "$SYSTEM_DISK_RAW_PATH" "$SYSTEM_DISK_GZ_PATH"

echo "Embedding installer payload into live initrd..."
mkdir -p "$INITRAMFS_ROOT/boot_payload"
cp "$KERNEL_PAYLOAD_PATH" "$INITRAMFS_ROOT/boot_payload/vmlinuz"
cp "$STAGE1_INITRD_PATH" "$INITRAMFS_ROOT/boot_payload/initrd.img"
cp "$INSTALLED_EFI_PATH" "$INITRAMFS_ROOT/boot_payload/BOOTX64.EFI"
cp "$SYSTEM_DISK_GZ_PATH" "$INITRAMFS_ROOT/boot_payload/system_disk.img.gz"

echo "Packing live initrd..."
pack_initrd "$INITRAMFS_ROOT" "$FINAL_INITRD_PATH"

cp "$KERNEL_PAYLOAD_PATH" "$ISO_ROOT/boot/vmlinuz"
cp "$FINAL_INITRD_PATH" "$ISO_ROOT/boot/initrd.img"

cat >"$GRUB_CFG_PATH" <<'GRUBCFG'
set timeout=5
set default=0
terminal_output console

menuentry "VantisOS Live" {
    linux /boot/vmlinuz console=ttyS0 loglevel=3 rdinit=/init vantis.mode=live
    initrd /boot/initrd.img
}

menuentry "VantisOS Installer" {
    linux /boot/vmlinuz console=ttyS0 loglevel=3 rdinit=/init vantis.mode=installer
    initrd /boot/initrd.img
}
GRUBCFG

echo "Creating ISO image..."
grub-mkrescue -o "$OUTPUT_ISO" "$ISO_ROOT" >/dev/null
echo "Built ISO: $OUTPUT_ISO"

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

qemu_boot_capture() {
  local timeout_seconds="$1"
  local log_path="$2"
  shift 2
  local rc=0
  if [[ -n "$OVMF_CODE" && -n "$OVMF_VARS" ]]; then
    local ovmf_vars_copy="$WORK_DIR/OVMF_VARS_$(date -u +%Y%m%dT%H%M%SZ%N).fd"
    cp "$OVMF_VARS" "$ovmf_vars_copy"
    timeout "${timeout_seconds}s" qemu-system-x86_64 \
      -machine q35 \
      -m 1024 \
      -serial mon:stdio \
      -display none \
      -no-reboot \
      -drive if=pflash,format=raw,readonly=on,file="$OVMF_CODE" \
      -drive if=pflash,format=raw,file="$ovmf_vars_copy" \
      "$@" >"$log_path" 2>&1 || rc=$?
  else
    timeout "${timeout_seconds}s" qemu-system-x86_64 \
      -m 1024 \
      -serial mon:stdio \
      -display none \
      -no-reboot \
      "$@" >"$log_path" 2>&1 || rc=$?
  fi
  return "$rc"
}

generate_onboarding_telemetry_summary() {
  local boot_log="$1"
  local reboot_log="$2"
  local stamp
  stamp="$(date -u +%Y%m%dT%H%M%SZ)"
  local json_path="$ANALYSIS_DIR/iso_onboarding_telemetry_summary_${stamp}.json"
  local md_path="$ANALYSIS_DIR/iso_onboarding_telemetry_summary_${stamp}.md"

  python3 - "$boot_log" "$reboot_log" "$json_path" "$md_path" <<'PY'
import json
import re
import sys
from pathlib import Path

boot_log_path, reboot_log_path, json_path, md_path = sys.argv[1:]

def parse_log(path: str) -> dict:
    text = Path(path).read_text(errors="replace")
    failures = [int(m.group(1)) for m in re.finditer(r"encrypted_import_failures=(\d+)", text)]
    blocked_until = [int(m.group(1)) for m in re.finditer(r"encrypted_import_blocked_until_unix=(\d+)", text)]
    lock_states = re.findall(r"encrypted_import_lock=(active|inactive)", text)
    lock_remaining = [int(m.group(1)) for m in re.finditer(r"encrypted_import_lock_seconds_remaining=(\d+)", text)]
    onboarding_sources = re.findall(r"onboarding_source=([a-zA-Z0-9_]+)", text)
    history_events = re.findall(r"event=([a-z_]+)", text)
    telemetry_last_events = re.findall(r'"last_event": "([^"]+)"', text)
    return {
        "path": path,
        "integrity_failures": len(re.findall(r"integrity check failed", text)),
        "failed_attempt_messages": len(re.findall(r"failed to decrypt onboarding backup: integrity check failed", text)),
        "lockout_messages": len(re.findall(r"temporarily locked; retry in", text)),
        "max_failures_observed": max(failures) if failures else 0,
        "blocked_until_unix_max": max(blocked_until) if blocked_until else 0,
        "lock_state_active_observed": "active" in lock_states,
        "lock_state_inactive_observed": "inactive" in lock_states,
        "max_lock_seconds_remaining": max(lock_remaining) if lock_remaining else 0,
        "onboarding_sources_observed": onboarding_sources,
        "final_onboarding_source": onboarding_sources[-1] if onboarding_sources else "unknown",
        "history_events_observed": sorted(set(history_events)),
        "history_event_count": len(history_events),
        "telemetry_last_events_observed": telemetry_last_events,
        "telemetry_last_event_final": telemetry_last_events[-1] if telemetry_last_events else "unknown",
    }

boot = parse_log(boot_log_path)
reboot = parse_log(reboot_log_path)
summary = {
    "schema": "vantis.iso.onboarding_telemetry_summary.v1",
    "generated_unix": int(Path(boot_log_path).stat().st_mtime),
    "boot_log": boot,
    "reboot_log": reboot,
    "aggregate": {
        "lockout_triggered": boot["lockout_messages"] > 0 or reboot["lockout_messages"] > 0,
        "max_failures_observed": max(boot["max_failures_observed"], reboot["max_failures_observed"]),
        "final_onboarding_source": reboot["final_onboarding_source"],
        "final_telemetry_last_event": reboot["telemetry_last_event_final"],
        "history_contains_lockout": "lockout_activated" in set(boot["history_events_observed"]) | set(reboot["history_events_observed"]),
        "history_contains_guard_cleared": "guard_cleared" in set(boot["history_events_observed"]) | set(reboot["history_events_observed"]),
    },
}

Path(json_path).write_text(json.dumps(summary, indent=2) + "\n")

md_lines = [
    "# ISO Onboarding Telemetry Summary",
    "",
    f"- Boot log: `{boot_log_path}`",
    f"- Reboot log: `{reboot_log_path}`",
    "",
    "## Aggregate",
    "",
    f"- Lockout triggered: `{summary['aggregate']['lockout_triggered']}`",
    f"- Max failures observed: `{summary['aggregate']['max_failures_observed']}`",
    f"- Final onboarding source: `{summary['aggregate']['final_onboarding_source']}`",
    f"- Final telemetry last_event: `{summary['aggregate']['final_telemetry_last_event']}`",
    f"- History contains `lockout_activated`: `{summary['aggregate']['history_contains_lockout']}`",
    f"- History contains `guard_cleared`: `{summary['aggregate']['history_contains_guard_cleared']}`",
    "",
    "## Boot log telemetry",
    "",
    f"- failed decrypt messages: `{boot['failed_attempt_messages']}`",
    f"- lockout messages: `{boot['lockout_messages']}`",
    f"- max failures observed: `{boot['max_failures_observed']}`",
    f"- final onboarding source: `{boot['final_onboarding_source']}`",
    f"- final telemetry last_event: `{boot['telemetry_last_event_final']}`",
    "",
    "## Reboot log telemetry",
    "",
    f"- failed decrypt messages: `{reboot['failed_attempt_messages']}`",
    f"- lockout messages: `{reboot['lockout_messages']}`",
    f"- max failures observed: `{reboot['max_failures_observed']}`",
    f"- final onboarding source: `{reboot['final_onboarding_source']}`",
    f"- final telemetry last_event: `{reboot['telemetry_last_event_final']}`",
    "",
]
Path(md_path).write_text("\n".join(md_lines))
PY

  echo "Onboarding telemetry summary JSON: $json_path"
  echo "Onboarding telemetry summary Markdown: $md_path"
}

if (( RUN_QEMU_SMOKE == 1 )); then
  echo "Running QEMU smoke boot test..."
  LOG_PATH="$ANALYSIS_DIR/iso_smoke_boot_$(date -u +%Y%m%dT%H%M%SZ).log"
  BOOT_RC=0
  qemu_boot_capture "$QEMU_TIMEOUT_SECONDS" "$LOG_PATH" -cdrom "$OUTPUT_ISO" || BOOT_RC=$?
  if [[ "$BOOT_RC" != "0" && "$BOOT_RC" != "124" ]]; then
    echo "Error: QEMU exited unexpectedly with code $BOOT_RC" >&2
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

if (( RUN_INSTALLER_SMOKE == 1 )); then
  echo "Running QEMU installer smoke test..."
  INSTALL_DISK="$WORK_DIR/installer_target.qcow2"
  qemu-img create -f qcow2 "$INSTALL_DISK" 2G >/dev/null

  INSTALL_LOG="$ANALYSIS_DIR/iso_installer_phase_$(date -u +%Y%m%dT%H%M%SZ).log"
  INSTALL_RC=0
  if [[ -n "$OVMF_CODE" && -n "$OVMF_VARS" ]]; then
    OVMF_VARS_COPY="$WORK_DIR/OVMF_VARS_INSTALL.fd"
    cp "$OVMF_VARS" "$OVMF_VARS_COPY"
    {
      sleep 18
      echo ""
      sleep 2
      echo "install /dev/vda --yes"
      sleep 35
    } | timeout "${INSTALLER_TIMEOUT_SECONDS}s" qemu-system-x86_64 \
      -machine q35 \
      -m 1024 \
      -serial mon:stdio \
      -display none \
      -no-reboot \
      -drive if=pflash,format=raw,readonly=on,file="$OVMF_CODE" \
      -drive if=pflash,format=raw,file="$OVMF_VARS_COPY" \
      -drive file="$INSTALL_DISK",if=virtio,format=qcow2 \
      -cdrom "$OUTPUT_ISO" >"$INSTALL_LOG" 2>&1 || INSTALL_RC=$?
  else
    {
      sleep 18
      echo ""
      sleep 2
      echo "install /dev/vda --yes"
      sleep 35
    } | timeout "${INSTALLER_TIMEOUT_SECONDS}s" qemu-system-x86_64 \
      -m 1024 \
      -serial mon:stdio \
      -display none \
      -no-reboot \
      -drive file="$INSTALL_DISK",if=virtio,format=qcow2 \
      -cdrom "$OUTPUT_ISO" >"$INSTALL_LOG" 2>&1 || INSTALL_RC=$?
  fi

  if [[ "$INSTALL_RC" != "0" && "$INSTALL_RC" != "124" ]]; then
    echo "Error: installer phase failed (qemu code=$INSTALL_RC)" >&2
    echo "Installer log: $INSTALL_LOG" >&2
    exit 1
  fi
  if ! rg -q '\[VANTIS\] Installation complete' "$INSTALL_LOG"; then
    echo "Error: installer completion marker not found in installer log" >&2
    echo "Installer log: $INSTALL_LOG" >&2
    exit 1
  fi

  BOOT_LOG="$ANALYSIS_DIR/iso_installed_boot_$(date -u +%Y%m%dT%H%M%SZ).log"
  BOOT_RC=0
  if [[ -n "$OVMF_CODE" && -n "$OVMF_VARS" ]]; then
    OVMF_VARS_COPY_BOOT="$WORK_DIR/OVMF_VARS_INSTALL_BOOT.fd"
    cp "$OVMF_VARS" "$OVMF_VARS_COPY_BOOT"
    {
      sleep 14
      echo "firstboot"
      sleep 1
      echo "config show"
      sleep 1
      echo "onboard"
      sleep 1
      echo "vantis-lab"
      sleep 1
      echo "operator"
      sleep 1
      echo "wraith"
      sleep 1
      echo "onboard status"
      sleep 1
      echo "config show"
      sleep 1
      echo "onboard export-encrypted /home/onboard_backup.enc --pass vantis123"
      sleep 1
      echo "onboard reset --yes"
      sleep 1
      echo "firstboot"
      sleep 1
      echo "onboard import-encrypted /home/onboard_backup.enc --pass badpass"
      sleep 1
      echo "onboard import-encrypted /home/onboard_backup.enc --pass badpass"
      sleep 1
      echo "onboard import-encrypted /home/onboard_backup.enc --pass badpass"
      sleep 1
      echo "onboard import-encrypted /home/onboard_backup.enc --pass badpass"
      sleep 1
      echo "onboard status"
      sleep 1
      echo "onboard telemetry"
      sleep 1
      echo "firstboot"
      sleep 10
      echo "onboard import-encrypted /home/onboard_backup.enc --pass vantis123"
      sleep 1
      echo "onboard status"
      sleep 1
      echo "onboard telemetry"
      sleep 1
      echo "config show"
      sleep 1
      echo "firstboot"
      sleep 1
    } | timeout "${QEMU_TIMEOUT_SECONDS}s" qemu-system-x86_64 \
      -machine q35 \
      -m 1024 \
      -serial mon:stdio \
      -display none \
      -no-reboot \
      -drive if=pflash,format=raw,readonly=on,file="$OVMF_CODE" \
      -drive if=pflash,format=raw,file="$OVMF_VARS_COPY_BOOT" \
      -drive file="$INSTALL_DISK",if=virtio,format=qcow2 >"$BOOT_LOG" 2>&1 || BOOT_RC=$?
  else
    {
      sleep 14
      echo "firstboot"
      sleep 1
      echo "config show"
      sleep 1
      echo "onboard"
      sleep 1
      echo "vantis-lab"
      sleep 1
      echo "operator"
      sleep 1
      echo "wraith"
      sleep 1
      echo "onboard status"
      sleep 1
      echo "config show"
      sleep 1
      echo "onboard export-encrypted /home/onboard_backup.enc --pass vantis123"
      sleep 1
      echo "onboard reset --yes"
      sleep 1
      echo "firstboot"
      sleep 1
      echo "onboard import-encrypted /home/onboard_backup.enc --pass badpass"
      sleep 1
      echo "onboard import-encrypted /home/onboard_backup.enc --pass badpass"
      sleep 1
      echo "onboard import-encrypted /home/onboard_backup.enc --pass badpass"
      sleep 1
      echo "onboard import-encrypted /home/onboard_backup.enc --pass badpass"
      sleep 1
      echo "onboard status"
      sleep 1
      echo "onboard telemetry"
      sleep 1
      echo "firstboot"
      sleep 10
      echo "onboard import-encrypted /home/onboard_backup.enc --pass vantis123"
      sleep 1
      echo "onboard status"
      sleep 1
      echo "onboard telemetry"
      sleep 1
      echo "config show"
      sleep 1
      echo "firstboot"
      sleep 1
    } | timeout "${QEMU_TIMEOUT_SECONDS}s" qemu-system-x86_64 \
      -m 1024 \
      -serial mon:stdio \
      -display none \
      -no-reboot \
      -drive file="$INSTALL_DISK",if=virtio,format=qcow2 >"$BOOT_LOG" 2>&1 || BOOT_RC=$?
  fi
  if [[ "$BOOT_RC" != "0" && "$BOOT_RC" != "124" ]]; then
    echo "Error: installed-disk boot phase failed (qemu code=$BOOT_RC)" >&2
    echo "Installed boot log: $BOOT_LOG" >&2
    exit 1
  fi
  if rg -q '\[VANTIS\] WRAITH MODE ACTIVE|vantis> ' "$BOOT_LOG" \
    && rg -q '\[VANTIS\] persistent storage active:' "$BOOT_LOG" \
    && ! rg -q '\[VANTIS\] persistent storage unavailable; setup is volatile' "$BOOT_LOG" \
    && rg -q '\[VANTIS\] FIRST BOOT SETUP COMPLETE|\[VANTIS\] FIRST BOOT SETUP ALREADY COMPLETE' "$BOOT_LOG" \
    && rg -q 'first_boot: done' "$BOOT_LOG" \
    && rg -q 'onboarding: pending' "$BOOT_LOG" \
    && rg -q '\[VANTIS\] onboarding wizard started' "$BOOT_LOG" \
    && rg -q '\[VANTIS\] ONBOARDING COMPLETE' "$BOOT_LOG" \
    && rg -q 'onboarding_state=done' "$BOOT_LOG" \
    && rg -q 'onboarding_source=interactive' "$BOOT_LOG" \
    && rg -q '\[VANTIS\] ONBOARDING EXPORTED ENCRYPTED: /home/onboard_backup.enc' "$BOOT_LOG" \
    && rg -q '\[VANTIS\] ONBOARDING RESET' "$BOOT_LOG" \
    && rg -q 'failed to decrypt onboarding backup: integrity check failed' "$BOOT_LOG" \
    && rg -q 'encrypted onboarding import temporarily locked; retry in ' "$BOOT_LOG" \
    && rg -q 'encrypted_import_failures=3' "$BOOT_LOG" \
    && rg -q 'encrypted_import_lock=active' "$BOOT_LOG" \
    && rg -q 'encrypted_import_lock_seconds_remaining=' "$BOOT_LOG" \
    && rg -q 'encrypted_import_blocked_until_unix=' "$BOOT_LOG" \
    && rg -q 'encrypted_import_lock=inactive' "$BOOT_LOG" \
    && rg -q 'encrypted_import_failures=0' "$BOOT_LOG" \
    && rg -q 'telemetry_json_begin' "$BOOT_LOG" \
    && rg -q '"last_event": "blocked_attempt"' "$BOOT_LOG" \
    && rg -q '"last_event": "guard_cleared"' "$BOOT_LOG" \
    && rg -q 'event=lockout_activated' "$BOOT_LOG" \
    && rg -q '\[VANTIS\] ONBOARDING IMPORTED ENCRYPTED: /home/onboard_backup.enc' "$BOOT_LOG" \
    && rg -q 'onboarding_source=import_encrypted' "$BOOT_LOG" \
    && rg -q 'onboarding: done' "$BOOT_LOG" \
    && rg -q 'profile=core' "$BOOT_LOG" \
    && rg -q 'hostname=vantis-' "$BOOT_LOG" \
    && rg -q 'user=vantis' "$BOOT_LOG" \
    && rg -q 'hostname=vantis-lab' "$BOOT_LOG" \
    && rg -q 'user=operator' "$BOOT_LOG" \
    && rg -q 'profile=wraith' "$BOOT_LOG"; then
    echo "Installer smoke passed: installed disk booted to Vantis shell"
    echo "Installer log: $INSTALL_LOG"
    echo "Installed boot log: $BOOT_LOG"
  else
    echo "Error: installed disk did not satisfy first-boot validation checks" >&2
    echo "Installed boot log: $BOOT_LOG" >&2
    exit 1
  fi

  REBOOT_LOG="$ANALYSIS_DIR/iso_installed_reboot_$(date -u +%Y%m%dT%H%M%SZ).log"
  REBOOT_RC=0
  if [[ -n "$OVMF_CODE" && -n "$OVMF_VARS" ]]; then
    OVMF_VARS_COPY_REBOOT="$WORK_DIR/OVMF_VARS_INSTALL_REBOOT.fd"
    cp "$OVMF_VARS" "$OVMF_VARS_COPY_REBOOT"
    {
      sleep 14
      echo "firstboot"
      sleep 1
      echo "onboard status"
      sleep 1
      echo "onboard telemetry"
      sleep 1
      echo "config show"
      sleep 1
    } | timeout "${QEMU_TIMEOUT_SECONDS}s" qemu-system-x86_64 \
      -machine q35 \
      -m 1024 \
      -serial mon:stdio \
      -display none \
      -no-reboot \
      -drive if=pflash,format=raw,readonly=on,file="$OVMF_CODE" \
      -drive if=pflash,format=raw,file="$OVMF_VARS_COPY_REBOOT" \
      -drive file="$INSTALL_DISK",if=virtio,format=qcow2 >"$REBOOT_LOG" 2>&1 || REBOOT_RC=$?
  else
    {
      sleep 14
      echo "firstboot"
      sleep 1
      echo "onboard status"
      sleep 1
      echo "onboard telemetry"
      sleep 1
      echo "config show"
      sleep 1
    } | timeout "${QEMU_TIMEOUT_SECONDS}s" qemu-system-x86_64 \
      -m 1024 \
      -serial mon:stdio \
      -display none \
      -no-reboot \
      -drive file="$INSTALL_DISK",if=virtio,format=qcow2 >"$REBOOT_LOG" 2>&1 || REBOOT_RC=$?
  fi
  if [[ "$REBOOT_RC" != "0" && "$REBOOT_RC" != "124" ]]; then
    echo "Error: installed-disk reboot validation failed (qemu code=$REBOOT_RC)" >&2
    echo "Installed reboot log: $REBOOT_LOG" >&2
    exit 1
  fi
  if rg -q '\[VANTIS\] persistent storage active:' "$REBOOT_LOG" \
    && rg -q '\[VANTIS\] FIRST BOOT SETUP ALREADY COMPLETE' "$REBOOT_LOG" \
    && rg -q 'first_boot: done' "$REBOOT_LOG" \
    && rg -q 'onboarding_state=done' "$REBOOT_LOG" \
    && rg -q 'onboarding_source=import_encrypted' "$REBOOT_LOG" \
    && rg -q 'encrypted_import_failures=0' "$REBOOT_LOG" \
    && rg -q 'encrypted_import_lock=inactive' "$REBOOT_LOG" \
    && rg -q '"last_event": "guard_cleared"' "$REBOOT_LOG" \
    && rg -q 'event=lockout_activated' "$REBOOT_LOG" \
    && rg -q 'onboarding: done' "$REBOOT_LOG" \
    && rg -q 'hostname=vantis-lab' "$REBOOT_LOG" \
    && rg -q 'user=operator' "$REBOOT_LOG" \
    && rg -q 'profile=wraith' "$REBOOT_LOG"; then
    echo "Installer reboot validation passed: persisted config survived reboot"
    echo "Installed reboot log: $REBOOT_LOG"
  else
    echo "Error: installed reboot did not preserve expected persisted config" >&2
    echo "Installed reboot log: $REBOOT_LOG" >&2
    exit 1
  fi

  generate_onboarding_telemetry_summary "$BOOT_LOG" "$REBOOT_LOG"

  ROLLUP_SCRIPT="$ROOT/scripts/generate_iso_onboarding_telemetry_rollup.sh"
  if [[ -f "$ROLLUP_SCRIPT" ]]; then
    ROLLUP_CMD=(
      bash "$ROLLUP_SCRIPT"
      --analysis-dir "$ANALYSIS_DIR"
      --window "$ONBOARDING_ROLLUP_WINDOW"
      --max-lockout-ratio "$ONBOARDING_ROLLUP_MAX_LOCKOUT_RATIO"
      --max-mean-failures "$ONBOARDING_ROLLUP_MAX_MEAN_FAILURES"
      --min-guard-cleared-ratio "$ONBOARDING_ROLLUP_MIN_GUARD_CLEARED_RATIO"
    )
    if [[ -n "$ONBOARDING_ROLLUP_REQUIRE_FINAL_SOURCE" ]]; then
      ROLLUP_CMD+=(--require-final-source "$ONBOARDING_ROLLUP_REQUIRE_FINAL_SOURCE")
    fi
    if [[ -n "$ONBOARDING_ROLLUP_REQUIRE_FINAL_LAST_EVENT" ]]; then
      ROLLUP_CMD+=(--require-final-last-event "$ONBOARDING_ROLLUP_REQUIRE_FINAL_LAST_EVENT")
    fi
    if (( ENFORCE_ONBOARDING_ROLLUP_THRESHOLDS == 1 )); then
      ROLLUP_CMD+=(--fail-on-threshold-breach)
    fi
    "${ROLLUP_CMD[@]}"
  else
    echo "Warning: onboarding telemetry rollup script missing: $ROLLUP_SCRIPT" >&2
  fi
fi
