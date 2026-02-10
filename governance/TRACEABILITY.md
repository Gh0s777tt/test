# Traceability Matrix (DO-178C style)

| Requirement ID | Requirement | Implementation Module | Test / Evidence |
|---------------|-------------|-----------------------|----------------|
| REQ-SEC-001 | Process isolation | src/verified/sentinel_sandbox.rs | unit tests, integration tests |
| REQ-SEC-002 | IPC authorization | src/verified/ipc.rs | IPC policy tests |
| REQ-SEC-003 | Key protection | security/vault.rs | Vault tests |
| REQ-SEC-004 | Atomic A/B update | src/verified/vantisfs_ab.rs | integration tests |
| REQ-SEC-005 | Supply chain signing | .github/workflows/release.yml | signed builds |
| REQ-SEC-006 | Provenance generation | .github/workflows/provenance.yml | provenance bundle artifact |
| REQ-SEC-007 | Provenance verification | .github/workflows/verification.yml | signature + digest verification |
