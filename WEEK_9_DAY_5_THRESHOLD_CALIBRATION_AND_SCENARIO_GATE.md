# Week 9 Day 5: Threshold Calibration and Expanded Scenario Gate

**Date**: 2026-02-10  
**Status**: Completed  
**Scope**: CI gate threshold calibration and expansion to dual-stage (strict + monitor) benchmarking

---

## 1) Objective

Continue Week 9 tooling hardening by:

1. Calibrating practical spread thresholds for shared CI runners.
2. Expanding benchmark gate beyond one strict benchmark to include a scenario monitor benchmark.
3. Producing consolidated evidence from a CI-like local execution path.

---

## 2) Implementation

### 2.1 New CI gate orchestrator script

Added:

- `scripts/run_benchmark_ci_gate.sh`

Behavior:

1. **Strict stage** (blocking):
   - benchmark: `path_lookup_cache_benchmark`
   - threshold: configurable (`--strict-threshold-pct`, default 50)
   - strict mode enabled
2. **Monitor stage** (non-blocking):
   - benchmark: `timer_queue_benchmark`
   - threshold: configurable (`--monitor-threshold-pct`, default 25)
   - report-only drift visibility
3. Generates consolidated summary report:
   - `analysis/benchmark_reproducibility/ci_benchmark_gate_summary_<timestamp>.md`

### 2.2 CI workflow integration update

Updated:

- `.github/workflows/ci.yml`

`benchmark-reproducibility-gate` now runs:

```bash
./scripts/run_benchmark_ci_gate.sh \
  --runs 2 \
  --strict-bench path_lookup_cache_benchmark \
  --strict-threshold-pct 50 \
  --monitor-bench timer_queue_benchmark \
  --monitor-threshold-pct 25 \
  --baseline-prefix ci_repro
```

Artifact upload remains enabled for all generated markdown reports in:

- `analysis/benchmark_reproducibility/*.md`

### 2.3 Documentation updates

Updated:

- `docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md`
- `README.md`
- `docs/README.md`

to include the new CI gate runner and staged interpretation model.

---

## 3) Calibration notes

Observed during calibration:

- Strict 10% thresholds were too brittle on shared cloud runners.
- Path cache benchmark provided a stable strict candidate with conservative 50% guardrail.
- Timer queue benchmark was retained as monitor-stage signal to track drift trends without blocking merges.

This provides practical signal while avoiding excessive false-positive failures.

---

## 4) Validation run (CI-like local execution)

Executed:

```bash
./scripts/run_benchmark_ci_gate.sh \
  --runs 2 \
  --strict-bench path_lookup_cache_benchmark \
  --strict-threshold-pct 50 \
  --monitor-bench timer_queue_benchmark \
  --monitor-threshold-pct 25 \
  --baseline-prefix ci_repro
```

Result: **PASS**

Generated evidence:

- `analysis/benchmark_reproducibility/path_lookup_cache_benchmark_20260210T105039Z.md`
- `analysis/benchmark_reproducibility/timer_queue_benchmark_20260210T105237Z.md`
- `analysis/benchmark_reproducibility/ci_benchmark_gate_summary_20260210T105039Z.md`

Summary report highlights:

| Report | Metrics | Above threshold | Median spread | Max spread |
|---|---:|---:|---:|---:|
| `path_lookup_cache_benchmark_20260210T105039Z.md` | 6 | 0 | 2.292% | 11.533% |
| `timer_queue_benchmark_20260210T105237Z.md` | 4 | 0 | 1.872% | 2.951% |

---

## 5) Outcome

Week 9 Day 5 goals are complete:

- threshold calibration performed,
- expanded scenario-aware CI gate delivered,
- strict + monitor behavior validated locally,
- evidence artifacts and documentation published.

Next suggested step: Week 9 Day 6, expand monitored scenarios (e.g., IPC) with bounded runtime budgets.

