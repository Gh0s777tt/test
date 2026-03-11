//! VantisOS v1.6.0 Integration Tests
//!
//! Tests cross-module interactions and validates that all v1.6.0 Enhanced Features
//! modules work correctly both individually and together.

// ============================================================================
// AI Enhanced Module Tests
// ============================================================================

#[cfg(test)]
mod ai_tests {
    use crate::ai_enhanced::anomaly_detection::{
        AnomalyDetector, DetectorConfig, DetectionMethod, RollingStats, EwmaState,
    };
    use crate::ai_enhanced::inference_engine::{
        InferenceEngine, Tensor, TensorShape, ModelFormat, AcceleratorBackend, InferencePrecision,
    };
    use crate::ai_enhanced::federated_learning::{
        FederatedCoordinator, AggregationStrategy, PrivacyBudget,
    };
    use crate::ai_enhanced::model_optimizer::{
        ModelOptimizer, OptimizationConfig, WeightTensor, QuantizationMethod, PruningStrategy,
    };
    use crate::ai_enhanced::resource_predictor::{
        ResourcePredictor, ResourceType, PredictionHorizon, HoltState,
    };

    #[test]
    fn test_anomaly_detector_zscore() {
        let mut detector = AnomalyDetector::zscore("cpu_usage");
        // Feed normal data to build baseline
        for i in 0..50 {
            let value = 50.0 + (i as f64 % 10.0);
            let _ = detector.observe(value);
        }
        assert!(detector.total_observations() >= 50);
        // Feed anomalous value
        let event = detector.observe(999.0);
        assert!(event.is_some(), "Should detect anomaly for extreme value");
        assert!(detector.total_anomalies() > 0);
    }

    #[test]
    fn test_anomaly_detector_ewma() {
        let mut detector = AnomalyDetector::ewma("memory_usage", 0.3);
        for i in 0..30 {
            let _ = detector.observe(100.0 + (i as f64 * 0.5));
        }
        assert!(detector.total_observations() == 30);
        assert!(detector.anomaly_rate() >= 0.0);
    }

    #[test]
    fn test_anomaly_detector_config() {
        let config = DetectorConfig {
            metric_name: String::from("disk_io"),
            method: DetectionMethod::ZScore,
            threshold_multiplier: 3.0,
            warmup_period: 10,
            cooldown_ticks: 5,
            window_size: 100,
            ewma_alpha: 0.3,
        };
        let mut detector = AnomalyDetector::new(config);
        for _ in 0..20 {
            let _ = detector.observe(42.0);
        }
        assert_eq!(detector.total_observations(), 20);
        assert_eq!(detector.total_anomalies(), 0);
    }

    #[test]
    fn test_anomaly_detector_reset() {
        let mut detector = AnomalyDetector::zscore("test_metric");
        for _ in 0..20 {
            let _ = detector.observe(50.0);
        }
        assert!(detector.total_observations() > 0);
        detector.reset();
        assert_eq!(detector.total_observations(), 0);
        assert_eq!(detector.total_anomalies(), 0);
    }

    #[test]
    fn test_rolling_stats() {
        let mut stats = RollingStats::new(100);
        for i in 1..=10 {
            stats.push(i as f64);
        }
        assert_eq!(stats.count(), 10);
        let mean = stats.mean();
        assert!((mean - 5.5).abs() < 0.01, "Mean should be 5.5, got {}", mean);
        assert!(stats.std_dev() > 0.0);
        assert!(stats.iqr() > 0.0);
    }

    #[test]
    fn test_ewma_state() {
        let mut ewma = EwmaState::new(0.3);
        ewma.update(100.0);
        ewma.update(100.0);
        ewma.update(100.0);
        let sd = ewma.std_dev();
        assert!(sd < 1.0, "Std dev should be near zero for constant input, got {}", sd);
    }

    #[test]
    fn test_inference_engine_lifecycle() {
        let mut engine = InferenceEngine::new(64 * 1024 * 1024); // 64MB budget
        engine.register_backend(AcceleratorBackend::Cpu);
        assert!(engine.is_backend_available(AcceleratorBackend::Cpu));
        assert!(!engine.is_backend_available(AcceleratorBackend::Gpu));
        assert_eq!(engine.active_sessions(), 0);
        assert_eq!(engine.total_inferences(), 0);
    }

    #[test]
    fn test_inference_engine_model_load() {
        let mut engine = InferenceEngine::new(64 * 1024 * 1024);
        engine.register_backend(AcceleratorBackend::Cpu);

        let input_shape = TensorShape::new(vec![1, 784]);
        let output_shape = TensorShape::new(vec![1, 10]);

        let result = engine.load_model(
            "test_model",
            ModelFormat::Onnx,
            AcceleratorBackend::Cpu,
            InferencePrecision::Float32,
            input_shape,
            output_shape,
        );
        assert!(result.is_ok(), "Model load should succeed");
        assert_eq!(engine.active_sessions(), 1);
    }

