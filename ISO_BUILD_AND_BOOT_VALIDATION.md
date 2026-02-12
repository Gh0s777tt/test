# ISO Build and Boot Validation

## Summary

A bootable VantisOS ISO was built successfully and validated in QEMU in two modes:

1. Live mode boot smoke test.
2. Installer flow (`install /dev/vda --yes`) followed by boot from installed disk image.

- ISO path: `build/VantisOS-live.iso`
- Size: `70942720` bytes
- SHA-256:
  `bddc85f05538ba4898cecbbd29c463293781c67b9c1a41c4ec6cfc123ae80fd2`

## Build command

```bash
./scripts/build_iso.sh --output build/VantisOS-live.iso --run-qemu-smoke --run-installer-smoke
```

## Boot validation

Live smoke validation log:

- `analysis/benchmark_reproducibility/iso_smoke_boot_20260212T003135Z.log`

Installer phase validation log:

- `analysis/benchmark_reproducibility/iso_installer_phase_20260212T003325Z.log`

Installed disk boot validation log:

- `analysis/benchmark_reproducibility/iso_installed_boot_20260212T003745Z.log`

Installed disk reboot persistence validation log:

- `analysis/benchmark_reproducibility/iso_installed_reboot_20260212T003935Z.log`

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
- second installed boot confirms persistence (`FIRST BOOT SETUP ALREADY COMPLETE`) and retained:
  - `hostname=vantis-lab`
  - `user=operator`
  - `profile=wraith`
  - onboarding state `done` with source `interactive`.

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
