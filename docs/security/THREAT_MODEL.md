# 🛡️ VANTIS THREAT MODEL (STRIDE)

> **Note:** VantisOS is an experimental, early-stage hobby project. This threat
> model describes the *intended* security design. Many mitigations below are
> prototypes, partial, or planned — not certified, audited, or fully implemented.

## 🎯 System Assets
1. **Root of Trust:** The Signing Keys (Offline).
2. **User Data:** Stored in Vantis Vault (Encrypted).
3. **Kernel Memory:** Runtime execution space.

## ⚔️ Attacker Profiles
* **Script Kiddie:** Uses public exploits. -> *Mitigated by: Auto-Updates.*
* **State Actor (APT):** Unlimited budget, physical access. -> *Intended mitigation: Wraith Mode (prototype) & formal verification (partial/aspirational).*
* **Supply Chain:** Compromised build tools. -> *Intended mitigation: hermetic/reproducible builds (goal — SLSA level not assessed).*

## 🛑 Attack Vectors & Mitigations

Status legend: 🚧 = prototype / partial / planned (nothing here is certified or production-hardened).

| Threat (STRIDE) | Component | Mitigation Strategy | Status |
| :--- | :--- | :--- | :--- |
| **S**poofing | Bootloader | UEFI Secure Boot + TPM 2.0 | 🚧 Design |
| **T**ampering | Updates | Signed ISOs (Ed25519) | 🚧 Prototype |
| **R**epudiation | Git History | GPG Signed Commits | 🚧 Partial |
| **I**nformation Disclosure | RAM | ASLR + Stack Canaries + Rust Memory Safety | 🚧 Partial |
| **D**enial of Service | Scheduler | Neural Scheduler (Priority Enforcement) | 🚧 WIP |
| **E**levation of Privilege | Drivers | Sandboxed Wasm Modules | 🚧 Prototype |
