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
