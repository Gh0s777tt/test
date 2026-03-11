//! VantisOS v1.6.0 Benchmarks
//!
//! Performance benchmarks for the v1.6.0 Enhanced Features modules.
//! These benchmarks measure throughput, latency, and resource usage
//! of the AI, networking, security, and developer tools subsystems.

#[cfg(test)]
mod benchmarks {
    use crate::ai_enhanced::anomaly_detection::AnomalyDetector;
    use crate::ai_enhanced::inference_engine::{
        InferenceEngine, Tensor, TensorShape, ModelFormat, AcceleratorBackend, InferencePrecision,
    };
    use crate::ai_enhanced::federated_learning::{FederatedCoordinator, AggregationStrategy};
    use crate::ai_enhanced::model_optimizer::{ModelOptimizer, WeightTensor};
    use crate::ai_enhanced::resource_predictor::{ResourcePredictor, ResourceType, PredictionHorizon};
    use crate::networking_enhanced::sdn_controller::{SdnController, NetworkNode, NodeType, Link, FlowMatch, FlowAction};
    use crate::networking_enhanced::traffic_shaper::{TrafficShaper, TrafficClass, QosPolicy};
    use crate::networking_enhanced::zero_trust_network::{ZtnaController, Identity, AccessPolicy};
    use crate::security_enhanced::runtime_integrity::{IntegrityMonitor, ResourceKind};
    use crate::security_enhanced::secure_enclave::{SecureEnclave, KeyType, KeyPermission};
    use crate::developer_tools::profiler::Profiler;
    use crate::developer_tools::build_system::{BuildSystem, BuildTarget, TargetType, Architecture};

    // ========================================================================
    // Helper: Simple timing using core::hint::black_box to prevent optimization
    // ========================================================================

    fn black_box<T>(val: T) -> T {
        // Prevent compiler from optimizing away benchmark operations
        unsafe {
            let ret = core::ptr::read_volatile(&val);
            core::mem::forget(val);
            ret
        }
    }

    // ========================================================================
    // AI Inference Engine Benchmarks
    // ========================================================================

    #[test]
    fn bench_inference_engine_model_load_throughput() {
        let mut engine = InferenceEngine::new(256 * 1024 * 1024); // 256MB
        engine.register_backend(AcceleratorBackend::Cpu);

        let num_models = 10;
        let mut session_ids = Vec::new();

        for i in 0..num_models {
            let name = format!("model_{}", i);
            let input = TensorShape::new(vec![1, 128]);
            let output = TensorShape::new(vec![1, 64]);
            let result = engine.load_model(
                &name,
                ModelFormat::Onnx,
                AcceleratorBackend::Cpu,
                InferencePrecision::Float32,
                input,
                output,
            );
            assert!(result.is_ok(), "Failed to load model {}", i);
            session_ids.push(result.unwrap());
        }

        assert_eq!(engine.active_sessions(), num_models);

        // Benchmark inference throughput
        let iterations = 1000;
        let input_tensor = Tensor::filled(TensorShape::new(vec![1, 128]), 0.5);

        for _ in 0..iterations {
            for &sid in &session_ids {
                let result = engine.infer(sid, &input_tensor);
                assert!(result.is_ok());
                black_box(result.unwrap());
            }
        }

        let total_inferences = engine.total_inferences();
        assert!(total_inferences >= (iterations * num_models) as u64);

        // Check memory utilization
        let mem_util = engine.memory_utilization();
        assert!(mem_util > 0.0 && mem_util <= 1.0,
            "Memory utilization should be between 0 and 1, got {}", mem_util);
    }

