//! # VantisOS Quantum Computing Module
//!
//! This module provides quantum computing capabilities for VantisOS,
//! including quantum simulation, quantum algorithms, and quantum-classical
//! hybrid execution.
//!
//! ## Features
//!
//! - Quantum simulator framework
//! - Quantum gate operations (H, X, Y, Z, CNOT, SWAP)
//! - Quantum algorithm templates (Grover, Shor, QFT)
//! - Quantum error correction codes
//! - Quantum memory management
//! - Quantum process scheduling
//!
//! ## Architecture
//!
//! The quantum module is organized into several submodules:
//!
//! - `mod.rs` - Main module definition and exports
//! - `simulator` - Quantum simulator implementation
//! - `gates` - Quantum gate operations
//! - `algorithms` - Quantum algorithm templates
//! - `memory` - Quantum memory management
//! - `scheduler` - Quantum process scheduling
//! - `error_correction` - Quantum error correction codes
//!
//! ## Usage
//!
//! ```rust,no_run
//! use vantis::quantum::Simulator;
//! use vantis::quantum::gates::{H, X, CNOT};
//!
//! let mut sim = Simulator::new(2); // 2 qubits
//! sim.apply_gate(H::new(), 0);    // Apply Hadamard to qubit 0
//! sim.apply_gate(CNOT::new(), &[0, 1]); // Apply CNOT
//! let result = sim.measure_all();
//! ```

pub mod algorithms;
pub mod error_correction;
pub mod gates;
pub mod memory;
pub mod scheduler;
pub mod simulator;

use crate::verified::Vault;

/// Quantum computing configuration
#[derive(Debug, Clone)]
pub struct QuantumConfig {
    /// Number of qubits in the system
    pub num_qubits: usize,
    /// Coherence time in microseconds
    pub coherence_time: u64,
    /// Error rate per gate operation
    pub error_rate: f64,
    /// Whether to use error correction
    pub enable_error_correction: bool,
}

impl Default for QuantumConfig {
    fn default() -> Self {
        Self {
            num_qubits: 64,
            coherence_time: 100, // 100μs
            error_rate: 1e-4,    // 0.01%
            enable_error_correction: true,
        }
    }
}

impl QuantumConfig {
    /// Create a new quantum configuration
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            ..Default::default()
        }
    }

    /// Set the coherence time
    pub fn with_coherence_time(mut self, time: u64) -> Self {
        self.coherence_time = time;
        self
    }

    /// Set the error rate
    pub fn with_error_rate(mut self, rate: f64) -> Self {
        self.error_rate = rate;
        self
    }

    /// Enable or disable error correction
    pub fn with_error_correction(mut self, enabled: bool) -> Self {
        self.enable_error_correction = enabled;
        self
    }
}

/// Quantum computing system
pub struct QuantumSystem {
    config: QuantumConfig,
    simulator: simulator::Simulator,
    vault: Vault,
}

impl QuantumSystem {
    /// Create a new quantum system
    pub fn new(config: QuantumConfig) -> Result<Self, QuantumError> {
        let sim = simulator::Simulator::new(config.num_qubits)?;
        Ok(Self {
            config,
            simulator: sim,
            vault: Vault::new(),
        })
    }

    /// Get the simulator
    pub fn simulator(&self) -> &simulator::Simulator {
        &self.simulator
    }

    /// Get mutable simulator
    pub fn simulator_mut(&mut self) -> &mut simulator::Simulator {
        &mut self.simulator
    }

    /// Get the vault
    pub fn vault(&self) -> &Vault {
        &self.vault
    }

    /// Get the configuration
    pub fn config(&self) -> &QuantumConfig {
        &self.config
    }
}

/// Quantum error type
#[derive(Debug, thiserror::Error)]
pub enum QuantumError {
    #[error("Insufficient qubits: required {required}, available {available}")]
    InsufficientQubits { required: usize, available: usize },
    
    #[error("Invalid qubit index: {index} (max: {max})")]
    InvalidQubitIndex { index: usize, max: usize },
    
    #[error("Coherence time exceeded: {elapsed}μs > {limit}μs")]
    CoherenceExceeded { elapsed: u64, limit: u64 },
    
    #[error("Simulation error: {0}")]
    SimulationError(String),
    
    #[error("Quantum gate error: {0}")]
    GateError(String),
    
    #[error("Measurement error: {0}")]
    MeasurementError(String),
}

/// Result type for quantum operations
pub type Result<T> = std::result::Result<T, QuantumError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_config_default() {
        let config = QuantumConfig::default();
        assert_eq!(config.num_qubits, 64);
        assert_eq!(config.coherence_time, 100);
        assert!(config.error_rate < 0.001);
    }

    #[test]
    fn test_quantum_config_builder() {
        let config = QuantumConfig::new(32)
            .with_coherence_time(200)
            .with_error_rate(1e-5)
            .with_error_correction(false);
        
        assert_eq!(config.num_qubits, 32);
        assert_eq!(config.coherence_time, 200);
        assert_eq!(config.error_rate, 1e-5);
        assert!(!config.enable_error_correction);
    }

    #[test]
    fn test_quantum_system_creation() {
        let config = QuantumConfig::new(8);
        let system = QuantumSystem::new(config);
        assert!(system.is_ok());
    }
}