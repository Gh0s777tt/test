// Quantum Gates for VantisOS
// Comprehensive collection of quantum gate operations

use num_complex::Complex64;
use ndarray::Array2;
use std::f64::consts::{FRAC_1_SQRT_2, PI};

/// Quantum gate operations
#[derive(Clone, Debug)]
pub enum QuantumGate {
    /// Single-qubit gates
    PauliX,
    PauliY,
    PauliZ,
    Hadamard,
    Phase,
    S,
    T,
    RX(f64),
    RY(f64),
    RZ(f64),
    U { theta: f64, phi: f64, lambda: f64 },
    
    /// Two-qubit gates
    CNOT,
    CZ,
    SWAP,
    ControlledPhase(f64),
    
    /// Three-qubit gates
    Toffoli,
    Fredkin,
}

impl QuantumGate {
    /// Get the matrix representation of the gate
    pub fn matrix(&self) -> Array2<Complex64> {
        match self {
            // Single-qubit gates
            QuantumGate::PauliX => self.pauli_x(),
            QuantumGate::PauliY => self.pauli_y(),
            QuantumGate::PauliZ => self.pauli_z(),
            QuantumGate::Hadamard => self.hadamard(),
            QuantumGate::Phase => self.phase(),
            QuantumGate::S => self.s_gate(),
            QuantumGate::T => self.t_gate(),
            QuantumGate::RX(theta) => self.rx(*theta),
            QuantumGate::RY(theta) => self.ry(*theta),
            QuantumGate::RZ(theta) => self.rz(*theta),
            QuantumGate::U { theta, phi, lambda } => self.u(*theta, *phi, *lambda),
            
            // Two-qubit gates
            QuantumGate::CNOT => self.cnot(),
            QuantumGate::CZ => self.cz(),
            QuantumGate::SWAP => self.swap(),
            QuantumGate::ControlledPhase(phi) => self.controlled_phase(*phi),
            
            // Three-qubit gates
            QuantumGate::Toffoli => self.toffoli(),
            QuantumGate::Fredkin => self.fredkin(),
        }
    }

    /// Get the name of the gate
    pub fn name(&self) -> &str {
        match self {
            QuantumGate::PauliX => "Pauli-X",
            QuantumGate::PauliY => "Pauli-Y",
            QuantumGate::PauliZ => "Pauli-Z",
            QuantumGate::Hadamard => "Hadamard",
            QuantumGate::Phase => "Phase",
            QuantumGate::S => "S",
            QuantumGate::T => "T",
            QuantumGate::RX(_) => "RX",
            QuantumGate::RY(_) => "RY",
            QuantumGate::RZ(_) => "RZ",
            QuantumGate::U { .. } => "U",
            QuantumGate::CNOT => "CNOT",
            QuantumGate::CZ => "CZ",
            QuantumGate::SWAP => "SWAP",
            QuantumGate::ControlledPhase(_) => "CPhase",
            QuantumGate::Toffoli => "Toffoli",
            QuantumGate::Fredkin => "Fredkin",
        }
    }

    /// Check if the gate is unitary
    pub fn is_unitary(&self) -> bool {
        let matrix = self.matrix();
        let n = matrix.nrows();
        
        // Compute U * U†
        let identity = matrix.dot(&matrix.t().mapv(|c| c.conj()));
        
        // Check if it's identity
        for i in 0..n {
            for j in 0..n {
                let expected = if i == j {
                    Complex64::new(1.0, 0.0)
                } else {
                    Complex64::new(0.0, 0.0)
                };
                if (identity[(i, j)] - expected).norm() > 1e-10 {
                    return false;
                }
            }
        }
        
        true
    }

    /// Get the number of qubits the gate operates on
    pub fn num_qubits(&self) -> usize {
        match self {
            QuantumGate::PauliX | QuantumGate::PauliY | QuantumGate::PauliZ |
            QuantumGate::Hadamard | QuantumGate::Phase | QuantumGate::S | QuantumGate::T |
            QuantumGate::RX(_) | QuantumGate::RY(_) | QuantumGate::RZ(_) |
            QuantumGate::U { .. } => 1,
            
            QuantumGate::CNOT | QuantumGate::CZ | QuantumGate::SWAP |
            QuantumGate::ControlledPhase(_) => 2,
            
            QuantumGate::Toffoli | QuantumGate::Fredkin => 3,
        }
    }

    /// Check if the gate is reversible
    pub fn is_reversible(&self) -> bool {
        self.is_unitary()
    }

