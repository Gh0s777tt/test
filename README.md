<div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=0:0A0A0A,100:1A0000&height=300&section=header&text=VANTIS%20OS&fontSize=110&fontColor=DC143C&animation=fadeIn&fontAlignY=38&desc=EXPERIMENTAL%20RUST%20OPERATING%20SYSTEM&descAlignY=60&descAlign=50&descColor=FFFFFF&descSize=20" width="100%" />

<br/>

<img src="https://img.shields.io/badge/status-experimental-orange?style=for-the-badge" />
<img src="https://img.shields.io/badge/language-Rust-DC143C?style=for-the-badge&logo=rust&logoColor=white" />
<a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-0A0A0A?style=for-the-badge" /></a>
<img src="https://img.shields.io/badge/version-0.4.1%20early%20dev-0A0A0A?style=for-the-badge" />

</div>

---

## ⚠️ Project status

**VantisOS is an experimental, early-stage hobby / research operating system written in Rust.** It is a sandbox for exploring microkernel design, capability-based IPC, and formal-verification ideas — **not** a finished, production, or audited product.

A large part of this codebase was produced with heavy AI assistance, and much of it is scaffolding and work-in-progress rather than a working system. Please read the code before relying on any part of it.

To set expectations honestly:

- ❌ **Not certified.** No EAL / Common Criteria / ISO 27001 / SOC 2 / PCI DSS / HIPAA / FIPS evaluation has taken place. Any "compliance" code in this repo is a *data model*, not a certification.
- ❌ **No awards.** This project has not received industry awards.
- 🚧 **Formal verification is partial / aspirational.** A small number of Verus proof sketches exist under `src/verified/`; they are not a complete, machine-checked proof of the system.
- 📊 **Performance is unmeasured.** No benchmarks have been published yet.
- 🧪 **Not everything builds or runs.** Expect rough edges.

---

## What this project actually is

VantisOS explores how a modern OS could be built in Rust around:

- a **microkernel** target with capability-based IPC,
- **memory safety** via Rust (and, eventually, Verus formal verification),
- a clean, modular **userspace** workspace, and
- an experimental, bootable **x86_64 kernel** targeting QEMU.

It is best understood as a **learning and prototyping project**, not a deployable OS.

---

## Current state (honest snapshot)

| Area | What exists | Maturity |
|------|-------------|----------|
| Userspace workspace | ~25 Rust crates (drivers, security, AI, UI, accessibility experiments) | 🟡 Scaffolding / partial |
| Experimental kernel | `iso_build/kernel/` — bootloader, VGA text, paging / heap, basic drivers, a simple shell and demo apps; targets QEMU | 🟡 Minimal / experimental |
| Formal verification | `src/verified/` — a few Verus proof sketches plus compliance *data models* | 🔴 Aspirational |
| AI modules | `src/ai/` — optimization / scheduling experiments | 🟡 Prototype |
| Tooling / CI | GitHub Actions, Dependabot, pre-commit, ADRs | 🟢 Configured |

Rough size: **~950 Rust source files / ~320k lines** — but note that a significant share is data-structure definitions, stubs, and generated scaffolding rather than working OS logic.

---

## Repository layout

```
.
├── iso_build/kernel/   # Experimental bootable x86_64 kernel (QEMU) — excluded from root workspace
├── src/
│   ├── ai/             # AI / optimization experiments
│   └── verified/       # Verus proof sketches + compliance data models (excluded from workspace)
├── userspace/          # Workspace crates (drivers, security, ui, accessibility, ...)
├── bootloader/         # Bootloader
├── adr/                # Architecture Decision Records
├── docs/               # Documentation (aspirational in places)
├── scripts/            # Build / run helpers
├── tests/  benches/    # Tests and benchmarks
└── Cargo.toml          # Root workspace manifest
```

> Note: the kernel under `iso_build/kernel/` and the `src/verified/` tree are **excluded** from the root Cargo workspace (see [`Cargo.toml`](Cargo.toml)) and are built separately.

---

## Building & running

> ⚠️ This is experimental software. Not every crate builds cleanly yet.

### Prerequisites
- Rust nightly toolchain (see [`rust-toolchain`](rust-toolchain))
- QEMU (to run the kernel)
- Optional: [Verus](https://github.com/verus-lang/verus) for the verification experiments

### Userspace workspace
```bash
cargo build
cargo test
```

### Experimental kernel (separate)
```bash
cd iso_build/kernel
cargo build
# then boot the produced image in QEMU
```

Helper scripts live in `scripts/`, alongside the various `build_*.sh` / `create_*_iso.sh` files at the repo root. **Review any script before running it.**

---

## Roadmap & design

- Architecture decisions are recorded under [`adr/`](adr/).
- Longer-term goals live in [`ROADMAP.md`](ROADMAP.md) — treat these as **goals**, not delivered features.

---

## Contributing

Contributions and experiments are welcome — see [`CONTRIBUTING.md`](CONTRIBUTING.md). Given the project's early state, the most useful contributions are: getting things to build, replacing scaffolding with real implementations, and adding honest tests.

---

## License

Licensed under the [MIT License](LICENSE).

---

<div align="center">
<sub>VantisOS · an experimental Rust OS · built largely with AI assistance · no warranties, no certifications — just a sandbox.</sub>
</div>
