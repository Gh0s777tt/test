# ISO Build and Boot Validation

## Summary

A bootable VantisOS ISO was built successfully and validated in QEMU in two modes:

1. Live mode boot smoke test.
2. Installer flow (`install /dev/vda --yes`) followed by boot from installed disk image.

- ISO path: `build/VantisOS-live.iso`
- Size: `70969344` bytes
- SHA-256:
  `c96e9b73e93dd14170a36d683197c4bded50fc39689ca3083934d890bb236df0`

## Build command

```bash
./scripts/build_iso.sh --output build/VantisOS-live.iso --run-qemu-smoke --run-installer-smoke
```

## Boot validation

Live smoke validation log:

- `analysis/benchmark_reproducibility/iso_smoke_boot_20260212T013108Z.log`

Installer phase validation log:

- `analysis/benchmark_reproducibility/iso_installer_phase_20260212T013328Z.log`

Installed disk boot validation log:

- `analysis/benchmark_reproducibility/iso_installed_boot_20260212T013808Z.log`

Installed disk reboot persistence validation log:

- `analysis/benchmark_reproducibility/iso_installed_reboot_20260212T014028Z.log`

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
  - failed import attempt with wrong password is detected (`integrity check failed`),
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
