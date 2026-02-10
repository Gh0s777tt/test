# 📚 VantisOS Documentation Index

Welcome to the VantisOS documentation! This directory contains comprehensive documentation for all aspects of the project.

---

## 📖 Quick Navigation

### 🏗️ [Architecture](architecture/)
System architecture and design documents
- [Kernel Verification Plan](architecture/KERNEL_VERIFICATION_PLAN.md) - Formal verification strategy
- [Hardware Documentation](architecture/hardware.md) - Hardware compatibility

### 💻 [Implementation](implementation/)
Implementation guides for core components (20 documents)
- [Direct Metal Implementation](implementation/DIRECT_METAL_IMPLEMENTATION.md) - GPU access layer
- [Flux Engine](implementation/FLUX_ENGINE_COMPLETE.md) - Wayland compositor
- [Neural Scheduler](implementation/NEURAL_SCHEDULER_IMPLEMENTATION.md) - AI-based scheduler
- [Sentinel HAL](implementation/SENTINEL_IMPLEMENTATION_PLAN.md) - Hardware abstraction
- [Vantis Aegis](implementation/VANTIS_AEGIS_COMPLETE.md) - Kernel masquerade
- [Vantis Vault](implementation/VANTIS_VAULT_IMPLEMENTATION.md) - Cryptography
- [VantisFS](implementation/VANTISFS_COMPLETE.md) - File system
- [Syscall Interface Guide](implementation/SYSCALL_INTERFACE_GUIDE.md) - Practical syscall usage and troubleshooting
- [Microkernel Architecture](implementation/MICROKERNEL_ARCHITECTURE.md) - Layering, boundaries, and IPC-centric design rationale
- And more...

### 🚀 [Operations](operations/)
Deployment and operational guides
- [Deployment Guide](operations/DEPLOYMENT_INSTRUCTIONS.md) - How to deploy VantisOS
- [Production Crypto Guide](operations/PRODUCTION_CRYPTO_GUIDE.md) - Cryptography in production
- [Installation Guide](operations/INSTALLATION.md) - Installation instructions
- [Keybindings](operations/KEYBINDINGS.md) - Keyboard shortcuts
- [Push Instructions](operations/PUSH_INSTRUCTIONS.md) - Git workflow

