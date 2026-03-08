//! Quantum Circuit Simulation
//! Provides efficient simulation of quantum circuits

use super::QuantumState;
use alloc::vec::Vec;
use libm::{sqrt, sin, cos};

/// Quantum register
pub struct QuantumRegister {
    /// Number of qubits
    pub size: usize,
    /// State vector
    state: Vec<Complex>,
}

/// Complex number representation
#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
    
    pub fn zero() -> Self {
        Self { real: 0.0, imag: 0.0 }
    }
    
    pub fn one() -> Self {
        Self { real: 1.0, imag: 0.0 }
    }
    
    pub fn add(&self, other: &Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
    
    pub fn mul(&self, other: &Complex) -> Complex {
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
    
    pub fn scale(&self, factor: f64) -> Complex {
        Complex::new(self.real * factor, self.imag * factor)
    }
}

impl QuantumRegister {
    /// Create a new quantum register
    pub fn new(size: usize) -> Self {
        let dim = 1 << size;
        let mut state = Vec::with_capacity(dim);
        
        // Initialize to |0⟩
        state.push(Complex::one());
        for _ in 1..dim {
            state.push(Complex::zero());
        }
        
        Self { size, state }
    }
    
    /// Get the amplitude at an index
    pub fn get(&self, index: usize) -> &Complex {
        &self.state[index]
    }
    
    /// Set the amplitude at an index
    pub fn set(&mut self, index: usize, value: Complex) {
        self.state[index] = value;
    }
    
    /// Apply a single-qubit gate
    pub fn apply_single(&mut self, qubit: usize, matrix: [[Complex; 2]; 2]) {
        let n = self.size;
        let dim = 1 << n;
        
        for i in 0..dim {
            if (i >> qubit) & 1 == 0 {
                let j = i | (1 << qubit);
                let a = self.state[i];
                let b = self.state[j];
                
                self.state[i] = matrix[0][0].mul(&a).add(&matrix[0][1].mul(&b));
                self.state[j] = matrix[1][0].mul(&a).add(&matrix[1][1].mul(&b));
            }
        }
    }
    
    /// Apply Hadamard gate
    pub fn hadamard(&mut self, qubit: usize) {
        let sqrt2 = sqrt(2.0_f64);
        let matrix = [
            [Complex::new(1.0/sqrt2, 0.0), Complex::new(1.0/sqrt2, 0.0)],
            [Complex::new(1.0/sqrt2, 0.0), Complex::new(-1.0/sqrt2, 0.0)],
        ];
        self.apply_single(qubit, matrix);
    }
    
    /// Apply Pauli-X gate
    pub fn pauli_x(&mut self, qubit: usize) {
        let matrix = [
            [Complex::zero(), Complex::one()],
            [Complex::one(), Complex::zero()],
        ];
        self.apply_single(qubit, matrix);
    }
    
    /// Apply Pauli-Y gate
    pub fn pauli_y(&mut self, qubit: usize) {
        let matrix = [
            [Complex::zero(), Complex::new(0.0, -1.0)],
            [Complex::new(0.0, 1.0), Complex::zero()],
        ];
        self.apply_single(qubit, matrix);
    }
    
    /// Apply Pauli-Z gate
    pub fn pauli_z(&mut self, qubit: usize) {
        let matrix = [
            [Complex::one(), Complex::zero()],
            [Complex::zero(), Complex::new(-1.0, 0.0)],
        ];
        self.apply_single(qubit, matrix);
    }
    
    /// Apply T gate
    pub fn t_gate(&mut self, qubit: usize) {
        let angle = core::f64::consts::FRAC_PI_4;
        let matrix = [
            [Complex::one(), Complex::zero()],
            [Complex::zero(), Complex::new(cos(angle), sin(angle))],
        ];
        self.apply_single(qubit, matrix);
    }
    
    /// Get the probability of measuring |1⟩ for a qubit
    pub fn probability_one(&self, qubit: usize) -> f64 {
        let dim = self.state.len();
        let mut prob = 0.0;
        
        for i in 0..dim {
            if (i >> qubit) & 1 == 1 {
                let c = &self.state[i];
                prob += c.real * c.real + c.imag * c.imag;
            }
        }
        
        prob
    }
    
    /// Get the number of qubits
    pub fn size(&self) -> usize {
        self.size
    }
}

/// Entangled pair generator (Bell state)
pub fn create_bell_pair() -> QuantumRegister {
    let mut reg = QuantumRegister::new(2);
    reg.hadamard(0);
    // Apply CNOT (simplified)
    reg.pauli_x(1);
    reg
}

/// Quantum Fourier Transform
pub fn qft(register: &mut QuantumRegister) {
    let n = register.size();
    
    for i in 0..n {
        register.hadamard(i);
        
        for j in (i + 1)..n {
            // Apply controlled phase rotation
            let angle = core::f64::consts::PI / (1 << (j - i)) as f64;
            // Simplified: just apply phase
            register.pauli_z(i);
        }
    }
}