    #[test]
    fn test_tensor_operations() {
        let shape = TensorShape::new(vec![2, 3]);
        assert_eq!(shape.num_elements(), 6);
        assert_eq!(shape.rank(), 2);

        let t1 = Tensor::zeros(shape.clone());
        let t2 = Tensor::filled(shape.clone(), 1.0);
        let sum = t1.add(&t2);
        assert!(sum.is_ok());

        let relu = t2.relu();
        assert_eq!(relu.data.len(), 6);

        let softmax = t2.softmax();
        let total: f32 = softmax.data.iter().sum();
        assert!((total - 1.0).abs() < 0.01, "Softmax should sum to 1.0");
    }

    #[test]
    fn test_federated_coordinator_lifecycle() {
        let mut coord = FederatedCoordinator::new(
            AggregationStrategy::FedAvg,
            3,  // model_dimension
            2,  // min_participants
        );
        let p1 = coord.register_participant("node_a", 1000);
        let p2 = coord.register_participant("node_b", 2000);
        assert!(p1.is_ok());
        assert!(p2.is_ok());
        assert_eq!(coord.participant_count(), 2);
    }

    #[test]
    fn test_federated_training_round() {
        let mut coord = FederatedCoordinator::new(
            AggregationStrategy::FedAvg, 3, 2,
        );
        let p1_id = coord.register_participant("node_a", 1000).unwrap();
        let p2_id = coord.register_participant("node_b", 2000).unwrap();

        let round = coord.start_round();
        assert!(round.is_ok());

        let update1 = vec![0.1_f32, 0.2, 0.3];
        let update2 = vec![0.4_f32, 0.5, 0.6];
        assert!(coord.submit_update(p1_id, update1).is_ok());
        assert!(coord.submit_update(p2_id, update2).is_ok());

        let aggregated = coord.aggregate();
        assert!(aggregated.is_ok());
        let result = aggregated.unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(coord.completed_rounds(), 1);
    }

    #[test]
    fn test_federated_privacy_budget() {
        let mut budget = PrivacyBudget::new(10.0, 1e-5, 1.0);
        assert!(!budget.is_exhausted());
        assert!((budget.remaining() - 10.0).abs() < 0.01);
        assert!(budget.consume(3.0));
        assert!((budget.remaining() - 7.0).abs() < 0.01);
        assert!(!budget.consume(8.0)); // exceeds remaining
    }

    #[test]
    fn test_model_optimizer_quantization() {
        let config = OptimizationConfig {
            quantization: QuantizationMethod::Int8,
            pruning: PruningStrategy::None,
            ..Default::default()
        };
        let mut optimizer = ModelOptimizer::new(config);
        let weights = WeightTensor::new("layer1", vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        optimizer.add_weights(weights);
        assert_eq!(optimizer.num_layers(), 1);

        let result = optimizer.optimize();
        assert!(result.is_ok());
        let summary = result.unwrap();
        assert!(summary.size_reduction_pct() >= 0.0);
    }

    #[test]
    fn test_model_optimizer_pruning() {
        let config = OptimizationConfig {
            quantization: QuantizationMethod::None,
            pruning: PruningStrategy::MagnitudeBased,
            ..Default::default()
        };
        let mut optimizer = ModelOptimizer::new(config);
        let weights = WeightTensor::new(
            "dense1",
            vec![0.001, 5.0, 0.0001, 3.0, 0.002, 7.0],
            vec![2, 3],
        );
        optimizer.add_weights(weights);
        let result = optimizer.optimize();
        assert!(result.is_ok());
    }

    #[test]
    fn test_model_optimizer_defaults() {
        let optimizer = ModelOptimizer::with_defaults();
        assert_eq!(optimizer.num_layers(), 0);
    }

    #[test]
    fn test_resource_predictor_lifecycle() {
        let mut predictor = ResourcePredictor::new(5);
        predictor.track_resource(ResourceType::Cpu, 0.3, 0.1, 80.0);
        assert_eq!(predictor.tracked_count(), 1);

        for i in 0..10 {
            let val = 50.0 + (i as f64 * 2.0);
            assert!(predictor.observe(ResourceType::Cpu, val).is_ok());
        }
        assert!(predictor.observation_count(ResourceType::Cpu).unwrap() >= 10);
    }

    #[test]
    fn test_resource_predictor_forecast() {
        let mut predictor = ResourcePredictor::new(3);
        predictor.track_resource(ResourceType::Memory, 0.3, 0.1, 90.0);

        for i in 0..10 {
            let _ = predictor.observe(ResourceType::Memory, 40.0 + i as f64);
        }

        let prediction = predictor.predict(ResourceType::Memory, PredictionHorizon::OneMinute);
        assert!(prediction.is_ok());
    }

    #[test]
    fn test_resource_predictor_recommendation() {
        let mut predictor = ResourcePredictor::new(3);
        predictor.track_resource(ResourceType::DiskIo, 0.3, 0.1, 80.0);

        for i in 0..10 {
            let _ = predictor.observe(ResourceType::DiskIo, 80.0 + i as f64);
        }

        let action = predictor.recommend(ResourceType::DiskIo);
        assert!(action.is_ok());
    }

    #[test]
    fn test_holt_state_forecasting() {
        let mut holt = HoltState::new(0.3, 0.1);
        for i in 0..20 {
            holt.update(10.0 + i as f64);
        }
        let forecast = holt.forecast(5);
        assert!(forecast > 20.0, "Forecast should extrapolate upward trend, got {}", forecast);
    }
}

// ============================================================================
// Networking Enhanced Module Tests
// ============================================================================

#[cfg(test)]
mod networking_tests {
    use crate::networking_enhanced::sdn_controller::{
        SdnController, NetworkNode, NodeType, Link, FlowMatch, FlowAction,
    };
    use crate::networking_enhanced::traffic_shaper::{
        TrafficShaper, TrafficClass, QosPolicy, ShapingDecision,
    };
    use crate::networking_enhanced::zero_trust_network::{
        ZtnaController, Identity, AccessPolicy, TrustFactors,
    };

