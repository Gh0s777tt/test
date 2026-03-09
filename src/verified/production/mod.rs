//! Production Readiness Module
//! 
//! This module provides production readiness features for VantisOS including
//! deployment guides, operations manuals, troubleshooting guides, SLA documentation,
//! and support procedures.

pub mod deployment;
pub mod operations;
pub mod troubleshooting;
pub mod sla;
pub mod support;

use alloc::sync::Arc;
use spin::Mutex;

/// Production status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductionStatus {
    Development,
    Testing,
    Staging,
    Production,
    Maintenance,
}

/// Production environment type
#[derive(Debug, Clone, Copy)]
pub enum ProductionEnvironment {
    Development,
    Testing,
    Staging,
    Production,
}

/// Production manager that coordinates all production readiness features
pub struct ProductionManager {
    status: Arc<Mutex<ProductionStatus>>,
    environment: Arc<Mutex<ProductionEnvironment>>,
}

impl ProductionManager {
    /// Create a new production manager
    pub fn new() -> Self {
        Self {
            status: Arc::new(Mutex::new(ProductionStatus::Development)),
            environment: Arc::new(Mutex::new(ProductionEnvironment::Development)),
        }
    }

    /// Set production status
    pub fn set_status(&self, status: ProductionStatus) {
        *self.status.lock() = status;
    }

    /// Set production environment
    pub fn set_environment(&self, environment: ProductionEnvironment) {
        *self.environment.lock() = environment;
    }

    /// Get production status
    pub fn status(&self) -> ProductionStatus {
        *self.status.lock()
    }

    /// Get production environment
    pub fn environment(&self) -> ProductionEnvironment {
        *self.environment.lock()
    }

    /// Check if in production
    pub fn is_production(&self) -> bool {
        *self.environment.lock() == ProductionEnvironment::Production
    }
}

impl Default for ProductionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global production manager instance
static PRODUCTION_MANAGER: spin::Once<ProductionManager> = spin::Once::new();

/// Get the global production manager
pub fn production_manager() -> &'static ProductionManager {
    PRODUCTION_MANAGER.call_once(|| ProductionManager::new())
}

/// Set production status
pub fn set_production_status(status: ProductionStatus) {
    production_manager().set_status(status);
}

/// Set production environment
pub fn set_production_environment(environment: ProductionEnvironment) {
    production_manager().set_environment(environment);
}

/// Check if in production
pub fn is_production() -> bool {
    production_manager().is_production()
}