# 🔐 Capability Correctness - Proof Sketch / Design Intent

**Date**: February 9, 2026  
**Version**: 1.0  
**Status**: 🚧 Design sketch (not machine-checked)  
**Module**: `ipc_capability_correctness.rs`

---

> ⚠️ **IMPORTANT — these are proof SKETCHES, not machine-checked proofs.**
> The arguments below describe the *intended* correctness properties of the
> capability system and the reasoning we expect to hold. They are **design
> intent**, not verified results. The actual Verus `proof fn` items in the
> codebase are stubs (several with empty / English-comment bodies) and are
> **not verified end-to-end** by the Verus verifier. Nothing here should be
> read as a guarantee. VantisOS is experimental, early-stage (v0.4.1) software
> and is **not** certified, audited, or production-ready.

---

## 📋 Overview

This document sketches the intended **Capability Correctness** properties of the
VantisOS IPC system, and the reasoning we would need to establish to verify
them. It is one of five IPC properties for which design-intent sketches exist.

---

## 🎯 Property Definition

### Formal Statement

**Theorem**: Capabilities are managed correctly and securely.

**Mathematical Formulation**:
```
∀ cap ∈ Capabilities:
  secure_propagation(cap) ∧
  no_forgery(cap) ∧
  revocation_effective(cap) ∧
  no_privilege_escalation(cap)
```

### Sub-Properties

1. **Secure Propagation**: Capabilities are transferred securely
2. **No Forgery**: Capabilities cannot be forged or duplicated
3. **Revocation**: Capabilities can be revoked effectively
4. **No Privilege Escalation**: Processes cannot gain unauthorized capabilities

---

## 🔧 Implementation

### 1. CapabilityToken

Unforgeable token using secret:

```rust
pub struct CapabilityToken {
    id: u64,
    secret: u64,  // Known only to CapabilityManager
}
```

**Security Properties**:
- Secret prevents forgery
- ID uniquely identifies capability
- Verification requires matching secret

### 2. CapabilityType

Types of capabilities:

```rust
pub enum CapabilityType {
    Send(Pid),      // Send to specific process
    Receive,        // Receive messages
    SendAny,        // Send to any process
    Grant,          // Grant capabilities
    Revoke,         // Revoke capabilities
}
```

**Hierarchy**:
- `Grant` and `Revoke` are privileged
- `SendAny` is more powerful than `Send(Pid)`
- `Receive` is basic capability

### 3. SecureCapability

Complete capability with metadata:

```rust
pub struct SecureCapability {
    cap_type: CapabilityType,
    owner: Pid,
    token: CapabilityToken,
    granted_by: Option<Pid>,
    granted_at: u64,
    revoked: bool,
}
```

**Audit Trail**:
- `granted_by`: Who granted this capability
- `granted_at`: When it was granted
- `revoked`: Whether it's been revoked

### 4. CapabilityManager

Central authority for capabilities:

```rust
pub struct CapabilityManager {
    capabilities: HashMap<u64, SecureCapability>,
    process_caps: HashMap<Pid, HashSet<u64>>,
    secret: u64,
    next_cap_id: u64,
    current_time: u64,
    audit_log: Vec<AuditEntry>,
}
```

**Operations**:
- `grant_capability()` - Grant new capability
- `revoke_capability()` - Revoke existing capability
- `transfer_capability()` - Transfer between processes
- `verify_capability()` - Verify token authenticity
- `has_capability()` - Check if process has capability

---

## 📐 Proof Sketches (design intent — not machine-checked)

> The "proofs" below are informal arguments for the intended properties.
> They have **not** been mechanically verified. The accompanying Verus
> snippets are stubs / signatures, not passing proofs.

### Sketch 1: Secure Propagation

**Intended property**:
```
∀ manager, granter, grantee, cap_type:
  grant_capability(granter, grantee, cap_type) = Ok(_) ⟹
  has_grant_capability(granter)
```

**Argument (sketch) by precondition**:
1. `grant_capability` checks `has_grant_capability(granter)`
2. If check fails, returns `Err("No grant capability")`
3. If returns `Ok`, check must have passed
4. Therefore, granter has Grant capability (intended property — proof sketch, not machine-checked)

**Verus signature (stub — not a passing proof)**:
```rust
#[verifier::proof]
pub proof fn theorem_secure_propagation()
    ensures(
        forall|manager, granter, grantee, cap_type|
            manager.grant_capability(granter, grantee, cap_type).is_ok() ==>
            manager.has_grant_capability(granter)
    )
```

### Sketch 2: No Forgery

**Intended property**:
```
∀ manager, token:
  verify_capability(process, token, cap_type) = true ⟹
  exists_valid_capability(manager, token)
```

