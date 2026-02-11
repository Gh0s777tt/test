# Monitor Policy Drift Dashboard

**Source directory**: `/workspace/analysis/benchmark_reproducibility`  
**Runs analyzed**: 3 (lookback=10)  
**Recommendation snapshots analyzed**: 1 (lookback=6)

## Health Summary

- Strict non-pass runs: 0
- Strict status unknown runs: 1
- Runs with monitor drift cases: 1
- Runs with monitor failure/timeout/missing-report cases: 0

## MONPOL Signoff Telemetry

- Changelog MONPOL entries: 1
- Changelog approved entries: 0
- Approved entries with signoff metadata: 0
- Approved entries missing signoff metadata: 0
- Signoff decision distribution: none
- Latest signoff record: none

| Proposal | Decision | Owner | Reviewers | approved_at_utc |
|---|---|---|---:|---|
| _none_ | n/a | n/a | 0 | n/a |

## MONPOL Proposal-to-Merge Latency

- Changelog entries evaluated: 1
- Proposal artifacts discovered: 8
- Proposal artifacts skipped (unparseable time): 0
- Latency samples (days): 0
- Median latency (days): n/a
- P90 latency (days): n/a
- Max latency (days): n/a
- Entries missing proposal artifacts: 1
- Entries with invalid merge date: 0
- Entries with chronology errors: 0
- Latest merged entry with latency: none

| Proposal | Decision | Merge date | First proposal generated_at_utc | Latency (days) | Status | Proposal file | History files |
|---|---|---|---|---:|---|---|---:|
| `MONPOL-001` | unknown | 2026-02-10 | n/a | n/a | missing_proposal_artifact | `n/a` | 0 |

## Monitor Benchmark Health

| Benchmark | Samples | Drift count | Drift rate (%) | Failure count | Failure rate (%) | Latest status | Latest threshold (%) | Recommendation action | Recommended threshold (%) |
|---|---:|---:|---:|---:|---:|---|---:|---|---:|
| `directory_entry_cache_benchmark` | 2 | 0 | 0.00 | 0 | 0.00 | pass | 25 | tighten | 18.24 |
| `timer_queue_benchmark` | 2 | 0 | 0.00 | 0 | 0.00 | pass | 60 | hold | 58.73 |

## Recent Run Timeline

| Timestamp | Strict status | Monitor drift cases | Monitor failure cases | Total duration (s) | Summary |
|---|---|---:|---:|---:|---|
| 20260210T105039Z | unknown | 0 | 0 | 0 | `ci_benchmark_gate_summary_20260210T105039Z.md` |
| 20260210T111835Z | pass | 1 | 0 | 326 | `ci_benchmark_gate_summary_20260210T111835Z.md` |
| 20260210T143639Z | pass | 0 | 0 | 332 | `ci_benchmark_gate_summary_20260210T143639Z.md` |

## Latest Recommendation Snapshot

Source: `monitor_policy_recommendations_20260210T150630Z.json`

| Benchmark | Latest threshold (%) | Recommended (%) | Action | Drift reports | Drift rate (%) |
|---|---:|---:|---|---:|---:|
| `directory_entry_cache_benchmark` | 25.0 | 18.24 | tighten | 0 | 0.0 |
| `timer_queue_benchmark` | 60.0 | 58.73 | hold | 2 | 50.0 |

Dashboard is advisory; policy changes should follow governance gate requirements.