### 🛠️ [Development](development/)
Developer guides and best practices (20 documents)
- [Developer Onboarding](development/DEVELOPER_ONBOARDING.md) - Getting started
- [Formal Verification Guide](development/FORMAL_VERIFICATION_GUIDE.md) - Verification process
- [Code Review Guidelines](development/CODE_REVIEW_AND_OPTIMIZATION.md) - Review standards
- [Optimization Guide](development/OPTIMIZATION_IMPLEMENTATION_PLAN.md) - Performance optimization
- [Repository Analysis](development/REPOSITORY_ANALYSIS.md) - Repository structure
- [Repository Maintenance Scripts](../README.md#-repository-maintenance--audit) - Audit and hygiene automation
- And more...

### 🔌 [API](api/)
API documentation and examples
- [API Documentation](api/API_DOCUMENTATION.md) - Complete API reference
- [Verification Examples](api/VERIFICATION_EXAMPLES.md) - Code examples

### 🔒 [Security](security/)
Security documentation and policies
- [Threat Model](security/THREAT_MODEL.md) - Security analysis
- [Bug Bounty Program](security/BUG_BOUNTY.md) - Responsible disclosure
- [Trademark Policy](security/TRADEMARK_POLICY.md) - Trademark usage

### 🌍 [Translations](translations/)
Documentation in multiple languages
- [🇵🇱 Polski](translations/README_PL.md)
- [🇩🇪 Deutsch](translations/README_DE.md)
- [🇫🇷 Français](translations/README_FR.md)
- [🇪🇸 Español](translations/README_ES.md)
- [🇯🇵 日本語](translations/README_JA.md)
- [🇨🇳 中文](translations/README_ZH.md)
- [🇸🇦 العربية](translations/README_AR.md)
- [🇷🇺 Русский](translations/README_RU.md)

---

## 📜 Historical Records

See [../history/](../history/) for:
- **Milestones**: Major achievement celebrations
- **Sessions**: Development session summaries
- **Releases**: Release notes archive

---

## 🎯 Documentation by Topic

### Getting Started
1. [README](../README.md) - Project overview
2. [Installation Guide](operations/INSTALLATION.md) - How to install
3. [Developer Onboarding](development/DEVELOPER_ONBOARDING.md) - For contributors

### Core Systems
1. [Kernel Verification](architecture/KERNEL_VERIFICATION_PLAN.md)
2. [Neural Scheduler](implementation/NEURAL_SCHEDULER_IMPLEMENTATION.md)
3. [VantisFS](implementation/VANTISFS_COMPLETE.md)
4. [Vantis Vault](implementation/VANTIS_VAULT_IMPLEMENTATION.md)

### Advanced Features
1. [Direct Metal (GPU)](implementation/DIRECT_METAL_IMPLEMENTATION.md)
2. [Flux Engine (Compositor)](implementation/FLUX_ENGINE_COMPLETE.md)
3. [Vantis Aegis (Anti-cheat)](implementation/VANTIS_AEGIS_COMPLETE.md)
4. [Sentinel HAL](implementation/SENTINEL_IMPLEMENTATION_PLAN.md)

### Development
1. [Contributing Guide](../CONTRIBUTING.md)
2. [Formal Verification](development/FORMAL_VERIFICATION_GUIDE.md)
3. [API Documentation](api/API_DOCUMENTATION.md)
4. [Code Review](development/CODE_REVIEW_AND_OPTIMIZATION.md)

---

## 📊 Documentation Statistics

- **Total Documents**: 110+ markdown files
- **Languages**: 8 translations
- **Implementation Guides**: 18 detailed guides
- **Development Docs**: 20+ developer resources
- **API References**: Complete API documentation
- **Security Docs**: Comprehensive security guides

---

## 🔍 Search Tips

### By Component
- **Kernel**: Search in `architecture/` and `implementation/`
- **Security**: Check `security/` and `implementation/VANTIS_VAULT*`
- **Performance**: Look in `development/*OPTIMIZATION*`
- **Gaming**: See `implementation/VANTIS_AEGIS*` and `DIRECT_METAL*`

### By Task
- **Installing**: `operations/INSTALLATION.md`
- **Contributing**: `../CONTRIBUTING.md` and `development/DEVELOPER_ONBOARDING.md`
- **Deploying**: `operations/DEPLOYMENT_INSTRUCTIONS.md`
- **Verifying**: `development/FORMAL_VERIFICATION_GUIDE.md`
- **Auditing Git refs**: `../scripts/audit_git_refs.sh`
- **Checking traceability**: `../scripts/check_traceability.sh`
- **Enforcing requirement IDs**: `../scripts/check_requirement_ids.sh`
- **Generating evidence pack**: `../scripts/generate_evidence_pack.sh`
- **Running reproducibility benchmark profile**: `../scripts/benchmark_reproducibility.sh`
- **Running CI-style benchmark gate locally**: `../scripts/run_benchmark_ci_gate.sh`
- **Generating rolling monitor policy recommendations**: `../scripts/recommend_monitor_policy.sh`
- **Building monitor policy drift dashboard**: `../scripts/build_monitor_policy_dashboard.sh`
- **Generating governance-ready threshold proposal draft**: `../scripts/generate_monitor_threshold_proposal.sh`
- **Generating Week 9 governance transition pack**: `../scripts/generate_governance_transition_pack.sh`
- **Scaffolding MONPOL changelog entry draft**: `../scripts/scaffold_monpol_changelog_entry.sh`
- **Validating MONPOL reviewer signoff metadata**: `../scripts/validate_monpol_signoff_metadata.sh`
- **Checking monitor threshold governance gate**: `../scripts/check_monitor_threshold_governance.sh`
- **Store manifest contract**: `../store/manifest.schema.json` and `../store/verify.rs`
- **Benchmark reproducibility guide**: `development/BENCHMARK_REPRODUCIBILITY_PROFILE.md`
- **Week 7 Day 7 performance validation**: `../WEEK_7_DAY_7_PERFORMANCE_VALIDATION.md`
- **Week 7 Day 8 syscall interface guide**: `implementation/SYSCALL_INTERFACE_GUIDE.md`
- **Week 7 Day 9 microkernel architecture**: `implementation/MICROKERNEL_ARCHITECTURE.md`
- **Week 7 Day 10 integration testing**: `../WEEK_7_DAY_10_INTEGRATION_TESTING.md`
- **Week 7 Day 11 directory cache optimization**: `../WEEK_7_DAY_11_DIRECTORY_ENTRY_CACHING.md`
- **Week 7 Day 12 timer queue optimization**: `../WEEK_7_DAY_12_TIMER_QUEUE_OPTIMIZATION.md`
- **Week 7 Day 13 consolidated performance report**: `../WEEK_7_DAY_13_PERFORMANCE_REPORT.md`
- **Week 7 Day 14 final summary and handoff**: `../WEEK_7_DAY_14_WEEK_7_8_SUMMARY.md`
- **Week 9 Day 1 IPC benchmark migration**: `../WEEK_9_DAY_1_IPC_BENCHMARK_MIGRATION.md`
- **Week 9 Day 2 benchmark reproducibility profile**: `../WEEK_9_DAY_2_BENCHMARK_REPRODUCIBILITY.md`
- **Week 9 Day 3 synthetic benchmark hardening**: `../WEEK_9_DAY_3_SYNTHETIC_BENCHMARK_HARDENING.md`
- **Week 9 Day 4 CI benchmark reproducibility gate**: `../WEEK_9_DAY_4_CI_BENCHMARK_GATE.md`
- **Week 9 Day 5 threshold calibration and scenario gate**: `../WEEK_9_DAY_5_THRESHOLD_CALIBRATION_AND_SCENARIO_GATE.md`
- **Week 9 Day 6 monitored scenario expansion and runtime budget**: `../WEEK_9_DAY_6_MONITORED_SCENARIO_EXPANSION_AND_RUNTIME_BUDGET.md`
- **Week 9 Day 7 monitor noise stabilization and threshold recalibration**: `../WEEK_9_DAY_7_MONITOR_NOISE_STABILIZATION_AND_THRESHOLD_RECALIBRATION.md`
- **Week 9 Day 8 monitor policy automation**: `../WEEK_9_DAY_8_MONITOR_POLICY_AUTOMATION.md`
- **Week 9 Day 9 policy drift dashboard and governance**: `../WEEK_9_DAY_9_POLICY_DRIFT_DASHBOARD_AND_GOVERNANCE.md`
- **Week 9 Day 10 MONPOL proposal template automation**: `../WEEK_9_DAY_10_MONPOL_PROPOSAL_TEMPLATE_AUTOMATION.md`
- **Week 10 Day 1 governance toolchain transition pack**: `../WEEK_10_DAY_1_GOVERNANCE_TOOLCHAIN_TRANSITION_PACK.md`
- **Week 10 Day 2 MONPOL changelog scaffold automation**: `../WEEK_10_DAY_2_MONPOL_CHANGELOG_SCAFFOLD_AUTOMATION.md`
- **Monitor threshold changelog governance**: `../governance/performance/MONITOR_THRESHOLD_CHANGELOG.md`
- **Monitor threshold proposal template**: `../governance/performance/MONITOR_THRESHOLD_PROPOSAL_TEMPLATE.md`
- **Monitor threshold signoff metadata registry**: `../governance/performance/MONPOL_SIGNOFFS.json`

---

## 🆘 Need Help?

1. **Check the docs**: Browse this index
2. **Read the FAQ**: See main README
3. **Ask the community**: Join our Discord
4. **Report issues**: GitHub Issues
5. **Security concerns**: See [Bug Bounty](security/BUG_BOUNTY.md)

---

## 📝 Contributing to Documentation

Documentation improvements are always welcome! See:
- [Contributing Guide](../CONTRIBUTING.md)
- [Developer Onboarding](development/DEVELOPER_ONBOARDING.md)

---

**Last Updated**: February 10, 2026  
**Documentation Version**: 1.1  
**Project Version**: v0.5.0 (500 functions)