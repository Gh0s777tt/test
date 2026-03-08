#!/bin/bash
set -e

echo "[+] Building kernel"
cargo build --release

echo "[+] Creating initrd"
mkdir -p initrd/bin
cp userspace/vantis initrd/bin/
cp userspace/init initrd/bin/

echo "[+] Creating image"
# placeholder for FAT32 / EFI image
