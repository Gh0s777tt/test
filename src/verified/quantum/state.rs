// Quantum State Operations for VantisOS
// Comprehensive quantum state manipulation and analysis

use num_complex::Complex64;
use ndarray::{Array1, Array2};
use std::f64::consts::FRAC_1_SQRT_2;

/// Quantum state representation
#[derive(Clone, Debug)]
pub struct QuantumState {
    /// State vector
    state: Array1<Complex64>,
    /// Number of qubits
    num_qubits: usize,
}

impl QuantumState {
    /// Create a new quantum state from a state vector
    pub fn new(state: Array1<Complex64>) -> Result<Self, String> {
        let num_qubits = (state.len() as f64).log2() as usize;
        
        if (1 << num_qubits) != state.len() {
            return Err("State vector length must be a power of 2".to_string());
        }
        
        let norm = state.iter().map(|c| c.norm_sqr()).sum::<f64>().sqrt();
        if (norm - 1.0).abs() > 1e-10 {
            return Err("State must be normalized".to_string());
        }
        
        Ok(QuantumState { state, num_qubits })
    }

    /// Create the |0⟩ state
    pub fn zero(num_qubits: usize) -> Self {
        let size = 1 << num_qubits;
        let mut state = Array1::zeros(size);
        state[0] = Complex64::new(1.0, 0.0);
        QuantumState { state, num_qubits }
    }

    /// Create the |1⟩ state
    pub fn one(num_qubits: usize) -> Self {
        let size = 1 << num_qubits;
        let mut state = Array1::zeros(size);
        state[1] = Complex64::new(1.0, 0.0);
        QuantumState { state, num_qubits }
    }

    /// Create a superposition state (|0⟩ + |1⟩)/√2
    pub fn plus() -> Self {
        let h = FRAC_1_SQRT_2;
        let state = Array1::from(vec![
            Complex64::new(h, 0.0),
            Complex64::new(h, 0.0),
        ]);
        QuantumState { state, num_qubits: 1 }
    }

    /// Create a superposition state (|0⟩ - |1⟩)/√2
    pub fn minus() -> Self {
        let h = FRAC_1_SQRT_2;
        let state = Array1::from(vec![
            Complex64::new(h, 0.0),
            Complex64::new(-h, 0.0),
        ]);
        QuantumState { state, num_qubits: 1 }
    }