    #[test]
    fn test_sdn_controller_topology() {
        let mut sdn = SdnController::new();
        let node1 = NetworkNode::new(1, "switch_a", NodeType::Switch, 24);
        let node2 = NetworkNode::new(2, "switch_b", NodeType::Switch, 24);
        let node3 = NetworkNode::new(3, "router_a", NodeType::Router, 8);

        assert!(sdn.add_node(node1).is_ok());
        assert!(sdn.add_node(node2).is_ok());
        assert!(sdn.add_node(node3).is_ok());
        assert_eq!(sdn.node_count(), 3);

        let link1 = Link::new(1, 0, 2, 0, 10_000, 1); // 10Gbps, 1us
        let link2 = Link::new(2, 1, 3, 0, 1_000, 5);  // 1Gbps, 5us
        assert!(sdn.add_link(link1).is_ok());
        assert!(sdn.add_link(link2).is_ok());
        assert_eq!(sdn.link_count(), 2);
    }

    #[test]
    fn test_sdn_shortest_path() {
        let mut sdn = SdnController::new();
        sdn.add_node(NetworkNode::new(1, "a", NodeType::Switch, 4)).unwrap();
        sdn.add_node(NetworkNode::new(2, "b", NodeType::Switch, 4)).unwrap();
        sdn.add_node(NetworkNode::new(3, "c", NodeType::Switch, 4)).unwrap();

        sdn.add_link(Link::new(1, 0, 2, 0, 1_000, 1)).unwrap();
        sdn.add_link(Link::new(2, 1, 3, 0, 1_000, 1)).unwrap();

        let path = sdn.shortest_path(1, 3);
        assert!(path.is_ok());
        let path = path.unwrap();
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], 1);
        assert_eq!(path[2], 3);
    }

    #[test]
    fn test_sdn_flow_installation() {
        let mut sdn = SdnController::new();
        sdn.add_node(NetworkNode::new(1, "sw1", NodeType::Switch, 4)).unwrap();

        let flow_match = FlowMatch::new();
        let result = sdn.install_flow(1, 100, flow_match, FlowAction::Forward(2));
        assert!(result.is_ok());
        assert_eq!(sdn.flow_count(), 1);
    }

    #[test]
    fn test_sdn_duplicate_node() {
        let mut sdn = SdnController::new();
        sdn.add_node(NetworkNode::new(1, "sw1", NodeType::Switch, 4)).unwrap();
        let result = sdn.add_node(NetworkNode::new(1, "sw1_dup", NodeType::Switch, 4));
        assert!(result.is_err(), "Should reject duplicate node ID");
    }

    #[test]
    fn test_traffic_shaper_basic() {
        let mut shaper = TrafficShaper::new(1_000_000_000);
        let policy = QosPolicy::realtime(100_000_000, 500_000_000, 1000);
        shaper.add_policy(policy);
        assert_eq!(shaper.policy_count(), 1);
        assert_eq!(shaper.global_rate_limit(), 1_000_000_000);
    }

    #[test]
    fn test_traffic_shaper_packet_shaping() {
        let mut shaper = TrafficShaper::new(1_000_000_000);
        shaper.add_policy(QosPolicy::realtime(100_000_000, 500_000_000, 1000));
        shaper.add_policy(QosPolicy::best_effort(200_000_000));

        let decision = shaper.shape_packet(TrafficClass::RealTime, 1500, 1_000_000);
        match decision {
            ShapingDecision::Allow | ShapingDecision::Delay { .. } | ShapingDecision::Mark => {},
            ShapingDecision::Drop => panic!("First realtime packet should not be dropped"),
        }
    }

    #[test]
    fn test_traffic_shaper_stats() {
        let mut shaper = TrafficShaper::new(1_000_000_000);
        shaper.add_policy(QosPolicy::best_effort(500_000_000));

        for i in 0..10u64 {
            let _ = shaper.shape_packet(TrafficClass::BestEffort, 1500, i * 100_000);
        }

        let stats = shaper.global_stats();
        assert!(stats.total_packets() > 0);
    }

    #[test]
    fn test_ztna_controller_identity_management() {
        let mut ztna = ZtnaController::new();
        let mut identity = Identity::new(1, "user@example.com");
        identity.add_role("admin");
        identity.add_role("developer");

        assert!(ztna.register_identity(identity).is_ok());
        assert_eq!(ztna.identity_count(), 1);

        let retrieved = ztna.get_identity(1);
        assert!(retrieved.is_some());
        assert!(retrieved.unwrap().has_role("admin"));
    }

    #[test]
    fn test_ztna_policy_evaluation() {
        let mut ztna = ZtnaController::new();

        let mut identity = Identity::new(1, "admin@corp.com");
        identity.add_role("admin");
        ztna.register_identity(identity).unwrap();

        let policy = AccessPolicy::new(1, "admin_access", "/api/admin/*");
        ztna.add_policy(policy).unwrap();
        assert_eq!(ztna.policy_count(), 1);
    }

    #[test]
    fn test_ztna_audit_log() {
        let mut ztna = ZtnaController::new();
        let identity = Identity::new(1, "user@test.com");
        ztna.register_identity(identity).unwrap();

        let policy = AccessPolicy::new(1, "test_policy", "/test/*");
        ztna.add_policy(policy).unwrap();

        let _ = ztna.evaluate(1, "/test/resource");

        let log = ztna.audit_log();
        assert!(!log.is_empty(), "Audit log should have entries after evaluation");
    }

    #[test]
    fn test_trust_factors() {
        let high = TrustFactors::high_trust();
        let low = TrustFactors::low_trust();
        let default = TrustFactors::default();

        let high_score = high.compute_score();
        let low_score = low.compute_score();
        let default_score = default.compute_score();

        assert!(high_score > low_score, "High trust should score higher than low trust");
        assert!(default_score > 0.0, "Default trust should be positive");
    }
}

