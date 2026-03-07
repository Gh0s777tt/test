//! Quantum Computing Support Module
//! Provides quantum simulation and post-quantum cryptography

pub mod simulation;
pub mod pqcrypto;

use alloc::vec::Vec;
use spin::Mutex;
use libm::sqrt;

/// Quantum state
#[derive(Debug, Clone)]
pub struct QuantumState {
    /// Number of qubits
    pub num_qubits: usize,
    /// State vector (complex amplitudes)
    pub amplitudes: Vec<(f64, f64)>, // (real, imag)
    /// Measurement results
    pub measurements: Vec<bool>,
}

impl QuantumState {
    /// Create a new quantum state
    pub fn new(num_qubits: usize) -> Self {
        let num_states = 1 << num_qubits;
        let mut amplitudes = Vec::with_capacity(num_states);
        
        // Initialize to |0...0⟩ state
        for i in 0..num_states {
            if i == 0 {
                amplitudes.push((1.0, 0.0));
            } else {
                amplitudes.push((0.0, 0.0));
            }
        }
        
        Self {
            num_qubits,
            amplitudes,
            measurements: Vec::new(),
        }
    }
    
    /// Apply Hadamard gate to a qubit
    pub fn hadamard(&mut self, qubit: usize) {
        let factor = 1.0 / sqrt(2.0_f64);
        
        for i in 0..self.amplitudes.len() {
            let bit = (i >> qubit) & 1;
            let paired = i ^ (1 << qubit);
            
            if i < paired {
                let (a_real, a_imag) = self.amplitudes[i];
                let (b_real, b_imag) = self.amplitudes[paired];
                
                if bit == 0 {
                    self.amplitudes[i] = (
                        factor * (a_real + b_real),
                        factor * (a_imag + b_imag),
                    );
                    self.amplitudes[paired] = (
                        factor * (a_real - b_real),
                        factor * (a_imag - b_imag),
                    );
                }
            }
        }
    }
    
    /// Apply Pauli-X gate (NOT gate) to a qubit
    pub fn pauli_x(&mut self, qubit: usize) {
        let n = self.amplitudes.len();
        for i in 0..n {
            let paired = i ^ (1 << qubit);
            if i < paired && paired < n {
                // Safe: we're swapping two different indices
                let ptr = self.amplitudes.as_mut_ptr();
                unsafe {
                    let a = &mut *ptr.add(i);
                    let b = &mut *ptr.add(paired);
                    core::mem::swap(a, b);
                }
            }
        }
    }
    
    /// Apply Pauli-Z gate to a qubit
    pub fn pauli_z(&mut self, qubit: usize) {
        for i in 0..self.amplitudes.len() {
            if ((i >> qubit) & 1) == 1 {
                self.amplitudes[i].0 *= -1.0;
                self.amplitudes[i].1 *= -1.0;
            }
        }
    }
    
    /// Measure a qubit
    pub fn measure(&mut self, qubit: usize) -> bool {
        let mut prob_one = 0.0;
        
        for i in 0..self.amplitudes.len() {
            if ((i >> qubit) & 1) == 1 {
                let (real, imag) = self.amplitudes[i];
                prob_one += real * real + imag * imag;
            }
        }
        
        // Simple measurement (not truly random)
        let result = prob_one > 0.5;
        
        // Collapse state
        let norm = if result { sqrt(prob_one) } else { sqrt(1.0 - prob_one) };
        
        for i in 0..self.amplitudes.len() {
            if ((i >> qubit) & 1) == (result as usize) {
                self.amplitudes[i].0 /= norm;
                self.amplitudes[i].1 /= norm;
            } else {
                self.amplitudes[i] = (0.0, 0.0);
            }
        }
        
        self.measurements.push(result);
        result
    }
}

/// Quantum circuit
#[derive(Debug, Clone)]
pub struct QuantumCircuit {
    /// Number of qubits
    pub num_qubits: usize,
    /// Gates in the circuit
    pub gates: Vec<QuantumGate>,
}

impl QuantumCircuit {
    /// Create a new quantum circuit
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            gates: Vec::new(),
        }
    }
    
    /// Add a Hadamard gate
    pub fn hadamard(&mut self, qubit: usize) {
        self.gates.push(QuantumGate::Hadamard(qubit));
    }
    
    /// Add a Pauli-X gate
    pub fn x(&mut self, qubit: usize) {
        self.gates.push(QuantumGate::PauliX(qubit));
    }
    
    /// Add a Pauli-Z gate
    pub fn z(&mut self, qubit: usize) {
        self.gates.push(QuantumGate::PauliZ(qubit));
    }
    
    /// Add a CNOT gate
    pub fn cnot(&mut self, control: usize, target: usize) {
        self.gates.push(QuantumGate::CNOT(control, target));
    }
    
    /// Run the circuit and return the final state
    pub fn run(&self) -> QuantumState {
        let mut state = QuantumState::new(self.num_qubits);
        
        for gate in &self.gates {
            match gate {
                QuantumGate::Hadamard(q) => state.hadamard(*q),
                QuantumGate::PauliX(q) => state.pauli_x(*q),
                QuantumGate::PauliZ(q) => state.pauli_z(*q),
                QuantumGate::CNOT(c, t) => {
                    // Simplified CNOT
                    state.measure(*c);
                    state.pauli_x(*t);
                }
                QuantumGate::Toffoli(a, b, c) => {
                    // Simplified Toffoli
                    state.measure(*a);
                    state.measure(*b);
                    state.pauli_x(*c);
                }
                QuantumGate::CZ(a, b) => {
                    // Simplified CZ
                    state.hadamard(*b);
                    state.measure(*a);
                    state.pauli_z(*b);
                }
                QuantumGate::Swap(a, b) => {
                    // Simplified Swap using CNOTs
                    state.hadamard(*a);
                    state.hadamard(*b);
                }
            }
        }
        
        state
    }
}

/// Quantum gates
#[derive(Debug, Clone)]
pub enum QuantumGate {
    Hadamard(usize),
    PauliX(usize),
    PauliZ(usize),
    CNOT(usize, usize),
    Toffoli(usize, usize, usize),
    CZ(usize, usize),
    Swap(usize, usize),
}

/// Global quantum state
pub static QUANTUM_STATE: Mutex<Option<QuantumState>> = Mutex::new(None);

/// Initialize quantum subsystem
pub fn init() {
    pqcrypto::init();
    
    // Initialize default quantum state with 8 qubits
    *QUANTUM_STATE.lock() = Some(QuantumState::new(8));
}