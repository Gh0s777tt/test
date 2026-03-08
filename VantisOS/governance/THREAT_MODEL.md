# Threat Model (VantisOS)

## Scope
This threat model covers:
- Kernel and microkernel subsystems
- IPC and userspace isolation
- Filesystem integrity
- Cryptographic key management
- Supply-chain integrity

## Assets
- User data
- Cryptographic keys
- Kernel memory
- Filesystem metadata
- Build artifacts

## Threat Agents
- Local attacker with user privileges
- Remote attacker via network services
- Supply-chain attacker injecting malicious builds
- Malicious or buggy driver code

## Mitigations
- MMU isolation for each process
- Capability-based IPC
- Signed kernel and root filesystem
- Vault protection and panic protocol
- SLSA-based build pipeline
