#!/bin/bash
# Create Live USB
# Tworzy bootowalny USB z VantisOS

set -e

echo "USB 🔌 VantisOS - Create Live USB"
echo "================================="

ISO_PATH="${1:-release/vantisos-1.4.0.iso}"
USB_DEVICE="${2}"

if [[ ! -f "$ISO_PATH" ]]; then
    echo "❌ ISO not found: $ISO_PATH"
    exit 1
fi

if [[ -z "$USB_DEVICE" ]]; then
    echo "Usage: $0 <iso_path> <usb_device>"
    echo "Example: $0 release/vantisos-1.4.0.iso /dev/sdb"
    echo ""
    echo "Available devices:"
    lsblk -o NAME,SIZE,TYPE,MOUNTPOINT | grep -E "disk|part"
    exit 1
fi

echo "ISO: $ISO_PATH"
echo "USB: $USB_DEVICE"
echo ""

read -p "⚠️  This will erase all data on $USB_DEVICE. Continue? (y/n): " CONFIRM
if [[ "$CONFIRM" != "y" ]]; then
    echo "Cancelled"
    exit 0
fi

echo ""
echo "Writing ISO to USB..."
sudo dd if="$ISO_PATH" of="$USB_DEVICE" bs=4M status=progress conv=fsync
sync

echo ""
echo "✅ Live USB created successfully!"
echo "Boot from $USB_DEVICE to start VantisOS"
