# Build Provenance Specification

---

The following data must be included in provenance:

- Source repository URL
- Commit SHA
- Builder identity
- Build timestamp
- Build parameters
- Artifact digest (SHA-256)

Provenance must be:
- Automatically generated
- Cryptographically signed
- Immutable after generation

---

## CI Implementation

Provenance is generated and verified by GitHub Actions workflows:

- `.github/workflows/provenance.yml`
  - Creates `provenance.json`
  - Creates `source.tar.gz` and `source.tar.gz.sha256`
  - Signs both artifacts with keyless Sigstore (`cosign sign-blob`)
  - Uploads `provenance-bundle` artifact

- `.github/workflows/verification.yml`
  - Downloads `provenance-bundle` from the provenance run
  - Verifies signatures for provenance and source archive
  - Verifies checksum and JSON consistency (`artifact.sha256`)

- `.github/workflows/release.yml`
  - Rebuilds source archive for release tag
  - Generates `release-provenance.json`
  - Generates `analysis/EVIDENCE_PACK.md` via `scripts/generate_evidence_pack.sh`
  - Signs archive and release provenance with keyless Sigstore
  - Verifies signatures and digest consistency before publishing
  - Uploads signed assets directly to GitHub Release

Bundle files:
- `provenance.json`
- `provenance.json.sig`
- `provenance.json.pem`
- `source.tar.gz`
- `source.tar.gz.sha256`
- `source.tar.gz.sig`
- `source.tar.gz.pem`

Release bundle files:
- `vantis-source-<tag>.tar.gz`
- `vantis-source-<tag>.tar.gz.sha256`
- `vantis-source-<tag>.tar.gz.sig`
- `vantis-source-<tag>.tar.gz.pem`
- `release-provenance.json`
- `release-provenance.json.sig`
- `release-provenance.json.pem`
- `analysis/EVIDENCE_PACK.md`
