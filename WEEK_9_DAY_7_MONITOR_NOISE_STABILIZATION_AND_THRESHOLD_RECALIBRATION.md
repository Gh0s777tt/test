# Week 9 Day 7: Monitor Noise Stabilization and Threshold Recalibration

**Date**: 2026-02-10  
**Status**: Completed  
**Scope**: Reduce monitor-stage noise sensitivity and recalibrate per-scenario thresholds.

---

## 1) Objective

After Day 6, monitor-stage variance still produced occasional high spread outliers
(especially in timer queue sparse expiry cases). Day 7 targeted:

1. clearer drift signaling in monitor mode,
2. scenario-specific threshold calibration,
3. preserving strict gate behavior while reducing monitor false positives.

---

## 2) Implementation

### 2.1 `run_benchmark_ci_gate.sh` enhancements

Updated monitor handling with:

- **per-monitor threshold overrides**:
  - `--monitor-bench <name:pct>`
  - `--monitor-threshold <name:pct>`
- **default calibrated monitor threshold map**:
  - `timer_queue_benchmark` -> `60`
  - `directory_entry_cache_benchmark` -> `25`
- **drift classification**:
  - parses reproducibility report summary (`Metrics above spread threshold`)
  - marks monitor case as `drift` when threshold is exceeded
  - keeps monitor drift non-blocking but explicit in summary
- **summary quality improvements**:
  - adds case status counts (`pass`, `drift`, `timeout`, etc.)
  - preserves stage-specific thresholds in the case table
- **input validation hardening**:
  - numeric validation for threshold values (global and per-case overrides)

Strict stage remains blocking and unchanged in policy.

### 2.2 CI workflow recalibration

Updated benchmark gate invocation in `.github/workflows/ci.yml`:

```bash
./scripts/run_benchmark_ci_gate.sh \
  --runs 2 \
  --strict-bench path_lookup_cache_benchmark \
  --strict-threshold-pct 50 \
  --monitor-bench timer_queue_benchmark:60 \
  --monitor-bench directory_entry_cache_benchmark:25 \
  --monitor-budget-seconds 240 \
  --monitor-case-timeout-seconds 150 \
  --baseline-prefix ci_repro
```

### 2.3 Documentation alignment

Updated:

- `docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md`
- `README.md`

to reflect per-scenario monitor thresholds and drift semantics.

---

## 3) Validation

### Command executed

```bash
./scripts/run_benchmark_ci_gate.sh \
  --runs 2 \
  --strict-bench path_lookup_cache_benchmark \
  --strict-threshold-pct 50 \
  --monitor-bench timer_queue_benchmark:60 \
  --monitor-bench directory_entry_cache_benchmark:25 \
  --monitor-budget-seconds 240 \
  --monitor-case-timeout-seconds 150 \
  --baseline-prefix ci_repro
```

### Result

**PASS** (strict passed; monitor cases passed without drift status)

### Generated evidence

- `analysis/benchmark_reproducibility/ci_benchmark_gate_summary_20260210T143639Z.md`
- `analysis/benchmark_reproducibility/path_lookup_cache_benchmark_20260210T143639Z.md`
- `analysis/benchmark_reproducibility/timer_queue_benchmark_20260210T143840Z.md`
- `analysis/benchmark_reproducibility/directory_entry_cache_benchmark_20260210T143955Z.md`

### Summary snapshot

Case status:

| Stage | Benchmark | Threshold | Status | Duration |
|---|---|---:|---|---:|
| strict | `path_lookup_cache_benchmark` | 50% | pass | 121s |
| monitor | `timer_queue_benchmark` | 60% | pass | 75s |
| monitor | `directory_entry_cache_benchmark` | 25% | pass | 136s |

Reproducibility summary:

| Report | Metrics | Above threshold | Median spread | Max spread |
|---|---:|---:|---:|---:|
| `path_lookup_cache_benchmark_20260210T143639Z.md` | 6 | 0 | 6.687% | 30.318% |
| `timer_queue_benchmark_20260210T143840Z.md` | 4 | 0 | 3.739% | 7.940% |
| `directory_entry_cache_benchmark_20260210T143955Z.md` | 7 | 0 | 0.353% | 15.860% |

---

## 4) Outcome

Day 7 goals are complete:

- monitor noise signaling is explicit (`drift`) instead of silent variance,
- timer queue monitor threshold is recalibrated for shared-runner stability,
- strict gate remains unchanged and blocking,
- CI-like validation succeeded with clean repository verification.

Next suggested step: Week 9 Day 8, automate monitor-policy evolution
(e.g., threshold suggestions from rolling evidence summaries).