    #[test]
    fn bench_inference_tensor_operations() {
        let shape = TensorShape::new(vec![64, 64]);
        let iterations = 500;

        // Benchmark tensor creation
        for _ in 0..iterations {
            let t = Tensor::filled(shape.clone(), 1.0);
            black_box(t);
        }

        // Benchmark tensor addition
        let t1 = Tensor::filled(shape.clone(), 1.0);
        let t2 = Tensor::filled(shape.clone(), 2.0);
        for _ in 0..iterations {
            let result = t1.add(&t2).unwrap();
            black_box(result);
        }

        // Benchmark softmax
        let t3 = Tensor::filled(TensorShape::new(vec![1, 1000]), 0.01);
        for _ in 0..iterations {
            let result = t3.softmax();
            black_box(result);
        }

        // Benchmark ReLU
        let t4 = Tensor::filled(TensorShape::new(vec![32, 32]), -0.5);
        for _ in 0..iterations {
            let result = t4.relu();
            black_box(result);
        }
    }

    #[test]
    fn bench_inference_session_latency() {
        let mut engine = InferenceEngine::new(64 * 1024 * 1024);
        engine.register_backend(AcceleratorBackend::Cpu);

        // Load models of different sizes
        let sizes = vec![
            ("tiny", vec![1, 16], vec![1, 8]),
            ("small", vec![1, 128], vec![1, 64]),
            ("medium", vec![1, 512], vec![1, 256]),
            ("large", vec![1, 1024], vec![1, 512]),
        ];

        for (name, input_dims, output_dims) in &sizes {
            let sid = engine.load_model(
                name,
                ModelFormat::Onnx,
                AcceleratorBackend::Cpu,
                InferencePrecision::Float32,
                TensorShape::new(input_dims.clone()),
                TensorShape::new(output_dims.clone()),
            ).unwrap();

            let input = Tensor::filled(TensorShape::new(input_dims.clone()), 0.1);
            for _ in 0..100 {
                let _ = engine.infer(sid, &input).unwrap();
            }

            let session = engine.get_session(sid).unwrap();
            let avg_latency = session.avg_latency_us();
            assert!(avg_latency >= 0.0, "Latency should be non-negative for {}", name);
        }
    }

    // ========================================================================
    // Traffic Shaper Throughput Benchmarks
    // ========================================================================

    #[test]
    fn bench_traffic_shaper_throughput() {
        let mut shaper = TrafficShaper::new(100_000_000_000); // 100Gbps
        shaper.add_policy(QosPolicy::realtime(10_000_000_000, 50_000_000_000, 100));
        shaper.add_policy(QosPolicy::best_effort(20_000_000_000));

        let packet_count = 10_000u64;
        let packet_size = 1500u64; // standard MTU

        // Benchmark: process many packets
        for i in 0..packet_count {
            let class = match i % 4 {
                0 => TrafficClass::RealTime,
                1 => TrafficClass::BestEffort,
                2 => TrafficClass::RealTime,
                _ => TrafficClass::BestEffort,
            };
            let timestamp = i * 100; // 100us between packets
            let decision = shaper.shape_packet(class, packet_size, timestamp);
            black_box(decision);
        }

        let stats = shaper.global_stats();
        assert_eq!(stats.total_packets(), packet_count);

        // Check per-class stats
        if let Some(rt_stats) = shaper.class_stats(TrafficClass::RealTime) {
            assert!(rt_stats.total_packets() > 0);
        }
        if let Some(be_stats) = shaper.class_stats(TrafficClass::BestEffort) {
            assert!(be_stats.total_packets() > 0);
        }
    }

    #[test]
    fn bench_traffic_shaper_burst_handling() {
        let mut shaper = TrafficShaper::new(1_000_000_000); // 1Gbps
        shaper.add_policy(QosPolicy::realtime(100_000_000, 500_000_000, 500));

        // Simulate burst: many packets at same timestamp
        let burst_size = 100u64;
        let mut allowed = 0u64;
        let mut delayed = 0u64;
        let mut dropped = 0u64;

        for i in 0..burst_size {
            let decision = shaper.shape_packet(TrafficClass::RealTime, 1500, i);
            match decision {
                crate::networking_enhanced::traffic_shaper::ShapingDecision::Allow => allowed += 1,
                crate::networking_enhanced::traffic_shaper::ShapingDecision::Delay { .. } => delayed += 1,
                crate::networking_enhanced::traffic_shaper::ShapingDecision::Drop => dropped += 1,
                crate::networking_enhanced::traffic_shaper::ShapingDecision::Mark => allowed += 1,
            }
        }

        // At least some packets should be processed
        assert!(allowed + delayed + dropped == burst_size);
    }

