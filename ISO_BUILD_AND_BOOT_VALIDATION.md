# ISO Build and Boot Validation

## Summary

A bootable VantisOS ISO was built successfully and validated in QEMU in two modes:

1. Live mode boot smoke test.
2. Installer flow (`install /dev/vda --yes`) followed by boot from installed disk image.

- ISO path: `build/VantisOS-live.iso`
- Size: `70971392` bytes
- SHA-256:
  `c8ade5a9b4b99ced23cfa06a72bf43c9a88b99d6616e23e138d9fa1bb9afc7bb`

## Build command

```bash
./scripts/build_iso.sh --output build/VantisOS-live.iso --run-qemu-smoke --run-installer-smoke
```

## Boot validation

Live smoke validation log:

- `analysis/benchmark_reproducibility/iso_smoke_boot_20260212T021112Z.log`

Installer phase validation log:

- `analysis/benchmark_reproducibility/iso_installer_phase_20260212T021402Z.log`

Installed disk boot validation log:

- `analysis/benchmark_reproducibility/iso_installed_boot_20260212T021922Z.log`

Installed disk reboot persistence validation log:

- `analysis/benchmark_reproducibility/iso_installed_reboot_20260212T022212Z.log`

Interactive runtime validation log (live shell lifecycle):

- `analysis/benchmark_reproducibility/iso_interactive_test_20260211T205956Z.log`

Observed runtime behavior:

- initramfs boot starts correctly,
- shell prompt `vantis>` is reachable,
- `ai` command returns `Cortex: system ready (offline)`,
- `exit` no longer causes kernel panic; shell session is restarted by PID1,
- installer command writes a prebuilt bootable system image to target disk,
- installed disk boots to the same `vantis>` shell successfully,
- installed mode performs first-boot setup and reports completion telemetry,
- installed mode mounts persistent storage from `LABEL=VANTIS_DATA` (`/dev/vda2` in smoke run),
- installed shell validates first-boot and onboarding flow via:
  - `firstboot`
  - `config show`,
  - `onboard` (interactive prompts for hostname/user/profile),
  - `onboard status`,
- installed shell validates onboarding reset and re-apply flow:
  - `onboard reset --yes`,
  - `onboard export-encrypted /home/onboard_backup.enc --pass vantis123`,
  - repeated wrong-password imports trigger lockout/cooldown telemetry:
    - `failed to decrypt onboarding backup: integrity check failed`
    - `encrypted onboarding import temporarily locked; retry in <N>s`
    - `encrypted_import_lock=active` / `encrypted_import_lock=inactive`,
  - `onboard import-encrypted /home/onboard_backup.enc --pass vantis123`,
- second installed boot confirms persistence (`FIRST BOOT SETUP ALREADY COMPLETE`) and retained:
  - `hostname=vantis-lab`
  - `user=operator`
  - `profile=wraith`
  - onboarding state `done` with source `import_encrypted` (after reset/re-apply from backup).

## Reproduce

Build only:

```bash
./scripts/build_iso.sh --output build/VantisOS-live.iso
```

Build + smoke test:

```bash
./scripts/build_iso.sh --output build/VantisOS-live.iso --run-qemu-smoke
```

Build + live + installer smoke tests:

```bash
./scripts/build_iso.sh --output build/VantisOS-live.iso --run-qemu-smoke --run-installer-smoke
```
