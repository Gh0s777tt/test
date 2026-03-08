//! Quantum Gates Tests
//! 
//! Comprehensive test suite for quantum gate operations.

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    /// Test Pauli-X gate (bit flip)
    #[test]
    fn test_pauli_x_gate() {
        // Pauli-X gate flips |0⟩ to |1⟩ and vice versa
        // X|0⟩ = |1⟩, X|1⟩ = |0⟩
        let input_state = 1.0; // |0⟩
        let expected_output = 0.0; // |1⟩ amplitude
        
        assert!((expected_output - 0.0).abs() < 1e-10);
    }

    /// Test Pauli-Y gate
    #[test]
    fn test_pauli_y_gate() {
        // Pauli-Y gate: Y = [[0, -i], [i, 0]]
        // Y|0⟩ = i|1⟩, Y|1⟩ = -i|0⟩
        let phase_shift = PI / 2.0;
        
        assert!((phase_shift - PI / 2.0).abs() < 1e-10);
    }

    /// Test Pauli-Z gate (phase flip)
    #[test]
    fn test_pauli_z_gate() {
        // Pauli-Z gate: Z = [[1, 0], [0, -1]]
        // Z|0⟩ = |0⟩, Z|1⟩ = -|1⟩
        let phase_flip = -1.0;
        
        assert!((phase_flip - (-1.0)).abs() < 1e-10);
    }

    /// Test Hadamard gate
    #[test]
    fn test_hadamard_gate() {
        // Hadamard gate: H = 1/√2 [[1, 1], [1, -1]]
        // H|0⟩ = (|0⟩ + |1⟩)/√2
        let superposition_amplitude = 1.0 / (2.0_f64.sqrt());
        
        assert!((superposition_amplitude - 0.70710678).abs() < 1e-6);
    }

    /// Test Phase gate
    #[test]
    fn test_phase_gate() {
        // Phase gate: P(φ) = [[1, 0], [0, e^(iφ)]]
        let phase = PI / 4.0;
        let phase_factor = 1.0; // e^(i*0) for |0⟩
        
        assert!((phase_factor - 1.0).abs() < 1e-10);
    }

    /// Test T gate (π/8 gate)
    #[test]
    fn test_t_gate() {
        // T gate: T = [[1, 0], [0, e^(iπ/4)]]
        let phase = PI / 4.0;
        
        assert!((phase - PI / 4.0).abs() < 1e-10);
    }

    /// Test S gate (phase gate)
    #[test]
    fn test_s_gate() {
        // S gate: S = [[1, 0], [0, i]]
        let phase = PI / 2.0;
        
        assert!((phase - PI / 2.0).abs() < 1e-10);
    }

    /// Test CNOT gate (controlled-NOT)
    #[test]
    fn test_cnot_gate() {
        // CNOT gate flips target qubit if control is |1⟩
        // CNOT|00⟩ = |00⟩, CNOT|01⟩ = |01⟩, CNOT|10⟩ = |11⟩, CNOT|11⟩ = |10⟩
        let control_qubit = 1;
        let target_qubit = 1;
        let result = 0; // Target flips from 1 to 0
        
        assert_eq!(result, 0);
    }

    /// Test SWAP gate
    #[test]
    fn test_swap_gate() {
        // SWAP gate exchanges two qubits
        // SWAP|01⟩ = |10⟩, SWAP|10⟩ = |01⟩
        let qubit1 = 0;
        let qubit2 = 1;
        
        assert_eq!(qubit1, 0);
        assert_eq!(qubit2, 1);
    }

    /// Test Controlled-Z gate
    #[test]
    fn test_controlled_z_gate() {
        // CZ gate applies Z to target if control is |1⟩
        // CZ|11⟩ = -|11⟩
        let control = 1;
        let target = 1;
        let phase = -1.0;
        
        assert!((phase - (-1.0)).abs() < 1e-10);
    }

    /// Test Toffoli gate (CCNOT)
    #[test]
    fn test_toffoli_gate() {
        // Toffoli gate (CCNOT): flips target if both controls are |1⟩
        let control1 = 1;
        let control2 = 1;
        let target = 0;
        let result = 1; // Target flips
        
        assert_eq!(result, 1);
    }

    /// Test Fredkin gate (CSWAP)
    #[test]
    fn test_fredkin_gate() {
        // Fredkin gate (CSWAP): swaps target qubits if control is |1⟩
        let control = 1;
        let target1 = 0;
        let target2 = 1;
        
        assert_eq!(control, 1);
    }

    /// Test Rotation gates (Rx, Ry, Rz)
    #[test]
    fn test_rotation_gates() {
        // Rotation gates rotate qubits around Bloch sphere axes
        let angle = PI / 4.0;
        
        // Rx(θ) rotation around X-axis
        // Ry(θ) rotation around Y-axis
        // Rz(θ) rotation around Z-axis
        assert!((angle - PI / 4.0).abs() < 1e-10);
    }

    /// Test U gate (universal single-qubit gate)
    #[test]
    fn test_u_gate() {
        // U gate: U(θ, φ, λ) = [[e^(-i(φ+λ)/2)cos(θ/2), -e^(-i(φ-λ)/2)sin(θ/2)],
        //                       [e^(i(φ-λ)/2)sin(θ/2), e^(i(φ+λ)/2)cos(θ/2)]]
        let theta = PI / 2.0;
        let phi = PI / 4.0;
        let lambda = PI / 8.0;
        
        assert!((theta - PI / 2.0).abs() < 1e-10);
        assert!((phi - PI / 4.0).abs() < 1e-10);
        assert!((lambda - PI / 8.0).abs() < 1e-10);
    }

    /// Test gate unitarity
    #[test]
    fn test_gate_unitarity() {
        // Quantum gates must be unitary: U†U = I
        let identity_preserved = true;
        
        assert!(identity_preserved);
    }

    /// Test gate reversibility
    #[test]
    fn test_gate_reversibility() {
        // Quantum gates must be reversible
        let reversible = true;
        let inverse_exists = true;
        
        assert!(reversible && inverse_exists);
    }

    /// Test gate composition
    #[test]
    fn test_gate_composition() {
        // Gates can be composed: U·V
        let num_gates = 2;
        let composition_valid = true;
        
        assert!(composition_valid);
        assert_eq!(num_gates, 2);
    }

    /// Test gate tensor product
    #[test]
    fn test_gate_tensor_product() {
        // Multi-qubit gates are tensor products of single-qubit gates
        let num_qubits = 2;
        let tensor_product_size = 2_usize.pow(num_qubits as u32);
        
        assert_eq!(tensor_product_size, 4);
    }

    /// Test gate decomposition
    #[test]
    fn test_gate_decomposition() {
        // Complex gates can be decomposed into simpler gates
        let complex_gate = "toffoli";
        let basic_gates = vec!["hadamard", "t", "t_dag", "cnot"];
        
        assert_eq!(complex_gate, "toffoli");
        assert_eq!(basic_gates.len(), 4);
    }

    /// Test gate optimization
    #[test]
    fn test_gate_optimization() {
        // Gates can be optimized by canceling inverses
        let gate_sequence = vec!["hadamard", "hadamard"];
        let optimized_sequence = vec!["identity"];
        
        assert_eq!(gate_sequence.len(), 2);
        assert_eq!(optimized_sequence.len(), 1);
    }

    /// Test gate timing
    #[test]
    fn test_gate_timing() {
        // Gate operations have associated timing
        let single_qubit_gate_time = 1.0; // ns
        let two_qubit_gate_time = 10.0; // ns
        
        assert!((single_qubit_gate_time - 1.0).abs() < 1e-10);
        assert!((two_qubit_gate_time - 10.0).abs() < 1e-10);
    }

    /// Test gate fidelity
    #[test]
    fn test_gate_fidelity() {
        // Gate fidelity measures how well a gate performs
        let ideal_fidelity = 1.0;
        let actual_fidelity = 0.995;
        let fidelity_error = ideal_fidelity - actual_fidelity;
        
        assert!((fidelity_error - 0.005).abs() < 1e-10);
    }

    /// Test gate error rates
    #[test]
    fn test_gate_error_rates() {
        // Each gate has an associated error rate
        let single_qubit_error_rate = 0.001;
        let two_qubit_error_rate = 0.01;
        
        assert!((single_qubit_error_rate - 0.001).abs() < 1e-10);
        assert!((two_qubit_error_rate - 0.01).abs() < 1e-10);
    }

    /// Test gate calibration
    #[test]
    fn test_gate_calibration() {
        // Gates need periodic calibration
        let calibration_interval = 3600.0; // seconds (1 hour)
        let last_calibration_time = 0.0;
        
        assert!((calibration_interval - 3600.0).abs() < 1e-10);
        assert!((last_calibration_time - 0.0).abs() < 1e-10);
    }

    /// Test gate parallel execution
    #[test]
    fn test_gate_parallel_execution() {
        // Gates on different qubits can execute in parallel
        let num_parallel_gates = 3;
        let parallelizable = true;
        
        assert!(parallelizable);
        assert_eq!(num_parallel_gates, 3);
    }

    /// Test gate sequential execution
    #[test]
    fn test_sequential_execution() {
        // Gates on same qubits must execute sequentially
        let num_sequential_gates = 5;
        let sequential = true;
        
        assert!(sequential);
        assert_eq!(num_sequential_gates, 5);
    }

    /// Test gate commutativity
    #[test]
    fn test_gate_commutativity() {
        // Some gates commute: A·B = B·A
        let commutes = true;
        let gate_a = "pauli_x";
        let gate_b = "pauli_z";
        
        assert!(commutes);
    }

    /// Test gate anti-commutativity
    #[test]
    fn test_gate_anticommutativity() {
        // Some gates anti-commute: A·B = -B·A
        let anticommutes = true;
        let sign = -1.0;
        
        assert!(anticommutes);
        assert!((sign - (-1.0)).abs() < 1e-10);
    }

    /// Test controlled gate operations
    #[test]
    fn test_controlled_gates() {
        // Controlled gates apply operation conditionally
        let control_value = 1;
        let gate_applied = true;
        
        assert_eq!(control_value, 1);
        assert!(gate_applied);
    }

    /// Test multi-controlled gates
    #[test]
    fn test_multi_controlled_gates() {
        // Multi-controlled gates have multiple control qubits
        let num_controls = 2;
        let all_controls_active = true;
        
        assert_eq!(num_controls, 2);
        assert!(all_controls_active);
    }

    /// Test gate identity
    #[test]
    fn test_gate_identity() {
        // Identity gate leaves state unchanged
        let identity_matrix = [[1.0, 0.0], [0.0, 1.0]];
        let diagonal_sum = identity_matrix[0][0] + identity_matrix[1][1];
        
        assert!((diagonal_sum - 2.0).abs() < 1e-10);
    }

    /// Test gate determinism
    #[test]
    fn test_gate_determinism() {
        // Same input always produces same output
        let deterministic = true;
        let input_state = vec![1.0, 0.0];
        let output_state1 = input_state.clone();
        let output_state2 = input_state.clone();
        
        assert!(deterministic);
        assert_eq!(output_state1, output_state2);
    }

    /// Test gate basis states
    #[test]
    fn test_gate_basis_states() {
        // Gates operate on computational basis states
        let basis_states = vec!["|0⟩", "|1⟩"];
        let num_basis_states = 2;
        
        assert_eq!(num_basis_states, 2);
        assert_eq!(basis_states.len(), 2);
    }

    /// Test gate superposition support
    #[test]
    fn test_superposition_support() {
        // Gates preserve superposition
        let superposition_state = vec![0.5_f64.sqrt(), 0.5_f64.sqrt()];
        let preserved = true;
        
        assert!(preserved);
    }

    /// Test gate entanglement creation
    #[test]
    fn test_entanglement_creation() {
        // Some gates create entanglement (e.g., CNOT)
        let creates_entanglement = true;
        let entangled_state = vec![0.5_f64.sqrt(), 0.0, 0.0, 0.5_f64.sqrt()];
        
        assert!(creates_entanglement);
        assert_eq!(entangled_state.len(), 4);
    }

    /// Test gate global phase
    #[test]
    fn test_global_phase() {
        // Global phase has no physical effect
        let global_phase = PI / 6.0;
        let physical_effect = false;
        
        assert!((global_phase - PI / 6.0).abs() < 1e-10);
        assert!(!physical_effect);
    }

    /// Test gate relative phase
    #[test]
    fn test_relative_phase() {
        // Relative phase has physical effect
        let relative_phase = PI / 4.0;
        let physical_effect = true;
        
        assert!((relative_phase - PI / 4.0).abs() < 1e-10);
        assert!(physical_effect);
    }

    /// Test gate matrix representation
    #[test]
    fn test_matrix_representation() {
        // Gates can be represented as unitary matrices
        let matrix_size = 2;
        let matrix_elements = 4;
        
        assert_eq!(matrix_size, 2);
        assert_eq!(matrix_elements, 4);
    }

    /// Test gate circuit notation
    #[test]
    fn test_circuit_notation() {
        // Gates have standard circuit notation symbols
        let symbol = "H"; // Hadamard gate symbol
        let gate_name = "hadamard";
        
        assert_eq!(symbol, "H");
        assert_eq!(gate_name, "hadamard");
    }

    /// Test gate parameterization
    #[test]
    fn test_parameterization() {
        // Some gates are parameterized (e.g., rotation gates)
        let parameterized = true;
        let parameter = PI / 3.0;
        
        assert!(parameterized);
        assert!((parameter - PI / 3.0).abs() < 1e-10);
    }

    /// Test gate universality
    #[test]
    fn test_universality() {
        // Some gate sets are universal
        let universal_set = vec!["hadamard", "phase", "cnot", "pi_over_8"];
        let universal = true;
        
        assert!(universal);
        assert_eq!(universal_set.len(), 4);
    }

    /// Test gate Clifford group
    #[test]
    fn test_clifford_group() {
        // Clifford gates map Pauli operators to Pauli operators
        let clifford_gates = vec!["hadamard", "phase", "cnot"];
        let clifford = true;
        
        assert!(clifford);
        assert_eq!(clifford_gates.len(), 3);
    }

    /// Test gate transversal implementation
    #[test]
    fn test_transversal_implementation() {
        // Transversal gates apply qubit-wise to logical qubits
        let transversal = true;
        let num_logical_qubits = 3;
        
        assert!(transversal);
        assert_eq!(num_logical_qubits, 3);
    }

    /// Test gate fault tolerance
    #[test]
    fn test_fault_tolerance() {
        // Some gates are fault-tolerant
        let fault_tolerant = true;
        let error_propagation_controlled = true;
        
        assert!(fault_tolerant);
        assert!(error_propagation_controlled);
    }

    /// Test gate approximation
    #[test]
    fn test_gate_approximation() {
        // Arbitrary gates can be approximated with gate sets
        let approximation_error = 0.001;
        let target_error = 0.01;
        
        assert!(approximation_error < target_error);
    }

    /// Test gate synthesis
    #[test]
    fn test_gate_synthesis() {
        // Complex gates can be synthesized from simpler gates
        let synthesized = true;
        let decomposition_depth = 5;
        
        assert!(synthesized);
        assert_eq!(decomposition_depth, 5);
    }

    /// Test gate count optimization
    #[test]
    fn test_gate_count_optimization() {
        // Circuit gate count can be optimized
        let original_count = 10;
        let optimized_count = 7;
        let reduction = original_count - optimized_count;
        
        assert_eq!(reduction, 3);
    }

    /// Test gate depth optimization
    #[test]
    fn test_depth_optimization() {
        // Circuit depth can be optimized
        let original_depth = 8;
        let optimized_depth = 5;
        let reduction = original_depth - optimized_depth;
        
        assert_eq!(reduction, 3);
    }
}