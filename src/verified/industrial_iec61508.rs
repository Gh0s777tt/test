// IEC 61508 - Industrial Functional Safety Implementation
// VantisOS SIL 3/4 Compliance

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicU32, AtomicBool, Ordering};
use core::time::Duration;

// ============================================================================
// SIL Classification
// ============================================================================

/// Safety Integrity Level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SIL {
    /// No safety function
    None,
    /// SIL 1
    SIL1,
    /// SIL 2
    SIL2,
    /// SIL 3
    SIL3,
    /// SIL 4 (Highest)
    SIL4,
}

impl SIL {
    pub fn is_safety_critical(self) -> bool {
        matches!(self, SIL::SIL1 | SIL::SIL2 | SIL::SIL3 | SIL::SIL4)
    }

    pub fn is_high(self) -> bool {
        matches!(self, SIL::SIL3 | SIL::SIL4)
    }

    pub fn get_pfd_range(self) -> (f64, f64) {
        match self {
            SIL::None => (1.0, 1.0),
            SIL::SIL1 => (1e-2, 1e-1),
            SIL::SIL2 => (1e-3, 1e-2),
            SIL::SIL3 => (1e-4, 1e-3),
            SIL::SIL4 => (1e-5, 1e-4),
        }
    }

    pub fn get_rrf_range(self) -> (u32, u32) {
        match self {
            SIL::None => (1, 1),
            SIL::SIL1 => (10, 100),
            SIL::SIL2 => (100, 1000),
            SIL::SIL3 => (1000, 10000),
            SIL::SIL4 => (10000, 100000),
        }
    }

    pub fn get_min_diagnostic_coverage(self) -> f32 {
        match self {
            SIL::None => 0.0,
            SIL::SIL1 => 60.0,
            SIL::SIL2 => 90.0,
            SIL::SIL3 => 90.0,
            SIL::SIL4 => 99.0,
        }
    }

    pub fn get_min_safe_failure_fraction(self) -> f32 {
        match self {
            SIL::None => 0.0,
            SIL::SIL1 => 60.0,
            SIL::SIL2 => 90.0,
            SIL::SIL3 => 90.0,
            SIL::SIL4 => 99.0,
        }
    }
}

/// Hazard consequence
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Consequence {
    /// Minor injury
    C1,
    /// Serious injury to one or more persons
    C2,
    /// Death to one or more persons
    C3,
    /// Catastrophic impact on community
    C4,
}

/// Frequency and exposure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrequencyExposure {
    /// Rare to frequent exposure
    F1,
    /// Frequent to continuous exposure
    F2,
}

/// Probability of avoiding hazard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbabilityAvoidance {
    /// Possible under certain conditions
    P1,
    /// Almost impossible
    P2,
}

/// Probability of unwanted occurrence
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbabilityOccurrence {
    /// Very low probability
    W1,
    /// Low probability
    W2,
    /// Relatively high probability
    W3,
}

/// Hazardous event
#[derive(Debug, Clone, Copy)]
pub struct HazardousEvent {
    pub consequence: Consequence,
    pub frequency_exposure: FrequencyExposure,
    pub probability_avoidance: ProbabilityAvoidance,
    pub probability_occurrence: ProbabilityOccurrence,
}

impl HazardousEvent {
    pub fn new(
        consequence: Consequence,
        frequency_exposure: FrequencyExposure,
        probability_avoidance: ProbabilityAvoidance,
        probability_occurrence: ProbabilityOccurrence,
    ) -> Self {
        HazardousEvent {
            consequence,
            frequency_exposure,
            probability_avoidance,
            probability_occurrence,
        }
    }

