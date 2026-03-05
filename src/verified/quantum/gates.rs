//! Quantum Gate Operations
//!
//! This module provides implementations of standard quantum gates
//! including single-qubit gates, multi-qubit gates, and parameterized gates.

use super::{QuantumError, Result};
use std::f64::consts::{FRAC_1_SQRT_2, PI};

/// Single-qubit gate trait
pub trait SingleQubitGate: std::fmt::Debug + Clone + Send + Sync {
    /// Get the 2x2 matrix representation of the gate
    fn matrix(&self) -> [[num_complex::Complex64; 2]; 2];

    /// Get the name of the gate
    fn name(&self) -> &'static str;

    /// Check if the gate is parameterized
    fn is_parameterized(&self) -> bool {
        false
    }
}

/// Hadamard gate
#[derive(Debug, Clone, Copy, Default)]
pub struct H;

impl H {
    /// Create a new Hadamard gate
    pub fn new() -> Self {
        Self
    }
}

impl SingleQubitGate for H {
    fn matrix(&self) -> [[num_complex::Complex64; 2]; 2] {
        let val = num_complex::Complex64::new(FRAC_1_SQRT_2, 0.0);
        let neg_val = num_complex::Complex64::new(-FRAC_1_SQRT_2, 0.0);
        [[val, val], [val, neg_val]]
    }

    fn name(&self) -> &'static str {
        "H"
    }
}

/// Pauli-X gate (NOT gate)
#[derive(Debug, Clone, Copy, Default)]
pub struct X;

impl X {
    /// Create a new Pauli-X gate
    pub fn new() -> Self {
        Self
    }
}

impl SingleQubitGate for X {
    fn matrix(&self) -> [[num_complex::Complex64; 2]; 2] {
        let zero = num_complex::Complex64::new(0.0, 0.0);
        let one = num_complex::Complex64::new(1.0, 0.0);
        [[zero, one], [one, zero]]
    }

    fn name(&self) -> &'static str {
        "X"
    }
}

/// Pauli-Y gate
#[derive(Debug, Clone, Copy, Default)]
pub struct Y;

impl Y {
    /// Create a new Pauli-Y gate
    pub fn new() -> Self {
        Self
    }
}

impl SingleQubitGate for Y {
    fn matrix(&self) -> [[num_complex::Complex64; 2]; 2] {
        let zero = num_complex::Complex64::new(0.0, 0.0);
        let i = num_complex::Complex64::new(0.0, 1.0);
        let neg_i = num_complex::Complex64::new(0.0, -1.0);
        [[zero, neg_i], [i, zero]]
    }

    fn name(&self) -> &'static str {
        "Y"
    }
}

/// Pauli-Z gate
#[derive(Debug, Clone, Copy, Default)]
pub struct Z;

impl Z {
    /// Create a new Pauli-Z gate
    pub fn new() -> Self {
        Self
    }
}

impl SingleQubitGate for Z {
    fn matrix(&self) -> [[num_complex::Complex64; 2]; 2] {
        let zero = num_complex::Complex64::new(0.0, 0.0);
        let one = num_complex::Complex64::new(1.0, 0.0);
        let neg_one = num_complex::Complex64::new(-1.0, 0.0);
        [[one, zero], [zero, neg_one]]
    }

    fn name(&self) -> &'static str {
        "Z"
    }
}

/// Phase gate (S)
#[derive(Debug, Clone, Copy, Default)]
pub struct S;

impl S {
    /// Create a new Phase gate
    pub fn new() -> Self {
        Self
    }
}

impl SingleQubitGate for S {
    fn matrix(&self) -> [[num_complex::Complex64; 2]; 2] {
        let zero = num_complex::Complex64::new(0.0, 0.0);
        let one = num_complex::Complex64::new(1.0, 0.0);
        let i = num_complex::Complex64::new(0.0, 1.0);
        [[one, zero], [zero, i]]
    }

    fn name(&self) -> &'static str {
        "S"
    }
}

