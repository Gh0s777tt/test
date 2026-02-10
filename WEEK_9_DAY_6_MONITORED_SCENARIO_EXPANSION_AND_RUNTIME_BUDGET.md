# Week 9 Day 6: Monitored Scenario Expansion and Runtime Budget

**Date**: 2026-02-10  
**Status**: Completed  
**Scope**: Expand CI monitor coverage and enforce bounded runtime behavior for benchmark gating.

---

## 1) Objective

Day 6 focused on two goals:

1. Expand the non-blocking monitor stage beyond a single benchmark.
2. Keep CI benchmark execution bounded and predictable on shared runners.

This builds directly on Day 5 strict+monitor structure.

---

## 2) Implementation

### 2.1 `run_benchmark_ci_gate.sh` upgraded

Updated `scripts/run_benchmark_ci_gate.sh` with:

- **multi-monitor support**:
  - repeatable `--monitor-bench <name>`
  - `--monitor-benches <csv>`
  - duplicate monitor targets are deduplicated automatically
- **runtime controls**:
  - `--monitor-budget-seconds <n>` (global monitor wall-clock budget)
  - `--monitor-case-timeout-seconds <n>` (per-monitor timeout)
- **stronger reporting**:
  - stage/case status table (pass/failed/timeout/skipped/missing_report)
  - per-case duration and notes
  - consolidated metrics table with strict + monitor report rows
- **report detection hardening**:
  - fallback autodetection of generated report files by benchmark name and file mtime
  - avoids silent loss of metrics when direct log parsing misses the report path

### 2.2 CI workflow update

Updated `.github/workflows/ci.yml` benchmark gate invocation:

```bash
./scripts/run_benchmark_ci_gate.sh \
  --runs 2 \
  --strict-bench path_lookup_cache_benchmark \
  --strict-threshold-pct 50 \
  --monitor-bench timer_queue_benchmark \
  --monitor-bench directory_entry_cache_benchmark \
  --monitor-threshold-pct 25 \
  --monitor-budget-seconds 240 \
  --monitor-case-timeout-seconds 150 \
  --baseline-prefix ci_repro
```

### 2.3 Documentation update

Updated reproducibility profile docs to reflect:

- monitor stage expansion,
- runtime budget controls,
- CI-like local command pattern for reproducible validation.

---

## 3) Validation

### Command executed

```bash
./scripts/run_benchmark_ci_gate.sh \
  --runs 2 \
  --strict-bench path_lookup_cache_benchmark \
  --strict-threshold-pct 50 \
  --monitor-bench timer_queue_benchmark \
  --monitor-bench directory_entry_cache_benchmark \
  --monitor-threshold-pct 25 \
  --monitor-budget-seconds 240 \
  --monitor-case-timeout-seconds 150 \
  --baseline-prefix ci_repro
```

### Result

**PASS** (strict stage passed; monitor stage completed within configured budget constraints)

### Generated evidence

- `analysis/benchmark_reproducibility/ci_benchmark_gate_summary_20260210T111835Z.md`
- `analysis/benchmark_reproducibility/path_lookup_cache_benchmark_20260210T111835Z.md`
- `analysis/benchmark_reproducibility/timer_queue_benchmark_20260210T112029Z.md`
- `analysis/benchmark_reproducibility/directory_entry_cache_benchmark_20260210T112142Z.md`

### Summary snapshot

| Stage | Benchmark | Threshold | Status | Duration |
|---|---|---:|---|---:|
| strict | `path_lookup_cache_benchmark` | 50% | pass | 114s |
| monitor | `timer_queue_benchmark` | 25% | pass | 73s |
| monitor | `directory_entry_cache_benchmark` | 25% | pass | 139s |

| Report | Metrics | Above threshold | Median spread | Max spread |
|---|---:|---:|---:|---:|
| `path_lookup_cache_benchmark_20260210T111835Z.md` | 6 | 0 | 5.763% | 41.725% |
| `timer_queue_benchmark_20260210T112029Z.md` | 4 | 1 | 5.265% | 51.073% |
| `directory_entry_cache_benchmark_20260210T112142Z.md` | 7 | 0 | 0.741% | 3.558% |

---

## 4) Notes

- Timer queue monitor reports one metric over monitor threshold (`25%` spread).
  This is currently non-blocking by design and acts as drift signal for future tightening.
- Strict gate remains stable with conservative threshold suitable for shared CI runners.

---

## 5) Outcome

Day 6 deliverables completed:

- monitor scenario coverage expanded,
- runtime budget controls integrated,
- CI configuration aligned,
- validation evidence published.

Next suggested step: Week 9 Day 7, tighten noise controls around unstable monitor metrics and calibrate monitor thresholds per scenario class.

