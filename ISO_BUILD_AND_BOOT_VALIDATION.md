# ISO Build and Boot Validation

## Summary

A bootable VantisOS ISO was built successfully and validated in QEMU in two modes:

1. Live mode boot smoke test.
2. Installer flow (`install /dev/vda --yes`) followed by boot from installed disk image.

- ISO path: `build/VantisOS-live.iso`
- Size: `70983680` bytes
- SHA-256:
  `d3151249f400741d0e86a01b9c4f2fc501143a74a4666344539f07dc77b6aff0`

## Build command

```bash
./scripts/build_iso.sh --output build/VantisOS-live.iso --run-qemu-smoke --run-installer-smoke
```

## Boot validation

Live smoke validation log:

- `analysis/benchmark_reproducibility/iso_smoke_boot_20260212T194130Z.log`

Installer phase validation log:

- `analysis/benchmark_reproducibility/iso_installer_phase_20260212T194430Z.log`

Installed disk boot validation log:

- `analysis/benchmark_reproducibility/iso_installed_boot_20260212T194950Z.log`

Installed disk reboot persistence validation log:

- `analysis/benchmark_reproducibility/iso_installed_reboot_20260212T195250Z.log`

Onboarding telemetry summary artifacts:

- `analysis/benchmark_reproducibility/iso_onboarding_telemetry_summary_20260212T195550Z.json`
- `analysis/benchmark_reproducibility/iso_onboarding_telemetry_summary_20260212T195550Z.md`
- `analysis/benchmark_reproducibility/iso_onboarding_telemetry_rollup_20260212T195550Z.json`
- `analysis/benchmark_reproducibility/iso_onboarding_telemetry_rollup_20260212T195550Z.md`
- `analysis/benchmark_reproducibility/iso_onboarding_telemetry_rollup_latest.json`
- `analysis/benchmark_reproducibility/iso_onboarding_telemetry_rollup_latest.md`

Threshold gate dry-run:

```bash
./scripts/generate_iso_onboarding_telemetry_rollup.sh \
  --analysis-dir /workspace/analysis/benchmark_reproducibility \
  --window 30 \
  --max-lockout-ratio 0.5 \
  --max-mean-failures 2.0 \
  --require-final-source import_encrypted \
  --fail-on-threshold-breach
# exit code: 2 (expected for threshold breach)
```

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
- installer smoke auto-generates onboarding telemetry summary bundle (`json` + `md`) and rolling trend rollup (`json` + `md`) with aggregated lockout/import statistics.
- rollup artifacts include threshold policy + evaluation (`status=pass`) for:
  - max lockout ratio,
  - max mean failures,
  - required latest onboarding source,
  - optional fail-on-breach CI gate.
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

Build + smoke + enforced onboarding rollup threshold gate (CI-equivalent local run):

```bash
./scripts/run_iso_onboarding_ci_gate.sh
```

Gate thresholds are sourced from:

- `governance/performance/ISO_ONBOARDING_ROLLUP_GATE_POLICY.json` (profile `ci_default`)
- preflight validator: `scripts/validate_iso_onboarding_rollup_gate_policy.sh`

Tuned threshold gate example:

```bash
./scripts/run_iso_onboarding_ci_gate.sh \
  --window 30 \
  --max-lockout-ratio 1.0 \
  --max-mean-failures 3.0 \
  --require-final-source import_encrypted
```

Policy profile dry-run example:

```bash
./scripts/run_iso_onboarding_ci_gate.sh --policy-profile local_fast --dry-run
```

GitHub Actions workflow:

- `.github/workflows/iso-onboarding-rollup-gate.yml`
- manual dispatch input: `policy_profile` (`ci_default` or `local_fast`)