// ============================================================================
// Security Enhanced Module Tests
// ============================================================================

#[cfg(test)]
mod security_tests {
    use crate::security_enhanced::runtime_integrity::{
        IntegrityMonitor, ResourceKind,
    };
    use crate::security_enhanced::secure_enclave::{
        SecureEnclave, KeyType, KeyPermission, EnclaveState,
    };

    #[test]
    fn test_integrity_monitor_lifecycle() {
        let mut monitor = IntegrityMonitor::new();
        let hash = [0xAA_u8; 32];
        let result = monitor.register_resource("/kernel/vmlinuz", ResourceKind::SystemBinary, hash);
        assert!(result.is_ok());
        assert_eq!(monitor.resource_count(), 1);
    }

    #[test]
    fn test_integrity_monitor_verification_pass() {
        let mut monitor = IntegrityMonitor::new();
        let hash = [0xBB_u8; 32];
        monitor.register_resource("/boot/initrd", ResourceKind::BootConfig, hash).unwrap();

        let result = monitor.verify_resource("/boot/initrd", hash);
        assert!(result.is_ok());
        let passed = result.unwrap();
        assert!(passed, "Verification should pass with matching hash");
        assert_eq!(monitor.total_checks(), 1);
        assert_eq!(monitor.total_violations(), 0);
    }

    #[test]
    fn test_integrity_monitor_verification_fail() {
        let mut monitor = IntegrityMonitor::new();
        let expected_hash = [0xCC_u8; 32];
        monitor.register_resource("/lib/module.ko", ResourceKind::KernelModule, expected_hash).unwrap();

        let tampered_hash = [0xDD_u8; 32];
        let result = monitor.verify_resource("/lib/module.ko", tampered_hash);
        assert!(result.is_ok());
        let passed = result.unwrap();
        assert!(!passed, "Verification should fail with mismatched hash");
        assert_eq!(monitor.total_violations(), 1);
        assert!(!monitor.violation_log().is_empty());
    }

