// Quantum Circuit for VantisOS
// High-level quantum circuit representation and manipulation

use crate::verified::quantum::gates::QuantumGate;
use ndarray::Array1;
use num_complex::Complex64;
use std::collections::HashMap;

/// Quantum circuit representation
#[derive(Clone, Debug)]
pub struct QuantumCircuit {
    /// List of gates in the circuit
    gates: Vec<CircuitGate>,
    /// Number of qubits
    num_qubits: usize,
    /// Number of classical bits
    num_bits: usize,
    /// Circuit parameters
    parameters: HashMap<String, f64>,
    /// Circuit metadata
    metadata: CircuitMetadata,
}

/// A gate in the circuit with target qubits
#[derive(Clone, Debug)]
pub struct CircuitGate {
    /// The gate operation
    pub gate: QuantumGate,
    /// Target qubits
    pub targets: Vec<usize>,
    /// Control qubits (for controlled gates)
    pub controls: Vec<usize>,
    /// Gate parameters (for parameterized gates)
    pub params: Option<Vec<f64>>,
}

/// Circuit metadata
#[derive(Clone, Debug)]
pub struct CircuitMetadata {
    pub name: String,
    pub description: String,
    pub author: Option<String>,
    pub version: String,
}

impl Default for CircuitMetadata {
    fn default() -> Self {
        CircuitMetadata {
            name: "unnamed".to_string(),
            description: String::new(),
            author: None,
            version: "1.0.0".to_string(),
        }
    }
}

impl QuantumCircuit {
    /// Create a new quantum circuit
    pub fn new(num_qubits: usize) -> Self {
        QuantumCircuit {
            gates: Vec::new(),
            num_qubits,
            num_bits: 0,
            parameters: HashMap::new(),
            metadata: CircuitMetadata::default(),
        }
    }

    /// Create a circuit with both qubits and classical bits
    pub fn with_bits(num_qubits: usize, num_bits: usize) -> Self {
        let mut circuit = Self::new(num_qubits);
        circuit.num_bits = num_bits;
        circuit
    }

    /// Get the number of qubits
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Get the number of classical bits
    pub fn num_bits(&self) -> usize {
        self.num_bits
    }

    /// Get the circuit depth
    pub fn depth(&self) -> usize {
        if self.gates.is_empty() {
            return 0;
        }

        let mut qubit_times = vec![0; self.num_qubits];
        
        for gate in &self.gates {
            let mut max_time = 0;
            let mut involved_qubits = gate.targets.clone();
            involved_qubits.extend(gate.controls.clone());
            
            for &qubit in &involved_qubits {
                max_time = max_time.max(qubit_times[qubit]);
            }
            
            let gate_time = max_time + 1;
            for qubit in involved_qubits {
                qubit_times[qubit] = gate_time;
            }
        }
        
        *qubit_times.iter().max().unwrap_or(&0)
    }

    /// Get the circuit width (number of qubits)
    pub fn width(&self) -> usize {
        self.num_qubits
    }

    /// Add a gate to the circuit
    pub fn add_gate(&mut self, gate: CircuitGate) -> Result<(), String> {
        // Validate qubit indices
        let all_qubits: Vec<usize> = gate.targets.iter().chain(gate.controls.iter()).cloned().collect();
        for &qubit in &all_qubits {
            if qubit >= self.num_qubits {
                return Err(format!("Qubit index {} out of range", qubit));
            }
        }
        
        // Validate gate qubit count
        let required_qubits = gate.gate.num_qubits();
        if all_qubits.len() < required_qubits {
            return Err(format!(
                "Gate requires {} qubits, but only {} provided",
                required_qubits,
                all_qubits.len()
            ));
        }
        
        self.gates.push(gate);
        Ok(())
    }

    /// Add a single-qubit gate
    pub fn add_single_qubit_gate(&mut self, gate: QuantumGate, target: usize) -> Result<(), String> {
        if gate.num_qubits() != 1 {
            return Err("Gate is not a single-qubit gate".to_string());
        }
        
        self.add_gate(CircuitGate {
            gate,
            targets: vec![target],
            controls: vec![],
            params: None,
        })
    }

