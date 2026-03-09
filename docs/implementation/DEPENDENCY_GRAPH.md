# VantisOS Dependency Graph Visualization
## Visual Representation of Module Dependencies

**Version**: 1.0  
**Date**: February 9, 2025  
**Status**: Complete

---

## 1. High-Level Dependency Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    VantisOS Codebase                        │
│                    (75 files, 80K LOC)                      │
└─────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   No Deps    │    │  Alloc Only  │    │  Std Deps    │
│  (35 files)  │    │  (25 files)  │    │  (15 files)  │
│      ✅       │    │      ✅       │    │      ⚠️       │
└──────────────┘    └──────────────┘    └──────────────┘
       │                   │                   │
       │                   │                   ▼
       │                   │            ┌──────────────┐
       │                   │            │    POSIX     │
       │                   │            │  (indirect)  │
       │                   │            └──────────────┘
       │                   │
       └───────────────────┴───────────────────┐
                                               │
                                               ▼
                                    ┌──────────────────┐
                                    │  Microkernel     │
                                    │  (39 syscalls)   │
                                    └──────────────────┘
```

---

## 2. Dependency Layers

### Layer 1: Core (No Dependencies)
```
┌─────────────────────────────────────────┐
│         Core Modules (35 files)         │
│                                         │
│  ✅ math.rs                             │
│  ✅ memory.rs                           │
│  ✅ allocator.rs                        │
│  ✅ process.rs                          │
│  ✅ ipc.rs                              │
│  ✅ syscall.rs                          │
│  ✅ scheduler.rs                        │
│  ✅ vault_*.rs (6 files)                │
│                                         │
│  Dependencies: core only                │
│  Status: ✅ Ready for no_std            │
└─────────────────────────────────────────┘
```

### Layer 2: Alloc-Dependent (No std)
```
┌─────────────────────────────────────────┐
│      Alloc Modules (25 files)           │
│                                         │
│  ✅ flux_*.rs (6 files)                 │
│  ✅ sentinel_*.rs (6 files)             │
│  ✅ horizon_*.rs (5 files) *            │
│  ✅ direct_metal_*.rs (4 files) *       │
│                                         │
│  Dependencies: core + alloc             │
│  Status: ✅ Mostly ready                │
│  Note: * = some std usage               │
└─────────────────────────────────────────┘
```

### Layer 3: Std-Dependent (Need Migration)
```
┌─────────────────────────────────────────┐
│       Std Modules (15 files)            │
│                                         │
│  ⚠️ ipc_complete.rs                     │
│  ⚠️ ipc_*_ops.rs (5 files)              │
│  ⚠️ syscall_*_ops.rs (4 files)          │
│  ⚠️ neural_scheduler.rs                 │
│  ⚠️ vantisfs_*.rs (5 files)             │
│                                         │
│  Dependencies: core + alloc + std       │
│  Status: ⚠️ Need migration              │
└─────────────────────────────────────────┘
```

---

## 3. Detailed Dependency Tree

### 3.1 Core Modules (No Dependencies)

```
math.rs
├── core::ops
└── core::cmp

memory.rs
├── core::ptr
└── core::mem

allocator.rs
├── core::alloc
└── core::ptr

process.rs
├── core::sync::atomic
└── core::mem

ipc.rs
├── core::mem
└── core::ptr

syscall.rs
├── core::mem
└── process.rs

scheduler.rs
├── core::cmp
└── process.rs

vault_aes.rs
├── aes (no_std)
├── cipher (no_std)
└── cbc (no_std)

vault_twofish.rs
├── twofish (no_std)
├── cipher (no_std)
└── cbc (no_std)

vault_serpent.rs
├── serpent (no_std)
├── cipher (no_std)
└── cbc (no_std)
```

### 3.2 Alloc-Dependent Modules

```
flux_engine.rs
├── alloc::collections::BTreeMap
├── alloc::string::String
├── alloc::vec::Vec
└── core::sync::atomic

flux_wayland.rs
├── alloc::collections::BTreeMap
├── alloc::string::String
├── alloc::vec::Vec
└── flux_engine

flux_compositor.rs
├── alloc::collections::BTreeMap
├── alloc::vec::Vec
└── flux_engine

sentinel.rs
├── alloc::collections::BTreeMap
├── alloc::string::String
├── alloc::vec::Vec
└── core::sync::atomic

