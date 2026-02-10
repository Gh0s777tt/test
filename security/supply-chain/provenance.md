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

Bundle files:
- `provenance.json`
- `provenance.json.sig`
- `provenance.json.pem`
- `source.tar.gz`
- `source.tar.gz.sha256`
- `source.tar.gz.sig`
- `source.tar.gz.pem`