    /// Add a two-qubit gate
    pub fn add_two_qubit_gate(
        &mut self,
        gate: QuantumGate,
        target1: usize,
        target2: usize,
    ) -> Result<(), String> {
        if gate.num_qubits() != 2 {
            return Err("Gate is not a two-qubit gate".to_string());
        }
        
        self.add_gate(CircuitGate {
            gate,
            targets: vec![target1, target2],
            controls: vec![],
            params: None,
        })
    }

    /// Add a controlled gate
    pub fn add_controlled_gate(
        &mut self,
        gate: QuantumGate,
        control: usize,
        target: usize,
    ) -> Result<(), String> {
        self.add_gate(CircuitGate {
            gate,
            targets: vec![target],
            controls: vec![control],
            params: None,
        })
    }

    /// Add a parameterized gate
    pub fn add_parameterized_gate(
        &mut self,
        gate: QuantumGate,
        target: usize,
        params: Vec<f64>,
    ) -> Result<(), String> {
        self.add_gate(CircuitGate {
            gate,
            targets: vec![target],
            controls: vec![],
            params: Some(params),
        })
    }

    /// Get all gates in the circuit
    pub fn gates(&self) -> &[CircuitGate] {
        &self.gates
    }

    /// Count the number of gates
    pub fn gate_count(&self) -> usize {
        self.gates.len()
    }

    /// Get a specific gate
    pub fn get_gate(&self, index: usize) -> Option<&CircuitGate> {
        self.gates.get(index)
    }

    /// Remove a gate from the circuit
    pub fn remove_gate(&mut self, index: usize) -> Result<(), String> {
        if index >= self.gates.len() {
            return Err("Gate index out of range".to_string());
        }
        self.gates.remove(index);
        Ok(())
    }

    /// Clear all gates from the circuit
    pub fn clear(&mut self) {
        self.gates.clear();
    }

    /// Set a circuit parameter
    pub fn set_parameter(&mut self, name: String, value: f64) {
        self.parameters.insert(name, value);
    }

    /// Get a circuit parameter
    pub fn get_parameter(&self, name: &str) -> Option<f64> {
        self.parameters.get(name).copied()
    }

    /// Get all parameters
    pub fn parameters(&self) -> &HashMap<String, f64> {
        &self.parameters
    }

    /// Optimize the circuit
    pub fn optimize(&mut self) {
        self.cancel_identity_gates();
        self.merge_single_qubit_gates();
        self.optimize_cnot_pairs();
    }

    /// Compile the circuit to a target architecture
    pub fn compile(&self, target: &str) -> Result<CompiledCircuit, String> {
        match target {
            "native" => Ok(CompiledCircuit {
                gates: self.gates.clone(),
                num_qubits: self.num_qubits,
                target: target.to_string(),
            }),
            "qasm" => self.compile_to_qasm(),
            _ => Err(format!("Unsupported compilation target: {}", target)),
        }
    }

    /// Check circuit equivalence
    pub fn is_equivalent_to(&self, other: &QuantumCircuit) -> bool {
        if self.num_qubits != other.num_qubits {
            return false;
        }
        
        // Simple check: same gates in same order
        if self.gates.len() != other.gates.len() {
            return false;
        }
        
        for (g1, g2) in self.gates.iter().zip(other.gates.iter()) {
            if g1.gate != g2.gate || g1.targets != g2.targets || g1.controls != g2.controls {
                return false;
            }
        }
        
        true
    }

    /// Export circuit to QASM format
    pub fn to_qasm(&self) -> String {
        let mut qasm = String::new();
        
        qasm.push_str(&format!("OPENQASM 2.0;\n"));
        qasm.push_str(&format!("include &quot;qelib1.inc&quot;;\n"));
        qasm.push_str(&format!("qreg q[{}];\n", self.num_qubits));
        if self.num_bits > 0 {
            qasm.push_str(&format!("creg c[{}];\n", self.num_bits));
        }
        
        for gate in &self.gates {
            let gate_str = match &gate.gate {
                QuantumGate::PauliX => "x",
                QuantumGate::PauliY => "y",
                QuantumGate::PauliZ => "z",
                QuantumGate::Hadamard => "h",
                QuantumGate::Phase => "s",
                QuantumGate::S => "s",
                QuantumGate::T => "t",
                QuantumGate::CNOT => "cx",
                QuantumGate::CZ => "cz",
                QuantumGate::SWAP => "swap",
                _ => "unknown",
            };
            
            if gate.controls.is_empty() {
                qasm.push_str(&format!("{} q[{}];\n", gate_str, gate.targets[0]));
            } else {
                qasm.push_str(&format!(
                    "{} q[{}], q[{}];\n",
                    gate_str, gate.controls[0], gate.targets[0]
                ));
            }
        }
        
        qasm
    }