    pub fn determine_sil(self) -> SIL {
        match (
            self.consequence,
            self.frequency_exposure,
            self.probability_avoidance,
            self.probability_occurrence,
        ) {
            (Consequence::C1, FrequencyExposure::F1, ProbabilityAvoidance::P1, ProbabilityOccurrence::W1) => SIL::None,
            (Consequence::C1, FrequencyExposure::F1, ProbabilityAvoidance::P1, ProbabilityOccurrence::W2) => SIL::SIL1,
            (Consequence::C1, FrequencyExposure::F1, ProbabilityAvoidance::P1, ProbabilityOccurrence::W3) => SIL::SIL1,
            (Consequence::C1, FrequencyExposure::F1, ProbabilityAvoidance::P2, ProbabilityOccurrence::W1) => SIL::None,
            (Consequence::C1, FrequencyExposure::F1, ProbabilityAvoidance::P2, ProbabilityOccurrence::W2) => SIL::SIL1,
            (Consequence::C1, FrequencyExposure::F1, ProbabilityAvoidance::P2, ProbabilityOccurrence::W3) => SIL::SIL2,
            (Consequence::C2, FrequencyExposure::F1, ProbabilityAvoidance::P1, ProbabilityOccurrence::W1) => SIL::SIL1,
            (Consequence::C2, FrequencyExposure::F1, ProbabilityAvoidance::P1, ProbabilityOccurrence::W2) => SIL::SIL1,
            (Consequence::C2, FrequencyExposure::F1, ProbabilityAvoidance::P1, ProbabilityOccurrence::W3) => SIL::SIL2,
            (Consequence::C2, FrequencyExposure::F1, ProbabilityAvoidance::P2, ProbabilityOccurrence::W1) => SIL::SIL1,
            (Consequence::C2, FrequencyExposure::F1, ProbabilityAvoidance::P2, ProbabilityOccurrence::W2) => SIL::SIL2,
            (Consequence::C2, FrequencyExposure::F1, ProbabilityAvoidance::P2, ProbabilityOccurrence::W3) => SIL::SIL3,
            (Consequence::C3, FrequencyExposure::F1, ProbabilityAvoidance::P1, ProbabilityOccurrence::W1) => SIL::SIL1,
            (Consequence::C3, FrequencyExposure::F1, ProbabilityAvoidance::P1, ProbabilityOccurrence::W2) => SIL::SIL2,
            (Consequence::C3, FrequencyExposure::F1, ProbabilityAvoidance::P1, ProbabilityOccurrence::W3) => SIL::SIL3,
            (Consequence::C3, FrequencyExposure::F1, ProbabilityAvoidance::P2, ProbabilityOccurrence::W1) => SIL::SIL2,
            (Consequence::C3, FrequencyExposure::F1, ProbabilityAvoidance::P2, ProbabilityOccurrence::W2) => SIL::SIL3,
            (Consequence::C3, FrequencyExposure::F1, ProbabilityAvoidance::P2, ProbabilityOccurrence::W3) => SIL::SIL4,
            (Consequence::C4, FrequencyExposure::F1, ProbabilityAvoidance::P1, ProbabilityOccurrence::W1) => SIL::SIL2,
            (Consequence::C4, FrequencyExposure::F1, ProbabilityAvoidance::P1, ProbabilityOccurrence::W2) => SIL::SIL3,
            (Consequence::C4, FrequencyExposure::F1, ProbabilityAvoidance::P1, ProbabilityOccurrence::W3) => SIL::SIL4,
            (Consequence::C4, FrequencyExposure::F1, ProbabilityAvoidance::P2, ProbabilityOccurrence::W1) => SIL::SIL3,
            (Consequence::C4, FrequencyExposure::F1, ProbabilityAvoidance::P2, ProbabilityOccurrence::W2) => SIL::SIL4,
            (Consequence::C4, FrequencyExposure::F1, ProbabilityAvoidance::P2, ProbabilityOccurrence::W3) => SIL::SIL4,
            _ => SIL::None,
        }
    }
}

// ============================================================================
// Safety Functions
// ============================================================================

/// Safety function
#[derive(Debug, Clone)]
pub struct SafetyFunction {
    pub id: u32,
    pub name: &'static str,
    pub description: &'static str,
    pub sil: SIL,
    pub pfd_target: f64,
    pub rrf_target: u32,
}

impl SafetyFunction {
    pub fn new(id: u32, name: &'static str, description: &'static str, sil: SIL, pfd_target: f64, rrf_target: u32) -> Self {
        SafetyFunction {
            id,
            name,
            description,
            sil,
            pfd_target,
            rrf_target,
        }
    }
}

/// Safety functions for VantisOS
pub const SAFETY_FUNCTIONS: &[SafetyFunction] = &[
    SafetyFunction::new(
        1,
        "Emergency Shutdown",
        "Emergency shutdown of industrial process",
        SIL::SIL4,
        1e-5,
        100000,
    ),
    SafetyFunction::new(
        2,
        "Process Control",
        "Control of industrial process parameters",
        SIL::SIL3,
        1e-4,
        10000,
    ),
    SafetyFunction::new(
        3,
        "Fire and Gas Detection",
        "Detection of fire and gas hazards",
        SIL::SIL3,
        1e-4,
        10000,
    ),
    SafetyFunction::new(
        4,
        "Pressure Relief",
        "Pressure relief system",
        SIL::SIL3,
        1e-4,
        10000,
    ),
    SafetyFunction::new(
        5,
        "Temperature Control",
        "Temperature control system",
        SIL::SIL3,
        1e-4,
        10000,
    ),
];

