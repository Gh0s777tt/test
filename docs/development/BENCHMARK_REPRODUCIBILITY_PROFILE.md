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
- gate uses `scripts/run_benchmark_ci_gate.sh` and runs two stages:
  1. **strict**: `path_lookup_cache_benchmark` (50% spread threshold, blocking)
  2. **monitor**:
     - `timer_queue_benchmark` (recalibrated 60% spread threshold)
     - `directory_entry_cache_benchmark` (25% spread threshold)
     monitor drift is non-blocking and reported as `drift` in summary status.
- monitor stage is budgeted by wall-clock limit (`--monitor-budget-seconds`) and
  supports per-case timeout (`--monitor-case-timeout-seconds`) to prevent runaway
  benchmark duration on shared runners.
- monitor benchmark targets support per-case threshold overrides via
  `--monitor-bench <name:pct>` or `--monitor-threshold <name:pct>`.
- CI also runs `scripts/recommend_monitor_policy.sh` to generate advisory rolling
  threshold recommendations for monitor scenarios.
- CI runs `scripts/build_monitor_policy_dashboard.sh` to aggregate recent gate runs
  and recommendation snapshots into a drift dashboard artifact; dashboard outputs
  now include MONPOL signoff telemetry (decision distribution and approved-entry
  coverage), plus proposal-to-merge latency telemetry.
- CI runs `scripts/evaluate_monitor_drift_escalation.sh` to classify repeated
  monitor drift trends into escalation levels (`normal/watch/escalated/critical`)
  using rolling run windows and policy thresholds.
- CI runs `scripts/generate_monitor_drift_release_handoff.sh` to publish
  owner/SLA-aware release handoff checklist artifacts driven by the latest
  escalation assessment.
- CI runs `scripts/run_monitor_drift_release_readiness_drill.sh` to rehearse
  strict-mode release-readiness enforcement (`strict pass` + `expected blocked
  strict failure`) and publish dry-run artifacts.
- CI runs `scripts/route_monitor_drift_breach_evidence.sh` to publish
  breach-classification evidence routing and governance-gate promotion snapshot
  (`advisory` vs `enforced` mode context).
- CI runs `scripts/evaluate_governance_gate_promotion_readiness.sh` to publish
  readiness scorecard and enforced pilot rollout checklist (`go` / `no-go`)
  from rolling drill/handoff/breach telemetry.
- CI runs `scripts/generate_enforced_pilot_runbook.sh` to publish enforced-pilot
  execution runbook and rollback guardrail decisions (`continue` vs `rollback`)
  from latest readiness + breach/handoff/drill evidence.
- CI runs `scripts/evaluate_enforced_pilot_burn_in_slo.sh` to publish enforced
  pilot burn-in SLO status from rolling runbook telemetry.
- CI runs `scripts/scaffold_enforced_pilot_rollback_postmortem.sh` to maintain
  rollback postmortem scaffold readiness and required incident sections.
- CI runs `scripts/generate_enforced_pilot_handoff_signoff_packet.sh` to publish
  incident-closure handoff signoff packet status, closure requirement, and
  signoff role coverage.
- CI runs `scripts/generate_enforced_pilot_closure_audit.sh` to publish closure
  audit results and Week 10 governance rollout summary telemetry.
- CI runs `scripts/generate_monitor_threshold_proposal.sh` to produce a governance-ready
  `MONPOL` proposal draft with evidence bundle links and proposal-level signoff
  telemetry; proposal output also includes historical latency summary and
  current-proposal age telemetry.
- CI runs `scripts/generate_governance_transition_pack.sh` to publish a packaged
  transition snapshot (toolchain components, evidence status, CI policy baseline,
  signoff review-status telemetry, proposal-to-merge latency telemetry) for Week
  10 rollout tracking.
- CI runs `scripts/scaffold_monpol_changelog_entry.sh` to generate a ready-to-review
  changelog entry scaffold for the latest proposal draft.
- CI runs `scripts/validate_monpol_signoff_metadata.sh` to verify that approved
  changelog decisions have matching reviewer signoff metadata in
  `governance/performance/MONPOL_SIGNOFFS.json`.