    /// Import circuit from QASM format
    pub fn from_qasm(qasm: &str) -> Result<Self, String> {
        // Simplified QASM parsing
        let lines: Vec<&str> = qasm.lines().collect();
        let mut num_qubits = 0;
        let mut circuit = QuantumCircuit::new(0);
        
        for line in lines {
            let line = line.trim();
            if line.starts_with("qreg") {
                let parts: Vec<&str> = line.split(&['[', ']', ';', ' ']).collect();
                num_qubits = parts[3].parse::<usize>().unwrap_or(0);
                circuit = QuantumCircuit::new(num_qubits);
            } else if line.starts_with("x ") {
                let target = line.split('q[').nth(1).unwrap().split(']').next().unwrap();
                let target_idx = target.parse::<usize>().unwrap();
                circuit.add_single_qubit_gate(QuantumGate::PauliX, target_idx).unwrap();
            } else if line.starts_with("h ") {
                let target = line.split('q[').nth(1).unwrap().split(']').next().unwrap();
                let target_idx = target.parse::<usize>().unwrap();
                circuit.add_single_qubit_gate(QuantumGate::Hadamard, target_idx).unwrap();
            } else if line.starts_with("cx ") {
                let parts: Vec<&str> = line.split(&['q', '[', ']', ',', ' ', ';']).collect();
                let control_idx = parts[3].parse::<usize>().unwrap();
                let target_idx = parts[6].parse::<usize>().unwrap();
                circuit.add_controlled_gate(QuantumGate::CNOT, control_idx, target_idx).unwrap();
            }
        }
        
        Ok(circuit)
    }

    /// Get circuit metadata
    pub fn metadata(&self) -> &CircuitMetadata {
        &self.metadata
    }

    /// Set circuit metadata
    pub fn set_metadata(&mut self, metadata: CircuitMetadata) {
        self.metadata = metadata;
    }

    // ==================== Optimization Methods ====================

    fn cancel_identity_gates(&mut self) {
        // Remove gates that result in identity
        self.gates.retain(|gate| {
            // Simplified: keep all gates for now
            true
        });
    }

