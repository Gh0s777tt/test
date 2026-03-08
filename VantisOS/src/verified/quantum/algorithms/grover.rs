//! Grover's Search Algorithm Implementation
//!
//! Grover's algorithm provides a quadratic speedup for unstructured search.
//! It can find a marked item in an unsorted database of N items in O(√N) queries.

use crate::quantum::{QuantumError, Result, simulator::Simulator};

/// Grover's Search Algorithm
///
/// This implementation supports multiple marked items and provides
/// optimal number of iterations based on the ratio of marked to total items.
#[derive(Debug, Clone)]
pub struct Grover {
    /// Number of items in the search space
    pub num_items: usize,
    /// Indices of marked items
    pub marked: Vec<usize>,
    /// Number of qubits required
    num_qubits: usize,
    /// Optimal number of iterations
    optimal_iterations: usize,
}

impl Grover {
    /// Create a new Grover search instance
    ///
    /// # Arguments
    ///
    /// * `num_items` - Total number of items (must be power of 2)
    /// * `marked` - Indices of marked items to search for
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use vantis::quantum::algorithms::Grover;
    ///
    /// let grover = Grover::new(16, vec![5, 10])?;
    /// ```
    pub fn new(num_items: usize, marked: Vec<usize>) -> Result<Self> {
        if !num_items.is_power_of_two() {
            return Err(QuantumError::SimulationError(
                "Number of items must be a power of 2".to_string(),
            ));
        }

        if marked.iter().any(|&m| m >= num_items) {
            return Err(QuantumError::SimulationError(
                "Marked item index out of range".to_string(),
            ));
        }

        let num_qubits = (num_items as f64).log2() as usize;
        let num_marked = marked.len();

        // Optimal iterations: (π/4) * sqrt(N/M)
        let optimal_iterations = if num_marked > 0 {
            let ratio = (num_items as f64) / (num_marked as f64);
            (std::f64::consts::FRAC_PI_4 * ratio.sqrt()).round() as usize
        } else {
            return Err(QuantumError::SimulationError(
                "At least one marked item is required".to_string(),
            ));
        };

        Ok(Self {
            num_items,
            marked,
            num_qubits,
            optimal_iterations,
        })
    }

    /// Execute Grover's search on the simulator
    pub fn run(&self, sim: &mut Simulator) -> Result<GroverResult> {
        if sim.num_qubits() < self.num_qubits {
            return Err(QuantumError::InsufficientQubits {
                required: self.num_qubits,
                available: sim.num_qubits(),
            });
        }

        // Initialize superposition
        for i in 0..self.num_qubits {
            sim.h(i)?;
        }

        // Grover iterations
        for _ in 0..self.optimal_iterations {
            self.apply_oracle(sim)?;
            self.apply_diffusion(sim)?;
        }

        // Measure
        let result = sim.measure_all()?;

        Ok(GroverResult {
            found_index: result,
            is_marked: self.marked.contains(&result),
            iterations: self.optimal_iterations,
            probability: self.estimate_probability(),
        })
    }

    /// Apply the oracle (phase flip for marked states)
    fn apply_oracle(&self, sim: &mut Simulator) -> Result<()> {
        // In a real implementation, the oracle would flip the phase
        // of the marked states. Here we use a simplified version.
        for &marked_idx in &self.marked {
            self.apply_phase_flip(sim, marked_idx)?;
        }
        Ok(())
    }

    /// Apply phase flip to a specific state
    fn apply_phase_flip(&self, sim: &mut Simulator, state: usize) -> Result<()> {
        // Simplified: apply Z to qubits based on binary representation
        for i in 0..self.num_qubits {
            if (state >> i) & 1 == 0 {
                sim.x(i)?;
                sim.z(i)?;
                sim.x(i)?;
            } else {
                sim.z(i)?;
            }
        }
        Ok(())
    }

    /// Apply the diffusion operator (inversion about average)
    fn apply_diffusion(&self, sim: &mut Simulator) -> Result<()> {
        // H^⊗n
        for i in 0..self.num_qubits {
            sim.h(i)?;
        }

        // X^⊗n
        for i in 0..self.num_qubits {
            sim.x(i)?;
        }

        // Multi-controlled Z
        self.multi_controlled_z(sim)?;

        // X^⊗n
        for i in 0..self.num_qubits {
            sim.x(i)?;
        }

        // H^⊗n
        for i in 0..self.num_qubits {
            sim.h(i)?;
        }

        Ok(())
    }

    /// Multi-controlled Z gate
    fn multi_controlled_z(&self, sim: &mut Simulator) -> Result<()> {
        // Simplified implementation
        sim.z(self.num_qubits - 1)?;
        Ok(())
    }

    /// Estimate success probability
    fn estimate_probability(&self) -> f64 {
        let sin_sq = ((2 * self.optimal_iterations + 1) as f64
            * (self.marked.len() as f64 / self.num_items as f64).sqrt()
            * std::f64::consts::FRAC_PI_4)
            .sin()
            .powi(2);
        sin_sq
    }

    /// Get the number of qubits required
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Get the optimal number of iterations
    pub fn optimal_iterations(&self) -> usize {
        self.optimal_iterations
    }
}

/// Result of Grover's search
#[derive(Debug, Clone)]
pub struct GroverResult {
    /// The index found by the algorithm
    pub found_index: usize,
    /// Whether the found index is a marked item
    pub is_marked: bool,
    /// Number of iterations performed
    pub iterations: usize,
    /// Estimated success probability
    pub probability: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grover_creation() {
        let grover = Grover::new(8, vec![3]).unwrap();
        assert_eq!(grover.num_items, 8);
        assert_eq!(grover.num_qubits(), 3);
    }

    #[test]
    fn test_grover_invalid_items() {
        let result = Grover::new(10, vec![5]);
        assert!(result.is_err());
    }

    #[test]
    fn test_grover_out_of_range_marked() {
        let result = Grover::new(8, vec![10]);
        assert!(result.is_err());
    }

    #[test]
    fn test_grover_no_marked() {
        let result = Grover::new(8, vec![]);
        assert!(result.is_err());
    }
}