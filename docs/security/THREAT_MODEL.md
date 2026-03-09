# 🛡️ VANTIS THREAT MODEL (STRIDE)

## 🎯 System Assets
1. **Root of Trust:** The Signing Keys (Offline).
2. **User Data:** Stored in Vantis Vault (Encrypted).
3. **Kernel Memory:** Runtime execution space.

## ⚔️ Attacker Profiles
* **Script Kiddie:** Uses public exploits. -> *Mitigated by: Auto-Updates.*
* **State Actor (APT):** Unlimited budget, physical access. -> *Mitigated by: Wraith Mode & EAL 7+ Verification.*
* **Supply Chain:** Compromised build tools. -> *Mitigated by: SLSA L4 Hermetic Builds.*

## 🛑 Attack Vectors & Mitigations

| Threat (STRIDE) | Component | Mitigation Strategy | Status |
| :--- | :--- | :--- | :--- |
| **S**poofing | Bootloader | UEFI Secure Boot + TPM 2.0 | ✅ Active |
| **T**ampering | Updates | Signed ISOs (Ed25519) | ✅ Active |
| **R**epudiation | Git History | GPG Signed Commits Enforced | ✅ Active |
| **I**nformation Disclosure | RAM | ASLR + Stack Canaries + Rust Memory Safety | ✅ Active |
| **D**enial of Service | Scheduler | Neural Scheduler (Priority Enforcement) | 🚧 WIP |
| **E**levation of Privilege | Drivers | Sandboxed Wasm Modules | ✅ Active |