    #[test]
    fn test_integrity_monitor_full_check() {
        let mut monitor = IntegrityMonitor::new();
        let hash1 = [0x11_u8; 32];
        let hash2 = [0x22_u8; 32];
        monitor.register_resource("/kernel/a", ResourceKind::SystemBinary, hash1).unwrap();
        monitor.register_resource("/kernel/b", ResourceKind::KernelModule, hash2).unwrap();

        let report = monitor.full_check(|path: &str| -> Option<[u8; 32]> {
            match path {
                "/kernel/a" => Some([0x11_u8; 32]),
                "/kernel/b" => Some([0x22_u8; 32]),
                _ => None,
            }
        });
        assert!(report.is_ok());
        let report = report.unwrap();
        assert!(report.is_clean(), "All hashes match, report should be clean");
    }

    #[test]
    fn test_integrity_monitor_baseline_update() {
        let mut monitor = IntegrityMonitor::new();
        let old_hash = [0xAA_u8; 32];
        monitor.register_resource("/sys/config", ResourceKind::ConfigFile, old_hash).unwrap();

        let new_hash = [0xBB_u8; 32];
        let result = monitor.update_baseline("/sys/config", new_hash);
        assert!(result.is_ok());

        let check = monitor.verify_resource("/sys/config", new_hash);
        assert!(check.is_ok());
        assert!(check.unwrap());
    }

    #[test]
    fn test_secure_enclave_lifecycle() {
        let mut enclave = SecureEnclave::new(1);
        assert_eq!(enclave.enclave_id(), 1);
        assert!(matches!(enclave.state(), EnclaveState::Uninitialized));

        let secret = b"my_auth_secret_key_1234567890ab";
        let result = enclave.initialize(secret, 1);
        assert!(result.is_ok());
        assert!(matches!(enclave.state(), EnclaveState::Ready));
    }

    #[test]
    fn test_secure_enclave_authentication() {
        let mut enclave = SecureEnclave::new(2);
        let secret = b"auth_secret_for_enclave_testing";
        enclave.initialize(secret, 1).unwrap();

        // Correct authentication
        let auth = enclave.authenticate(secret);
        assert!(auth.is_ok());
        assert!(matches!(enclave.state(), EnclaveState::Ready));

        // Wrong authentication on a fresh enclave
        let mut enclave2 = SecureEnclave::new(3);
        enclave2.initialize(secret, 1).unwrap();
        let bad_auth = enclave2.authenticate(b"wrong_secret_key_should_fail!!");
        assert!(bad_auth.is_err());
    }

    #[test]
    fn test_secure_enclave_key_generation() {
        let mut enclave = SecureEnclave::new(4);
        let secret = b"key_gen_test_secret_1234567890";
        enclave.initialize(secret, 1).unwrap();
        enclave.authenticate(secret).unwrap();

        let key_result = enclave.generate_key(
            "signing_key",
            KeyType::SigningKeyPair,
            vec![KeyPermission::Sign, KeyPermission::Verify],
        );
        assert!(key_result.is_ok());
        assert_eq!(enclave.key_count(), 1);
    }

    #[test]
    fn test_secure_enclave_encrypt_decrypt() {
        let mut enclave = SecureEnclave::new(5);
        let secret = b"encrypt_test_secret_1234567890";
        enclave.initialize(secret, 1).unwrap();
        enclave.authenticate(secret).unwrap();

        let key_id = enclave.generate_key(
            "aes_key",
            KeyType::Symmetric256,
            vec![KeyPermission::Encrypt, KeyPermission::Decrypt],
        ).unwrap();

        let plaintext = b"Hello, VantisOS secure enclave!";
        let encrypted = enclave.encrypt(key_id, plaintext);
        assert!(encrypted.is_ok(), "Encryption should succeed");

        let enc_result = encrypted.unwrap();
        let decrypted = enclave.decrypt(key_id, &enc_result.ciphertext);
        assert!(decrypted.is_ok(), "Decryption should succeed");
    }

    #[test]
    fn test_secure_enclave_attestation() {
        let mut enclave = SecureEnclave::new(6);
        let secret = b"attest_test_secret_1234567890!";
        enclave.initialize(secret, 1).unwrap();
        enclave.authenticate(secret).unwrap();

        let report = enclave.attest();
        assert!(report.is_ok());
        let report = report.unwrap();
        assert_eq!(report.enclave_id, 6);
    }

    #[test]
    fn test_secure_enclave_destroy() {
        let mut enclave = SecureEnclave::new(7);
        let secret = b"destroy_test_secret_123456789";
        enclave.initialize(secret, 1).unwrap();
        enclave.authenticate(secret).unwrap();

        let _ = enclave.generate_key(
            "temp_key", KeyType::Symmetric256,
            vec![KeyPermission::Encrypt],
        );
        assert_eq!(enclave.key_count(), 1);

        let result = enclave.destroy();
        assert!(result.is_ok());
        assert!(matches!(enclave.state(), EnclaveState::Destroyed));
        assert_eq!(enclave.key_count(), 0);
    }
}