**Argument (sketch) by token verification**:
1. Token contains secret value
2. Secret is known only to CapabilityManager
3. `verify_capability` checks `token.verify(secret)`
4. Only valid tokens pass verification
5. Forged tokens have wrong secret
6. Therefore, forgery should be infeasible up to the secret's strength (intended property — proof sketch, not machine-checked)

**Verus signature (stub — not a passing proof)**:
```rust
#[verifier::proof]
pub proof fn theorem_no_forgery()
    ensures(
        forall|manager, token|
            manager.verify_capability(process, token, cap_type) ==>
            exists_valid_capability(manager, token)
    )
```

### Sketch 3: Revocation Effective

**Intended property**:
```
∀ manager, token:
  revoke_capability(revoker, token) = Ok(_) ⟹
  eventually(¬is_valid(manager, token))
```

**Argument (sketch) by state change**:
1. `revoke_capability` sets `cap.revoked = true`
2. `is_valid` checks `!cap.revoked`
3. After revocation, `is_valid` returns `false`
4. Therefore, revocation is effective (intended property — proof sketch, not machine-checked)

**Verus signature (stub — not a passing proof)**:
```rust
#[verifier::proof]
pub proof fn theorem_revocation_effective()
    ensures(
        forall|manager, token|
            manager.revoke_capability(revoker, token).is_ok() ==>
            eventually(!is_valid(manager, token))
    )
```

### Sketch 4: No Privilege Escalation

**Intended property**:
```
∀ manager, process, cap_type:
  has_capability(process, cap_type) ⟹
  was_granted(manager, process, cap_type)
```

**Argument (sketch) by capability origin**:
1. Capabilities can only be obtained via `grant_capability`
2. `grant_capability` requires granter to have Grant capability
3. Bootstrap grants initial capabilities to root
4. All capabilities trace back to legitimate grant
5. Therefore, privilege escalation should be prevented (intended property — proof sketch, not machine-checked)

**Verus signature (stub — not a passing proof)**:
```rust
#[verifier::proof]
pub proof fn theorem_no_privilege_escalation()
    ensures(
        forall|manager, process, cap_type|
            manager.has_capability(process, cap_type) ==>
            was_granted(manager, process, cap_type)
    )
```

---

## 🧪 Verification Approach (intended)

### 1. Verus Formal Proofs

**Status**: 🚧 Stubs only — not verified

Four proof functions are stubbed out (signatures / partial bodies, several
empty), corresponding to the sketches above. They do **not** currently pass
the Verus verifier:
- `theorem_secure_propagation`
- `theorem_no_forgery`
- `theorem_revocation_effective`
- `theorem_no_privilege_escalation`

### 2. Kani Model Checking

**Status**: 🚧 Planned / not established

Properties intended to be checked:
1. `verify_grant_requires_capability` - Grant requires Grant capability
2. `verify_revoked_capability_invalid` - Revoked capabilities don't verify
3. `verify_forged_token_rejected` - Forged tokens are rejected

### 3. Unit Tests

**Status**: Present (6 tests) — these are ordinary tests, not proofs

1. `test_capability_grant` - Basic grant operation
2. `test_capability_revocation` - Revocation works
3. `test_no_forgery` - Forged tokens rejected
4. `test_no_privilege_escalation` - Unauthorized grants fail
5. `test_capability_transfer` - Transfer works correctly
6. `test_audit_log` - Audit trail maintained

---

## 📊 Security Analysis

### Threat Model

**Protected Against**:
1. ✅ Capability forgery
2. ✅ Unauthorized capability grants
3. ✅ Privilege escalation
4. ✅ Capability theft
5. ✅ Revocation bypass

**Attack Scenarios**:

**Scenario 1: Forgery Attack**
- Attacker: Creates fake CapabilityToken
- System: Token has wrong secret
- Result: ✅ Verification fails

**Scenario 2: Privilege Escalation**
- Attacker: Tries to grant capability without Grant capability
- System: Checks `has_grant_capability()`
- Result: ✅ Grant fails

**Scenario 3: Revocation Bypass**
- Attacker: Tries to use revoked capability
- System: Checks `!cap.revoked`
- Result: ✅ Verification fails

**Scenario 4: Capability Theft**
- Attacker: Tries to use another process's capability
- System: Checks `cap.owner == process`
- Result: ✅ Verification fails

### Cryptographic Strength

**Secret Size**: 64 bits
**Forgery Probability (theoretical)**: 2^-64 ≈ 5.4 × 10^-20 per guess

**Note**: This is a back-of-the-envelope figure assuming an ideal, uniformly
random, non-leaking secret. The implementation has **not** been audited and the
PRNG / secret-handling has not been reviewed for this property. For any serious
use, consider 128-bit or 256-bit secrets and a real cryptographic review.

---

## 📈 Performance Analysis

### Time Complexity

| Operation | Time Complexity | Space Complexity |
|-----------|----------------|------------------|
| grant_capability | O(1) | O(1) |
| revoke_capability | O(1) | O(1) |
| transfer_capability | O(1) | O(1) |
| verify_capability | O(1) | O(1) |
| has_capability | O(n) | O(1) |

