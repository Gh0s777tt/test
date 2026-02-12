# ISO Build and Boot Validation

## Summary

A bootable VantisOS ISO was built successfully and validated in QEMU in two modes:

1. Live mode boot smoke test.
2. Installer flow (`install /dev/vda --yes`) followed by boot from installed disk image.

- ISO path: `build/VantisOS-live.iso`
- Size: `70983680` bytes
- SHA-256:
  `94a6d883d7afa8ff279122fa2121aae3d02c371a90e16c916c2ae3e98224eb96`

## Build command

```bash
./scripts/build_iso.sh --output build/VantisOS-live.iso --run-qemu-smoke --run-installer-smoke
```

## Boot validation

Live smoke validation log:

- `analysis/benchmark_reproducibility/iso_smoke_boot_20260212T084514Z.log`

Installer phase validation log:

- `analysis/benchmark_reproducibility/iso_installer_phase_20260212T084814Z.log`

Installed disk boot validation log:

- `analysis/benchmark_reproducibility/iso_installed_boot_20260212T085334Z.log`

Installed disk reboot persistence validation log:

- `analysis/benchmark_reproducibility/iso_installed_reboot_20260212T085634Z.log`

Onboarding telemetry summary artifacts:

- `analysis/benchmark_reproducibility/iso_onboarding_telemetry_summary_20260212T085934Z.json`
- `analysis/benchmark_reproducibility/iso_onboarding_telemetry_summary_20260212T085934Z.md`

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
  - `onboard telemetry` (JSON snapshot + recent history tail),
- installer smoke auto-generates onboarding telemetry summary bundle (`json` + `md`) with aggregated lockout/import statistics.
- installed shell validates onboarding reset and re-apply flow:
  - `onboard reset --yes`,
  - `onboard export-encrypted /home/onboard_backup.enc --pass vantis123`,
  - repeated wrong-password imports trigger lockout/cooldown telemetry:
    - `failed to decrypt onboarding backup: integrity check failed`
    - `encrypted onboarding import temporarily locked; retry in <N>s`
    - `encrypted_import_failures=3` then reset to `encrypted_import_failures=0`
    - `encrypted_import_lock=active` / `encrypted_import_lock=inactive`,
    - `encrypted_import_blocked_until_unix=<ts>` while lockout active,
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
