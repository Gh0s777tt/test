# VantisOS Security Target (ISO/IEC 15408)

## TOE Overview
VantisOS is a microkernel-based operating system derived from Redox OS,
designed with strong isolation, minimal TCB and formal verification goals.

## Security Objectives
- O1: Enforce strict process isolation
- O2: Protect cryptographic material at rest and in use
- O3: Prevent unauthorized kernel interaction
- O4: Support verifiable system integrity

## Assumptions
- Physical access is partially trusted
- Firmware is measured (TPM / Secure Boot)

## Threats
- T1: Privilege escalation
- T2: Kernel memory corruption
- T3: Supply-chain attack
