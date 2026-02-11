# ISO Build and Boot Validation

## Summary

A bootable VantisOS live ISO was built successfully and validated in QEMU.

- ISO path: `build/VantisOS-live.iso`
- Size: `30212096` bytes
- SHA-256:
  `2dcc7aea0416abf7153a7d5f006779a761c7e3caf7fbc631954b1c5cd42c4d61`

## Build command

```bash
./scripts/build_iso.sh --output build/VantisOS-live.iso --run-qemu-smoke
```

## Boot validation

Smoke validation log:

- `analysis/benchmark_reproducibility/iso_smoke_boot_20260211T205902Z.log`

Interactive runtime validation log:

- `analysis/benchmark_reproducibility/iso_interactive_test_20260211T205956Z.log`

Observed runtime behavior:

- initramfs boot starts correctly,
- shell prompt `vantis>` is reachable,
- `ai` command returns `Cortex: system ready (offline)`,
- `exit` no longer causes kernel panic; shell session is restarted by PID1.

## Reproduce

Build only:

```bash
./scripts/build_iso.sh --output build/VantisOS-live.iso
```

Build + smoke test:

```bash
./scripts/build_iso.sh --output build/VantisOS-live.iso --run-qemu-smoke
```