// ============================================================================
// Developer Tools Module Tests
// ============================================================================

#[cfg(test)]
mod devtools_tests {
    use crate::developer_tools::profiler::{
        Profiler, ProfilerState,
    };
    use crate::developer_tools::debugger::{
        KernelDebugger, DebuggerState, BreakpointType, MemoryRegion,
    };
    use crate::developer_tools::build_system::{
        BuildSystem, BuildConfig, BuildTarget, TargetType, Architecture, OptLevel,
    };

    #[test]
    fn test_profiler_lifecycle() {
        let mut profiler = Profiler::new();
        assert!(matches!(profiler.state(), ProfilerState::Idle));

        profiler.start(0).unwrap();
        assert!(matches!(profiler.state(), ProfilerState::Recording));

        profiler.stop(1_000_000).unwrap();
        assert!(matches!(profiler.state(), ProfilerState::Stopped));
    }

    #[test]
    fn test_profiler_spans() {
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();

        let span1 = profiler.begin_span("syscall_handler", 100).unwrap();
        let span2 = profiler.begin_span("memory_alloc", 200).unwrap();
        profiler.end_span(span2, 500).unwrap();
        profiler.end_span(span1, 800).unwrap();

        assert_eq!(profiler.span_count(), 2);

        let report = profiler.report();
        assert!(!report.entries.is_empty());
    }

    #[test]
    fn test_profiler_record_span() {
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();

        for _ in 0..5 {
            profiler.record_span("fast_path", 100).unwrap();
        }
        for _ in 0..3 {
            profiler.record_span("slow_path", 5000).unwrap();
        }

        let report = profiler.report();
        let top = report.top_n(2);
        assert!(!top.is_empty());
    }

    #[test]
    fn test_profiler_pause_resume() {
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();
        assert!(matches!(profiler.state(), ProfilerState::Recording));

        profiler.pause().unwrap();
        assert!(matches!(profiler.state(), ProfilerState::Paused));

        profiler.resume().unwrap();
        assert!(matches!(profiler.state(), ProfilerState::Recording));
    }

    #[test]
    fn test_profiler_report_table() {
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();
        profiler.record_span("test_fn", 1000).unwrap();
        profiler.stop(2000).unwrap();

        let report = profiler.report();
        let table = report.to_table();
        assert!(!table.is_empty());
        assert!(table.contains("test_fn"));
    }

    #[test]
    fn test_debugger_lifecycle() {
        let mut debugger = KernelDebugger::new();
        assert!(matches!(debugger.state(), DebuggerState::Detached));

        debugger.attach().unwrap();
        assert!(matches!(debugger.state(), DebuggerState::Stopped));

        debugger.detach().unwrap();
        assert!(matches!(debugger.state(), DebuggerState::Detached));
    }

    #[test]
    fn test_debugger_breakpoints() {
        let mut debugger = KernelDebugger::new();
        debugger.attach().unwrap();

        let bp1 = debugger.set_breakpoint(0x1000, BreakpointType::Hardware, "entry_point");
        assert!(bp1.is_ok());
        let bp2 = debugger.set_breakpoint(0x2000, BreakpointType::Software, "syscall_entry");
        assert!(bp2.is_ok());
        assert_eq!(debugger.breakpoint_count(), 2);

        let bp1_id = bp1.unwrap();
        debugger.disable_breakpoint(bp1_id).unwrap();
        debugger.enable_breakpoint(bp1_id).unwrap();
    }

    #[test]
    fn test_debugger_memory_operations() {
        let mut debugger = KernelDebugger::new();
        debugger.attach().unwrap();

        let region = MemoryRegion::new(0x1000, vec![0xDE, 0xAD, 0xBE, 0xEF], "test_region");
        debugger.add_memory_region(region);

        let read = debugger.read_memory(0x1000, 4);
        assert!(read.is_ok());
        assert_eq!(read.unwrap(), vec![0xDE, 0xAD, 0xBE, 0xEF]);

        let write = debugger.write_memory(0x1000, &[0xCA, 0xFE]);
        assert!(write.is_ok());

        let read2 = debugger.read_memory(0x1000, 2);
        assert_eq!(read2.unwrap(), vec![0xCA, 0xFE]);
    }

    #[test]
    fn test_debugger_registers() {
        let mut debugger = KernelDebugger::new();
        debugger.attach().unwrap();

        debugger.write_register("rax", 0x42).unwrap();
        let val = debugger.read_register("rax").unwrap();
        assert_eq!(val, 0x42);
    }