/// T gate (π/8 gate)
#[derive(Debug, Clone, Copy, Default)]
pub struct T;

impl T {
    /// Create a new T gate
    pub fn new() -> Self {
        Self
    }
}

impl SingleQubitGate for T {
    fn matrix(&self) -> [[num_complex::Complex64; 2]; 2] {
        let zero = num_complex::Complex64::new(0.0, 0.0);
        let one = num_complex::Complex64::new(1.0, 0.0);
        let exp = num_complex::Complex64::from_polar(1.0, PI / 4.0);
        [[one, zero], [zero, exp]]
    }

    fn name(&self) -> &'static str {
        "T"
    }
}

/// RX rotation gate
#[derive(Debug, Clone, Copy)]
pub struct RX {
    pub theta: f64,
}

impl RX {
    /// Create a new RX gate with angle theta
    pub fn new(theta: f64) -> Self {
        Self { theta }
    }
}

impl SingleQubitGate for RX {
    fn matrix(&self) -> [[num_complex::Complex64; 2]; 2] {
        let c = num_complex::Complex64::new(self.theta.cos(), 0.0);
        let s = num_complex::Complex64::new(0.0, -self.theta.sin());
        [[c, s], [s, c]]
    }

    fn name(&self) -> &'static str {
        "RX"
    }

    fn is_parameterized(&self) -> bool {
        true
    }
}

/// RY rotation gate
#[derive(Debug, Clone, Copy)]
pub struct RY {
    pub theta: f64,
}

impl RY {
    /// Create a new RY gate with angle theta
    pub fn new(theta: f64) -> Self {
        Self { theta }
    }
}

impl SingleQubitGate for RY {
    fn matrix(&self) -> [[num_complex::Complex64; 2]; 2] {
        let c = num_complex::Complex64::new(self.theta.cos(), 0.0);
        let s = num_complex::Complex64::new(-self.theta.sin(), 0.0);
        let sp = num_complex::Complex64::new(self.theta.sin(), 0.0);
        [[c, s], [sp, c]]
    }

    fn name(&self) -> &'static str {
        "RY"
    }

    fn is_parameterized(&self) -> bool {
        true
    }
}

/// RZ rotation gate
#[derive(Debug, Clone, Copy)]
pub struct RZ {
    pub theta: f64,
}

impl RZ {
    /// Create a new RZ gate with angle theta
    pub fn new(theta: f64) -> Self {
        Self { theta }
    }
}

impl SingleQubitGate for RZ {
    fn matrix(&self) -> [[num_complex::Complex64; 2]; 2] {
        let e = num_complex::Complex64::from_polar(1.0, self.theta / 2.0);
        let neg_e = num_complex::Complex64::from_polar(1.0, -self.theta / 2.0);
        let zero = num_complex::Complex64::new(0.0, 0.0);
        [[neg_e, zero], [zero, e]]
    }

    fn name(&self) -> &'static str {
        "RZ"
    }

    fn is_parameterized(&self) -> bool {
        true
    }
}

/// CNOT gate (controlled-X)
#[derive(Debug, Clone, Copy, Default)]
pub struct CNOT;

impl CNOT {
    /// Create a new CNOT gate
    pub fn new() -> Self {
        Self
    }

    /// Get the 4x4 matrix representation
    pub fn matrix(&self) -> [[num_complex::Complex64; 4]; 4] {
        let zero = num_complex::Complex64::new(0.0, 0.0);
        let one = num_complex::Complex64::new(1.0, 0.0);
        [
            [one, zero, zero, zero],
            [zero, one, zero, zero],
            [zero, zero, zero, one],
            [zero, zero, one, zero],
        ]
    }
}

/// SWAP gate
#[derive(Debug, Clone, Copy, Default)]
pub struct SWAP;

impl SWAP {
    /// Create a new SWAP gate
    pub fn new() -> Self {
        Self
    }

