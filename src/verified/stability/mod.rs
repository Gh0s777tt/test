//! Stability & Reliability Module
//! 
//! This module provides comprehensive stability and reliability features
//! for VantisOS production deployment, including stress testing, memory
//! leak detection, race condition detection, deadlock prevention, and
//! crash recovery mechanisms.

pub mod stress;
pub mod memory;
pub mod race;
pub mod deadlock;
pub mod crash;
pub mod health;

use alloc::sync::Arc;
use spin::Mutex;

/// Stability manager that coordinates all stability features
pub struct StabilityManager {
    state: Arc<Mutex<StabilityState>>,
}

impl StabilityManager {
    /// Create a new stability manager
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(StabilityState::new())),
        }
    }

    /// Initialize the stability manager
    pub fn init(&self) -> Result<(), StabilityError> {
        let mut state = self.state.lock();
        state.initialized = true;
        Ok(())
    }

    /// Get the current stability state
    pub fn state(&self) -> StabilityState {
        self.state.lock().clone()
    }

    /// Get health status
    pub fn health(&self) -> HealthStatus {
        let state = self.state.lock();
        if state.memory_leaks_detected > 0 {
            HealthStatus::Degraded
        } else if state.race_conditions_detected > 0 {
            HealthStatus::Degraded
        } else if state.deadlocks_detected > 0 {
            HealthStatus::Critical
        } else if state.crashes_count > 0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        }
    }
}

impl Default for StabilityManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Stability state
#[derive(Debug, Clone, Copy)]
pub struct StabilityState {
    pub initialized: bool,
    pub memory_leaks_detected: u64,
    pub race_conditions_detected: u64,
    pub deadlocks_detected: u64,
    pub crashes_count: u64,
    pub uptime_seconds: u64,
}

impl StabilityState {
    fn new() -> Self {
        Self {
            initialized: false,
            memory_leaks_detected: 0,
            race_conditions_detected: 0,
            deadlocks_detected: 0,
            crashes_count: 0,
            uptime_seconds: 0,
        }
    }
}

/// Health status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Critical,
    Unknown,
}

/// Stability error types
#[derive(Debug, Clone, Copy)]
pub enum StabilityError {
    NotInitialized,
    InitializationFailed,
    MemoryLeakDetected,
    RaceConditionDetected,
    DeadlockDetected,
    CrashDetected,
    TestFailed,
    RecoveryFailed,
}

impl core::fmt::Display for StabilityError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "Stability manager not initialized"),
            Self::InitializationFailed => write!(f, "Stability manager initialization failed"),
            Self::MemoryLeakDetected => write!(f, "Memory leak detected"),
            Self::RaceConditionDetected => write!(f, "Race condition detected"),
            Self::DeadlockDetected => write!(f, "Deadlock detected"),
            Self::CrashDetected => write!(f, "Crash detected"),
            Self::TestFailed => write!(f, "Stability test failed"),
            Self::RecoveryFailed => write!(f, "Recovery operation failed"),
        }
    }
}

/// Global stability manager instance
static STABILITY_MANAGER: spin::Once<StabilityManager> = spin::Once::new();

/// Get the global stability manager
pub fn stability_manager() -> &'static StabilityManager {
    STABILITY_MANAGER.call_once(|| StabilityManager::new())
}

/// Initialize the stability subsystem
pub fn init() -> Result<(), StabilityError> {
    stability_manager().init()
}