    #[test]
    fn bench_traffic_shaper_multi_class() {
        let mut shaper = TrafficShaper::new(10_000_000_000);
        shaper.add_policy(QosPolicy::realtime(1_000_000_000, 5_000_000_000, 200));
        shaper.add_policy(QosPolicy::best_effort(3_000_000_000));

        let classes = [TrafficClass::RealTime, TrafficClass::BestEffort];
        let iterations = 5000u64;

        for i in 0..iterations {
            let class = classes[(i % 2) as usize];
            let _ = shaper.shape_packet(class, 1500, i * 200);
        }

        assert_eq!(shaper.global_stats().total_packets(), iterations);
    }

    // ========================================================================
    // Anomaly Detection Benchmarks
    // ========================================================================

    #[test]
    fn bench_anomaly_detection_throughput() {
        let mut detector = AnomalyDetector::zscore("high_freq_metric");

        let observations = 10_000;
        let mut anomalies_found = 0u64;

        for i in 0..observations {
            // Normal data with occasional spikes
            let value = if i % 500 == 0 {
                999.0 // spike
            } else {
                50.0 + (i as f64 % 20.0) * 0.5
            };

            if let Some(_event) = detector.observe(value) {
                anomalies_found += 1;
            }
        }

        assert_eq!(detector.total_observations(), observations as u64);
        assert!(anomalies_found > 0, "Should detect at least some anomalies");
        assert!(detector.anomaly_rate() < 0.1, "Anomaly rate should be low for mostly normal data");
    }

    #[test]
    fn bench_anomaly_detection_ewma_throughput() {
        let mut detector = AnomalyDetector::ewma("ewma_metric", 0.2);

        for i in 0..5000 {
            let value = 100.0 + (i as f64 * 0.01).sin() * 10.0;
            let _ = detector.observe(value);
        }

        assert_eq!(detector.total_observations(), 5000);
    }

    // ========================================================================
    // SDN Controller Benchmarks
    // ========================================================================

    #[test]
    fn bench_sdn_topology_scaling() {
        let mut sdn = SdnController::new();

        // Build a mesh topology
        let node_count = 50;
        for i in 0..node_count {
            let node_type = if i < 10 { NodeType::Router } else { NodeType::Switch };
            sdn.add_node(NetworkNode::new(i, &format!("node_{}", i), node_type, 8)).unwrap();
        }

        // Connect nodes in a chain + some cross-links
        for i in 0..(node_count - 1) {
            sdn.add_link(Link::new(i, 0, i + 1, 0, 10_000, 1)).unwrap();
        }
        // Add some cross-links for shorter paths
        for i in (0..node_count - 5).step_by(5) {
            let _ = sdn.add_link(Link::new(i, 1, i + 5, 1, 10_000, 2));
        }

        assert_eq!(sdn.node_count(), node_count as usize);

        // Benchmark shortest path computation
        for src in 0..10u64 {
            for dst in 40..50u64 {
                let path = sdn.shortest_path(src, dst);
                assert!(path.is_ok(), "Path from {} to {} should exist", src, dst);
                black_box(path.unwrap());
            }
        }
    }

    #[test]
    fn bench_sdn_flow_installation_throughput() {
        let mut sdn = SdnController::new();
        sdn.add_node(NetworkNode::new(1, "switch", NodeType::Switch, 48)).unwrap();

        let flow_count = 500;
        for i in 0..flow_count {
            let flow_match = FlowMatch::new();
            let result = sdn.install_flow(1, i as u32, flow_match, FlowAction::Forward(2));
            assert!(result.is_ok(), "Failed to install flow {}", i);
        }

        assert_eq!(sdn.flow_count(), flow_count);
    }

