# High-Level Requirements (HLR)
## VantisOS Core

---

HLR-001:
The kernel shall isolate all user processes from kernel memory.

HLR-002:
The kernel shall prevent execution of user code in kernel mode.

HLR-003:
The kernel shall provide deterministic scheduling in certification mode.

HLR-004:
The kernel shall enforce capability-based access control for IPC.

HLR-005:
The kernel shall detect and handle invalid system calls safely.

HLR-006:
The kernel shall operate without dynamic memory allocation during runtime.

HLR-007:
The kernel shall fail safely in the presence of internal errors.

HLR-008:
The kernel shall be fully analyzable by static analysis tools.