sentinel_sandbox.rs
├── alloc::collections::BTreeMap
├── alloc::vec::Vec
└── sentinel

horizon_profiles.rs
├── alloc::collections::BTreeMap (planned)
├── std::collections::HashMap (current) ⚠️
├── std::sync::{Arc, RwLock} (current) ⚠️
└── core::sync::atomic
```

### 3.3 Std-Dependent Modules

```
ipc_complete.rs
├── std::collections::HashMap ⚠️
├── std::collections::VecDeque ⚠️
├── std::sync::{Arc, RwLock, Mutex} ⚠️
├── std::time::{Duration, Instant} ⚠️
├── alloc::vec::Vec
└── ipc.rs

syscall_file_ops.rs
├── std::path::{Path, PathBuf} ⚠️
├── std::time::SystemTime ⚠️
├── syscall.rs
└── process.rs

syscall_dir_ops.rs
├── std::path::{Path, PathBuf} ⚠️
├── syscall.rs
└── process.rs

neural_scheduler.rs
├── verus::prelude::* 🔧
├── vstd::prelude::* 🔧
├── builtin::* 🔧
├── builtin_macros::* 🔧
└── scheduler.rs

vantisfs_block_allocator.rs
├── verus::prelude::* 🔧
├── vstd::prelude::* 🔧
├── builtin::* 🔧
├── builtin_macros::* 🔧
└── allocator.rs
```

---

## 4. External Crate Dependencies

### 4.1 Cryptography Stack

```
Vault Modules
    │
    ├── aes (0.8)
    │   └── cipher (0.4)
    │       └── block-padding (0.3)
    │
    ├── twofish (0.7)
    │   └── cipher (0.4)
    │
    ├── serpent (0.5)
    │   └── cipher (0.4)
    │
    └── cbc (0.1)
        └── cipher (0.4)

Status: ✅ All no_std compatible
```

### 4.2 Random Number Generation

```
Vault Modules
    │
    └── rand (0.8)
        ├── rand_core (0.6)
        └── getrandom (0.2)
            └── OS syscalls ⚠️

Current: Uses std features
Target: no_std with custom RNG
```

### 4.3 Verification Tools

```
Verified Modules
    │
    ├── verus 🔧
    ├── vstd 🔧
    ├── builtin 🔧
    └── builtin_macros 🔧

Status: 🔧 Requires special compiler
Action: Separate with feature flags
```

### 4.4 GPU Backends (Optional)

```
Direct Metal
    │
    ├── ash (0.37) [optional]
    │   └── Vulkan API
    │
    └── metal-rs (0.27) [optional]
        └── Metal API (macOS)

Status: ✅ Already optional features
```

---

## 5. Module Interaction Map

### 5.1 Core System Modules

```
                    ┌──────────┐
                    │ syscall  │
                    └────┬─────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
        ▼                ▼                ▼
   ┌─────────┐     ┌─────────┐     ┌─────────┐
   │ process │     │   ipc   │     │ memory  │
   └────┬────┘     └────┬────┘     └────┬────┘
        │               │               │
        │               │               │
        └───────┬───────┴───────┬───────┘
                │               │
                ▼               ▼
          ┌──────────┐    ┌──────────┐
          │scheduler │    │allocator │
          └──────────┘    └──────────┘
```

### 5.2 Filesystem Modules

```
   ┌──────────────────┐
   │ syscall_file_ops │
   └────────┬─────────┘
            │
            ▼
   ┌──────────────────┐
   │  syscall_dir_ops │
   └────────┬─────────┘
            │
            ▼
   ┌──────────────────┐
   │    vantisfs_*    │
   └──────────────────┘
```

### 5.3 Security Modules

```
   ┌──────────────────┐
   │   ipc_complete   │
   └────────┬─────────┘
            │
            ├─────────────────┐
            │                 │
            ▼                 ▼
   ┌──────────────┐   ┌──────────────┐
   │ ipc_message_ │   │ ipc_resource_│
   │  integrity   │   │    bounds    │
   └──────────────┘   └──────────────┘
            │                 │
            └────────┬────────┘
                     │
                     ▼
            ┌──────────────────┐
            │ ipc_capability_  │
            │   correctness    │
            └──────────────────┘