    // ========================================================================
    // Zero Trust Network Benchmarks
    // ========================================================================

    #[test]
    fn bench_ztna_evaluation_throughput() {
        let mut ztna = ZtnaController::new();

        // Register identities
        for i in 0..100u64 {
            let mut identity = Identity::new(i, &format!("user_{}@corp.com", i));
            identity.add_role("employee");
            if i % 10 == 0 {
                identity.add_role("admin");
            }
            ztna.register_identity(identity).unwrap();
        }

        // Add policies
        for i in 0..20u64 {
            let policy = AccessPolicy::new(i, &format!("policy_{}", i), &format!("/api/v{}/", i));
            ztna.add_policy(policy).unwrap();
        }

        // Benchmark evaluation
        let evaluations = 1000;
        for i in 0..evaluations {
            let identity_id = (i % 100) as u64;
            let resource = format!("/api/v{}/data", i % 20);
            let decision = ztna.evaluate(identity_id, &resource);
            black_box(decision);
        }

        assert!(ztna.audit_log().len() >= evaluations);
    }

    // ========================================================================
    // Secure Enclave Benchmarks
    // ========================================================================

    #[test]
    fn bench_secure_enclave_crypto_operations() {
        let mut enclave = SecureEnclave::new(1);
        let secret = b"benchmark_secret_key_1234567890";
        enclave.initialize(secret, 1).unwrap();
        enclave.authenticate(secret).unwrap();

        // Generate encryption key
        let key_id = enclave.generate_key(
            "bench_key",
            KeyType::Symmetric256,
            vec![KeyPermission::Encrypt, KeyPermission::Decrypt],
        ).unwrap();

        // Benchmark encrypt/decrypt cycles
        let iterations = 500;
        let plaintext = b"Benchmark data for encryption performance testing in VantisOS";

        for _ in 0..iterations {
            let encrypted = enclave.encrypt(key_id, plaintext).unwrap();
            let decrypted = enclave.decrypt(key_id, &encrypted.ciphertext).unwrap();
            black_box(decrypted);
        }

        assert!(enclave.total_operations() >= (iterations * 2) as u64 + 1); // +1 for key gen
    }

    #[test]
    fn bench_secure_enclave_key_generation() {
        let mut enclave = SecureEnclave::new(2);
        let secret = b"keygen_benchmark_secret_1234567";
        enclave.initialize(secret, 1).unwrap();
        enclave.authenticate(secret).unwrap();

        let key_types = [
            KeyType::Symmetric256,
            KeyType::SigningKeyPair,
            KeyType::DerivationMaster,
            KeyType::SealingKey,
        ];

        for (i, kt) in key_types.iter().enumerate() {
            let label = format!("bench_key_{}", i);
            let result = enclave.generate_key(
                &label,
                *kt,
                vec![KeyPermission::Encrypt, KeyPermission::Sign],
            );
            assert!(result.is_ok(), "Failed to generate key type {:?}", kt);
        }

        assert_eq!(enclave.key_count(), key_types.len());
    }

    // ========================================================================
    // Integrity Monitor Benchmarks
    // ========================================================================

    #[test]
    fn bench_integrity_monitor_scaling() {
        let mut monitor = IntegrityMonitor::new();

        // Register many resources
        let resource_count = 200;
        for i in 0..resource_count {
            let path = format!("/sys/resource_{}", i);
            let kind = match i % 5 {
                0 => ResourceKind::KernelModule,
                1 => ResourceKind::SystemBinary,
                2 => ResourceKind::SharedLibrary,
                3 => ResourceKind::ConfigFile,
                _ => ResourceKind::SecurityPolicy,
            };
            let mut hash = [0u8; 32];
            hash[0] = (i & 0xFF) as u8;
            hash[1] = ((i >> 8) & 0xFF) as u8;
            monitor.register_resource(&path, kind, hash).unwrap();
        }

        assert_eq!(monitor.resource_count(), resource_count);

        // Benchmark individual verifications
        for i in 0..resource_count {
            let path = format!("/sys/resource_{}", i);
            let mut hash = [0u8; 32];
            hash[0] = (i & 0xFF) as u8;
            hash[1] = ((i >> 8) & 0xFF) as u8;
            let result = monitor.verify_resource(&path, hash).unwrap();
            assert!(result, "Resource {} should verify", i);
        }

        assert_eq!(monitor.total_checks(), resource_count as u64);
        assert_eq!(monitor.total_violations(), 0);
    }

