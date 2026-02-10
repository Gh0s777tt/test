#!/usr/bin/env bash
# Generate a local evidence pack summary.
# Usage:
#   ./scripts/generate_evidence_pack.sh [--full]

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

MODE="quick"
if [[ "${1:-}" == "--full" ]]; then
  MODE="full"
fi

OUT_DIR="$ROOT/analysis"
OUT_FILE="$OUT_DIR/EVIDENCE_PACK.md"
mkdir -p "$OUT_DIR"

status_of() {
  if "$@" >/dev/null 2>&1; then
    echo "PASS"
  else
    echo "FAIL"
  fi
}

branch="$(git branch --show-current)"
commit="$(git rev-parse HEAD)"
commit_short="$(git rev-parse --short HEAD)"
timestamp="$(date -u +"%Y-%m-%d %H:%M:%SZ")"

verified_check="$(status_of bash -lc 'cd src/verified && cargo check --locked')"
security_check="$(status_of bash -lc 'cd security && cargo check --locked')"
cortex_check="$(status_of bash -lc 'cd cortex && cargo check --locked')"
cytadela_check="$(status_of bash -lc 'cd cytadela && cargo check --locked')"
horizon_check="$(status_of bash -lc 'cd horizon && cargo check --locked')"
store_check="$(status_of bash -lc 'cd store && cargo check --locked')"
traceability_status="$(status_of bash -lc './scripts/check_traceability.sh')"

if [[ "$MODE" == "full" ]]; then
  verified_test="$(status_of bash -lc 'cd src/verified && cargo test --locked')"
  security_test="$(status_of bash -lc 'cd security && cargo test --locked')"
  cortex_test="$(status_of bash -lc 'cd cortex && cargo test --locked')"
  cytadela_test="$(status_of bash -lc 'cd cytadela && cargo test --locked')"
  horizon_test="$(status_of bash -lc 'cd horizon && cargo test --locked')"
  store_test="$(status_of bash -lc 'cd store && cargo test --locked')"
else
  verified_test="SKIPPED"
  security_test="SKIPPED"
  cortex_test="SKIPPED"
  cytadela_test="SKIPPED"
  horizon_test="SKIPPED"
  store_test="SKIPPED"
fi

cat > "$OUT_FILE" <<EOF
# VantisOS Evidence Pack

- Generated (UTC): $timestamp
- Branch: \`$branch\`
- Commit: \`$commit_short\` (\`$commit\`)
- Mode: \`$MODE\`

## Quality gate snapshot

| Component | cargo check | cargo test |
|---|---|---|
| src/verified | $verified_check | $verified_test |
| security | $security_check | $security_test |
| cortex | $cortex_check | $cortex_test |
| cytadela | $cytadela_check | $cytadela_test |
| horizon | $horizon_check | $horizon_test |
| store | $store_check | $store_test |

## Governance snapshot

| Control | Status |
|---|---|
| Traceability check | $traceability_status |

## Supply-chain files

| File | SHA-256 |
|---|---|
| .github/workflows/provenance.yml | $(sha256sum .github/workflows/provenance.yml | awk '{print $1}') |
| .github/workflows/verification.yml | $(sha256sum .github/workflows/verification.yml | awk '{print $1}') |
| .github/workflows/release.yml | $(sha256sum .github/workflows/release.yml | awk '{print $1}') |
| security/supply-chain/EVIDENCE_PACK_CHECKLIST.md | $(sha256sum security/supply-chain/EVIDENCE_PACK_CHECKLIST.md | awk '{print $1}') |
EOF

echo "Evidence pack generated: $OUT_FILE"