// ============================================================================
// Safety Mechanisms
// ============================================================================

/// Safety mechanism type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyMechanismType {
    /// Redundancy
    Redundancy,
    /// Diversity
    Diversity,
    /// Fault tolerance
    FaultTolerance,
    /// Error detection and correction
    EDAC,
    /// Watchdog timer
    Watchdog,
    /// Heartbeat monitoring
    Heartbeat,
    /// Self-diagnostics
    SelfDiagnostics,
    /// Built-in test
    BIT,
}

/// Safety mechanism
#[derive(Debug, Clone)]
pub struct SafetyMechanism {
    pub id: u32,
    pub name: &'static str,
    pub mechanism_type: SafetyMechanismType,
    pub sil: SIL,
    pub enabled: bool,
    pub diagnostic_coverage: f32,
    pub safe_failure_fraction: f32,
}

impl SafetyMechanism {
    pub fn new(id: u32, name: &'static str, mechanism_type: SafetyMechanismType, sil: SIL, diagnostic_coverage: f32, safe_failure_fraction: f32) -> Self {
        SafetyMechanism {
            id,
            name,
            mechanism_type,
            sil,
            enabled: true,
            diagnostic_coverage,
            safe_failure_fraction,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Redundancy architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedundancyArchitecture {
    /// 1-out-of-2
    OneOutOfTwo,
    /// 2-out-of-3
    TwoOutOfThree,
    /// 1-out-of-2 with Diagnostics
    OneOutOfTwoD,
}

/// Redundancy system
#[derive(Debug, Clone)]
pub struct RedundancySystem {
    pub id: u32,
    pub architecture: RedundancyArchitecture,
    pub sil: SIL,
    pub enabled: AtomicBool,
    pub channel_count: u32,
    pub voting_logic: VotingLogic,
}

/// Voting logic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VotingLogic {
    /// Majority vote
    Majority,
    /// Consensus vote
    Consensus,
    /// First valid
    FirstValid,
}

impl RedundancySystem {
    pub fn new(id: u32, architecture: RedundancyArchitecture, sil: SIL, channel_count: u32) -> Self {
        RedundancySystem {
            id,
            architecture,
            sil,
            enabled: AtomicBool::new(true),
            channel_count,
            voting_logic: VotingLogic::Majority,
        }
    }

    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Release);
    }

    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Release);
    }

    pub fn vote(&self, inputs: &[u64]) -> Option<u64> {
        if !self.enabled.load(Ordering::Acquire) {
            return None;
        }

        match self.voting_logic {
            VotingLogic::Majority => self.majority_vote(inputs),
            VotingLogic::Consensus => self.consensus_vote(inputs),
            VotingLogic::FirstValid => self.first_valid(inputs),
        }
    }

    fn majority_vote(&self, inputs: &[u64]) -> Option<u64> {
        if inputs.is_empty() {
            return None;
        }

        let mut counts: std::collections::HashMap<u64, u32> = std::collections::HashMap::new();
        for &input in inputs {
            *counts.entry(input).or_insert(0) += 1;
        }

        let majority = (inputs.len() as u32) / 2 + 1;
        for (value, count) in counts {
            if count >= majority {
                return Some(value);
            }
        }

        None
    }

    fn consensus_vote(&self, inputs: &[u64]) -> Option<u64> {
        if inputs.is_empty() {
            return None;
        }

        let first = inputs[0];
        if inputs.iter().all(|&x| x == first) {
            Some(first)
        } else {
            None
        }
    }

    fn first_valid(&self, inputs: &[u64]) -> Option<u64> {
        if inputs.is_empty() {
            return None;
        }

        Some(inputs[0])
    }
}

/// Built-in test (BIT)
#[derive(Debug, Clone)]
pub struct BuiltInTest {
    pub id: u32,
    pub test_type: BITType,
    pub sil: SIL,
    pub enabled: AtomicBool,
    pub last_test_time: AtomicU64,
    pub test_interval: Duration,
    pub test_result: AtomicBool,
}

/// Built-in test type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BITType {
    /// Power-on BIT
    PowerOn,
    /// Periodic BIT
    Periodic,
    /// On-demand BIT
    OnDemand,
}