    #[test]
    fn bench_integrity_full_check() {
        let mut monitor = IntegrityMonitor::new();

        let count = 100;
        for i in 0..count {
            let path = format!("/check/item_{}", i);
            let mut hash = [0u8; 32];
            hash[0] = i as u8;
            monitor.register_resource(&path, ResourceKind::SystemBinary, hash).unwrap();
        }

        // Full check benchmark
        let report = monitor.full_check(|path: &str| -> Option<[u8; 32]> {
            // Extract index from path
            let idx: u8 = path.rsplit('_').next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            let mut hash = [0u8; 32];
            hash[0] = idx;
            Some(hash)
        }).unwrap();

        assert!(report.is_clean());
        assert!((report.success_rate() - 100.0).abs() < 0.01);
    }

    // ========================================================================
    // Model Optimizer Benchmarks
    // ========================================================================

    #[test]
    fn bench_model_optimizer_large_model() {
        let mut optimizer = ModelOptimizer::with_defaults();

        // Simulate a model with many layers
        for i in 0..20 {
            let size = (i + 1) * 100;
            let data: Vec<f32> = (0..size).map(|j| (j as f32) * 0.01 - 0.5).collect();
            let shape = vec![size / 10, 10];
            let tensor = WeightTensor::new(&format!("layer_{}", i), data, shape);
            optimizer.add_weights(tensor);
        }

        assert_eq!(optimizer.num_layers(), 20);
        let original_size = optimizer.original_size_bytes();
        assert!(original_size > 0);

        let result = optimizer.optimize().unwrap();
        assert!(result.size_reduction_pct() >= 0.0);
    }

    // ========================================================================
    // Federated Learning Benchmarks
    // ========================================================================

    #[test]
    fn bench_federated_learning_rounds() {
        let model_dim = 100;
        let num_participants = 10;
        let num_rounds = 5;

        let mut coord = FederatedCoordinator::new(
            AggregationStrategy::FedAvg,
            model_dim,
            num_participants / 2, // min participants
        );

        // Register participants
        let mut participant_ids = Vec::new();
        for i in 0..num_participants {
            let id = coord.register_participant(&format!("node_{}", i), 1000 + i * 100).unwrap();
            participant_ids.push(id);
        }

        // Run multiple training rounds
        for round in 0..num_rounds {
            coord.start_round().unwrap();

            for &pid in &participant_ids {
                let update: Vec<f32> = (0..model_dim)
                    .map(|j| ((round * model_dim + j) as f32) * 0.001)
                    .collect();
                coord.submit_update(pid, update).unwrap();
            }

            let aggregated = coord.aggregate().unwrap();
            assert_eq!(aggregated.len(), model_dim);
            black_box(aggregated);
        }

        assert_eq!(coord.completed_rounds(), num_rounds as u64);
    }

    // ========================================================================
    // Resource Predictor Benchmarks
    // ========================================================================

