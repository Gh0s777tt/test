// ISO 26262 - Automotive Functional Safety Implementation
// VantisOS ASIL D Compliance

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicU32, AtomicBool, Ordering};
use core::time::Duration;

// ============================================================================
// ASIL Classification
// ============================================================================

/// Automotive Safety Integrity Level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ASIL {
    /// Quality Management (QM)
    QM,
    /// ASIL A
    A,
    /// ASIL B
    B,
    /// ASIL C
    C,
    /// ASIL D (Highest)
    D,
}

impl ASIL {
    pub fn is_safety_critical(self) -> bool {
        matches!(self, ASIL::A | ASIL::B | ASIL::C | ASIL::D)
    }

    pub fn is_highest(self) -> bool {
        self == ASIL::D
    }
}

/// Hazard severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// No injuries
    S0,
    /// Light to moderate injuries
    S1,
    /// Severe to life-threatening injuries (survival probable)
    S2,
    /// Life-threatening to fatal injuries (survival uncertain)
    S3,
}

/// Hazard exposure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exposure {
    /// Incredibly unlikely
    E0,
    /// Very low probability
    E1,
    /// Low probability
    E2,
    /// Medium probability
    E3,
    /// High probability
    E4,
}

/// Hazard controllability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Controllability {
    /// Controllable in general
    C1,
    /// Normally controllable
    C2,
    /// Difficult to control
    C3,
}

/// Hazardous event
#[derive(Debug, Clone, Copy)]
pub struct HazardousEvent {
    pub severity: Severity,
    pub exposure: Exposure,
    pub controllability: Controllability,
}

impl HazardousEvent {
    pub fn new(severity: Severity, exposure: Exposure, controllability: Controllability) -> Self {
        HazardousEvent {
            severity,
            exposure,
            controllability,
        }
    }

    pub fn determine_asil(self) -> ASIL {
        match (self.severity, self.exposure, self.controllability) {
            (Severity::S1, Exposure::E1, _) => ASIL::A,
            (Severity::S1, Exposure::E2, Controllability::C2 | Controllability::C3) => ASIL::B,
            (Severity::S1, Exposure::E3, Controllability::C2 | Controllability::C3) => ASIL::C,
            (Severity::S1, Exposure::E4, Controllability::C2 | Controllability::C3) => ASIL::D,
            (Severity::S2, Exposure::E1, Controllability::C2 | Controllability::C3) => ASIL::A,
            (Severity::S2, Exposure::E2, Controllability::C2 | Controllability::C3) => ASIL::B,
            (Severity::S2, Exposure::E3, Controllability::C2 | Controllability::C3) => ASIL::C,
            (Severity::S2, Exposure::E4, Controllability::C2 | Controllability::C3) => ASIL::D,
            (Severity::S3, Exposure::E1, Controllability::C2 | Controllability::C3) => ASIL::A,
            (Severity::S3, Exposure::E2, Controllability::C2 | Controllability::C3) => ASIL::B,
            (Severity::S3, Exposure::E3, Controllability::C2 | Controllability::C3) => ASIL::C,
            (Severity::S3, Exposure::E4, Controllability::C1 | Controllability::C2 | Controllability::C3) => ASIL::D,
            _ => ASIL::QM,
        }
    }
}

// ============================================================================
// Safety Goals
// ============================================================================

/// Safety goal
#[derive(Debug, Clone)]
pub struct SafetyGoal {
    pub id: u32,
    pub name: &'static str,
    pub description: &'static str,
    pub asil: ASIL,
    pub ftti: Duration,
}

impl SafetyGoal {
    pub fn new(id: u32, name: &'static str, description: &'static str, asil: ASIL, ftti: Duration) -> Self {
        SafetyGoal {
            id,
            name,
            description,
            asil,
            ftti,
        }
    }
}

/// Safety goals for VantisOS
pub const SAFETY_GOALS: &[SafetyGoal] = &[
    SafetyGoal::new(
        1,
        "Prevent unauthorized access",
        "Prevent unauthorized access to vehicle control systems",
        ASIL::D,
        Duration::from_millis(100),
    ),
    SafetyGoal::new(
        2,
        "Ensure timely response",
        "Ensure timely response to safety-critical events",
        ASIL::D,
        Duration::from_millis(100),
    ),
    SafetyGoal::new(
        3,
        "Maintain system integrity",
        "Maintain system integrity under fault conditions",
        ASIL::D,
        Duration::from_millis(100),
    ),
    SafetyGoal::new(
        4,
        "Prevent data corruption",
        "Prevent data corruption in safety-critical systems",
        ASIL::D,
        Duration::from_millis(100),
    ),
    SafetyGoal::new(
        5,
        "Ensure safe shutdown",
        "Ensure safe shutdown in case of critical failures",
        ASIL::D,
        Duration::from_millis(100),
    ),
];