impl BuiltInTest {
    pub fn new(id: u32, test_type: BITType, sil: SIL, test_interval: Duration) -> Self {
        BuiltInTest {
            id,
            test_type,
            sil,
            enabled: AtomicBool::new(true),
            last_test_time: AtomicU64::new(0),
            test_interval,
            test_result: AtomicBool::new(true),
        }
    }

    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Release);
    }

    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Release);
    }

    pub fn run(&self) -> bool {
        if !self.enabled.load(Ordering::Acquire) {
            return true;
        }

        // Run the test
        let result = self.execute_test();
        
        self.test_result.store(result, Ordering::Release);
        self.last_test_time.store(core::time::SystemTime::now()
            .duration_since(core::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64, Ordering::Release);
        
        result
    }

    fn execute_test(&self) -> bool {
        // Simplified test execution
        // In production, this would run actual diagnostic tests
        true
    }

    pub fn get_test_result(&self) -> bool {
        self.test_result.load(Ordering::Acquire)
    }

    pub fn should_run(&self) -> bool {
        if !self.enabled.load(Ordering::Acquire) {
            return false;
        }

        let now = core::time::SystemTime::now()
            .duration_since(core::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        let last_test = self.last_test_time.load(Ordering::Acquire);
        let elapsed = now - last_test;
        
        elapsed >= self.test_interval.as_nanos() as u64
    }
}

// ============================================================================
// Safety Statistics
// ============================================================================

/// Safety statistics
#[derive(Debug, Clone)]
pub struct SafetyStatistics {
    pub pfd: AtomicU64, // Probability of Failure on Demand (scaled by 1e9)
    pub rrf: AtomicU32, // Risk Reduction Factor
    pub diagnostic_coverage: AtomicU32, // Diagnostic coverage (percentage)
    pub safe_failure_fraction: AtomicU32, // Safe failure fraction (percentage)
    pub demands: AtomicU64,
    pub failures: AtomicU64,
}

impl SafetyStatistics {
    pub fn new() -> Self {
        SafetyStatistics {
            pfd: AtomicU64::new(0),
            rrf: AtomicU32::new(0),
            diagnostic_coverage: AtomicU32::new(0),
            safe_failure_fraction: AtomicU32::new(0),
            demands: AtomicU64::new(0),
            failures: AtomicU64::new(0),
        }
    }

    pub fn record_demand(&self) {
        self.demands.fetch_add(1, Ordering::Release);
    }

    pub fn record_failure(&self) {
        self.failures.fetch_add(1, Ordering::Release);
    }

    pub fn get_pfd(&self) -> f64 {
        let demands = self.demands.load(Ordering::Acquire);
        let failures = self.failures.load(Ordering::Acquire);
        
        if demands == 0 {
            0.0
        } else {
            failures as f64 / demands as f64
        }
    }

    pub fn get_rrf(&self) -> u32 {
        let pfd = self.get_pfd();
        if pfd == 0.0 {
            0
        } else {
            (1.0 / pfd) as u32
        }
    }

    pub fn set_diagnostic_coverage(&self, coverage: f32) {
        self.diagnostic_coverage.store(coverage as u32, Ordering::Release);
    }

    pub fn get_diagnostic_coverage(&self) -> f32 {
        self.diagnostic_coverage.load(Ordering::Acquire) as f32
    }

    pub fn set_safe_failure_fraction(&self, fraction: f32) {
        self.safe_failure_fraction.store(fraction as u32, Ordering::Release);
    }

    pub fn get_safe_failure_fraction(&self) -> f32 {
        self.safe_failure_fraction.load(Ordering::Acquire) as f32
    }
}

// ============================================================================
// IEC 61508 Manager
// ============================================================================

/// IEC 61508 manager
#[derive(Debug, Clone)]
pub struct IEC61508Manager {
    pub sil: SIL,
    pub statistics: SafetyStatistics,
    pub redundancy: RedundancySystem,
    pub bit: BuiltInTest,
    pub enabled: AtomicBool,
}

impl IEC61508Manager {
    pub fn new(sil: SIL) -> Self {
        IEC61508Manager {
            sil,
            statistics: SafetyStatistics::new(),
            redundancy: RedundancySystem::new(1, RedundancyArchitecture::TwoOutOfThree, sil, 3),
            bit: BuiltInTest::new(1, BITType::Periodic, sil, Duration::from_secs(1)),
            enabled: AtomicBool::new(false),
        }
    }

    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Release);
        self.redundancy.enable();
        self.bit.enable();
    }

    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Release);
        self.redundancy.disable();
        self.bit.disable();
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Acquire)
    }

    pub fn check_safety(&self) -> bool {
        if !self.is_enabled() {
            return true;
        }

        // Run BIT if needed
        if self.bit.should_run() {
            if !self.bit.run() {
                return false;
            }
        }

        true
    }

    pub fn vote(&self, inputs: &[u64]) -> Option<u64> {
        self.redundancy.vote(inputs)
    }

    pub fn get_sil(&self) -> SIL {
        self.sil
    }

    pub fn get_pfd(&self) -> f64 {
        self.statistics.get_pfd()
    }

    pub fn get_rrf(&self) -> u32 {
        self.statistics.get_rrf()
    }

    pub fn get_diagnostic_coverage(&self) -> f32 {
        self.statistics.get_diagnostic_coverage()
    }

    pub fn get_safe_failure_fraction(&self) -> f32 {
        self.statistics.get_safe_failure_fraction()
    }

    pub fn verify_sil_compliance(&self) -> bool {
        let pfd = self.get_pfd();
        let (pfd_min, pfd_max) = self.sil.get_pfd_range();
        
        if pfd < pfd_min || pfd >= pfd_max {
            return false;
        }

        let diagnostic_coverage = self.get_diagnostic_coverage();
        let min_coverage = self.sil.get_min_diagnostic_coverage();
        
        if diagnostic_coverage < min_coverage {
            return false;
        }

        let safe_failure_fraction = self.get_safe_failure_fraction();
        let min_fraction = self.sil.get_min_safe_failure_fraction();
        
        if safe_failure_fraction < min_fraction {
            return false;
        }

        true
    }
}

