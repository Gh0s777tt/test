# Week 9 Day 8: Monitor Policy Automation from Rolling Evidence

**Date**: 2026-02-10  
**Status**: Completed  
**Scope**: Automate advisory threshold recommendations for monitor benchmarks using rolling reproducibility evidence.

---

## 1) Objective

After Day 7 threshold recalibration, Day 8 focused on policy automation:

1. derive monitor threshold recommendations from historical evidence,
2. publish machine-readable and human-readable outputs,
3. integrate recommendation generation into CI artifact flow.

---

## 2) Implementation

### 2.1 New script: `scripts/recommend_monitor_policy.sh`

Added a new executable automation script that:

- scans `analysis/benchmark_reproducibility/*.md`,
- parses reproducibility summaries per benchmark,
- computes rolling-window recommendations,
- writes:
  - markdown report (`monitor_policy_recommendations_<timestamp>.md`)
  - JSON report (`monitor_policy_recommendations_<timestamp>.json`)

Supported options:

- `--bench <name>` (repeatable bench filter)
- `--lookback <n>`
- `--min-samples <n>`
- `--headroom-pct <n>`
- `--floor-pct <n>`
- `--ceil-pct <n>`
- `--output <path>`
- `--output-json <path>`

Recommendation heuristic:

`max(p90(max_spread), p75(median_spread)) * (1 + headroom)` with clamp bounds.

This keeps policy evolution conservative and robust to outliers.

### 2.2 CI integration

Updated `.github/workflows/ci.yml`:

- after benchmark gate execution, CI now runs:

```bash
./scripts/recommend_monitor_policy.sh \
  --bench timer_queue_benchmark \
  --bench directory_entry_cache_benchmark \
  --lookback 6 \
  --min-samples 2 \
  --headroom-pct 15 \
  --floor-pct 5 \
  --ceil-pct 80
```

- artifact upload now includes both markdown and JSON policy outputs.

### 2.3 Documentation updates

Updated:

- `README.md`
- `docs/README.md`
- `docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md`

to include policy automation command and CI integration details.

---

## 3) Validation

### Command executed

```bash
./scripts/recommend_monitor_policy.sh \
  --bench timer_queue_benchmark \
  --bench directory_entry_cache_benchmark \
  --lookback 6 \
  --min-samples 2 \
  --headroom-pct 15 \
  --floor-pct 5 \
  --ceil-pct 80
```

### Result

**PASS** (2 monitored benchmarks analyzed)

Generated evidence:

- `analysis/benchmark_reproducibility/monitor_policy_recommendations_20260210T150630Z.md`
- `analysis/benchmark_reproducibility/monitor_policy_recommendations_20260210T150630Z.json`

Recommendation snapshot:

| Benchmark | Samples | Latest threshold | Drift reports | Recommended | Action |
|---|---:|---:|---:|---:|---|
| `directory_entry_cache_benchmark` | 2 | 25.00% | 0 | 18.24% | tighten |
| `timer_queue_benchmark` | 4 | 60.00% | 2 | 58.73% | hold |

Repository verification:

- `./scripts/verify_repo.sh` -> PASS

---

## 4) Outcome

Day 8 goals are complete:

- rolling monitor policy recommendations are automated,
- output is publishable in both markdown and JSON,
- CI now produces recommendation artifacts for review-driven threshold updates.

Next suggested step: Week 9 Day 9, add policy drift trend dashboarding and threshold-change governance rules.