// ============================================================================
// Functional Safety Requirements
// ============================================================================

/// Functional safety requirement
#[derive(Debug, Clone)]
pub struct FunctionalSafetyRequirement {
    pub id: u32,
    pub safety_goal_id: u32,
    pub description: &'static str,
    pub asil: ASIL,
    pub ftti: Duration,
}

impl FunctionalSafetyRequirement {
    pub fn new(id: u32, safety_goal_id: u32, description: &'static str, asil: ASIL, ftti: Duration) -> Self {
        FunctionalSafetyRequirement {
            id,
            safety_goal_id,
            description,
            asil,
            ftti,
        }
    }
}

/// Functional safety requirements for VantisOS
pub const FUNCTIONAL_SAFETY_REQUIREMENTS: &[FunctionalSafetyRequirement] = &[
    FunctionalSafetyRequirement::new(
        1,
        1,
        "Detect sensor faults within 10ms",
        ASIL::D,
        Duration::from_millis(10),
    ),
    FunctionalSafetyRequirement::new(
        2,
        1,
        "Detect control unit faults within 20ms",
        ASIL::D,
        Duration::from_millis(20),
    ),
    FunctionalSafetyRequirement::new(
        3,
        1,
        "Detect actuator faults within 10ms",
        ASIL::D,
        Duration::from_millis(10),
    ),
    FunctionalSafetyRequirement::new(
        4,
        2,
        "Transition to safe state within 100ms of fault detection",
        ASIL::D,
        Duration::from_millis(100),
    ),
    FunctionalSafetyRequirement::new(
        5,
        2,
        "Maintain safety-critical functions with > 99.99% availability",
        ASIL::D,
        Duration::from_millis(100),
    ),
    FunctionalSafetyRequirement::new(
        6,
        3,
        "Prevent unauthorized access to safety-critical functions",
        ASIL::D,
        Duration::from_millis(100),
    ),
    FunctionalSafetyRequirement::new(
        7,
        4,
        "Detect data corruption in safety-critical memory",
        ASIL::D,
        Duration::from_millis(100),
    ),
    FunctionalSafetyRequirement::new(
        8,
        5,
        "Perform periodic self-diagnostics every 100ms",
        ASIL::D,
        Duration::from_millis(100),
    ),
    FunctionalSafetyRequirement::new(
        9,
        5,
        "Log all safety events with timestamps",
        ASIL::D,
        Duration::from_millis(100),
    ),
    FunctionalSafetyRequirement::new(
        10,
        5,
        "Support safe shutdown on critical failures",
        ASIL::D,
        Duration::from_millis(100),
    ),
];

// ============================================================================
// Technical Safety Requirements
// ============================================================================

/// Technical safety requirement
#[derive(Debug, Clone)]
pub struct TechnicalSafetyRequirement {
    pub id: u32,
    pub fsr_id: u32,
    pub description: &'static str,
    pub asil: ASIL,
    pub ftti: Duration,
}

impl TechnicalSafetyRequirement {
    pub fn new(id: u32, fsr_id: u32, description: &'static str, asil: ASIL, ftti: Duration) -> Self {
        TechnicalSafetyRequirement {
            id,
            fsr_id,
            description,
            asil,
            ftti,
        }
    }
}

/// Technical safety requirements for VantisOS
pub const TECHNICAL_SAFETY_REQUIREMENTS: &[TechnicalSafetyRequirement] = &[
    TechnicalSafetyRequirement::new(
        1,
        1,
        "Implement memory protection with hardware support",
        ASIL::D,
        Duration::from_millis(10),
    ),
    TechnicalSafetyRequirement::new(
        2,
        2,
        "Implement task scheduling with safety-critical priority",
        ASIL::D,
        Duration::from_millis(20),
    ),
    TechnicalSafetyRequirement::new(
        3,
        3,
        "Implement I/O safety mechanisms",
        ASIL::D,
        Duration::from_millis(10),
    ),
    TechnicalSafetyRequirement::new(
        4,
        4,
        "Implement watchdog timer with configurable timeout",
        ASIL::D,
        Duration::from_millis(100),
    ),
    TechnicalSafetyRequirement::new(
        5,
        5,
        "Implement error detection and correction (EDAC) for memory",
        ASIL::D,
        Duration::from_millis(100),
    ),
    TechnicalSafetyRequirement::new(
        6,
        6,
        "Implement redundant execution for safety-critical tasks",
        ASIL::D,
        Duration::from_millis(100),
    ),
    TechnicalSafetyRequirement::new(
        7,
        7,
        "Implement lockstep execution for safety-critical functions",
        ASIL::D,
        Duration::from_millis(100),
    ),
    TechnicalSafetyRequirement::new(
        8,
        8,
        "Implement heartbeat monitoring for safety-critical tasks",
        ASIL::D,
        Duration::from_millis(100),
    ),
    TechnicalSafetyRequirement::new(
        9,
        9,
        "Implement safe state transition mechanisms",
        ASIL::D,
        Duration::from_millis(100),
    ),
    TechnicalSafetyRequirement::new(
        10,
        10,
        "Implement fault isolation between safety partitions",
        ASIL::D,
        Duration::from_millis(100),
    ),
];

