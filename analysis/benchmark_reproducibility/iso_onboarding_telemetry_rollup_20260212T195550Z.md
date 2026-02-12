# ISO Onboarding Telemetry Rollup

- Source directory: `/workspace/analysis/benchmark_reproducibility`
- Window requested: `30`
- Runs aggregated: `3`
- Lockout runs: `3`
- Lockout ratio: `100.00%`
- Guard-cleared runs: `3`
- Max failures peak: `3`
- Mean max failures: `3.00`

## Threshold evaluation

- status: `pass`
- policy.max_lockout_ratio: `1.0`
- policy.max_mean_failures: `3.0`
- policy.require_final_source: `import_encrypted`
- policy.fail_on_threshold_breach: `False`

- no threshold breaches detected

## Final onboarding source distribution

- `import_encrypted`: `3`

## Final telemetry last_event distribution

- `guard_cleared`: `3`

## Recent runs

| timestamp | lockout | max_failures | final_source | final_last_event |
|---|---:|---:|---|---|
| `20260212T195550Z` | `True` | `3` | `import_encrypted` | `guard_cleared` |
| `20260212T181545Z` | `True` | `3` | `import_encrypted` | `guard_cleared` |
| `20260212T085934Z` | `True` | `3` | `import_encrypted` | `guard_cleared` |
