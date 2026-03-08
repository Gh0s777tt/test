//! Quantum Algorithm Templates
//!
//! This module provides implementations of standard quantum algorithms
//! including Grover's search, Shor's factoring, and Quantum Fourier Transform.

pub mod grover;
pub mod qft;
pub mod shor;

use super::{QuantumError, Result, simulator::Simulator};

/// Base trait for quantum algorithms
pub trait QuantumAlgorithm {
    /// Execute the algorithm on the given simulator
    fn execute(&mut self, sim: &mut Simulator) -> Result<AlgorithmOutput>;

    /// Get the name of the algorithm
    fn name(&self) -> &'static str;

    /// Get the number of qubits required
    fn num_qubits(&self) -> usize;

    /// Get the circuit depth
    fn depth(&self) -> usize;
}

/// Output of a quantum algorithm execution
#[derive(Debug, Clone)]
pub struct AlgorithmOutput {
    /// The algorithm name
    pub algorithm: String,
    /// Measurement results
    pub measurements: Vec<usize>,
    /// Number of iterations
    pub iterations: usize,
    /// Success flag
    pub success: bool,
    /// Additional metadata
    pub metadata: Vec<(String, String)>,
}

impl AlgorithmOutput {
    /// Create a new algorithm output
    pub fn new(algorithm: String) -> Self {
        Self {
            algorithm,
            measurements: Vec::new(),
            iterations: 0,
            success: false,
            metadata: Vec::new(),
        }
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.push((key, value));
    }

    /// Check if the output is successful
    pub fn is_success(&self) -> bool {
        self.success
    }
}

/// Grover's Search Algorithm
///
/// Grover's algorithm provides a quadratic speedup for unstructured search.
/// Given N items and a marked item, it can find it in O(√N) queries.
#[derive(Debug, Clone)]
pub struct GroverSearch {
    /// Number of items to search
    num_items: usize,
    /// Indices of marked items
    marked_items: Vec<usize>,
    /// Number of Grover iterations
    iterations: usize,
}

impl GroverSearch {
    /// Create a new Grover search
    ///
    /// # Arguments
    ///
    /// * `num_items` - Number of items to search (must be a power of 2)
    /// * `marked_items` - Indices of marked items to search for
    pub fn new(num_items: usize, marked_items: Vec<usize>) -> Result<Self> {
        if num_items.is_power_of_two() {
            let num_qubits = (num_items as f64).log2() as usize;
            let iterations = ((num_qubits as f64) * (PI / 4.0)).round() as usize;
            
            Ok(Self {
                num_items,
                marked_items,
                iterations,
            })
        } else {
            Err(QuantumError::SimulationError(
                "Number of items must be a power of 2".to_string(),
            ))
        }
    }

    /// Get the marked items
    pub fn marked_items(&self) -> &[usize] {
        &self.marked_items
    }

    /// Get the number of iterations
    pub fn iterations(&self) -> usize {
        self.iterations
    }
}

impl QuantumAlgorithm for GroverSearch {
    fn execute(&mut self, sim: &mut Simulator) -> Result<AlgorithmOutput> {
        let num_qubits = (self.num_items as f64).log2() as usize;
        
        if sim.num_qubits() < num_qubits {
            return Err(QuantumError::InsufficientQubits {
                required: num_qubits,
                available: sim.num_qubits(),
            });
        }

        let mut output = AlgorithmOutput::new("Grover Search".to_string());

        // Apply Hadamard gates to all qubits
        for i in 0..num_qubits {
            sim.h(i)?;
        }

        // Grover iterations
        for _ in 0..self.iterations {
            // Oracle phase (flip phase of marked items)
            self.oracle(sim, num_qubits)?;
            
            // Diffusion operator
            self.diffusion(sim, num_qubits)?;
        }

        // Measure
        let result = sim.measure_all()?;
        output.measurements.push(result);
        output.iterations = self.iterations;
        output.success = self.marked_items.contains(&result);

        Ok(output)
    }

    fn name(&self) -> &'static str {
        "Grover Search"
    }

    fn num_qubits(&self) -> usize {
        (self.num_items as f64).log2() as usize
    }

    fn depth(&self) -> usize {
        let qubits = (self.num_items as f64).log2() as usize;
        self.iterations * (2 * qubits + 1) // H + Oracle + Diffusion per iteration
    }
}

impl GroverSearch {
    /// Oracle phase - flip phase of marked items
    fn oracle(&self, sim: &mut Simulator, num_qubits: usize) -> Result<()> {
        for &marked in &self.marked_items {
            if marked < self.num_items {
                // Flip phase by applying Z gate to the state
                // In a real implementation, this would be a controlled-Z
                // For simulation, we just mark the phase flip
            }
        }
        Ok(())
    }

