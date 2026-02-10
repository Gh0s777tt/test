# Week 9 Day 10: MONPOL Proposal Template Automation

**Date**: 2026-02-10  
**Status**: Completed  
**Scope**: Automate governance-ready monitor threshold proposal drafts with evidence bundle links.

---

## 1) Objective

Day 10 completed the Week 9 governance loop by automating proposal packaging:

1. generate governance-ready `MONPOL` draft proposals from rolling evidence,
2. include ready-to-use evidence bundle links,
3. integrate proposal draft generation into CI artifacts.

---

## 2) Implementation

### 2.1 New proposal generator script

Added:

- `scripts/generate_monitor_threshold_proposal.sh`

Key behavior:

- reads latest recommendation snapshot (`monitor_policy_recommendations_*.json`),
- reads latest dashboard snapshot (`monitor_policy_dashboard_*.json`),
- resolves next proposal ID from changelog (`MONPOL-<NNN>`),
- generates:
  - markdown draft:
    - `analysis/benchmark_reproducibility/monitor_threshold_proposal_<MONPOL-ID>_<timestamp>.md`
  - machine-readable JSON draft:
    - `analysis/benchmark_reproducibility/monitor_threshold_proposal_<MONPOL-ID>_<timestamp>.json`
- emits:
  - threshold review table (current vs recommended),
  - suggested CI monitor flag lines,
  - governance checklist,
  - evidence bundle links.

### 2.2 Governance template

Added canonical template:

- `governance/performance/MONITOR_THRESHOLD_PROPOSAL_TEMPLATE.md`

This establishes a stable structure for manual or generated `MONPOL` proposals.

### 2.3 CI integration

Updated `.github/workflows/ci.yml` benchmark job:

- after recommendation + dashboard steps, CI now runs:

```bash
./scripts/generate_monitor_threshold_proposal.sh \
  --bench timer_queue_benchmark \
  --bench directory_entry_cache_benchmark
```

Generated proposal drafts are included in existing artifact upload globs.

### 2.4 Documentation updates

Updated:

- `README.md`
- `docs/README.md`
- `docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md`

to document proposal generation flow and usage.

---

## 3) Validation

### Commands executed

```bash
./scripts/build_monitor_policy_dashboard.sh --lookback-runs 10 --lookback-recommendations 6
./scripts/generate_monitor_threshold_proposal.sh --bench timer_queue_benchmark --bench directory_entry_cache_benchmark
./scripts/verify_repo.sh
```

### Results

- dashboard generation: **PASS**
- proposal generation: **PASS**
- repository verification: **PASS**

Generated evidence:

- `analysis/benchmark_reproducibility/monitor_policy_dashboard_20260210T154551Z.md`
- `analysis/benchmark_reproducibility/monitor_policy_dashboard_20260210T154551Z.json`
- `analysis/benchmark_reproducibility/monitor_threshold_proposal_MONPOL-002_20260210T154552Z.md`
- `analysis/benchmark_reproducibility/monitor_threshold_proposal_MONPOL-002_20260210T154552Z.json`

Proposal snapshot:

- generated `MONPOL-002` draft automatically,
- detected actionable recommendation:
  - `directory_entry_cache_benchmark`: `25% -> 18.24%` (`tighten`)
- retained hold recommendation:
  - `timer_queue_benchmark`: keep at `60%` (`hold`)

---

## 4) Outcome

Day 10 goals are complete:

- governance-ready proposal drafts are now automated,
- evidence-linking is generated consistently,
- CI publishes proposal artifacts for review-driven policy updates.

Week 9 governance/tooling stream is now operational end-to-end: gate -> recommendation -> dashboard -> proposal.

