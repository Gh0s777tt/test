# Week 10 Day 10 - Promotion Readiness Criteria and Enforced Pilot Rollout Checklist

## Objective

Define measurable readiness criteria for promotion-mode transition (`advisory -> enforced`) and automate an enforced pilot rollout checklist with explicit `go/no-go` output.

## What was implemented

### 1) New promotion readiness automation

Added:

- `scripts/evaluate_governance_gate_promotion_readiness.sh`

The script:

- evaluates rolling telemetry from canonical artifacts:
  - `monitor_drift_release_readiness_drill_<timestamp>.json`
  - `monitor_drift_release_handoff_<timestamp>.json`
  - `monitor_drift_breach_route_<timestamp>.json`
- computes readiness criteria pass/fail:
  - drill sample sufficiency,
  - drill pass-rate threshold,
  - blocked-handoff limit,
  - breach-detected limit,
- builds enforced pilot checklist with required checks and blocking items,
- emits:
  - `analysis/benchmark_reproducibility/governance_gate_promotion_readiness_<timestamp>.md`
  - `analysis/benchmark_reproducibility/governance_gate_promotion_readiness_<timestamp>.json`
- supports:
  - `--window-days <n>`
  - `--pilot-ack-token <token>`
  - `--fail-on-not-ready` (exit code `2` when criteria are not met).

### 2) Machine-readable readiness thresholds in policy

Extended:

- `governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json`

with:

- `readiness_thresholds`
  - `window_days`
  - `min_drill_samples`
  - `min_drill_pass_rate_pct`
  - `max_blocked_handoff_count`
  - `max_breach_detected_count`
- `pilot_rollout`
  - `ops_ack_token`
  - `required_checks`

and documented the new policy fields/flow in:

- `governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.md`

### 3) CI and governance-chain integration

Updated:

- `.github/workflows/ci.yml`

Added step:

- `Evaluate governance gate promotion readiness`

so promotion-readiness artifacts are generated in CI and uploaded with other benchmark governance evidence.

### 4) Transition-pack telemetry integration

Extended:

- `scripts/generate_governance_transition_pack.sh`

to ingest and report:

- latest promotion-readiness artifact,
- readiness status (`ready`/`not_ready`),
- pilot decision (`go`/`no-go`),
- recommended mode and criteria table,
- transition readiness checks:
  - `promotion_readiness_artifact_present`
  - `promotion_readiness_ready`
  - `promotion_pilot_go`

### 5) Repository verification integration

Extended:

- `scripts/verify_repo.sh`

to execute readiness assessment in temp output mode and verify successful generation.

### 6) Canonical artifact selector hardening follow-up

Updated:

- `scripts/check_monitor_threshold_governance.sh`

to select canonical breach-route artifacts using:

- `monitor_drift_breach_route_[0-9]*.json`

instead of broad wildcard matching that could include non-canonical drill/simulated files.

### 7) Documentation updates

Updated:

- `README.md`
- `docs/README.md`
- `docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md`
- `governance/performance/MONITOR_DRIFT_ESCALATION_POLICY.md`
- `governance/performance/MONITOR_THRESHOLD_PROPOSAL_TEMPLATE.md`
- `todo.md`

## Validation

Executed full governance chain:

```bash
./scripts/validate_monpol_signoff_metadata.sh
./scripts/build_monitor_policy_dashboard.sh --lookback-runs 10 --lookback-recommendations 6
./scripts/evaluate_monitor_drift_escalation.sh --window-runs 5 --watch-drift-rate-pct 20 --escalate-consecutive-drift 2 --critical-consecutive-drift 3 --escalate-drift-rate-pct 40 --critical-drift-rate-pct 60 --escalate-failure-rate-pct 10 --critical-failure-rate-pct 20
./scripts/generate_monitor_threshold_proposal.sh --bench timer_queue_benchmark --bench directory_entry_cache_benchmark
./scripts/scaffold_monpol_changelog_entry.sh
./scripts/generate_monitor_drift_release_handoff.sh --strict
./scripts/run_monitor_drift_release_readiness_drill.sh
./scripts/route_monitor_drift_breach_evidence.sh
./scripts/evaluate_governance_gate_promotion_readiness.sh
./scripts/generate_governance_transition_pack.sh
./scripts/verify_repo.sh
```

Observed:

- `Overall readiness: ready`
- `Pilot decision: go`
- `Recommended mode: enforced`
- `Scripts ready: yes`
- `Artifacts ready: yes`
- `Errors: 0, Warnings: 0` in repository verification

## Evidence artifacts (Day 10 run)

- `analysis/benchmark_reproducibility/governance_gate_promotion_readiness_20260211T013948Z.md`
- `analysis/benchmark_reproducibility/governance_gate_promotion_readiness_20260211T013948Z.json`
- `analysis/benchmark_reproducibility/governance_transition_pack_20260211T013948Z.md`
- `analysis/benchmark_reproducibility/governance_transition_pack_20260211T013948Z.json`
- `analysis/benchmark_reproducibility/monitor_drift_breach_route_20260211T013948Z.md`
- `analysis/benchmark_reproducibility/monitor_drift_breach_route_20260211T013948Z.json`
- `analysis/benchmark_reproducibility/monitor_drift_release_handoff_20260211T013948Z.md`
- `analysis/benchmark_reproducibility/monitor_drift_release_handoff_20260211T013948Z.json`
- `analysis/benchmark_reproducibility/monitor_drift_release_readiness_drill_20260211T013948Z.md`
- `analysis/benchmark_reproducibility/monitor_drift_release_readiness_drill_20260211T013948Z.json`

## Outcome

Promotion-mode readiness criteria are now explicit, machine-readable, and continuously assessed. The governance chain now produces a deterministic enforced pilot rollout checklist (`go/no-go`) that can drive controlled activation of enforced mode.