    /// Get the inverse of the gate
    pub fn inverse(&self) -> QuantumGate {
        match self {
            QuantumGate::PauliX | QuantumGate::PauliY | QuantumGate::PauliZ => self.clone(),
            QuantumGate::Hadamard => QuantumGate::Hadamard,
            QuantumGate::Phase => QuantumGate::Phase,
            QuantumGate::S => QuantumGate::T,  // S = T^2, so S† = T^6 ≈ T
            QuantumGate::T => QuantumGate::Phase,  // T = S^(1/2)
            QuantumGate::RX(theta) => QuantumGate::RX(-theta),
            QuantumGate::RY(theta) => QuantumGate::RY(-theta),
            QuantumGate::RZ(theta) => QuantumGate::RZ(-theta),
            QuantumGate::U { theta, phi, lambda } => {
                QuantumGate::U {
                    theta: -theta,
                    phi: -lambda,
                    lambda: -phi,
                }
            }
            QuantumGate::CNOT | QuantumGate::CZ | QuantumGate::SWAP => self.clone(),
            QuantumGate::ControlledPhase(phi) => QuantumGate::ControlledPhase(-phi),
            QuantumGate::Toffoli | QuantumGate::Fredkin => self.clone(),
        }
    }

    // ==================== Pauli Gates ====================

    fn pauli_x(&self) -> Array2<Complex64> {
        Array2::from(vec![
            [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        ])
    }

    fn pauli_y(&self) -> Array2<Complex64> {
        Array2::from(vec![
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, -1.0)],
            [Complex64::new(0.0, 1.0), Complex64::new(0.0, 0.0)],
        ])
    }

    fn pauli_z(&self) -> Array2<Complex64> {
        Array2::from(vec![
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0)],
        ])
    }

    // ==================== Hadamard Gate ====================

    fn hadamard(&self) -> Array2<Complex64> {
        let h = Complex64::new(FRAC_1_SQRT_2, 0.0);
        Array2::from(vec![
            [h, h],
            [h, -h],
        ])
    }

    // ==================== Phase Gates ====================

    fn phase(&self) -> Array2<Complex64> {
        Array2::from(vec![
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0)],
        ])
    }

    fn s_gate(&self) -> Array2<Complex64> {
        Array2::from(vec![
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, 1.0)],
        ])
    }

    fn t_gate(&self) -> Array2<Complex64> {
        let t = Complex64::new(1.0, 1.0) / Complex64::new(2.0, 0.0).sqrt();
        Array2::from(vec![
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), t],
        ])
    }

    // ==================== Rotation Gates ====================

    fn rx(&self, theta: f64) -> Array2<Complex64> {
        let cos = (theta / 2.0).cos();
        let sin = (theta / 2.0).sin();
        Array2::from(vec![
            [Complex64::new(cos, 0.0), Complex64::new(0.0, -sin)],
            [Complex64::new(0.0, -sin), Complex64::new(cos, 0.0)],
        ])
    }

    fn ry(&self, theta: f64) -> Array2<Complex64> {
        let cos = (theta / 2.0).cos();
        let sin = (theta / 2.0).sin();
        Array2::from(vec![
            [Complex64::new(cos, 0.0), Complex64::new(-sin, 0.0)],
            [Complex64::new(sin, 0.0), Complex64::new(cos, 0.0)],
        ])
    }

    fn rz(&self, theta: f64) -> Array2<Complex64> {
        Array2::from(vec![
            [Complex64::new((-theta / 2.0).cos(), (-theta / 2.0).sin()), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new((theta / 2.0).cos(), (theta / 2.0).sin())],
        ])
    }

    // ==================== Universal Gate ====================

    fn u(&self, theta: f64, phi: f64, lambda: f64) -> Array2<Complex64> {
        let cos_theta_2 = (theta / 2.0).cos();
        let sin_theta_2 = (theta / 2.0).sin();
        
        Array2::from(vec![
            [
                Complex64::new(cos_theta_2, 0.0),
                Complex64::new(-sin_theta_2 * lambda.cos(), -sin_theta_2 * lambda.sin()),
            ],
            [
                Complex64::new(sin_theta_2 * phi.cos(), sin_theta_2 * phi.sin()),
                Complex64::new(cos_theta_2 * (phi + lambda).cos(), cos_theta_2 * (phi + lambda).sin()),
            ],
        ])
    }

    // ==================== Two-Qubit Gates ====================

    fn cnot(&self) -> Array2<Complex64> {
        Array2::from(vec![
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        ])
    }

    fn cz(&self) -> Array2<Complex64> {
        Array2::from(vec![
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(-1.0, 0.0)],
        ])
    }

    fn swap(&self) -> Array2<Complex64> {
        Array2::from(vec![
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0)],
        ])
    }

    fn controlled_phase(&self, phi: f64) -> Array2<Complex64> {
        let phase = Complex64::new(phi.cos(), phi.sin());
        Array2::from(vec![
            [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
            [Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), Complex64::new(0.0, 0.0), phase],
        ])
    }

    // ==================== Three-Qubit Gates ====================

    fn toffoli(&self) -> Array2<Complex64> {
        let mut matrix = Array2::eye(8);
        matrix[(6, 7)] = Complex64::new(1.0, 0.0);
        matrix[(6, 6)] = Complex64::new(0.0, 0.0);
        matrix[(7, 6)] = Complex64::new(1.0, 0.0);
        matrix[(7, 7)] = Complex64::new(0.0, 0.0);
        matrix
    }

    fn fredkin(&self) -> Array2<Complex64> {
        let mut matrix = Array2::eye(8);
        // Swap states 5 and 6 (binary 101 and 110) when control is 1
        matrix[(5, 6)] = Complex64::new(1.0, 0.0);
        matrix[(6, 5)] = Complex64::new(1.0, 0.0);
        matrix[(5, 5)] = Complex64::new(0.0, 0.0);
        matrix[(6, 6)] = Complex64::new(0.0, 0.0);
        matrix
    }
}