Where n = number of capabilities per process (typically small)

### Space Complexity

**Per Capability**:
```
SecureCapability:
  - cap_type:     8 bytes
  - owner:        8 bytes
  - token:        16 bytes
  - granted_by:   16 bytes (Option)
  - granted_at:   8 bytes
  - revoked:      1 byte
  ─────────────────────
  Total:          ~57 bytes
```

**Per Process**:
```
HashSet<u64>:   ~48 bytes + (8 bytes × num_capabilities)
```

**Total Overhead**: ~100 bytes per capability

---

## 🔒 Audit Trail

### Audit Log

Every capability operation is logged:

```rust
pub struct AuditEntry {
    timestamp: u64,
    action: AuditAction,
    actor: Pid,
    target: Option<Pid>,
    capability_id: u64,
}

pub enum AuditAction {
    Grant,
    Revoke,
    Transfer,
    Use,
    Deny,
}
```

**Benefits**:
- Complete audit trail
- Forensic analysis
- Compliance requirements
- Security monitoring

### Example Audit Log

```
[1] Grant: root(1) -> user(2), cap_id=1, type=Receive
[2] Grant: root(1) -> user(3), cap_id=2, type=Send(4)
[3] Transfer: user(2) -> user(5), cap_id=1
[4] Revoke: root(1), cap_id=2
[5] Deny: user(3) tried to grant without Grant capability
```

---

## 🎯 Comparison with Other Systems

> Note: the "VantisOS" row reflects *design intent*, not verified or audited
> results. seL4's proofs are real, machine-checked, and peer-reviewed; VantisOS's
> are not.

| System | Capability Model | Forgery Protection | Revocation | Audit |
|--------|-----------------|-------------------|------------|-------|
| **VantisOS** | **Design intent (unverified)** | Secret-based (unaudited) | Yes (design) | Audit log (design) |
| seL4 | ✅ Proven (machine-checked) | ✅ Hardware-based | ✅ Yes | ⚠️ Limited |
| Capsicum | ⚠️ Best effort | ✅ Kernel-based | ✅ Yes | ❌ No |
| EROS | ✅ Proven | ✅ Cryptographic | ✅ Yes | ⚠️ Limited |

---

## 📚 Real-World Use Cases

### Use Case 1: Secure Service

**Scenario**: Web server needs to send responses

**Setup**:
1. Root grants `Send(client)` capability to server
2. Server can only send to authorized clients
3. Cannot send to other processes

**Security**: Server cannot abuse capability

### Use Case 2: Temporary Access

**Scenario**: Grant temporary file access

**Setup**:
1. Grant `Read(file)` capability
2. After use, revoke capability
3. Process can no longer access file

**Security**: Time-limited access enforced

### Use Case 3: Delegation

**Scenario**: Process delegates work to helper

**Setup**:
1. Process transfers capability to helper
2. Helper performs work
3. Helper returns capability

**Security**: Controlled delegation

---

## ✅ Completion Checklist

- [x] CapabilityToken implementation
- [x] CapabilityType implementation
- [x] SecureCapability implementation
- [x] CapabilityManager implementation
- [ ] Verus formal proofs (currently stubs — not verified)
- [ ] Kani model checking (planned)
- [x] Unit tests (6 tests)
- [x] Security analysis (informal)
- [x] Audit trail implementation
- [x] Performance analysis (complexity only — unmeasured)
- [x] Comparison with other systems
- [x] Real-world use cases
- [x] Documentation

---

## 🎯 Status

**Capability Correctness: design sketch in place; proofs NOT machine-checked.**

This is one of five IPC properties for which a design-intent sketch exists. So far:

- 📝 Proof *sketches* written in Verus syntax (stubs — do not pass the verifier)
- 🚧 Kani model checking planned, not established
- ✅ Ordinary unit tests present
- 📝 Informal security analysis
- ✅ Audit trail implementation

**Reality**: VantisOS has a *capability design* with *intended* correctness
properties. It does **not** have mathematically proven capability correctness.
The proofs are stubs and are not verified end-to-end.

---

## IPC properties — current state

Design-intent sketches exist for all five IPC properties; **none** are
machine-checked / verified end-to-end:

1. 📝 **Message Integrity** - corruption detection (sketch)
2. 📝 **Resource Bounds** - DoS prevention (sketch)
3. 📝 **Information Leakage Prevention** - process isolation (sketch)
4. 📝 **Deadlock Freedom** - no deadlocks (sketch)
5. 📝 **Capability Correctness** - capability management (sketch)

---

**Status**: 🚧 Design sketch — proofs are stubs, not verified  
**Next**: Attempt to actually discharge the proofs in Verus  
**Progress**: 5 of 5 IPC properties have design-intent sketches (0 machine-checked)