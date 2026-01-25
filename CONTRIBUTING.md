# Contributing to Vantis OS

Welcome to the Citadel. We are building the last OS you will ever need.

## 🛠️ The Process

1.  **Fork** the repo.
2.  **Clone** it: `git clone https://github.com/YOUR_USER/VantisOS.git`
3.  **Create a branch**: `git checkout -b feature/NeuralScheduler`
4.  **Test**: Run `cargo test` and `cargo kani` (for formal verification).
5.  **Commit**: Please sign your commits (`git commit -S -m "feat: adds AI logic"`).
6.  **Push** and create a PR.

## 📐 Code Standards

- **Language**: Rust (2024 edition).
- **Formatting**: `cargo fmt` is mandatory.
- **Verification**: Critical kernel components MUST have formal proofs (Verus).
- **Documentation**: All public APIs must be documented.

## 📜 Legal
By contributing, you agree that your code becomes part of the Vantis Open Source Initiative under the MIT/Apache-2.0 license.