/// Gate composition utilities
pub struct GateComposer;

impl GateComposer {
    /// Compose two gates sequentially
    pub fn compose(gate1: &QuantumGate, gate2: &QuantumGate) -> Result<Array2<Complex64>, String> {
        if gate1.num_qubits() != gate2.num_qubits() {
            return Err("Cannot compose gates with different qubit counts".to_string());
        }
        
        let matrix1 = gate1.matrix();
        let matrix2 = gate2.matrix();
        Ok(matrix2.dot(&matrix1))
    }

    /// Tensor product of two gates
    pub fn tensor_product(gate1: &QuantumGate, gate2: &QuantumGate) -> Array2<Complex64> {
        let matrix1 = gate1.matrix();
        let matrix2 = gate2.matrix();
        
        let rows1 = matrix1.nrows();
        let cols1 = matrix1.ncols();
        let rows2 = matrix2.nrows();
        let cols2 = matrix2.ncols();
        
        let mut result = Array2::zeros((rows1 * rows2, cols1 * cols2));
        
        for i in 0..rows1 {
            for j in 0..cols1 {
                for k in 0..rows2 {
                    for l in 0..cols2 {
                        result[(i * rows2 + k, j * cols2 + l)] = matrix1[(i, j)] * matrix2[(k, l)];
                    }
                }
            }
        }
        
        result
    }

    /// Create a controlled version of a gate
    pub fn controlled(gate: &QuantumGate) -> QuantumGate {
        match gate {
            QuantumGate::PauliX => QuantumGate::CNOT,
            QuantumGate::PauliZ => QuantumGate::CZ,
            _ => unimplemented!("Controlled version of this gate not implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pauli_gates() {
        assert!(QuantumGate::PauliX.is_unitary());
        assert!(QuantumGate::PauliY.is_unitary());
        assert!(QuantumGate::PauliZ.is_unitary());
    }

    #[test]
    fn test_hadamard_gate() {
        let h = QuantumGate::Hadamard;
        assert!(h.is_unitary());
        assert_eq!(h.num_qubits(), 1);
    }

    #[test]
    fn test_cnot_gate() {
        let cnot = QuantumGate::CNOT;
        assert!(cnot.is_unitary());
        assert_eq!(cnot.num_qubits(), 2);
    }

    #[test]
    fn test_gate_composition() {
        let x = QuantumGate::PauliX;
        let h = QuantumGate::Hadamard;
        let composed = GateComposer::compose(&x, &h);
        assert!(composed.is_ok());
    }

    #[test]
    fn test_tensor_product() {
        let h = QuantumGate::Hadamard;
        let x = QuantumGate::PauliX;
        let tensor = GateComposer::tensor_product(&h, &x);
        assert_eq!(tensor.shape(), &[4, 4]);
    }

    #[test]
    fn test_rotation_gates() {
        let rx = QuantumGate::RX(PI / 4.0);
        let ry = QuantumGate::RY(PI / 4.0);
        let rz = QuantumGate::RZ(PI / 4.0);
        
        assert!(rx.is_unitary());
        assert!(ry.is_unitary());
        assert!(rz.is_unitary());
    }

    #[test]
    fn test_gate_inverse() {
        let rx = QuantumGate::RX(PI / 4.0);
        let inverse = rx.inverse();
        assert!(matches!(inverse, QuantumGate::RX(_)));
    }

    #[test]
    fn test_toffoli_gate() {
        let toffoli = QuantumGate::Toffoli;
        assert!(toffoli.is_unitary());
        assert_eq!(toffoli.num_qubits(), 3);
    }

    #[test]
    fn test_fredkin_gate() {
        let fredkin = QuantumGate::Fredkin;
        assert!(fredkin.is_unitary());
        assert_eq!(fredkin.num_qubits(), 3);
    }
}