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
- **Installing Ubuntu host dependencies for ISO/QEMU build tooling**: `../scripts/install_iso_build_deps_ubuntu.sh`
- **Building bootable live/installer ISO (optional QEMU live+installer smoke tests)**: `../scripts/build_iso.sh`
- **Running ISO onboarding CI gate locally (full smoke + rollup threshold enforcement)**: `../scripts/run_iso_onboarding_ci_gate.sh`
- **Validating ISO onboarding rollup gate policy schema/bounds**: `../scripts/validate_iso_onboarding_rollup_gate_policy.sh`
- **ISO onboarding rollup gate policy profiles**: `../governance/performance/ISO_ONBOARDING_ROLLUP_GATE_POLICY.json`
- **Rolling up ISO onboarding telemetry summaries (JSON + Markdown trend report + threshold pass/fail evaluation)**: `../scripts/generate_iso_onboarding_telemetry_rollup.sh`
- **Enforcing ISO onboarding telemetry threshold gate in CI (build + smoke + rollup fail-on-breach)**: `../.github/workflows/iso-onboarding-rollup-gate.yml`
- **Running reproducibility benchmark profile**: `../scripts/benchmark_reproducibility.sh`
- **Running CI-style benchmark gate locally**: `../scripts/run_benchmark_ci_gate.sh`
- **Generating rolling monitor policy recommendations**: `../scripts/recommend_monitor_policy.sh`
- **Building monitor policy drift dashboard (includes signoff + proposal-to-merge latency telemetry)**: `../scripts/build_monitor_policy_dashboard.sh`
- **Evaluating monitor drift escalation policy**: `../scripts/evaluate_monitor_drift_escalation.sh`
- **Generating monitor drift release handoff checklist**: `../scripts/generate_monitor_drift_release_handoff.sh`
- **Running strict release-readiness drill dry-run**: `../scripts/run_monitor_drift_release_readiness_drill.sh`
- **Routing escalation breach evidence snapshot**: `../scripts/route_monitor_drift_breach_evidence.sh`
- **Evaluating governance gate promotion readiness scorecard**: `../scripts/evaluate_governance_gate_promotion_readiness.sh`
- **Generating enforced pilot execution runbook and rollback guardrails snapshot**: `../scripts/generate_enforced_pilot_runbook.sh`
- **Evaluating enforced pilot burn-in telemetry SLO**: `../scripts/evaluate_enforced_pilot_burn_in_slo.sh`
- **Scaffolding enforced pilot rollback postmortem template**: `../scripts/scaffold_enforced_pilot_rollback_postmortem.sh`
- **Generating enforced pilot incident-closure handoff signoff packet**: `../scripts/generate_enforced_pilot_handoff_signoff_packet.sh`
- **Generating enforced pilot closure audit and governance rollout summary**: `../scripts/generate_enforced_pilot_closure_audit.sh`
- **Generating governance-ready threshold proposal draft (includes signoff + latency telemetry)**: `../scripts/generate_monitor_threshold_proposal.sh`
- **Generating Week 9 governance transition pack (includes signoff review-status + latency telemetry)**: `../scripts/generate_governance_transition_pack.sh`
- **Scaffolding MONPOL changelog entry draft**: `../scripts/scaffold_monpol_changelog_entry.sh`
- **Validating MONPOL reviewer signoff metadata**: `../scripts/validate_monpol_signoff_metadata.sh`
- **Checking monitor threshold governance gate**: `../scripts/check_monitor_threshold_governance.sh`
- **Checking monitor threshold governance gate with explicit promotion mode**: `../scripts/check_monitor_threshold_governance.sh --promotion-mode advisory`
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
- **Week 10 Day 3 MONPOL signoff metadata validation**: `../WEEK_10_DAY_3_MONPOL_SIGNOFF_METADATA_VALIDATION.md`
- **Week 10 Day 4 signoff review-status telemetry**: `../WEEK_10_DAY_4_SIGNOFF_REVIEW_STATUS_TELEMETRY.md`
- **Week 10 Day 5 proposal-to-merge latency telemetry**: `../WEEK_10_DAY_5_PROPOSAL_TO_MERGE_LATENCY_TELEMETRY.md`
- **Week 10 Day 6 monitor drift escalation policy rollout**: `../WEEK_10_DAY_6_MONITOR_DRIFT_ESCALATION_POLICY.md`
- **Week 10 Day 7 escalation owner/SLA drills and release handoff checklist**: `../WEEK_10_DAY_7_ESCALATION_OWNER_SLA_AND_HANDOFF.md`
- **Week 10 Day 8 strict release-readiness enforcement dry-run**: `../WEEK_10_DAY_8_RELEASE_READINESS_ENFORCEMENT_DRY_RUN.md`
- **Week 10 Day 9 escalation breach evidence routing + gate promotion strategy**: `../WEEK_10_DAY_9_BREACH_EVIDENCE_ROUTING_AND_GATE_PROMOTION.md`
- **Week 10 Day 10 promotion readiness criteria + enforced pilot checklist**: `../WEEK_10_DAY_10_PROMOTION_READINESS_AND_ENFORCED_PILOT_CHECKLIST.md`
- **Week 10 Day 11 enforced pilot execution runbook + rollback guardrails**: `../WEEK_10_DAY_11_ENFORCED_PILOT_RUNBOOK_AND_ROLLBACK_GUARDRAILS.md`
- **Week 10 Day 12 enforced pilot burn-in telemetry SLO + rollback postmortem template**: `../WEEK_10_DAY_12_ENFORCED_PILOT_BURN_IN_SLO_AND_ROLLBACK_POSTMORTEM_TEMPLATE.md`
- **Week 10 Day 13 enforced pilot incident closure governance + handoff signoff packet**: `../WEEK_10_DAY_13_INCIDENT_CLOSURE_GOVERNANCE_AND_HANDOFF_SIGNOFF_PACKET.md`
- **Week 10 Day 14 enforced pilot closure audit + governance rollout summary**: `../WEEK_10_DAY_14_CLOSURE_AUDIT_AND_GOVERNANCE_ROLLOUT_SUMMARY.md`
- **ISO build and boot validation report**: `../ISO_BUILD_AND_BOOT_VALIDATION.md`
- **Monitor threshold changelog governance**: `../governance/performance/MONITOR_THRESHOLD_CHANGELOG.md`
- **Monitor threshold proposal template**: `../governance/performance/MONITOR_THRESHOLD_PROPOSAL_TEMPLATE.md`
- **Monitor threshold signoff metadata registry**: `../governance/performance/MONPOL_SIGNOFFS.json`
- **Monitor drift escalation policy**: `../governance/performance/MONITOR_DRIFT_ESCALATION_POLICY.md`
- **Monitor drift escalation owners/SLA registry**: `../governance/performance/MONITOR_DRIFT_ESCALATION_OWNERS.json`
- **Monitor threshold governance gate promotion policy**: `../governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.md`
- **Monitor threshold governance gate promotion config**: `../governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json`

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

**Last Updated**: February 11, 2026  
**Documentation Version**: 1.1  
**Project Version**: v0.5.0 (500 functions)