    /// Create a Bell state (|00⟩ + |11⟩)/√2
    pub fn bell_state() -> Self {
        let h = FRAC_1_SQRT_2;
        let state = Array1::from(vec![
            Complex64::new(h, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(h, 0.0),
        ]);
        QuantumState { state, num_qubits: 2 }
    }

    /// Create a GHZ state (|000...0⟩ + |111...1⟩)/√2
    pub fn ghz_state(num_qubits: usize) -> Self {
        let size = 1 << num_qubits;
        let mut state = Array1::zeros(size);
        let h = FRAC_1_SQRT_2;
        state[0] = Complex64::new(h, 0.0);
        state[size - 1] = Complex64::new(h, 0.0);
        QuantumState { state, num_qubits }
    }

    /// Create a W state (|001⟩ + |010⟩ + |100⟩)/√3
    pub fn w_state(num_qubits: usize) -> Result<Self, String> {
        if num_qubits < 3 {
            return Err("W state requires at least 3 qubits".to_string());
        }
        
        let size = 1 << num_qubits;
        let mut state = Array1::zeros(size);
        let amplitude = 1.0 / (num_qubits as f64).sqrt();
        
        for i in 0..num_qubits {
            let index = 1 << i;
            state[index] = Complex64::new(amplitude, 0.0);
        }
        
        Ok(QuantumState { state, num_qubits })
    }

    /// Get the state vector
    pub fn state_vector(&self) -> &Array1<Complex64> {
        &self.state
    }

    /// Get the number of qubits
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Get the amplitude of a specific basis state
    pub fn amplitude(&self, index: usize) -> Complex64 {
        if index < self.state.len() {
            self.state[index]
        } else {
            Complex64::new(0.0, 0.0)
        }
    }

    /// Get the probability of measuring a specific basis state
    pub fn probability(&self, index: usize) -> f64 {
        if index < self.state.len() {
            self.state[index].norm_sqr()
        } else {
            0.0
        }
    }

    /// Check if the state is pure
    pub fn is_pure(&self) -> bool {
        let density = self.density_matrix();
        let trace = (density.dot(&density)).diag().sum();
        (trace.re - 1.0).abs() < 1e-10
    }

    /// Check if the state is mixed
    pub fn is_mixed(&self) -> bool {
        !self.is_pure()
    }

    /// Compute the density matrix
    pub fn density_matrix(&self) -> Array2<Complex64> {
        let n = self.state.len();
        let mut density = Array2::zeros((n, n));
        
        for i in 0..n {
            for j in 0..n {
                density[(i, j)] = self.state[i] * self.state[j].conj();
            }
        }
        
        density
    }

    /// Compute the purity of the state
    pub fn purity(&self) -> f64 {
        let density = self.density_matrix();
        let trace = (density.dot(&density)).diag().sum();
        trace.re
    }

    /// Compute the von Neumann entropy
    pub fn entropy(&self) -> f64 {
        let density = self.density_matrix();
        let eigenvalues = density.eigenvalues(false);
        
        eigenvalues
            .iter()
            .filter(|&&e| e.re > 1e-10)
            .map(|&e| -e.re * e.re.ln())
            .sum()
    }

    /// Compute the fidelity with another state
    pub fn fidelity(&self, other: &QuantumState) -> Result<f64, String> {
        if self.num_qubits != other.num_qubits {
            return Err("Qubit count mismatch".to_string());
        }
        
        let inner_product = self.state.iter().zip(other.state.iter())
            .map(|(&s, &o)| s.conj() * o)
            .sum::<Complex64>();
        
        Ok(inner_product.norm_sqr())
    }

    /// Compute the trace distance to another state
    pub fn trace_distance(&self, other: &QuantumState) -> Result<f64, String> {
        if self.num_qubits != other.num_qubits {
            return Err("Qubit count mismatch".to_string());
        }
        
        let diff = &self.density_matrix() - &other.density_matrix();
        let eigenvalues = diff.eigenvalues(false);
        
        Ok(eigenvalues.iter().map(|&e| e.re.abs()).sum::<f64>() / 2.0)
    }

    /// Compute the Bloch vector (for single qubit states)
    pub fn bloch_vector(&self) -> Result<(f64, f64, f64), String> {
        if self.num_qubits != 1 {
            return Err("Bloch vector only defined for single qubit states".to_string());
        }
        
        let pauli_x = Array2::from(vec![
            [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        ]);
        
        let pauli_y = Array2::from(vec![
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, -1.0)],
            [Complex64::new(0.0, 1.0), Complex64::new(0.0, 0.0)],
        ]);
        
        let pauli_z = Array2::from(vec![
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0)],
        ]);
        
        let density = self.density_matrix();
        let x = (density.dot(&pauli_x)).diag().sum().re;
        let y = (density.dot(&pauli_y)).diag().sum().re;
        let z = (density.dot(&pauli_z)).diag().sum().re;
        
        Ok((x, y, z))
    }

    /// Compute the concurrence (measure of entanglement for 2-qubit states)
    pub fn concurrence(&self) -> Result<f64, String> {
        if self.num_qubits != 2 {
            return Err("Concurrence only defined for 2-qubit states".to_string());
        }
        
        let rho = self.density_matrix();
        let sigma_y = Complex64::new(0.0, -1.0);
        
        // Compute spin-flipped density matrix
        let mut rho_tilde = Array2::zeros((4, 4));
        for i in 0..4 {
            for j in 0..4 {
                rho_tilde[(i, j)] = sigma_y * rho[(3 - j, 3 - i)] * sigma_y;
            }
        }
        
        // Compute eigenvalues of sqrt(sqrt(rho) * rho_tilde * sqrt(rho))
        let matrix = rho.dot(&rho_tilde);
        let eigenvalues = matrix.eigenvalues(false);
        
        let sorted: Vec<f64> = eigenvalues.iter()
            .map(|&e| e.re.sqrt())
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|&x| x >= 0.0)
            .collect();
        
        let sorted_vec = {
            let mut v = sorted;
            v.sort_by(|a, b| b.partial_cmp(a).unwrap());
            v
        };
        
        Ok(0_f64.max(sorted_vec[0] - sorted_vec[1] - sorted_vec[2] - sorted_vec[3]))
    }

    /// Compute the partial trace (trace out some qubits)
    pub fn partial_trace(&self, trace_qubits: &[usize]) -> Result<Array2<Complex64>, String> {
        let remaining_qubits = self.num_qubits - trace_qubits.len();
        let remaining_size = 1 << remaining_qubits;
        let trace_size = 1 << trace_qubits.len();
        
        let mut reduced = Array2::zeros((remaining_size, remaining_size));
        
        for i in 0..self.state.len() {
            for j in 0..self.state.len() {
                // Check if the trace qubits are the same in both states
                let mut trace_match = true;
                for &qubit in trace_qubits {
                    let bit_i = (i >> qubit) & 1;
                    let bit_j = (j >> qubit) & 1;
                    if bit_i != bit_j {
                        trace_match = false;
                        break;
                    }
                }
                
                if trace_match {
                    let reduced_i = Self::trace_out_bits(i, trace_qubits);
                    let reduced_j = Self::trace_out_bits(j, trace_qubits);
                    reduced[(reduced_i, reduced_j)] += self.state[i] * self.state[j].conj();
                }
            }
        }
        
        Ok(reduced)
    }

    /// Apply a unitary operator to the state
    pub fn apply_unitary(&mut self, unitary: &Array2<Complex64>) -> Result<(), String> {
        if unitary.shape() != &[self.state.len(), self.state.len()] {
            return Err("Unitary dimension mismatch".to_string());
        }
        
        self.state = unitary.dot(&self.state);
        Ok(())
    }

    /// Tensor product with another state
    pub fn tensor_product(&self, other: &QuantumState) -> QuantumState {
        let mut new_state = Array1::zeros(self.state.len() * other.state.len());
        
        for i in 0..self.state.len() {
            for j in 0..other.state.len() {
                new_state[i * other.state.len() + j] = self.state[i] * other.state[j];
            }
        }
        
        QuantumState {
            state: new_state,
            num_qubits: self.num_qubits + other.num_qubits,
        }
    }

    /// Check if the state is entangled
    pub fn is_entangled(&self) -> bool {
        if self.num_qubits < 2 {
            return false;
        }
        
        // Check if the reduced density matrix is mixed
        match self.partial_trace(&[0]) {
            Ok(reduced) => {
                let trace = (reduced.dot(&reduced)).diag().sum();
                (trace.re - 1.0).abs() > 1e-10
            }
            Err(_) => false,
        }
    }

    /// Perform state tomography (reconstruct state from measurements)
    pub fn tomography(&self, measurements: &[(usize, bool)]) -> Result<QuantumState, String> {
        // Simplified tomography - in practice would need many measurements
        let mut reconstructed = self.state.clone();
        
        // Adjust state based on measurements
        for (qubit, result) in measurements {
            let index = if *result { 1 << qubit } else { 0 };
            let probability = self.probability(index);
            
            if probability > 1e-10 {
                reconstructed[index] = Complex64::new(probability.sqrt(), 0.0);
            }
        }
        
        // Renormalize
        let norm = reconstructed.iter().map(|c| c.norm_sqr()).sum::<f64>().sqrt();
        if norm > 1e-10 {
            reconstructed /= norm;
        }
        
        Ok(QuantumState {
            state: reconstructed,
            num_qubits: self.num_qubits,
        })
    }

    // ==================== Private Helper Methods ====================

    fn trace_out_bits(index: usize, trace_qubits: &[usize]) -> usize {
        let mut result = 0;
        let mut bit_pos = 0;
        
        for i in 0..(index.leading_zeros() as usize) {
            if !trace_qubits.contains(&i) {
                if index & (1 << i) != 0 {
                    result |= 1 << bit_pos;
                }
                bit_pos += 1;
            }
        }
        
        result
    }
}

