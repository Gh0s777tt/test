# Week 9 Day 4: CI Benchmark Reproducibility Gate Report

**Date**: 2026-02-10  
**Status**: Completed  
**Scope**: CI integration of reproducibility benchmark gate and local strict-mode validation

---

## 1) Objective

Integrate a reproducibility-aware benchmark regression gate into CI and verify it is practically runnable on shared cloud runners.

Target outcomes:

1. CI job wired to `benchmark_reproducibility.sh`.
2. Strict-mode benchmark drift check enabled.
3. Artifact upload of reproducibility report.
4. Local validation mirroring CI command.

---

## 2) Implementation

### 2.1 CI workflow integration

Updated:

- `.github/workflows/ci.yml`

Added pull-request job:

- `benchmark-reproducibility-gate`

Job behavior:

1. Checkout + stable Rust toolchain.
2. Run strict reproducibility script:
   - benchmark: `path_lookup_cache_benchmark`
   - runs: `2`
   - threshold: `50%`
   - strict mode enabled
3. Upload generated report artifact:
   - `analysis/benchmark_reproducibility/path_lookup_cache_benchmark_*.md`

### 2.2 Profile guide sync

Updated:

- `docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md`

to reflect current CI gate configuration and rationale for conservative thresholding on shared runners.

---

## 3) Validation executed

Locally executed command mirroring CI:

```bash
./scripts/benchmark_reproducibility.sh \
  --bench path_lookup_cache_benchmark \
  --runs 2 \
  --spread-threshold-pct 50 \
  --strict \
  --baseline-prefix ci_repro
```

Result: **PASS**

Generated evidence:

- `analysis/benchmark_reproducibility/path_lookup_cache_benchmark_20260210T101311Z.md`

Summary from report:

- Common metrics: 6
- Metrics over threshold (50%): 0
- Median spread: 0.802%
- Max spread: 8.356%

---

## 4) Notes on threshold calibration

During calibration, stricter thresholds (10%) were observed to be flaky on shared runner conditions for some benchmark families.

Current CI gate intentionally uses:

- stable benchmark target (`path_lookup_cache_benchmark`)
- conservative threshold (`50%`)

This keeps the gate actionable while avoiding frequent false positives in non-dedicated benchmarking environments.

Future direction:

- tighten threshold incrementally after dedicated benchmark runner infrastructure is available.

---

## 5) Outcome

Week 9 Day 4 goals are complete:

- benchmark reproducibility gate integrated into CI,
- strict-mode flow validated end-to-end,
- artifact reporting wired for diagnostics,
- documentation updated for maintainers.

Next suggested task: Week 9 Day 5, focusing on stricter threshold calibration with dedicated-runner assumptions and expanded scenario-gate coverage.

