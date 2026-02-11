# Monitor Threshold Change Proposal Template

Use this template for `MONPOL` governance proposals that update benchmark monitor thresholds.

---

## Metadata

- **Proposal ID**: `MONPOL-XXX`
- **Status**: Draft / Proposed / Approved / Rejected
- **Date (UTC)**: `YYYY-MM-DD`
- **Author / Owner**: `<name or team>`
- **Escalation owner / SLA source**: `governance/performance/MONITOR_DRIFT_ESCALATION_OWNERS.json`
- **Changelog target**: `governance/performance/MONITOR_THRESHOLD_CHANGELOG.md`

---

## Scope

Describe which monitor benchmarks are affected and why this proposal is needed.

---

## Proposed Threshold Review Table

| Benchmark | Current (%) | Recommended (%) | Delta (%) | Action | Drift reports | Drift rate (%) | Latest status | Latest report |
|---|---:|---:|---:|---|---:|---:|---|---|
| `example_benchmark` | 25 | 30 | 5 | relax | 2 | 40.0 | pass | `analysis/benchmark_reproducibility/example.md` |

---

## Suggested CI Monitor Flags

```bash
--monitor-bench benchmark_name:threshold
```

---

## Evidence Bundle

- `analysis/benchmark_reproducibility/monitor_policy_recommendations_<timestamp>.md`
- `analysis/benchmark_reproducibility/monitor_policy_recommendations_<timestamp>.json`
- `analysis/benchmark_reproducibility/monitor_policy_dashboard_<timestamp>.md`
- `analysis/benchmark_reproducibility/monitor_policy_dashboard_<timestamp>.json`
- `analysis/benchmark_reproducibility/monitor_drift_escalation_<timestamp>.md`
- `analysis/benchmark_reproducibility/monitor_drift_escalation_<timestamp>.json`
- `analysis/benchmark_reproducibility/monitor_drift_release_handoff_<timestamp>.md`
- `analysis/benchmark_reproducibility/monitor_drift_release_handoff_<timestamp>.json`
- `analysis/benchmark_reproducibility/monitor_drift_release_readiness_drill_<timestamp>.md`
- `analysis/benchmark_reproducibility/monitor_drift_release_readiness_drill_<timestamp>.json`
- `analysis/benchmark_reproducibility/monitor_drift_breach_route_<timestamp>.md`
- `analysis/benchmark_reproducibility/monitor_drift_breach_route_<timestamp>.json`
- `analysis/benchmark_reproducibility/governance_gate_promotion_readiness_<timestamp>.md`
- `analysis/benchmark_reproducibility/governance_gate_promotion_readiness_<timestamp>.json`
- `analysis/benchmark_reproducibility/enforced_pilot_runbook_<timestamp>.md`
- `analysis/benchmark_reproducibility/enforced_pilot_runbook_<timestamp>.json`
- `analysis/benchmark_reproducibility/enforced_pilot_burn_in_slo_<timestamp>.md`
- `analysis/benchmark_reproducibility/enforced_pilot_burn_in_slo_<timestamp>.json`
- `analysis/benchmark_reproducibility/enforced_pilot_rollback_postmortem_<timestamp>.md`
- `analysis/benchmark_reproducibility/enforced_pilot_rollback_postmortem_<timestamp>.json`
- latest `ci_benchmark_gate_summary_<timestamp>.md`
- benchmark report(s) referenced by proposal

---

## Rationale

Summarize:

1. observed drift/failure behavior,
2. recommendation outputs,
3. expected impact of threshold change.

---

## Risk Assessment

- **False positives impact**:
- **False negatives impact**:
- **Rollback approach**:
- **Monitoring plan after merge**:
- **Escalation level (normal/watch/escalated/critical)**:
- **Escalation response owner + SLA**:

---

## Governance Checklist

- [ ] `MONPOL-XXX` is included in PR title/body.
- [ ] `.github/workflows/ci.yml` monitor thresholds updated.
- [ ] `docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md` updated.
- [ ] `governance/performance/MONITOR_THRESHOLD_CHANGELOG.md` updated with final decision.
- [ ] `governance/performance/MONPOL_SIGNOFFS.json` updated when decision is approved.
- [ ] Latest monitor drift escalation artifact reviewed and linked.
- [ ] If escalation level is `escalated`/`critical`, mitigation plan and owner are explicit.
- [ ] Evidence bundle links included and valid.

---

## Reviewer Signoff Metadata (required for approved decisions)

Add/update corresponding entry in `governance/performance/MONPOL_SIGNOFFS.json`:

```json
{
  "proposal_id": "MONPOL-XXX",
  "decision": "approved",
  "owner": "Team or owner name",
  "approved_at_utc": "YYYY-MM-DDTHH:MM:SSZ",
  "reviewers": [
    {
      "name": "Reviewer Name",
      "role": "Role",
      "signed_at_utc": "YYYY-MM-DDTHH:MM:SSZ"
    }
  ],
  "notes": "Optional notes"
}
```

