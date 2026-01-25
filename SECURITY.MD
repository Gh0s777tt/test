# Security Policy

## 🛡️ Vantis OS Security Philosophy
Vantis OS is designed with a **Zero-Trust Architecture** and targets **EAL 7+ (Augmented)** certification.

## 📦 Supported Versions

| Version | Supported | End of Life | Kernel |
| ------- | ------------------ | ----------- | ------ |
| 5.0.x   | ✅ Yes (LTS)       | 2030-01-01  | Rust Microkernel |
| 4.x     | ❌ No              | 2025-12-31  | Hybrid |

## 🐞 Reporting a Vulnerability

We treat security reports with the highest priority (P0).

1.  **Do NOT** open a public GitHub issue.
2.  Email details to `security@vantis-os.com` or use our [PGP Key](https://vantis-os.com/pgp).
3.  We adhere to a **Responsible Disclosure** policy (90 days).

## 🎖️ Certifications Strategy
- **FIPS 140-3 Level 4:** In progress for Vantis Vault module.
- **SLSA Level 4:** Build pipeline is hermetic and signed.
