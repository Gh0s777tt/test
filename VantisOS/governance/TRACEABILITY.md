# Traceability Matrix (DO-178C style)

| Requirement ID | Requirement | Implementation Module | Test / Evidence |
|---------------|-------------|-----------------------|----------------|
| REQ-SEC-001 | Process isolation | kernel/mmu | unit tests, formal proof |
| REQ-SEC-002 | IPC authorization | kernel/ipc | IPC policy tests |
| REQ-SEC-003 | Key protection | security/vault | Vault tests |
| REQ-SEC-004 | Atomic A/B update | redoxfs/ab_update | integration tests |
| REQ-SEC-005 | Supply chain signing | .github/workflows | signed builds |
