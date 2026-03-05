//! Quantum Simulator Implementation
//!
//! This module provides a full quantum state vector simulator for
//! simulating quantum circuits on classical hardware.

use super::{QuantumError, Result};
use std::f64::consts::FRAC_1_SQRT_2;
use num_complex::Complex64;

/// Complex number type used in quantum state vectors
pub type C = Complex64;

/// Quantum state vector representation
#[derive(Debug, Clone)]
pub struct StateVector {
    /// Amplitudes for each computational basis state
    amplitudes: Vec<C>,
    /// Number of qubits
    num_qubits: usize,
}

impl StateVector {
    /// Create a new state vector initialized to |0...0⟩
    pub fn new(num_qubits: usize) -> Result<Self> {
        if num_qubits == 0 {
            return Err(QuantumError::SimulationError(
                "Number of qubits must be at least 1".to_string(),
            ));
        }

        let dim = 1usize << num_qubits;
        let mut amplitudes = vec![C::new(0.0, 0.0); dim];
        amplitudes[0] = C::new(1.0, 0.0); // |0...0⟩ state

        Ok(Self { amplitudes, num_qubits })
    }

    /// Get the number of qubits
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Get the dimension of the state vector
    pub fn dim(&self) -> usize {
        self.amplitudes.len()
    }

    /// Get the amplitude at a given index
    pub fn amplitude(&self, index: usize) -> Option<C> {
        self.amplitudes.get(index).copied()
    }

    /// Set the amplitude at a given index
    pub fn set_amplitude(&mut self, index: usize, value: C) -> Result<()> {
        if index >= self.dim() {
            return Err(QuantumError::InvalidQubitIndex {
                index,
                max: self.dim() - 1,
            });
        }
        self.amplitudes[index] = value;
        Ok(())
    }

    /// Normalize the state vector
    pub fn normalize(&mut self) {
        let norm: f64 = self.amplitudes.iter().map(|a| a.norm_sqr()).sum();
        let norm = norm.sqrt();
        if norm > 0.0 {
            for amp in &mut self.amplitudes {
                *amp /= norm;
            }
        }
    }

    /// Calculate the probability of measuring a specific state
    pub fn probability(&self, state: usize) -> f64 {
        self.amplitudes
            .get(state)
            .map(|a| a.norm_sqr())
            .unwrap_or(0.0)
    }
}

/// Quantum gate representation
#[derive(Debug, Clone)]
pub enum Gate {
    /// Hadamard gate
    H,
    /// Pauli-X gate (NOT gate)
    X,
    /// Pauli-Y gate
    Y,
    /// Pauli-Z gate
    Z,
    /// Phase gate (S)
    S,
    /// T gate
    T,
    /// RX rotation gate
    RX(f64),
    /// RY rotation gate
    RY(f64),
    /// RZ rotation gate
    RZ(f64),
    /// CNOT gate (controlled-X)
    CNOT,
    /// SWAP gate
    SWAP,
    /// Toffoli gate (CCX)
    Toffoli,
    /// Custom unitary gate
    Custom(Vec<Vec<C>>),
}

impl Gate {
    /// Get the matrix representation of a single-qubit gate
    pub fn matrix(&self) -> Option<Vec<Vec<C>>> {
        match self {
            Gate::H => Some(vec![
                vec![C::new(FRAC_1_SQRT_2, 0.0), C::new(FRAC_1_SQRT_2, 0.0)],
                vec![C::new(FRAC_1_SQRT_2, 0.0), C::new(-FRAC_1_SQRT_2, 0.0)],
            ]),
            Gate::X => Some(vec![
                vec![C::new(0.0, 0.0), C::new(1.0, 0.0)],
                vec![C::new(1.0, 0.0), C::new(0.0, 0.0)],
            ]),
            Gate::Y => Some(vec![
                vec![C::new(0.0, 0.0), C::new(0.0, -1.0)],
                vec![C::new(0.0, 1.0), C::new(0.0, 0.0)],
            ]),
            Gate::Z => Some(vec![
                vec![C::new(1.0, 0.0), C::new(0.0, 0.0)],
                vec![C::new(0.0, 0.0), C::new(-1.0, 0.0)],
            ]),
            Gate::S => Some(vec![
                vec![C::new(1.0, 0.0), C::new(0.0, 0.0)],
                vec![C::new(0.0, 0.0), C::new(0.0, 1.0)],
            ]),
            Gate::T => Some(vec![
                vec![C::new(1.0, 0.0), C::new(0.0, 0.0)],
                vec![C::new(0.0, 0.0), C::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2)],
            ]),
            Gate::RX(theta) => {
                let c = theta.cos();
                let s = theta.sin();
                Some(vec![
                    vec![C::new(c, 0.0), C::new(0.0, -s)],
                    vec![C::new(0.0, -s), C::new(c, 0.0)],
                ])
            }
            Gate::RY(theta) => {
                let c = theta.cos();
                let s = theta.sin();
                Some(vec![
                    vec![C::new(c, 0.0), C::new(-s, 0.0)],
                    vec![C::new(s, 0.0), C::new(c, 0.0)],
                ])
            }
            Gate::RZ(theta) => {
                let e = (Complex64::i() * (*theta) / 2.0).exp();
                Some(vec![
                    vec![e.conj(), C::new(0.0, 0.0)],
                    vec![C::new(0.0, 0.0), e],
                ])
            }
            Gate::CNOT | Gate::SWAP | Gate::Toffoli => None, // Multi-qubit gates
            Gate::Custom(m) => Some(m.clone()),
        }
    }

    /// Check if this is a multi-qubit gate
    pub fn is_multi_qubit(&self) -> bool {
        matches!(self, Gate::CNOT | Gate::SWAP | Gate::Toffoli)
    }
}

