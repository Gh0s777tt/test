#!/bin/bash
# Test Installer
# Testuje instalator w QEMU

set -e

echo "🧪 VantisOS - Installer Test"
echo "============================="

ISO_PATH="${1:-release/vantisos-1.4.0.iso}"

if [[ ! -f "$ISO_PATH" ]]; then
    echo "❌ ISO not found: $ISO_PATH"
    echo "Run 'make build' first"
    exit 1
fi

echo "Testing installer with ISO: $ISO_PATH"
echo ""

# Test w QEMU
qemu-system-x86_64 \
    -cdrom "$ISO_PATH" \
    -m 2G \
    -smp 2 \
    -enable-kvm \
    -cpu host \
    -nographic \
    -serial mon:stdio

echo ""
echo "✅ Installer test complete"
