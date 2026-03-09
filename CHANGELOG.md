# 📝 Changelog VANTIS OS

Wszystkie godne uwagi zmiany w tym projekcie będą udokumentowane w tym pliku.

Format ten oparty jest na [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
i ten projekt przestrzega [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.4.1] - 2025-03-09

### Repository Restructure & Audit

#### Structure Migration
- Migrated all files from `VantisOS/` subdirectory to repository root
- Eliminated duplicate directory structure (1719 files deduplicated)
- Consolidated workspace: 25 crate members now properly accessible at `userspace/`
- Added `Cargo.lock` to root (was missing, preventing reproducible builds)

#### CI/CD Pipeline Fixes
- Rewrote `ci.yml`: proper cargo fmt, clippy, test, build commands
- Rewrote `build.yml`: real build verification with failure reporting
- Rewrote `test.yml`: actual test execution without error suppression
- **Critical**: Removed all error masking (`2>/dev/null || echo "..."`) from workflows
- Added cargo caching for faster CI runs

#### DevContainer Fix
- Fixed `devcontainer.json`: added `username`/`uid`/`gid` to common-utils feature
- Removed unavailable `/dev/kvm` device mount
- Removed broken vscode extensions volume mount
- Downgraded Rust image from 1.93 to 1.85 (stable)

#### Version Consistency
- Unified version to `0.4.1` across all files:
  - `Cargo.toml`, `README.md`, `CITATION.cff`, `SECURITY.md`, `CHANGELOG.md`
- Removed inflated version references (v1.0.0 through v1.5.0)
- Standardized all git tags to use `v` prefix (v0.0.1 through v0.4.1)

#### Documentation Cleanup
- Removed AI session summary files (FINAL_SUMMARY.md, PHASE*_SUMMARY.md, etc.)
- Fixed placeholder badge IDs in README (Discord, YouTube)
- Removed non-existent links (vantis.os, podcast, forum)
- Updated contact information to real channels

#### Code Cleanup
- Removed binary files from tracking (*.o, *.backup)
- Updated `.gitignore` with proper exclusions
- Added `.gitkeep` to empty structural directories

### Original 0.4.1 Release (February 28, 2025)

#### Cytadela Complete - Minimal Kernel Phase

- **Core Kernel Components**
  - Kernel entry point and boot process (entry.rs)
  - Interrupt handling with 256-vector IDT (interrupt.rs)
  - Timer system at 1000 Hz with performance counter (timer.rs)
  - PS/2 keyboard driver with scancode translation (keyboard.rs)
  - Serial port driver with logging system (serial.rs)
  - Memory management (memory_region.rs, memory_protection.rs, memory_stats.rs)

- **Process and Thread Management**
  - Process manager with 1024 slots (process_manager.rs)
  - Process scheduler with round-robin and priority algorithms
  - Thread manager with 4096 slots (thread_manager.rs)
  - Thread scheduler with round-robin and priority algorithms
  - Synchronization primitives (Mutex, Semaphore, Condition Variable, RwLock)

- **I/O and Integration**
  - Character device driver with 256 devices
  - Block device driver with 32 devices
  - Integration tests (11 tests covering all kernel components)

**Statistics**:
- Total Files: 23 kernel files
- Total LOC: ~8,700 lines
- Total Tests: 78 tests (67 unit + 11 integration)
- Test Pass Rate: 100%

---

## Previous Versions (0.0.1 - 0.3.5)

For detailed information about earlier versions, please refer to the git tag history:
- **v0.3.x**: Security and compliance modules
- **v0.2.0**: Microkernel architecture foundations
- **v0.1.x**: Initial development, driver framework
- **v0.0.x**: Project bootstrap and prototyping

---

**Changelog maintained by VantisOS Development Team**
**Last Updated: March 9, 2025**