/// Quantum circuit simulator
pub struct Simulator {
    /// Current state vector
    state: StateVector,
    /// Number of gates applied
    gate_count: usize,
    /// Circuit depth
    depth: usize,
    /// Random number generator seed
    seed: Option<u64>,
}

impl Simulator {
    /// Create a new simulator with the given number of qubits
    pub fn new(num_qubits: usize) -> Result<Self> {
        let state = StateVector::new(num_qubits)?;
        Ok(Self {
            state,
            gate_count: 0,
            depth: 0,
            seed: None,
        })
    }

    /// Get the number of qubits
    pub fn num_qubits(&self) -> usize {
        self.state.num_qubits()
    }

    /// Get the current state vector
    pub fn state(&self) -> &StateVector {
        &self.state
    }

    /// Set the random seed
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = Some(seed);
    }

    /// Reset the state to |0...0⟩
    pub fn reset(&mut self) {
        self.state = StateVector::new(self.state.num_qubits()).unwrap();
        self.gate_count = 0;
        self.depth = 0;
    }

    /// Apply a single-qubit gate to the specified qubit
    pub fn apply_single_qubit_gate(&mut self, gate: &Gate, qubit: usize) -> Result<()> {
        let matrix = gate.matrix().ok_or_else(|| {
            QuantumError::GateError(format!("{:?} is not a single-qubit gate", gate))
        })?;

        let n = self.state.num_qubits();
        if qubit >= n {
            return Err(QuantumError::InvalidQubitIndex {
                index: qubit,
                max: n - 1,
            });
        }

        let dim = self.state.dim();
        let mut new_amps = self.state.amplitudes.clone();

        // Apply gate using tensor product
        let step = 1usize << qubit;
        let step2 = step << 1;

        for i in (0..dim).step_by(step2) {
            for j in 0..step {
                let idx0 = i + j;
                let idx1 = i + j + step;

                let a0 = self.state.amplitudes[idx0];
                let a1 = self.state.amplitudes[idx1];

                new_amps[idx0] = matrix[0][0] * a0 + matrix[0][1] * a1;
                new_amps[idx1] = matrix[1][0] * a0 + matrix[1][1] * a1;
            }
        }

        self.state.amplitudes = new_amps;
        self.gate_count += 1;
        self.depth += 1;

        Ok(())
    }

    /// Apply Hadamard gate
    pub fn h(&mut self, qubit: usize) -> Result<()> {
        self.apply_single_qubit_gate(&Gate::H, qubit)
    }

    /// Apply Pauli-X gate
    pub fn x(&mut self, qubit: usize) -> Result<()> {
        self.apply_single_qubit_gate(&Gate::X, qubit)
    }

    /// Apply Pauli-Y gate
    pub fn y(&mut self, qubit: usize) -> Result<()> {
        self.apply_single_qubit_gate(&Gate::Y, qubit)
    }

    /// Apply Pauli-Z gate
    pub fn z(&mut self, qubit: usize) -> Result<()> {
        self.apply_single_qubit_gate(&Gate::Z, qubit)
    }

    /// Apply CNOT gate
    pub fn cnot(&mut self, control: usize, target: usize) -> Result<()> {
        let n = self.state.num_qubits();
        if control >= n || target >= n {
            return Err(QuantumError::InvalidQubitIndex {
                index: usize::max(control, target),
                max: n - 1,
            });
        }

        let dim = self.state.dim();
        let control_bit = 1usize << control;
        let target_bit = 1usize << target;

        for i in 0..dim {
            if i & control_bit != 0 {
                let j = i ^ target_bit;
                if i < j {
                    self.state.amplitudes.swap(i, j);
                }
            }
        }

        self.gate_count += 1;
        self.depth += 1;

        Ok(())
    }

    /// Apply SWAP gate
    pub fn swap(&mut self, qubit1: usize, qubit2: usize) -> Result<()> {
        let n = self.state.num_qubits();
        if qubit1 >= n || qubit2 >= n {
            return Err(QuantumError::InvalidQubitIndex {
                index: usize::max(qubit1, qubit2),
                max: n - 1,
            });
        }

        // SWAP can be decomposed into 3 CNOTs
        self.cnot(qubit1, qubit2)?;
        self.cnot(qubit2, qubit1)?;
        self.cnot(qubit1, qubit2)?;

        Ok(())
    }

    /// Measure a specific qubit
    pub fn measure(&mut self, qubit: usize) -> Result<u8> {
        let n = self.state.num_qubits();
        if qubit >= n {
            return Err(QuantumError::InvalidQubitIndex {
                index: qubit,
                max: n - 1,
            });
        }

        // Calculate probability of |1⟩ for this qubit
        let mut prob_one = 0.0;
        let qubit_bit = 1usize << qubit;

        for (i, amp) in self.state.amplitudes.iter().enumerate() {
            if i & qubit_bit != 0 {
                prob_one += amp.norm_sqr();
            }
        }

        // Simulate measurement using deterministic "random" for reproducibility
        let random = self.seed.map(|s| {
            // Simple deterministic "random" based on state
            let state_sum: f64 = self.state.amplitudes.iter()
                .enumerate()
                .map(|(i, a)| (i as f64 + a.re) * a.im)
                .sum();
            ((state_sum.abs() * 1e10) as u64 % 1000) as f64 / 1000.0
        }).unwrap_or(0.5);

        let result = if random < prob_one { 1u8 } else { 0u8 };

        // Collapse the state
        let mask = result as usize << qubit;
        for i in 0..self.state.amplitudes.len() {
            if (i & qubit_bit != 0) != (result == 1) {
                self.state.amplitudes[i] = C::new(0.0, 0.0);
            }
        }
        self.state.normalize();

        Ok(result)
    }

    /// Measure all qubits and return the result as an integer
    pub fn measure_all(&mut self) -> Result<usize> {
        let mut result = 0usize;
        for qubit in 0..self.state.num_qubits() {
            let bit = self.measure(qubit)? as usize;
            result |= bit << qubit;
        }
        Ok(result)
    }

    /// Get the gate count
    pub fn gate_count(&self) -> usize {
        self.gate_count
    }

    /// Get the circuit depth
    pub fn depth(&self) -> usize {
        self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_vector_creation() {
        let sv = StateVector::new(2).unwrap();
        assert_eq!(sv.num_qubits(), 2);
        assert_eq!(sv.dim(), 4);
    }

    #[test]
    fn test_state_vector_initial_state() {
        let sv = StateVector::new(3).unwrap();
        assert_eq!(sv.probability(0), 1.0);
        assert_eq!(sv.probability(1), 0.0);
    }

    #[test]
    fn test_simulator_creation() {
        let sim = Simulator::new(4).unwrap();
        assert_eq!(sim.num_qubits(), 4);
    }

    #[test]
    fn test_hadamard_gate() {
        let mut sim = Simulator::new(1).unwrap();
        sim.h(0).unwrap();
        
        let state = sim.state();
        let amp0 = state.amplitude(0).unwrap();
        let amp1 = state.amplitude(1).unwrap();
        
        // After H, amplitudes should be 1/sqrt(2)
        assert!((amp0.re - FRAC_1_SQRT_2).abs() < 1e-10);
        assert!((amp1.re - FRAC_1_SQRT_2).abs() < 1e-10);
    }

    #[test]
    fn test_x_gate() {
        let mut sim = Simulator::new(1).unwrap();
        sim.x(0).unwrap();
        
        let state = sim.state();
        assert_eq!(state.probability(1), 1.0);
        assert_eq!(state.probability(0), 0.0);
    }

    #[test]
    fn test_cnot_gate() {
        let mut sim = Simulator::new(2).unwrap();
        sim.h(0).unwrap(); // Put qubit 0 in superposition
        sim.cnot(0, 1).unwrap(); // Entangle with qubit 1
        
        // After H on qubit 0 and CNOT, we should have a Bell state
        // |00⟩ + |11⟩ normalized
        let state = sim.state();
        assert!((state.probability(0) - 0.5).abs() < 1e-10);
        assert!((state.probability(3) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_gate_count() {
        let mut sim = Simulator::new(2).unwrap();
        sim.h(0).unwrap();
        sim.x(1).unwrap();
        sim.cnot(0, 1).unwrap();
        assert_eq!(sim.gate_count(), 3);
    }
}