- transition pack readiness additionally tracks `approved_entries_have_signoff`
  plus enforced-pilot closure packet readiness (`handoff_signoff_packet_ready_or_not_required`)
  and closure-audit pass state (`closure_audit_passed`) to expose governance
  completeness at a glance.
- latency telemetry highlights changelog entries that do not yet have matching
  proposal artifacts, so policy-cycle evidence gaps are visible in CI artifacts.
- escalation policy reference:
  - `governance/performance/MONITOR_DRIFT_ESCALATION_POLICY.md`
- escalation owner/SLA registry:
  - `governance/performance/MONITOR_DRIFT_ESCALATION_OWNERS.json`
- governance gate promotion strategy:
  - `governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.md`
  - `governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json`
- CI enforces `scripts/check_monitor_threshold_governance.sh` on PRs; threshold-affecting
  policy changes require:
  - `MONPOL-<NNN>` reference in PR title/body,
  - update to `governance/performance/MONITOR_THRESHOLD_CHANGELOG.md`,
  - update to this profile document.
- all markdown reports are uploaded as CI artifacts.
- strict threshold is intentionally conservative on shared runners and should be
  tightened as dedicated benchmarking infrastructure is introduced.

Example CI-like command:

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

Generate rolling policy recommendations from existing evidence:

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

Build policy drift dashboard:

```bash
./scripts/build_monitor_policy_dashboard.sh \
  --lookback-runs 10 \
  --lookback-recommendations 6
```

Evaluate monitor drift escalation policy:

```bash
./scripts/evaluate_monitor_drift_escalation.sh \
  --window-runs 5 \
  --watch-drift-rate-pct 20 \
  --escalate-consecutive-drift 2 \
  --critical-consecutive-drift 3 \
  --escalate-drift-rate-pct 40 \
  --critical-drift-rate-pct 60 \
  --escalate-failure-rate-pct 10 \
  --critical-failure-rate-pct 20
```

Generate monitor drift release handoff checklist:

```bash
./scripts/generate_monitor_drift_release_handoff.sh
```

Run strict release-readiness drill dry-run:

```bash
./scripts/run_monitor_drift_release_readiness_drill.sh
```

Route escalation breach evidence snapshot:

```bash
./scripts/route_monitor_drift_breach_evidence.sh
```

Evaluate governance gate promotion readiness:

```bash
./scripts/evaluate_governance_gate_promotion_readiness.sh
```

Generate enforced pilot execution runbook:

```bash
./scripts/generate_enforced_pilot_runbook.sh
```

Evaluate enforced pilot burn-in SLO:

```bash
./scripts/evaluate_enforced_pilot_burn_in_slo.sh
```

Scaffold enforced pilot rollback postmortem:

```bash
./scripts/scaffold_enforced_pilot_rollback_postmortem.sh
```

Generate enforced pilot incident-closure handoff signoff packet:

```bash
./scripts/generate_enforced_pilot_handoff_signoff_packet.sh
```

Generate enforced pilot closure audit and governance rollout summary:

```bash
./scripts/generate_enforced_pilot_closure_audit.sh
```

Run monitor-threshold governance gate with explicit promotion mode:

```bash
./scripts/check_monitor_threshold_governance.sh --promotion-mode advisory
```

Generate governance-ready MONPOL proposal draft:

```bash
./scripts/generate_monitor_threshold_proposal.sh \
  --bench timer_queue_benchmark \
  --bench directory_entry_cache_benchmark
```

Generate governance transition pack:

```bash
./scripts/generate_governance_transition_pack.sh
```

Generate changelog scaffold draft for latest MONPOL proposal:

```bash
./scripts/scaffold_monpol_changelog_entry.sh
```

Validate reviewer signoff metadata:

```bash
./scripts/validate_monpol_signoff_metadata.sh
```

Canonical governance template:

- `governance/performance/MONITOR_THRESHOLD_PROPOSAL_TEMPLATE.md`

