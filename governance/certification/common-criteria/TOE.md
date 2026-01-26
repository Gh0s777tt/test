# Target of Evaluation (TOE)
## VantisOS Core

---

## 1. TOE Boundary

The TOE boundary encompasses only the components executing in kernel mode.

Included:
- Kernel entry points
- IPC mechanisms
- Virtual memory manager
- Scheduler
- Capability enforcement logic

Excluded:
- Filesystems
- Device drivers
- Cryptographic services
- Networking
- UI components

---

## 2. Physical Boundary

The TOE operates on general-purpose hardware without reliance on proprietary
security hardware.

Optional trusted hardware:
- TPM 2.0 (for future extensions)

---

## 3. Logical Boundary

The TOE exposes only the following interfaces:
- System calls (syscalls)
- IPC channels
- Interrupt handling

No dynamic code loading is permitted in kernel space.

---

## 4. Trusted Computing Base (TCB)

The TCB consists of:
- Vantis Microkernel source code
- Verified Rust compiler toolchain
- Build scripts under SLSA Level 4 controls

All other components are untrusted by default.

---

## 5. Assumptions

- A.TRUSTED_BUILD:
  The build environment is controlled and verified.

- A.HARDWARE_INTEGRITY:
  The hardware is not maliciously altered.

---

## 6. TOE Environment

The TOE is intended for deployment in:
- Government systems
- Critical infrastructure
- High-assurance workstations