// ============================================================================
// Safety Mechanisms
// ============================================================================

/// Safety mechanism type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyMechanismType {
    /// Watchdog timer
    Watchdog,
    /// Lockstep execution
    Lockstep,
    /// Redundant execution
    Redundant,
    /// Error detection and correction
    EDAC,
    /// Heartbeat monitoring
    Heartbeat,
    /// Memory protection
    MemoryProtection,
    /// Fault isolation
    FaultIsolation,
    /// Safe state transition
    SafeStateTransition,
}

/// Safety mechanism
#[derive(Debug, Clone)]
pub struct SafetyMechanism {
    pub id: u32,
    pub name: &'static str,
    pub mechanism_type: SafetyMechanismType,
    pub asil: ASIL,
    pub enabled: bool,
    pub diagnostic_coverage: f32,
}

impl SafetyMechanism {
    pub fn new(id: u32, name: &'static str, mechanism_type: SafetyMechanismType, asil: ASIL, diagnostic_coverage: f32) -> Self {
        SafetyMechanism {
            id,
            name,
            mechanism_type,
            asil,
            enabled: true,
            diagnostic_coverage,
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

/// Watchdog timer
#[derive(Debug, Clone)]
pub struct WatchdogTimer {
    pub id: u32,
    pub timeout: Duration,
    pub last_pet: AtomicU64,
    pub enabled: AtomicBool,
    pub triggered: AtomicBool,
}

impl WatchdogTimer {
    pub fn new(id: u32, timeout: Duration) -> Self {
        WatchdogTimer {
            id,
            timeout,
            last_pet: AtomicU64::new(0),
            enabled: AtomicBool::new(false),
            triggered: AtomicBool::new(false),
        }
    }

    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Release);
        self.pet();
    }

    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Release);
    }

    pub fn pet(&self) {
        self.last_pet.store(core::time::SystemTime::now()
            .duration_since(core::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64, Ordering::Release);
    }

    pub fn check(&self) -> bool {
        if !self.enabled.load(Ordering::Acquire) {
            return true;
        }

        let now = core::time::SystemTime::now()
            .duration_since(core::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        let last_pet = self.last_pet.load(Ordering::Acquire);
        let elapsed = now - last_pet;
        
        if elapsed > self.timeout.as_nanos() as u64 {
            self.triggered.store(true, Ordering::Release);
            return false;
        }
        
        true
    }

    pub fn is_triggered(&self) -> bool {
        self.triggered.load(Ordering::Acquire)
    }
}

/// Lockstep execution
#[derive(Debug, Clone)]
pub struct LockstepExecution {
    pub id: u32,
    pub enabled: AtomicBool,
    pub mismatch_count: AtomicU32,
    pub max_mismatches: u32,
}

impl LockstepExecution {
    pub fn new(id: u32, max_mismatches: u32) -> Self {
        LockstepExecution {
            id,
            enabled: AtomicBool::new(true),
            mismatch_count: AtomicU32::new(0),
            max_mismatches,
        }
    }

    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Release);
    }

    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Release);
    }

    pub fn check(&self, result1: u64, result2: u64) -> bool {
        if !self.enabled.load(Ordering::Acquire) {
            return true;
        }

        if result1 != result2 {
            let count = self.mismatch_count.fetch_add(1, Ordering::Release) + 1;
            if count >= self.max_mismatches {
                return false;
            }
        }
        
        true
    }

    pub fn reset_mismatch_count(&self) {
        self.mismatch_count.store(0, Ordering::Release);
    }
}