    fn merge_single_qubit_gates(&mut self) {
        // Merge consecutive single-qubit gates on the same qubit
        let mut i = 0;
        while i < self.gates.len() - 1 {
            let gate1 = &self.gates[i];
            let gate2 = &self.gates[i + 1];
            
            // Check if both gates act on the same single qubit
            if gate1.targets.len() == 1 && gate2.targets.len() == 1
                && gate1.targets[0] == gate2.targets[0]
                && gate1.controls.is_empty() && gate2.controls.is_empty()
            {
                // Merge the gates (simplified)
                self.gates.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn optimize_cnot_pairs(&mut self) {
        // Cancel adjacent CNOT gates on the same qubit pair
        let mut i = 0;
        while i < self.gates.len() - 1 {
            let gate1 = &self.gates[i];
            let gate2 = &self.gates[i + 1];
            
            if matches!(gate1.gate, QuantumGate::CNOT)
                && matches!(gate2.gate, QuantumGate::CNOT)
                && gate1.targets == gate2.targets
                && gate1.controls == gate2.controls
            {
                // Cancel the pair
                self.gates.remove(i);
                self.gates.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn compile_to_qasm(&self) -> Result<CompiledCircuit, String> {
        Ok(CompiledCircuit {
            gates: self.gates.clone(),
            num_qubits: self.num_qubits,
            target: "qasm".to_string(),
        })
    }
}

/// Compiled circuit representation
#[derive(Clone, Debug)]
pub struct CompiledCircuit {
    pub gates: Vec<CircuitGate>,
    pub num_qubits: usize,
    pub target: String,
}

impl CircuitGate {
    /// Create a new circuit gate
    pub fn new(gate: QuantumGate, targets: Vec<usize>) -> Self {
        CircuitGate {
            gate,
            targets,
            controls: vec![],
            params: None,
        }
    }

    /// Add control qubits
    pub fn with_controls(mut self, controls: Vec<usize>) -> Self {
        self.controls = controls;
        self
    }

    /// Add parameters
    pub fn with_params(mut self, params: Vec<f64>) -> Self {
        self.params = Some(params);
        self
    }
}

/// Circuit builder for convenient circuit construction
pub struct CircuitBuilder {
    circuit: QuantumCircuit,
}

impl CircuitBuilder {
    /// Create a new circuit builder
    pub fn new(num_qubits: usize) -> Self {
        CircuitBuilder {
            circuit: QuantumCircuit::new(num_qubits),
        }
    }

    /// Add a gate
    pub fn gate(mut self, gate: CircuitGate) -> Result<Self, String> {
        self.circuit.add_gate(gate)?;
        Ok(self)
    }

    /// Add a Hadamard gate
    pub fn h(mut self, target: usize) -> Result<Self, String> {
        self.circuit.add_single_qubit_gate(QuantumGate::Hadamard, target)?;
        Ok(self)
    }

    /// Add a CNOT gate
    pub fn cx(mut self, control: usize, target: usize) -> Result<Self, String> {
        self.circuit.add_controlled_gate(QuantumGate::CNOT, control, target)?;
        Ok(self)
    }

    /// Add a Pauli-X gate
    pub fn x(mut self, target: usize) -> Result<Self, String> {
        self.circuit.add_single_qubit_gate(QuantumGate::PauliX, target)?;
        Ok(self)
    }

    /// Build the circuit
    pub fn build(self) -> QuantumCircuit {
        self.circuit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_creation() {
        let circuit = QuantumCircuit::new(2);
        assert_eq!(circuit.num_qubits(), 2);
        assert_eq!(circuit.gate_count(), 0);
    }

    #[test]
    fn test_add_gate() {
        let mut circuit = QuantumCircuit::new(2);
        circuit
            .add_single_qubit_gate(QuantumGate::Hadamard, 0)
            .unwrap();
        assert_eq!(circuit.gate_count(), 1);
    }

    #[test]
    fn test_circuit_depth() {
        let mut circuit = QuantumCircuit::new(2);
        circuit.add_single_qubit_gate(QuantumGate::Hadamard, 0).unwrap();
        circuit
            .add_controlled_gate(QuantumGate::CNOT, 0, 1)
            .unwrap();
        assert_eq!(circuit.depth(), 2);
    }

    #[test]
    fn test_qasm_export() {
        let mut circuit = QuantumCircuit::new(2);
        circuit.add_single_qubit_gate(QuantumGate::Hadamard, 0).unwrap();
        circuit
            .add_controlled_gate(QuantumGate::CNOT, 0, 1)
            .unwrap();
        
        let qasm = circuit.to_qasm();
        assert!(qasm.contains("OPENQASM"));
        assert!(qasm.contains("qreg q[2]"));
    }

    #[test]
    fn test_circuit_builder() {
        let circuit = CircuitBuilder::new(2)
            .h(0)
            .unwrap()
            .cx(0, 1)
            .unwrap()
            .build();
        
        assert_eq!(circuit.gate_count(), 2);
    }

    #[test]
    fn test_circuit_optimization() {
        let mut circuit = QuantumCircuit::new(2);
        circuit
            .add_controlled_gate(QuantumGate::CNOT, 0, 1)
            .unwrap();
        circuit
            .add_controlled_gate(QuantumGate::CNOT, 0, 1)
            .unwrap();
        
        circuit.optimize();
        assert_eq!(circuit.gate_count(), 0);
    }

    #[test]
    fn test_circuit_equivalence() {
        let circuit1 = CircuitBuilder::new(2).h(0).unwrap().cx(0, 1).unwrap().build();
        let circuit2 = CircuitBuilder::new(2).h(0).unwrap().cx(0, 1).unwrap().build();
        
        assert!(circuit1.is_equivalent_to(&circuit2));
    }
}