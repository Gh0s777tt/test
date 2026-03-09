# Build Pipeline Threat Model

---

## Threat T-001: Compromised Build Runner

Description:
An attacker gains control over the build environment.

Mitigation:
- Ephemeral runners
- No persistent credentials
- Minimal permissions

---

## Threat T-002: Source Code Tampering

Description:
Unauthorized changes introduced into source code.

Mitigation:
- Branch protection
- Mandatory reviews
- Signed commits

---

## Threat T-003: Dependency Poisoning

Description:
Malicious dependency version is introduced.

Mitigation:
- Hash-pinned dependencies
- SBOM verification
- No dynamic downloads during build

---

## Threat T-004: Provenance Forgery

Description:
Fake build provenance is generated.

Mitigation:
- Sigstore signing
- Identity-bound provenance
