// Quantum Computing Module for VantisOS
// Provides quantum simulation, gates, circuits, algorithms, and state operations

pub mod simulator;
pub mod gates;
pub mod circuit;
pub mod algorithms;
pub mod state;

// Re-export main types and structs
pub use simulator::{QuantumSimulator, NoiseModel};
pub use gates::{QuantumGate, GateComposer};
pub use circuit::{QuantumCircuit, CircuitGate, CircuitBuilder, CompiledCircuit};
pub use algorithms::{QuantumAlgorithms, GroverOracle};
pub use state::{QuantumState, StatePrep};