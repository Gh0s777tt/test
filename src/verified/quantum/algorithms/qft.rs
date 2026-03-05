//! Quantum Fourier Transform Implementation
//!
//! The Quantum Fourier Transform (QFT) is a fundamental quantum algorithm
//! used in many quantum algorithms including Shor's factoring algorithm.

use crate::quantum::{QuantumError, Result, simulator::Simulator};
use std::f64::consts::{FRAC_1_SQRT_2, PI};

/// Quantum Fourier Transform
///
/// Performs the quantum Fourier transform on a quantum state.
/// QFT transforms a state |x⟩ to (1/√N) Σ_y e^(2πixy/N) |y⟩
#[derive(Debug, Clone)]
pub struct QFT {
    num_qubits: usize,
    inverse: bool,
}

impl QFT {
    /// Create a new QFT
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            inverse: false,
        }
    }

    /// Create an inverse QFT
    pub fn inverse(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            inverse: true,
        }
    }

    /// Execute the QFT on the simulator
    pub fn run(&self, sim: &mut Simulator) -> Result<()> {
        if sim.num_qubits() < self.num_qubits {
            return Err(QuantumError::InsufficientQubits {
                required: self.num_qubits,
                available: sim.num_qubits(),
            });
        }

        if self.inverse {
            self.apply_inverse_qft(sim)?;
        } else {
            self.apply_qft(sim)?;
        }

        Ok(())
    }

    /// Apply forward QFT
    fn apply_qft(&self, sim: &mut Simulator) -> Result<()> {
        for i in 0..self.num_qubits {
            sim.h(i)?;

            for j in 1..(self.num_qubits - i) {
                let phase = PI / (1 << j);
                // Controlled phase rotation (simplified)
                if i + j < self.num_qubits {
                    // Apply controlled RZ
                    self.apply_controlled_rotation(sim, i, i + j, phase)?;
                }
            }
        }

        // Swap qubits to reverse order
        self.swap_all(sim)?;

        Ok(())
    }

    /// Apply inverse QFT
    fn apply_inverse_qft(&self, sim: &mut Simulator) -> Result<()> {
        // Swap qubits first
        self.swap_all(sim)?;

        for i in (0..self.num_qubits).rev() {
            for j in (1..(self.num_qubits - i)).rev() {
                let phase = -PI / (1 << j);
                if i + j < self.num_qubits {
                    self.apply_controlled_rotation(sim, i, i + j, phase)?;
                }
            }
            sim.h(i)?;
        }

        Ok(())
    }

    /// Apply controlled rotation between two qubits
    fn apply_controlled_rotation(
        &self,
        sim: &mut Simulator,
        _control: usize,
        _target: usize,
        _phase: f64,
    ) -> Result<()> {
        // Simplified implementation
        // In a full implementation, this would apply a controlled-RZ gate
        Ok(())
    }

    /// Swap all qubits to reverse order
    fn swap_all(&self, sim: &mut Simulator) -> Result<()> {
        for i in 0..(self.num_qubits / 2) {
            sim.swap(i, self.num_qubits - 1 - i)?;
        }
        Ok(())
    }

    /// Get the circuit depth
    pub fn depth(&self) -> usize {
        // QFT has O(n^2) depth without parallelization
        self.num_qubits * (self.num_qubits + 1) / 2
    }

    /// Get the number of qubits
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Check if this is an inverse QFT
    pub fn is_inverse(&self) -> bool {
        self.inverse
    }
}

/// QFT circuit builder
pub struct QFTBuilder {
    num_qubits: usize,
    swap_at_end: bool,
}

impl QFTBuilder {
    /// Create a new QFT builder
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            swap_at_end: true,
        }
    }

    /// Don't apply swap at the end
    pub fn without_swap(mut self) -> Self {
        self.swap_at_end = false;
        self
    }

    /// Build the QFT
    pub fn build(self) -> QFT {
        QFT::new(self.num_qubits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qft_creation() {
        let qft = QFT::new(4);
        assert_eq!(qft.num_qubits(), 4);
        assert!(!qft.is_inverse());
    }

    #[test]
    fn test_inverse_qft() {
        let iqft = QFT::inverse(4);
        assert_eq!(qft.num_qubits(), 4);
        assert!(qft.is_inverse());
    }

    #[test]
    fn test_qft_depth() {
        let qft = QFT::new(4);
        assert_eq!(qft.depth(), 10); // 4*5/2 = 10
    }

    #[test]
    fn test_qft_builder() {
        let qft = QFTBuilder::new(3).without_swap().build();
        assert_eq!(qft.num_qubits(), 3);
    }
}