    /// Get the 4x4 matrix representation
    pub fn matrix(&self) -> [[num_complex::Complex64; 4]; 4] {
        let zero = num_complex::Complex64::new(0.0, 0.0);
        let one = num_complex::Complex64::new(1.0, 0.0);
        [
            [one, zero, zero, zero],
            [zero, zero, one, zero],
            [zero, one, zero, zero],
            [zero, zero, zero, one],
        ]
    }
}

/// Toffoli gate (CCX)
#[derive(Debug, Clone, Copy, Default)]
pub struct Toffoli;

impl Toffoli {
    /// Create a new Toffoli gate
    pub fn new() -> Self {
        Self
    }
}

/// CZ gate (controlled-Z)
#[derive(Debug, Clone, Copy, Default)]
pub struct CZ;

impl CZ {
    /// Create a new CZ gate
    pub fn new() -> Self {
        Self
    }

    /// Get the 4x4 matrix representation
    pub fn matrix(&self) -> [[num_complex::Complex64; 4]; 4] {
        let zero = num_complex::Complex64::new(0.0, 0.0);
        let one = num_complex::Complex64::new(1.0, 0.0);
        let neg_one = num_complex::Complex64::new(-1.0, 0.0);
        [
            [one, zero, zero, zero],
            [zero, one, zero, zero],
            [zero, zero, one, zero],
            [zero, zero, zero, neg_one],
        ]
    }
}

/// Gate builder for creating gates
pub struct GateBuilder;

impl GateBuilder {
    /// Create a Hadamard gate
    pub fn h() -> H {
        H::new()
    }

    /// Create a Pauli-X gate
    pub fn x() -> X {
        X::new()
    }

    /// Create a Pauli-Y gate
    pub fn y() -> Y {
        Y::new()
    }

    /// Create a Pauli-Z gate
    pub fn z() -> Z {
        Z::new()
    }

    /// Create a Phase gate
    pub fn s() -> S {
        S::new()
    }

    /// Create a T gate
    pub fn t() -> T {
        T::new()
    }

    /// Create an RX gate
    pub fn rx(theta: f64) -> RX {
        RX::new(theta)
    }

    /// Create an RY gate
    pub fn ry(theta: f64) -> RY {
        RY::new(theta)
    }

    /// Create an RZ gate
    pub fn rz(theta: f64) -> RZ {
        RZ::new(theta)
    }

    /// Create a CNOT gate
    pub fn cnot() -> CNOT {
        CNOT::new()
    }

    /// Create a SWAP gate
    pub fn swap() -> SWAP {
        SWAP::new()
    }

    /// Create a Toffoli gate
    pub fn toffoli() -> Toffoli {
        Toffoli::new()
    }

    /// Create a CZ gate
    pub fn cz() -> CZ {
        CZ::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_gate_matrix() {
        let h = H::new();
        let m = h.matrix();
        let val = FRAC_1_SQRT_2;
        assert!((m[0][0].re - val).abs() < 1e-10);
        assert!((m[1][1].re + val).abs() < 1e-10);
    }

    #[test]
    fn test_x_gate_matrix() {
        let x = X::new();
        let m = x.matrix();
        assert_eq!(m[0][0].re, 0.0);
        assert_eq!(m[0][1].re, 1.0);
        assert_eq!(m[1][0].re, 1.0);
        assert_eq!(m[1][1].re, 0.0);
    }

    #[test]
    fn test_cnot_gate_matrix() {
        let cnot = CNOT::new();
        let m = cnot.matrix();
        assert_eq!(m[0][0].re, 1.0);
        assert_eq!(m[3][3].re, 0.0);
        assert_eq!(m[3][2].re, 1.0);
    }

    #[test]
    fn test_rx_gate() {
        let rx = RX::new(PI);
        let m = rx.matrix();
        assert!((m[0][0].re - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_gate_builder() {
        let h = GateBuilder::h();
        assert_eq!(h.name(), "H");

        let rx = GateBuilder::rx(PI / 2.0);
        assert!(rx.is_parameterized());
    }
}