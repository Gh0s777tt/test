# Low-Level Requirements (LLR)
## VantisOS Core

---

LLR-001:
Each process shall execute in a unique virtual address space.

LLR-002:
The MMU shall mark kernel pages as non-user accessible.

LLR-003:
All system calls shall validate input parameters before execution.

LLR-004:
IPC endpoints shall require explicit capability tokens.

LLR-005:
Scheduler decisions shall complete within a bounded time.

LLR-006:
No heap allocation shall occur after kernel initialization.

LLR-007:
Kernel panic shall place the system into a defined safe state.

LLR-008:
All unsafe Rust blocks shall be documented and reviewed.
