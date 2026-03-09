#!/bin/bash
set -e

ISO=VantisOS-0.5-alpha.iso

mkdir -p iso/EFI/BOOT
cp BOOTX64.EFI iso/EFI/BOOT/
cp kernel.elf iso/
cp initrd.img iso/
cp vantis.cfg iso/

xorriso -as mkisofs \
  -efi-boot EFI/BOOT/BOOTX64.EFI \
  -no-emul-boot \
  -o $ISO iso/

echo "[+] Built $ISO"
