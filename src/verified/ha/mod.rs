//! # High Availability Module
//! 
//! Ten moduł zawiera funkcje wysokiej dostępności dla systemów serwerowych.

pub mod failover;
pub mod loadbalancer;
pub mod monitoring;
pub mod autoscaling;
pub mod recovery;

pub use failover::{FailoverManager, FailoverConfig, FailoverState};
pub use loadbalancer::{LoadBalancer, LoadBalancerConfig, LoadBalancerAlgorithm};
pub use monitoring::{HealthMonitor, HealthCheck, HealthStatus};
pub use autoscaling::{AutoScaler, ScalingPolicy, ScalingAction};
pub use recovery::{DisasterRecovery, RecoveryPlan, RecoveryAction};

use core::sync::atomic::{AtomicU32, Ordering};

/// Stan systemu HA
static HA_STATE: AtomicU32 = AtomicU32::new(0);

/// Inicjalizuje moduł wysokiej dostępności
pub fn init() -> Result<(), HaError> {
    // Inicjalizuj failover
    failover::init()?;
    
    // Inicjalizuj load balancer
    loadbalancer::init()?;
    
    // Inicjalizuj monitoring
    monitoring::init()?;
    
    // Inicjalizuj auto-scaling
    autoscaling::init()?;
    
    // Inicjalizuj disaster recovery
    recovery::init()?;
    
    HA_STATE.store(1, Ordering::Release);
    
    Ok(())
}

/// Zwraca stan systemu HA
pub fn ha_state() -> bool {
    HA_STATE.load(Ordering::Acquire) == 1
}

/// Błędy HA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HaError {
    /// Błąd failover
    FailoverError,
    /// Błąd load balancera
    LoadBalancerError,
    /// Błąd monitoringu
    MonitoringError,
    /// Błąd auto-scaling
    AutoScalingError,
    /// Błąd disaster recovery
    RecoveryError,
}

impl core::fmt::Display for HaError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HaError::FailoverError => write!(f, "Failover error"),
            HaError::LoadBalancerError => write!(f, "Load balancer error"),
            HaError::MonitoringError => write!(f, "Monitoring error"),
            HaError::AutoScalingError => write!(f, "Auto-scaling error"),
            HaError::RecoveryError => write!(f, "Recovery error"),
        }
    }
}

impl core::error::Error for HaError {}