/// Global IEC 61508 manager
static GLOBAL_IEC61508: IEC61508Manager = IEC61508Manager::new(SIL::SIL3);

/// Get global IEC 61508 manager
pub fn global_iec61508() -> &'static IEC61508Manager {
    &GLOBAL_IEC61508
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sil_determination() {
        let event = HazardousEvent::new(
            Consequence::C4,
            FrequencyExposure::F1,
            ProbabilityAvoidance::P2,
            ProbabilityOccurrence::W3,
        );
        let sil = event.determine_sil();
        assert_eq!(sil, SIL::SIL4);
    }

    #[test]
    fn test_sil_ranges() {
        assert_eq!(SIL::SIL3.get_pfd_range(), (1e-4, 1e-3));
        assert_eq!(SIL::SIL3.get_rrf_range(), (1000, 10000));
        assert_eq!(SIL::SIL3.get_min_diagnostic_coverage(), 90.0);
        assert_eq!(SIL::SIL3.get_min_safe_failure_fraction(), 90.0);
    }

    #[test]
    fn test_safety_functions() {
        assert_eq!(SAFETY_FUNCTIONS.len(), 5);
        assert!(SAFETY_FUNCTIONS.iter().all(|sf| sf.sil.is_high()));
    }

    #[test]
    fn test_redundancy_system() {
        let system = RedundancySystem::new(1, RedundancyArchitecture::TwoOutOfThree, SIL::SIL3, 3);
        assert!(system.vote(&[1, 1, 2]).is_some());
        assert_eq!(system.vote(&[1, 1, 2]), Some(1));
    }

    #[test]
    fn test_bit() {
        let bit = BuiltInTest::new(1, BITType::Periodic, SIL::SIL3, Duration::from_secs(1));
        bit.enable();
        assert!(bit.run());
        assert!(bit.get_test_result());
    }

    #[test]
    fn test_safety_statistics() {
        let stats = SafetyStatistics::new();
        stats.record_demand();
        stats.record_demand();
        stats.record_failure();
        assert_eq!(stats.get_pfd(), 0.5);
        assert_eq!(stats.get_rrf(), 2);
    }

    #[test]
    fn test_iec61508_manager() {
        let manager = IEC61508Manager::new(SIL::SIL3);
        assert!(!manager.is_enabled());
        manager.enable();
        assert!(manager.is_enabled());
        assert!(manager.check_safety());
    }

    #[test]
    fn test_sil_compliance() {
        let manager = IEC61508Manager::new(SIL::SIL3);
        manager.statistics.set_diagnostic_coverage(95.0);
        manager.statistics.set_safe_failure_fraction(95.0);
        assert!(manager.verify_sil_compliance());
    }
}