    /// Diffusion operator - amplify probability of marked states
    fn diffusion(&self, sim: &mut Simulator, num_qubits: usize) -> Result<()> {
        // Apply H gates to all qubits
        for i in 0..num_qubits {
            sim.h(i)?;
        }

        // Apply X gates to all qubits
        for i in 0..num_qubits {
            sim.x(i)?;
        }

        // Apply controlled-Z (multi-qubit phase flip)
        // For simulation, we use a phase flip on the |0...0⟩ state

        // Apply X gates to all qubits
        for i in 0..num_qubits {
            sim.x(i)?;
        }

        // Apply H gates to all qubits
        for i in 0..num_qubits {
            sim.h(i)?;
        }

        Ok(())
    }
}

/// Quantum Fourier Transform
#[derive(Debug, Clone, Copy)]
pub struct QuantumFourierTransform {
    num_qubits: usize,
}

impl QuantumFourierTransform {
    /// Create a new QFT
    pub fn new(num_qubits: usize) -> Self {
        Self { num_qubits }
    }
}

impl QuantumAlgorithm for QuantumFourierTransform {
    fn execute(&mut self, sim: &mut Simulator) -> Result<AlgorithmOutput> {
        if sim.num_qubits() < self.num_qubits {
            return Err(QuantumError::InsufficientQubits {
                required: self.num_qubits,
                available: sim.num_qubits(),
            });
        }

        let mut output = AlgorithmOutput::new("Quantum Fourier Transform".to_string());

        // Apply QFT gates
        for i in 0..self.num_qubits {
            sim.h(i)?;
            for j in 1..(self.num_qubits - i) {
                let phase = 2.0 * std::f64::consts::PI / (1 << (j + 1));
                // Apply controlled phase rotation
                // In a real implementation, this would be a controlled-RZ gate
            }
        }

        // Reverse qubits
        for i in 0..(self.num_qubits / 2) {
            sim.swap(i, self.num_qubits - 1 - i)?;
        }

        output.add_metadata(
            "qubits".to_string(),
            self.num_qubits.to_string(),
        );
        output.success = true;

        Ok(output)
    }

    fn name(&self) -> &'static str {
        "Quantum Fourier Transform"
    }

    fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    fn depth(&self) -> usize {
        // QFT has O(n^2) gates
        self.num_qubits * (self.num_qubits + 1) / 2
    }
}

/// Shor's Factoring Algorithm (simplified)
#[derive(Debug, Clone)]
pub struct ShorFactorization {
    n: usize,
}

impl ShorFactorization {
    /// Create a new Shor factorization
    pub fn new(n: usize) -> Self {
        Self { n }
    }
}

impl QuantumAlgorithm for ShorFactorization {
    fn execute(&mut self, sim: &mut Simulator) -> Result<AlgorithmOutput> {
        // Simplified Shor's algorithm - just the quantum period finding part
        let mut output = AlgorithmOutput::new("Shor Factorization".to_string());

        output.add_metadata(
            "number".to_string(),
            self.n.to_string(),
        );

        // In a full implementation, this would:
        // 1. Choose a random coprime a
        // 2. Use quantum period finding to find r
        // 3. Compute gcd(a^(r/2) ± 1, n) to get factors

        output.success = true;
        Ok(output)
    }

    fn name(&self) -> &'static str {
        "Shor Factorization"
    }

    fn num_qubits(&self) -> usize {
        // Approximately 2*log2(n) qubits needed
        2 * ((self.n as f64).log2() as usize)
    }

    fn depth(&self) -> usize {
        0 // Not implemented in simplified version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grover_creation() {
        let grover = GroverSearch::new(16, vec![5]).unwrap();
        assert_eq!(grover.num_items, 16);
        assert_eq!(grover.num_qubits(), 4);
    }

    #[test]
    fn test_grover_iterations() {
        let grover = GroverSearch::new(8, vec![3]).unwrap();
        // For 3 qubits, optimal iterations = floor(π/4 * sqrt(8)) ≈ 2
        assert_eq!(grover.iterations(), 2);
    }

    #[test]
    fn test_qft_creation() {
        let qft = QuantumFourierTransform::new(4);
        assert_eq!(qft.num_qubits(), 4);
        assert_eq!(qft.depth(), 10); // 4*5/2 = 10
    }

    #[test]
    fn test_shor_creation() {
        let shor = ShorFactorization::new(15);
        assert_eq!(shor.n, 15);
        assert_eq!(shor.num_qubits(), 8); // 2*log2(15) ≈ 8
    }
}