/// Heartbeat monitoring
#[derive(Debug, Clone)]
pub struct HeartbeatMonitor {
    pub id: u32,
    pub task_id: u32,
    pub interval: Duration,
    pub last_heartbeat: AtomicU64,
    pub enabled: AtomicBool,
    pub missed_count: AtomicU32,
    pub max_missed: u32,
}

impl HeartbeatMonitor {
    pub fn new(id: u32, task_id: u32, interval: Duration, max_missed: u32) -> Self {
        HeartbeatMonitor {
            id,
            task_id,
            interval,
            last_heartbeat: AtomicU64::new(0),
            enabled: AtomicBool::new(true),
            missed_count: AtomicU32::new(0),
            max_missed,
        }
    }

    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Release);
    }

    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Release);
    }

    pub fn heartbeat(&self) {
        self.last_heartbeat.store(core::time::SystemTime::now()
            .duration_since(core::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64, Ordering::Release);
        self.missed_count.store(0, Ordering::Release);
    }

    pub fn check(&self) -> bool {
        if !self.enabled.load(Ordering::Acquire) {
            return true;
        }

        let now = core::time::SystemTime::now()
            .duration_since(core::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        let last_heartbeat = self.last_heartbeat.load(Ordering::Acquire);
        let elapsed = now - last_heartbeat;
        
        if elapsed > self.interval.as_nanos() as u64 {
            let missed = self.missed_count.fetch_add(1, Ordering::Release) + 1;
            if missed >= self.max_missed {
                return false;
            }
        }
        
        true
    }
}

// ============================================================================
// Safety State
// ============================================================================

/// Safety state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyState {
    /// Normal operation
    Normal,
    /// Degraded operation
    Degraded,
    /// Safe state
    Safe,
    /// Emergency state
    Emergency,
}

/// Safety state manager
#[derive(Debug, Clone)]
pub struct SafetyStateManager {
    pub current_state: AtomicU32,
    pub transition_count: AtomicU32,
}

impl SafetyStateManager {
    pub fn new() -> Self {
        SafetyStateManager {
            current_state: AtomicU32::new(SafetyState::Normal as u32),
            transition_count: AtomicU32::new(0),
        }
    }

    pub fn get_state(&self) -> SafetyState {
        match self.current_state.load(Ordering::Acquire) {
            0 => SafetyState::Normal,
            1 => SafetyState::Degraded,
            2 => SafetyState::Safe,
            3 => SafetyState::Emergency,
            _ => SafetyState::Normal,
        }
    }

    pub fn set_state(&self, state: SafetyState) {
        self.current_state.store(state as u32, Ordering::Release);
        self.transition_count.fetch_add(1, Ordering::Release);
    }

    pub fn transition_to_safe(&self) {
        self.set_state(SafetyState::Safe);
    }

    pub fn transition_to_emergency(&self) {
        self.set_state(SafetyState::Emergency);
    }

    pub fn is_safe_state(&self) -> bool {
        matches!(self.get_state(), SafetyState::Safe | SafetyState::Emergency)
    }
}

// ============================================================================
// Safety Statistics
// ============================================================================

/// Safety statistics
#[derive(Debug, Clone)]
pub struct SafetyStatistics {
    pub faults_detected: AtomicU64,
    pub faults_corrected: AtomicU64,
    pub safe_state_transitions: AtomicU64,
    pub uptime_ns: AtomicU64,
    pub downtime_ns: AtomicU64,
}

impl SafetyStatistics {
    pub fn new() -> Self {
        SafetyStatistics {
            faults_detected: AtomicU64::new(0),
            faults_corrected: AtomicU64::new(0),
            safe_state_transitions: AtomicU64::new(0),
            uptime_ns: AtomicU64::new(0),
            downtime_ns: AtomicU64::new(0),
        }
    }

    pub fn record_fault(&self) {
        self.faults_detected.fetch_add(1, Ordering::Release);
    }

    pub fn record_correction(&self) {
        self.faults_corrected.fetch_add(1, Ordering::Release);
    }

    pub fn record_safe_state_transition(&self) {
        self.safe_state_transitions.fetch_add(1, Ordering::Release);
    }

    pub fn get_availability(&self) -> f64 {
        let uptime = self.uptime_ns.load(Ordering::Acquire) as f64;
        let downtime = self.downtime_ns.load(Ordering::Acquire) as f64;
        let total = uptime + downtime;
        
        if total == 0.0 {
            1.0
        } else {
            uptime / total
        }
    }