    #[test]
    fn test_debugger_stepping() {
        let mut debugger = KernelDebugger::new();
        debugger.attach().unwrap();

        debugger.step().unwrap();
        debugger.step().unwrap();
        assert_eq!(debugger.step_count(), 2);
    }

    #[test]
    fn test_debugger_commands() {
        let mut debugger = KernelDebugger::new();
        debugger.attach().unwrap();

        let result = debugger.execute_command("info registers");
        assert!(result.is_ok());
        assert!(!debugger.command_history().is_empty());
    }

    #[test]
    fn test_build_system_lifecycle() {
        let config = BuildConfig {
            default_opt: OptLevel::O3,
            ..Default::default()
        };
        let build = BuildSystem::new(config);
        assert_eq!(build.target_count(), 0);
    }

    #[test]
    fn test_build_system_targets() {
        let mut build = BuildSystem::with_defaults();

        let mut target = BuildTarget::new("kernel", TargetType::Kernel, Architecture::X86_64);
        target.add_source("src/main.rs");
        target.add_source("src/lib.rs");
        assert_eq!(target.source_count(), 2);

        assert!(build.add_target(target).is_ok());
        assert_eq!(build.target_count(), 1);

        let retrieved = build.get_target("kernel");
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_build_system_dependencies() {
        let mut build = BuildSystem::with_defaults();

        let mut core = BuildTarget::new("core", TargetType::StaticLib, Architecture::X86_64);
        core.add_source("src/core.rs");

        let mut kernel = BuildTarget::new("kernel", TargetType::Kernel, Architecture::X86_64);
        kernel.add_source("src/kernel.rs");
        kernel.add_dependency("core");

        build.add_target(core).unwrap();
        build.add_target(kernel).unwrap();

        let validation = build.validate_dependencies();
        assert!(validation.is_ok());

        let order = build.compute_build_order();
        assert!(order.is_ok());
        let order = order.unwrap();
        let core_pos = order.iter().position(|n| n == "core").unwrap();
        let kernel_pos = order.iter().position(|n| n == "kernel").unwrap();
        assert!(core_pos < kernel_pos, "core must be built before kernel");
    }

    #[test]
    fn test_build_system_build() {
        let mut build = BuildSystem::with_defaults();
        let mut target = BuildTarget::new("test_lib", TargetType::StaticLib, Architecture::X86_64);
        target.add_source("src/test.rs");
        build.add_target(target).unwrap();

        let summary = build.build();
        assert!(summary.is_ok());
        let summary = summary.unwrap();
        assert!(summary.is_success());
        assert!((summary.success_rate() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_build_system_duplicate_target() {
        let mut build = BuildSystem::with_defaults();
        let t1 = BuildTarget::new("lib", TargetType::StaticLib, Architecture::X86_64);
        let t2 = BuildTarget::new("lib", TargetType::StaticLib, Architecture::Aarch64);
        build.add_target(t1).unwrap();
        let result = build.add_target(t2);
        assert!(result.is_err(), "Should reject duplicate target name");
    }
}

// ============================================================================
// Cross-Module Integration Tests
// ============================================================================

#[cfg(test)]
mod cross_module_tests {
    use crate::ai_enhanced::anomaly_detection::AnomalyDetector;
    use crate::ai_enhanced::resource_predictor::{ResourcePredictor, ResourceType, PredictionHorizon};
    use crate::networking_enhanced::traffic_shaper::{TrafficShaper, TrafficClass, QosPolicy};
    use crate::security_enhanced::runtime_integrity::{IntegrityMonitor, ResourceKind};
    use crate::security_enhanced::secure_enclave::{SecureEnclave, KeyType, KeyPermission};
    use crate::developer_tools::profiler::Profiler;

    /// Test: Anomaly detection feeds into resource prediction
    #[test]
    fn test_anomaly_detection_feeds_resource_predictor() {
        let mut detector = AnomalyDetector::zscore("cpu_load");
        let mut predictor = ResourcePredictor::new(5);
        predictor.track_resource(ResourceType::Cpu, 0.3, 0.1, 80.0);

        for i in 0..30 {
            let value = 45.0 + (i as f64 * 0.5);
            let anomaly = detector.observe(value);
            let _ = predictor.observe(ResourceType::Cpu, value);

            if anomaly.is_some() {
                let _ = predictor.predict(ResourceType::Cpu, PredictionHorizon::OneMinute);
            }
        }

        assert!(detector.total_observations() >= 30);
        assert!(predictor.observation_count(ResourceType::Cpu).unwrap() >= 30);
    }

    /// Test: Traffic shaper with profiler measuring overhead
    #[test]
    fn test_traffic_shaper_with_profiling() {
        let mut shaper = TrafficShaper::new(10_000_000_000);
        shaper.add_policy(QosPolicy::realtime(1_000_000_000, 5_000_000_000, 500));
        shaper.add_policy(QosPolicy::best_effort(2_000_000_000));

        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();

        for i in 0..100u64 {
            let start_ns = i * 1000;
            let span_id = profiler.begin_span("shape_packet", start_ns).unwrap();

            let class = if i % 3 == 0 { TrafficClass::RealTime } else { TrafficClass::BestEffort };
            let _ = shaper.shape_packet(class, 1500, i * 100_000);

            profiler.end_span(span_id, start_ns + 500).unwrap();
        }

        profiler.stop(100_000).unwrap();
        let report = profiler.report();
        assert!(!report.entries.is_empty());
        assert!(shaper.global_stats().total_packets() >= 100);
    }

    /// Test: Integrity monitor protecting enclave resources
    #[test]
    fn test_integrity_protects_enclave() {
        let mut monitor = IntegrityMonitor::new();
        let enclave_hash = [0x42_u8; 32];
        monitor.register_resource(
            "/secure/enclave_binary",
            ResourceKind::KernelModule,
            enclave_hash,
        ).unwrap();

        let check = monitor.verify_resource("/secure/enclave_binary", enclave_hash).unwrap();
        assert!(check, "Enclave binary integrity must pass before initialization");

        if check {
            let mut enclave = SecureEnclave::new(100);
            let secret = b"integrity_verified_secret_key!";
            enclave.initialize(secret, 1).unwrap();
            enclave.authenticate(secret).unwrap();

            let key_id = enclave.generate_key(
                "verified_key",
                KeyType::Symmetric256,
                vec![KeyPermission::Encrypt, KeyPermission::Decrypt],
            ).unwrap();

            let encrypted = enclave.encrypt(key_id, b"protected data");
            assert!(encrypted.is_ok());
        }
    }

    /// Test: Resource predictor with multiple resource types
    #[test]
    fn test_multi_resource_prediction() {
        let mut predictor = ResourcePredictor::new(5);
        predictor.track_resource(ResourceType::Cpu, 0.3, 0.1, 80.0);
        predictor.track_resource(ResourceType::Memory, 0.3, 0.1, 90.0);
        predictor.track_resource(ResourceType::DiskIo, 0.3, 0.1, 85.0);
        assert_eq!(predictor.tracked_count(), 3);

        for i in 0..20 {
            let _ = predictor.observe(ResourceType::Cpu, 30.0 + (i as f64 * 1.5));
            let _ = predictor.observe(ResourceType::Memory, 60.0 + (i as f64 * 0.5));
            let _ = predictor.observe(ResourceType::DiskIo, 70.0 + (i as f64 * 0.2));
        }

        assert!(predictor.predict(ResourceType::Cpu, PredictionHorizon::FiveMinutes).is_ok());
        assert!(predictor.predict(ResourceType::Memory, PredictionHorizon::OneMinute).is_ok());
        assert!(predictor.predict(ResourceType::DiskIo, PredictionHorizon::FifteenMinutes).is_ok());
    }

    /// Test: Full system scenario - detect anomaly, predict, and secure
    #[test]
    fn test_full_system_scenario() {
        // 1. Monitor system integrity
        let mut monitor = IntegrityMonitor::new();
        monitor.register_resource("/sys/kernel", ResourceKind::SystemBinary, [0xFF; 32]).unwrap();
        assert!(monitor.verify_resource("/sys/kernel", [0xFF; 32]).unwrap());

        // 2. Start profiling
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();

        // 3. Detect anomalies in network traffic
        let mut detector = AnomalyDetector::zscore("network_throughput");
        let span = profiler.begin_span("anomaly_detection", 100).unwrap();
        for i in 0..50 {
            let _ = detector.observe(100.0 + (i as f64 % 10.0));
        }
        profiler.end_span(span, 5000).unwrap();

        // 4. Shape traffic based on conditions
        let mut shaper = TrafficShaper::new(10_000_000_000);
        shaper.add_policy(QosPolicy::realtime(1_000_000_000, 5_000_000_000, 1000));

        let span2 = profiler.begin_span("traffic_shaping", 6000).unwrap();
        for i in 0..20u64 {
            let _ = shaper.shape_packet(TrafficClass::RealTime, 1500, i * 50_000);
        }
        profiler.end_span(span2, 10000).unwrap();

        // 5. Secure sensitive data
        let mut enclave = SecureEnclave::new(999);
        let secret = b"full_scenario_secret_key_12345";
        enclave.initialize(secret, 1).unwrap();
        enclave.authenticate(secret).unwrap();

        profiler.stop(20000).unwrap();

        // Verify all systems operated correctly
        assert!(detector.total_observations() >= 50);
        assert!(shaper.global_stats().total_packets() >= 20);
        assert_eq!(monitor.total_violations(), 0);
        assert!(profiler.span_count() >= 2);
    }
}