# VantisOS Security Target (ISO/IEC 15408)

## TOE Overview
VantisOS is a microkernel-based operating system derived from Redox OS,
designed with strong isolation, minimal TCB and formal verification goals.

## Security Objectives
- O1: Enforce strict process isolation
- O2: Protect cryptographic material at rest and in use
- O3: Prevent unauthorized kernel interaction
- O4: Support verifiable system integrity
- O5: Provide reproducible builds and supply-chain guarantees

## Assumptions
- Physical access is partially trusted
- Firmware is measured (TPM / Secure Boot)
- Network access is untrusted by default

## Threats
- T1: Privilege escalation
- T2: Kernel memory corruption
- T3: Supply-chain compromise
- T4: Data exfiltration via IPC
- T5: Unauthorized kernel modification

## Security Functional Requirements
- SFR1: Kernel enforces process isolation (MMU)
- SFR2: IPC is authenticated and authorized
- SFR3: Cryptographic keys are protected by Vantis Vault
- SFR4: A/B update integrity verification
- SFR5: Reproducible build provenance