/// State preparation utilities
pub struct StatePrep;

impl StatePrep {
    /// Prepare an arbitrary single-qubit state
    pub fn arbitrary_state(theta: f64, phi: f64) -> QuantumState {
        let alpha = Complex64::new((theta / 2.0).cos(), 0.0);
        let beta = Complex64::new(
            (theta / 2.0).sin() * phi.cos(),
            (theta / 2.0).sin() * phi.sin(),
        );
        
        let state = Array1::from(vec![alpha, beta]);
        QuantumState::new(state).unwrap()
    }

    /// Prepare a superposition of multiple basis states
    pub fn superposition(basis_states: Vec<usize>, amplitudes: Vec<Complex64>) -> Result<QuantumState, String> {
        if basis_states.is_empty() {
            return Err("No basis states provided".to_string());
        }
        
        if basis_states.len() != amplitudes.len() {
            return Err("Number of states and amplitudes must match".to_string());
        }
        
        let max_index = *basis_states.iter().max().unwrap();
        let num_qubits = (max_index + 1).next_power_of_two().ilog2() as usize;
        let size = 1 << num_qubits;
        
        let mut state = Array1::zeros(size);
        for (index, amplitude) in basis_states.iter().zip(amplitudes.iter()) {
            state[*index] = *amplitude;
        }
        
        // Normalize
        let norm = state.iter().map(|c| c.norm_sqr()).sum::<f64>().sqrt();
        if norm > 1e-10 {
            state /= norm;
        }
        
        QuantumState::new(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_state() {
        let state = QuantumState::zero(2);
        assert_eq!(state.num_qubits(), 2);
        assert_eq!(state.amplitude(0), Complex64::new(1.0, 0.0));
    }

    #[test]
    fn test_bell_state() {
        let state = QuantumState::bell_state();
        assert!(state.is_entangled());
        assert_eq!(state.num_qubits(), 2);
    }

    #[test]
    fn test_ghz_state() {
        let state = QuantumState::ghz_state(3);
        assert!(state.is_entangled());
        assert_eq!(state.num_qubits(), 3);
    }

    #[test]
    fn test_purity() {
        let state = QuantumState::zero(2);
        assert!((state.purity() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_fidelity() {
        let state1 = QuantumState::zero(2);
        let state2 = QuantumState::zero(2);
        let fidelity = state1.fidelity(&state2).unwrap();
        assert!((fidelity - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_concurrence() {
        let state = QuantumState::bell_state();
        let concurrence = state.concurrence().unwrap();
        assert!((concurrence - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tensor_product() {
        let state1 = QuantumState::zero(1);
        let state2 = QuantumState::one(1);
        let combined = state1.tensor_product(&state2);
        assert_eq!(combined.num_qubits(), 2);
    }

    #[test]
    fn test_arbitrary_state() {
        let state = StatePrep::arbitrary_state(0.0, 0.0);
        assert_eq!(state.num_qubits(), 1);
    }

    #[test]
    fn test_superposition() {
        let basis = vec![0, 1];
        let amplitudes = vec![
            Complex64::new(0.70710678, 0.0),
            Complex64::new(0.70710678, 0.0),
        ];
        let state = StatePrep::superposition(basis, amplitudes);
        assert!(state.is_ok());
    }
}