    #[test]
    fn bench_resource_predictor_continuous() {
        let mut predictor = ResourcePredictor::new(10);
        predictor.track_resource(ResourceType::Cpu, 0.3, 0.1, 80.0);
        predictor.track_resource(ResourceType::Memory, 0.2, 0.05, 90.0);

        // Feed continuous observations
        let observation_count = 1000;
        for i in 0..observation_count {
            let cpu = 40.0 + (i as f64 * 0.03).sin() * 20.0;
            let mem = 55.0 + (i as f64 * 0.01) + (i as f64 * 0.05).cos() * 5.0;
            let _ = predictor.observe(ResourceType::Cpu, cpu);
            let _ = predictor.observe(ResourceType::Memory, mem);
        }

        // Benchmark predictions
        for _ in 0..100 {
            let cpu_pred = predictor.predict(ResourceType::Cpu, PredictionHorizon::FiveMinutes);
            let mem_pred = predictor.predict(ResourceType::Memory, PredictionHorizon::FifteenMinutes);
            assert!(cpu_pred.is_ok());
            assert!(mem_pred.is_ok());
            black_box(cpu_pred);
            black_box(mem_pred);
        }

        // Benchmark recommendations
        let cpu_rec = predictor.recommend(ResourceType::Cpu);
        let mem_rec = predictor.recommend(ResourceType::Memory);
        assert!(cpu_rec.is_ok());
        assert!(mem_rec.is_ok());
    }

    // ========================================================================
    // Profiler Self-Benchmarks
    // ========================================================================

    #[test]
    fn bench_profiler_span_overhead() {
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();

        let span_count = 2000;
        for i in 0..span_count {
            let ts = i as u64 * 100;
            let sid = profiler.begin_span("bench_span", ts).unwrap();
            profiler.end_span(sid, ts + 50).unwrap();
        }

        profiler.stop(span_count as u64 * 100 + 1000).unwrap();

        assert_eq!(profiler.span_count(), span_count);
        let report = profiler.report();
        assert!(!report.entries.is_empty());

        let table = report.to_table();
        assert!(table.contains("bench_span"));
    }

    #[test]
    fn bench_profiler_record_span_throughput() {
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();

        let iterations = 5000;
        for _ in 0..iterations {
            profiler.record_span("hot_path", 100).unwrap();
        }
        for _ in 0..iterations {
            profiler.record_span("cold_path", 5000).unwrap();
        }

        let report = profiler.report();
        let top = report.top_n(5);
        assert!(top.len() >= 2);
    }

    // ========================================================================
    // Build System Benchmarks
    // ========================================================================

    #[test]
    fn bench_build_system_dependency_resolution() {
        let mut build = BuildSystem::with_defaults();

        // Create a dependency chain: lib_0 <- lib_1 <- ... <- lib_19 <- kernel
        for i in 0..20 {
            let mut target = BuildTarget::new(
                &format!("lib_{}", i),
                TargetType::StaticLib,
                Architecture::X86_64,
            );
            target.add_source(&format!("src/lib_{}.rs", i));
            if i > 0 {
                target.add_dependency(&format!("lib_{}", i - 1));
            }
            build.add_target(target).unwrap();
        }

        let mut kernel = BuildTarget::new("kernel", TargetType::Kernel, Architecture::X86_64);
        kernel.add_source("src/kernel.rs");
        kernel.add_dependency("lib_19");
        build.add_target(kernel).unwrap();

        // Benchmark dependency validation and ordering
        for _ in 0..100 {
            let valid = build.validate_dependencies();
            assert!(valid.is_ok());
            let order = build.compute_build_order().unwrap();
            assert_eq!(order.len(), 21);
            // Verify ordering
            assert_eq!(order[0], "lib_0");
            assert_eq!(order[20], "kernel");
            black_box(order);
        }
    }

    #[test]
    fn bench_build_system_full_build() {
        let mut build = BuildSystem::with_defaults();

        for i in 0..10 {
            let mut target = BuildTarget::new(
                &format!("module_{}", i),
                TargetType::StaticLib,
                Architecture::X86_64,
            );
            for j in 0..5 {
                target.add_source(&format!("src/mod_{}/file_{}.rs", i, j));
            }
            if i > 0 {
                target.add_dependency(&format!("module_{}", i - 1));
            }
            build.add_target(target).unwrap();
        }

        let summary = build.build().unwrap();
        assert!(summary.is_success());
        assert_eq!(summary.success_rate(), 100.0);
    }
}