```

### 5.4 UI/Gaming Modules

```
   ┌──────────────────┐
   │   flux_engine    │
   └────────┬─────────┘
            │
            ├─────────────────┬─────────────────┐
            │                 │                 │
            ▼                 ▼                 ▼
   ┌──────────────┐   ┌──────────────┐   ┌──────────────┐
   │flux_wayland  │   │flux_compositor│   │ flux_gaming  │
   └──────────────┘   └──────────────┘   └──────────────┘
            │                 │                 │
            └────────┬────────┴────────┬────────┘
                     │                 │
                     ▼                 ▼
            ┌──────────────┐   ┌──────────────┐
            │ direct_metal │   │ flux_window  │
            └──────────────┘   └──────────────┘
```

---

## 6. Dependency Migration Path

### 6.1 Current State

```
┌─────────────────────────────────────────┐
│         Current Dependencies            │
├─────────────────────────────────────────┤
│  std::collections::HashMap (13 uses)    │
│  std::time (7 uses)                     │
│  std::sync (7 uses)                     │
│  std::path (2 uses)                     │
│  verus/vstd (62 uses)                   │
└─────────────────────────────────────────┘
```

### 6.2 Target State (After Migration)

```
┌─────────────────────────────────────────┐
│         Target Dependencies             │
├─────────────────────────────────────────┤
│  alloc::collections::BTreeMap           │
│  crate::time (custom)                   │
│  spin::{RwLock, Mutex}                  │
│  crate::fs::Path (custom)               │
│  verus (feature-gated)                  │
└─────────────────────────────────────────┘
```

### 6.3 Migration Steps

```
Step 1: Separate Verus
    ├── Add feature flags
    ├── Separate verification code
    └── Test compilation

Step 2: Replace HashMap
    ├── Use BTreeMap from alloc
    ├── Or use hashbrown crate
    └── Benchmark performance

Step 3: Replace Time
    ├── Implement custom Duration
    ├── Implement custom Instant
    └── Use hardware timers

Step 4: Replace Sync
    ├── Use spin::RwLock
    ├── Use spin::Mutex
    └── Test synchronization

Step 5: Handle Remaining
    ├── Custom Path implementation
    ├── Custom getrandom
    └── Final testing
```

---

## 7. Dependency Statistics

### 7.1 By Category

```
Category          | Files | LOC   | Status
------------------|-------|-------|--------
No Dependencies   |   35  | 35K   | ✅ Ready
Alloc Only        |   25  | 30K   | ✅ Ready
Std Dependencies  |   15  | 15K   | ⚠️ Work
------------------|-------|-------|--------
Total             |   75  | 80K   |
```

### 7.2 By Priority

```
Priority  | Dependencies        | Files | Effort
----------|---------------------|-------|--------
CRITICAL  | Verus separation    |   15  | 2-3 days
HIGH      | HashMap replacement |   13  | 1-2 days
HIGH      | Time implementation |    7  | 2-3 days
MEDIUM    | Sync replacement    |    7  | 1 day
LOW       | Path/Thread/etc     |    3  | 1-2 days
----------|---------------------|-------|--------
Total     |                     |   45  | 7-11 days
```

### 7.3 By Module Type

```
Module Type       | Files | Std Deps | Status
------------------|-------|----------|--------
Core System       |   20  |     0    | ✅
Filesystem        |   10  |     2    | ⚠️
IPC/Security      |   15  |     5    | ⚠️
UI/Gaming         |   12  |     0    | ✅
Profiles          |    5  |     2    | ⚠️
Verification      |   13  |    15    | 🔧
------------------|-------|----------|--------
Total             |   75  |    24    |
```

---

## 8. Conclusion

### 8.1 Dependency Health

**Overall Status**: 🟢 Good

- ✅ 47% of files have no dependencies
- ✅ 33% use only alloc (no_std ready)
- ⚠️ 20% use std (need migration)

### 8.2 Migration Feasibility

**Feasibility**: 🟢 High

- Most dependencies are replaceable
- Clear migration path exists
- Estimated effort: 7-11 days
- No blocking issues identified

### 8.3 Risk Level

**Risk**: 🟡 Medium

- Verus separation: Medium risk
- HashMap replacement: Low risk
- Time implementation: Medium risk
- Sync replacement: Low risk

---

**Document Version**: 1.0  
**Last Updated**: February 9, 2025  
**Status**: Complete  
**Next Steps**: Begin migration (Week 7 Day 4)