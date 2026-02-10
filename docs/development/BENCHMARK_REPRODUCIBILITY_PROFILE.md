# Benchmark Reproducibility Profile

Practical profile for running Criterion benchmarks with improved repeatability in local and CI environments.

---

## 1) Why this exists

Synthetic and micro-benchmark workloads are sensitive to environment noise (CPU scaling, background load, incremental compilation, and baseline drift).  
This profile standardizes execution so benchmark trend analysis stays actionable.

---

## 2) Reproducibility runner

Use:

```bash
./scripts/benchmark_reproducibility.sh --bench <bench-name> [options]
```

Examples:

```bash
# Two repeated runs for migrated IPC benchmark
./scripts/benchmark_reproducibility.sh --bench ipc_complete_benchmark --runs 2

# Three runs with stricter spread check
./scripts/benchmark_reproducibility.sh --bench timer_queue_benchmark --runs 3 --strict --spread-threshold-pct 3
```

Generated output:

- Markdown report in `analysis/benchmark_reproducibility/`
- Criterion baselines tagged as:
  - `<prefix>_<bench>_<timestamp>_runN`

---

## 3) Applied profile settings

The runner enforces these environment settings before invoking `cargo bench`:

- `CARGO_INCREMENTAL=0`
- `CARGO_PROFILE_BENCH_CODEGEN_UNITS=1`
- `CARGO_PROFILE_BENCH_LTO=true`
- `CARGO_PROFILE_BENCH_DEBUG=false`

This reduces run-to-run variance from compilation-side factors.

---

## 4) Host environment recommendations

For tighter reproducibility:

1. Prefer Linux `performance` CPU governor.
2. Avoid heavy background jobs during benchmark runs.
3. Keep thermal state stable (avoid first-run turbo spikes when comparing trends).
4. Use repeated runs and inspect spread/CV instead of single-number comparisons.

The runner checks and reports current governor when available.

---

## 5) Baseline retention policy

To limit Criterion baseline growth, the runner keeps only the newest baseline families per benchmark case.

Policy defaults:

- `--retain-families 5`

Meaning:

- For each benchmark case directory, only the latest 5 `<family>_runN` sets are kept.
- Older families for the same `<prefix> + <bench>` are pruned automatically.

---

## 6) CI usage guidance

Recommended minimal CI trend flow:

1. Use fixed benchmark target set (no ad-hoc mix).
2. Run `benchmark_reproducibility.sh` with `--runs 2` or `--runs 3`.
3. Store generated markdown report as CI artifact.
4. Fail only on clearly-defined spread threshold in strict mode.

Example:

```bash
./scripts/benchmark_reproducibility.sh \
  --bench fd_allocator_benchmark \
  --runs 2 \
  --spread-threshold-pct 5 \
  --strict
```

Current repository integration:

- `.github/workflows/ci.yml` includes `benchmark-reproducibility-gate`
- gate currently runs strict reproducibility checks for `timer_queue_benchmark`
  with a 10% spread threshold on pull requests.

