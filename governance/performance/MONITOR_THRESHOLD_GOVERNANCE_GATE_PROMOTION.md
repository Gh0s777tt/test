# Monitor Threshold Governance Gate Promotion Strategy

This document defines how monitor-threshold governance checks are promoted from advisory behavior to enforced behavior when escalation-breach evidence is present.

---

## 1) Purpose

Threshold governance checks already validate:

- `MONPOL-<NNN>` linkage in PR metadata,
- changelog and profile updates for threshold-affecting changes,
- signoff metadata requirements for approved decisions.

Day 9 adds a breach-aware promotion strategy so these checks can progressively enforce stronger requirements when escalation telemetry indicates material governance risk.

---

## 2) Machine-Readable Policy

Canonical policy source:

- `governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json`

Key fields:

- `active_mode`: `advisory` or `enforced`
- `modes.<mode>.enforce_on_breach`
- `modes.<mode>.require_pr_mitigation`
- `modes.<mode>.require_breach_ack_token`

---

## 3) Operating Modes

### `advisory`

- breach routing artifacts are generated and reviewed,
- governance gate reports what would be required under enforced mode,
- CI remains non-blocking on breach-specific acknowledgments.

### `enforced`

For threshold-affecting PRs with active breach evidence:

- governance gate becomes blocking,
- PR metadata must include breach acknowledgment token (`BREACH-ACK`),
- PR body should include mitigation plan context.

---

## 4) Promotion Plan

Suggested phased rollout:

1. `phase_0_baseline` (advisory):
   - collect breach-route telemetry and drill pass-rate baseline.
2. `phase_1_enforced`:
   - switch `active_mode` to `enforced` once readiness criteria are met.

Readiness criteria are tracked in the JSON policy file and transition-pack telemetry.

---

## 5) Related Artifacts and Automation

- Breach evidence route:
  - `scripts/route_monitor_drift_breach_evidence.sh`
  - `analysis/benchmark_reproducibility/monitor_drift_breach_route_<timestamp>.md`
  - `analysis/benchmark_reproducibility/monitor_drift_breach_route_<timestamp>.json`
- Governance gate:
  - `scripts/check_monitor_threshold_governance.sh`
- Transition pack telemetry:
  - `scripts/generate_governance_transition_pack.sh`

---

## 6) Revision

- Version: `1`
- Last updated: `2026-02-11`

