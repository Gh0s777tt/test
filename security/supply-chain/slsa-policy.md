# SLSA Level 4 Policy – VantisOS

---

## 1. Target Level

This project targets:
- SLSA Level: 4
- Build isolation: Mandatory
- Provenance: Non-falsifiable
- Source integrity: Enforced

---

## 2. Source Requirements

- All source code must be stored in Git
- Branch protection enabled
- Mandatory code review (minimum 2 reviewers)
- Signed commits required

---

## 3. Build Requirements

- Builds must be fully reproducible
- Builds must run in ephemeral, isolated environments
- No manual build steps allowed
- Build scripts must be version-controlled

---

## 4. Provenance Requirements

- Provenance must be generated automatically
- Provenance must be cryptographically signed
- Provenance must include:
  - Source commit hash
  - Builder identity
  - Build parameters

---

## 5. Dependency Control

- All dependencies must be pinned by hash
- No floating versions allowed
- SBOM generation is mandatory

---

## 6. Verification

- Provenance must be verified before release
- Verification failures block deployment
