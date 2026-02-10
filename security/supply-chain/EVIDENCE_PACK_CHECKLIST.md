# Security Evidence Pack Checklist

This checklist defines the minimum evidence required for a release candidate.

## 1. Build and test quality gates

- [ ] `src/verified`: `cargo check --locked` passed
- [ ] `src/verified`: `cargo test --locked` passed
- [ ] `src/verified`: `cargo clippy --locked -- -D warnings` passed
- [ ] `security`: `cargo check --locked` passed
- [ ] `security`: `cargo test --locked` passed
- [ ] `security`: `cargo clippy --locked -- -D warnings` passed
- [ ] `cortex`: `cargo check --locked` passed
- [ ] `cortex`: `cargo test --locked` passed
- [ ] `cortex`: `cargo clippy --locked -- -D warnings` passed
- [ ] `cytadela`: `cargo check --locked` passed
- [ ] `cytadela`: `cargo test --locked` passed
- [ ] `cytadela`: `cargo clippy --locked -- -D warnings` passed
- [ ] `horizon`: `cargo check --locked` passed
- [ ] `horizon`: `cargo test --locked` passed
- [ ] `horizon`: `cargo clippy --locked -- -D warnings` passed
- [ ] `store`: `cargo check --locked` passed
- [ ] `store`: `cargo test --locked` passed
- [ ] `store`: `cargo clippy --locked -- -D warnings` passed

## 2. Provenance and signatures

- [ ] `provenance.json` generated in CI
- [ ] `provenance.json` signed (`provenance.json.sig`, `provenance.json.pem`)
- [ ] `source.tar.gz` signed (`source.tar.gz.sig`, `source.tar.gz.pem`)
- [ ] `verification.yml` signature checks passed
- [ ] JSON digest (`artifact.sha256`) matches computed archive digest

## 3. Release artifacts

- [ ] `vantis-source-<tag>.tar.gz` attached to release
- [ ] `vantis-source-<tag>.tar.gz.sha256` attached to release
- [ ] `vantis-source-<tag>.tar.gz.sig` attached to release
- [ ] `vantis-source-<tag>.tar.gz.pem` attached to release
- [ ] `release-provenance.json` attached to release
- [ ] `release-provenance.json.sig` attached to release
- [ ] `release-provenance.json.pem` attached to release

## 4. Governance and traceability

- [ ] `./scripts/check_traceability.sh` passed
- [ ] `./scripts/check_requirement_ids.sh` passed (or skipped for non-critical PR scope)
- [ ] `analysis/EVIDENCE_PACK.md` generated (`./scripts/generate_evidence_pack.sh`)
- [ ] Security-impacting changes include traceability reference
- [ ] Threat model reviewed for new attack surface
- [ ] Release notes include security-relevant deltas