    pub fn get_failure_rate(&self) -> f64 {
        let faults = self.faults_detected.load(Ordering::Acquire) as f64;
        let uptime = self.uptime_ns.load(Ordering::Acquire) as f64;
        
        if uptime == 0.0 {
            0.0
        } else {
            // Convert to FIT (Failures in Time = failures per billion hours)
            let uptime_hours = uptime / (1_000_000_000.0 * 3600.0);
            (faults / uptime_hours) * 1_000_000_000.0
        }
    }
}

// ============================================================================
// ISO 26262 Manager
// ============================================================================

/// ISO 26262 manager
#[derive(Debug, Clone)]
pub struct ISO26262Manager {
    pub safety_state: SafetyStateManager,
    pub statistics: SafetyStatistics,
    pub watchdog: WatchdogTimer,
    pub lockstep: LockstepExecution,
    pub heartbeat: HeartbeatMonitor,
    pub enabled: AtomicBool,
}

impl ISO26262Manager {
    pub fn new() -> Self {
        ISO26262Manager {
            safety_state: SafetyStateManager::new(),
            statistics: SafetyStatistics::new(),
            watchdog: WatchdogTimer::new(1, Duration::from_millis(100)),
            lockstep: LockstepExecution::new(1, 3),
            heartbeat: HeartbeatMonitor::new(1, 1, Duration::from_millis(100), 3),
            enabled: AtomicBool::new(false),
        }
    }

    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Release);
        self.watchdog.enable();
        self.lockstep.enable();
        self.heartbeat.enable();
    }

    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Release);
        self.watchdog.disable();
        self.lockstep.disable();
        self.heartbeat.disable();
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Acquire)
    }

    pub fn check_safety(&self) -> bool {
        if !self.is_enabled() {
            return true;
        }

        // Check watchdog
        if !self.watchdog.check() {
            self.safety_state.transition_to_emergency();
            self.statistics.record_fault();
            return false;
        }

        // Check heartbeat
        if !self.heartbeat.check() {
            self.safety_state.transition_to_safe();
            self.statistics.record_fault();
            return false;
        }

        true
    }

    pub fn pet_watchdog(&self) {
        self.watchdog.pet();
    }

    pub fn send_heartbeat(&self) {
        self.heartbeat.heartbeat();
    }

    pub fn get_safety_state(&self) -> SafetyState {
        self.safety_state.get_state()
    }

    pub fn get_availability(&self) -> f64 {
        self.statistics.get_availability()
    }

    pub fn get_failure_rate(&self) -> f64 {
        self.statistics.get_failure_rate()
    }
}

/// Global ISO 26262 manager
static GLOBAL_ISO26262: ISO26262Manager = ISO26262Manager::new();

/// Get global ISO 26262 manager
pub fn global_iso26262() -> &'static ISO26262Manager {
    &GLOBAL_ISO26262
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asil_determination() {
        let event = HazardousEvent::new(Severity::S3, Exposure::E4, Controllability::C1);
        let asil = event.determine_asil();
        assert_eq!(asil, ASIL::D);
    }

    #[test]
    fn test_safety_goals() {
        assert_eq!(SAFETY_GOALS.len(), 5);
        assert!(SAFETY_GOALS.iter().all(|sg| sg.asil == ASIL::D));
    }

    #[test]
    fn test_watchdog() {
        let watchdog = WatchdogTimer::new(1, Duration::from_millis(100));
        watchdog.enable();
        assert!(watchdog.check());
        watchdog.pet();
        assert!(!watchdog.is_triggered());
    }

    #[test]
    fn test_lockstep() {
        let lockstep = LockstepExecution::new(1, 3);
        assert!(lockstep.check(42, 42));
        assert!(lockstep.check(42, 43));
        assert!(lockstep.check(42, 43));
        assert!(lockstep.check(42, 43));
        assert!(!lockstep.check(42, 43));
    }

    #[test]
    fn test_heartbeat() {
        let heartbeat = HeartbeatMonitor::new(1, 1, Duration::from_millis(100), 3);
        heartbeat.enable();
        heartbeat.heartbeat();
        assert!(heartbeat.check());
    }

    #[test]
    fn test_safety_state() {
        let state = SafetyStateManager::new();
        assert_eq!(state.get_state(), SafetyState::Normal);
        state.transition_to_safe();
        assert_eq!(state.get_state(), SafetyState::Safe);
        assert!(state.is_safe_state());
    }

    #[test]
    fn test_iso26262_manager() {
        let manager = ISO26262Manager::new();
        assert!(!manager.is_enabled());
        manager.enable();
        assert!(manager.is_enabled());
        manager.pet_watchdog();
        manager.send_heartbeat();
        assert!(manager.check_safety());
    }
}