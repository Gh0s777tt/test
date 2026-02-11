# ISO Build and Boot Validation

## Summary

A bootable VantisOS ISO was built successfully and validated in QEMU in two modes:

1. Live mode boot smoke test.
2. Installer flow (`install /dev/vda --yes`) followed by boot from installed disk image.

- ISO path: `build/VantisOS-live.iso`
- Size: `70590464` bytes
- SHA-256:
  `a9b2b665400a0de7dc53961753eee6249e53d4a1721e34146c14346f5c45907b`

## Build command

```bash
./scripts/build_iso.sh --output build/VantisOS-live.iso --run-qemu-smoke --run-installer-smoke
```

## Boot validation

Live smoke validation log:

- `analysis/benchmark_reproducibility/iso_smoke_boot_20260211T225939Z.log`

Installer phase validation log:

- `analysis/benchmark_reproducibility/iso_installer_phase_20260211T230054Z.log`

Installed disk boot validation log:

- `analysis/benchmark_reproducibility/iso_installed_boot_20260211T230444Z.log`

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
- installed shell validates first-boot status and generated config values via:
  - `firstboot`
  - `config show`,
  - `config set hostname vantis-lab`,
  - `config set user operator`,
  - `config set profile wraith`.

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
