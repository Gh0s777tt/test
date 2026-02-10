# Week 10 Day 5: Proposal-to-Merge Latency Telemetry

**Date**: 2026-02-10  
**Status**: Completed  
**Scope**: Add MONPOL proposal-to-merge latency telemetry across governance artifacts.

---

## 1) Objective

Week 10 Day 5 introduced governance telemetry for policy-cycle timing:

1. measure `proposal -> merge` latency (in days) for MONPOL entries,
2. expose latency trend signals in dashboard/proposal/transition outputs,
3. highlight evidence gaps (for example, changelog entries without matching proposal artifacts).

---

## 2) Implementation

### 2.1 Dashboard latency telemetry

Updated:

- `scripts/build_monitor_policy_dashboard.sh`

Added:

- `MONPOL Proposal-to-Merge Latency` section in markdown output,
- `latency_telemetry` object in JSON output.

Telemetry fields include:

- changelog entries evaluated,
- proposal artifacts discovered and skipped,
- latency samples count,
- median / P90 / max latency (days),
- missing proposal artifact count,
- invalid merge-date count,
- chronology error count,
- latest merged entry with computed latency,
- detailed per-entry table.

### 2.2 Proposal draft latency telemetry

Updated:

- `scripts/generate_monitor_threshold_proposal.sh`

Added:

- `Proposal-to-Merge Latency Telemetry` section in markdown output,
- `latency_telemetry` object in proposal JSON output.

Proposal now shows:

- historical latency summary imported from dashboard telemetry,
- latest merged latency sample (if available),
- current proposal draft history (`first_seen`, `latest_seen`, `age_days`).

### 2.3 Transition pack latency telemetry

Updated:

- `scripts/generate_governance_transition_pack.sh`

Added:

- `Proposal-to-Merge Latency Telemetry` section in markdown output,
- `latency_telemetry` object in transition JSON output,
- readiness signal:
  - `latency_telemetry_present`.

Transition pack now reports policy-cycle timing and missing proposal evidence directly in the rollout snapshot.

---

## 3) Validation

### Commands executed

```bash
./scripts/validate_monpol_signoff_metadata.sh
./scripts/build_monitor_policy_dashboard.sh --lookback-runs 10 --lookback-recommendations 6
./scripts/generate_monitor_threshold_proposal.sh --bench timer_queue_benchmark --bench directory_entry_cache_benchmark
./scripts/scaffold_monpol_changelog_entry.sh
./scripts/generate_governance_transition_pack.sh
./scripts/verify_repo.sh
```

### Results

- governance chain: **PASS**
- repository verification: **PASS**
- signoff validation: **PASS** (`Errors: 0`, `Warnings: 0`)

Observed telemetry state from latest artifacts:

- MONPOL changelog entries evaluated: `1`
- latency samples available: `0`
- missing proposal artifacts for changelog entries: `1` (`MONPOL-001`)

This is expected for current repository history and is now explicitly surfaced by telemetry.

---

## 4) Evidence

Generated artifacts:

- `analysis/benchmark_reproducibility/monitor_policy_dashboard_20260210T233858Z.md`
- `analysis/benchmark_reproducibility/monitor_policy_dashboard_20260210T233858Z.json`
- `analysis/benchmark_reproducibility/monitor_threshold_proposal_MONPOL-002_20260210T233858Z.md`
- `analysis/benchmark_reproducibility/monitor_threshold_proposal_MONPOL-002_20260210T233858Z.json`
- `analysis/benchmark_reproducibility/monpol_changelog_scaffold_MONPOL-002_20260210T233858Z.md`
- `analysis/benchmark_reproducibility/monpol_changelog_scaffold_MONPOL-002_20260210T233858Z.json`
- `analysis/benchmark_reproducibility/governance_transition_pack_20260210T233858Z.md`
- `analysis/benchmark_reproducibility/governance_transition_pack_20260210T233858Z.json`
- `analysis/benchmark_reproducibility/monpol_signoff_validation_20260210T233858Z.md`
- `analysis/benchmark_reproducibility/monpol_signoff_validation_20260210T233858Z.json`
- `analysis/benchmark_reproducibility/monpol_signoff_validation_20260210T233859Z.md`
- `analysis/benchmark_reproducibility/monpol_signoff_validation_20260210T233859Z.json`

---

## 5) Outcome

Week 10 Day 5 goals are complete:

- proposal-to-merge latency is now measured and exposed in all governance artifacts,
- historical policy-cycle timing is visible in both markdown and JSON outputs,
- missing-proposal-evidence conditions are explicitly visible for governance cleanup.

Next suggested step: Week 10 Day 6, define escalation policy for repeated monitor drift across releases.

