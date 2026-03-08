//! Integration tests for VantisOS AI Data Pipeline
//!
//! These tests verify the complete flow of the data pipeline:
//! - Data collection → Processing → Training
//! - Integration with scheduler, power manager, load balancer
//! - End-to-end functionality

#![allow(dead_code)]
#![allow(unused_variables)]

use vantisos_ai::{
    modules::{DataCollector, DataProcessor, ModelTrainer},
    integration::{
        AIIntegration, SchedulerIntegration, PowerManagerIntegration,
        LoadBalancerIntegration, ThreatDetectionIntegration,
    },
    types::{SystemMetrics, AIModel, TrainingRequest},
    error::AIError,
};

/// Test: Complete data pipeline flow from collection to training
#[test]
fn test_data_pipeline_complete_flow() {
    // Initialize data pipeline modules
    let collector = DataCollector::new().expect("Failed to create DataCollector");
    let processor = DataProcessor::new().expect("Failed to create DataProcessor");
    let trainer = ModelTrainer::new().expect("Failed to create ModelTrainer");

    // Simulate data collection
    let metrics = vec![
        SystemMetrics {
            cpu_usage: 50.0,
            memory_usage: 60.0,
            disk_usage: 70.0,
            network_throughput: 1000.0,
            power_consumption: 45.0,
            timestamp: 1000,
        },
        SystemMetrics {
            cpu_usage: 55.0,
            memory_usage: 62.0,
            disk_usage: 71.0,
            network_throughput: 1050.0,
            power_consumption: 47.0,
            timestamp: 1001,
        },
    ];

    // Add metrics to collector
    for metric in metrics {
        collector.add_metric(metric).expect("Failed to add metric");
    }

    // Collect metrics
    let collected_data = collector.collect_all_metrics()
        .expect("Failed to collect metrics");
    assert!(!collected_data.is_empty(), "Should have collected metrics");

    // Process metrics (extract features)
    let processed_data = processor.extract_features(&collected_data)
        .expect("Failed to extract features");
    assert!(!processed_data.is_empty(), "Should have processed features");

    // Prepare training data
    let training_data = processor.prepare_training_data(&processed_data)
        .expect("Failed to prepare training data");
    assert!(!training_data.is_empty(), "Should have training data");

    // Train model
    let request = TrainingRequest {
        model_type: vantisos_ai::types::ModelType::Regression,
        algorithm: vantisos_ai::types::TrainingAlgorithm::SGD,
        epochs: 10,
        batch_size: 32,
        learning_rate: 0.01,
        validation_split: 0.2,
    };

    let result = trainer.train_model(&training_data, request.clone())
        .expect("Failed to train model");
    assert!(result.is_trained, "Model should be trained");
    assert!(result.accuracy > 0.0, "Model should have accuracy");
}

/// Test: Scheduler integration with data pipeline
#[test]
fn test_scheduler_integration() {
    let collector = DataCollector::new().expect("Failed to create DataCollector");
    let processor = DataProcessor::new().expect("Failed to create DataProcessor");
    let trainer = ModelTrainer::new().expect("Failed to create ModelTrainer");

    let mut scheduler_integration = SchedulerIntegration::new(collector, processor, trainer)
        .expect("Failed to create SchedulerIntegration");

    // Simulate adding scheduler metrics
    let metrics = SystemMetrics {
        cpu_usage: 45.0,
        memory_usage: 50.0,
        disk_usage: 55.0,
        network_throughput: 800.0,
        power_consumption: 40.0,
        timestamp: 2000,
    };

    scheduler_integration.add_scheduler_metrics(metrics.clone())
        .expect("Failed to add scheduler metrics");

    // Get optimization suggestions
    let suggestions = scheduler_integration.get_optimization_suggestions()
        .expect("Failed to get optimization suggestions");
    assert!(!suggestions.is_empty(), "Should have optimization suggestions");
}

/// Test: Power manager integration with data pipeline
#[test]
fn test_power_manager_integration() {
    let collector = DataCollector::new().expect("Failed to create DataCollector");
    let processor = DataProcessor::new().expect("Failed to create DataProcessor");
    let trainer = ModelTrainer::new().expect("Failed to create ModelTrainer");

    let mut power_integration = PowerManagerIntegration::new(collector, processor, trainer)
        .expect("Failed to create PowerManagerIntegration");

    // Simulate adding power metrics
    let metrics = SystemMetrics {
        cpu_usage: 30.0,
        memory_usage: 40.0,
        disk_usage: 35.0,
        network_throughput: 500.0,
        power_consumption: 25.0,
        timestamp: 3000,
    };

    power_integration.add_power_metrics(metrics.clone())
        .expect("Failed to add power metrics");

    // Get power optimization recommendations
    let recommendations = power_integration.get_power_optimizations()
        .expect("Failed to get power optimizations");
    assert!(!recommendations.is_empty(), "Should have power optimization recommendations");
}

/// Test: Load balancer integration with data pipeline
#[test]
fn test_load_balancer_integration() {
    let collector = DataCollector::new().expect("Failed to create DataCollector");
    let processor = DataProcessor::new().expect("Failed to create DataProcessor");
    let trainer = ModelTrainer::new().expect("Failed to create ModelTrainer");

    let mut lb_integration = LoadBalancerIntegration::new(collector, processor, trainer)
        .expect("Failed to create LoadBalancerIntegration");

    // Simulate adding load balancer metrics
    let metrics = SystemMetrics {
        cpu_usage: 60.0,
        memory_usage: 65.0,
        disk_usage: 50.0,
        network_throughput: 1500.0,
        power_consumption: 55.0,
        timestamp: 4000,
    };

    lb_integration.add_load_balancer_metrics(metrics.clone())
        .expect("Failed to add load balancer metrics");

    // Get load balancing decisions
    let decisions = lb_integration.get_load_balancing_decisions()
        .expect("Failed to get load balancing decisions");
    assert!(!decisions.is_empty(), "Should have load balancing decisions");
}

/// Test: Threat detection integration with data pipeline
#[test]
fn test_threat_detection_integration() {
    let collector = DataCollector::new().expect("Failed to create DataCollector");
    let processor = DataProcessor::new().expect("Failed to create DataProcessor");
    let trainer = ModelTrainer::new().expect("Failed to create ModelTrainer");

    let mut threat_integration = ThreatDetectionIntegration::new(collector, processor, trainer)
        .expect("Failed to create ThreatDetectionIntegration");

    // Simulate adding security metrics
    let metrics = SystemMetrics {
        cpu_usage: 80.0,
        memory_usage: 75.0,
        disk_usage: 60.0,
        network_throughput: 2000.0,
        power_consumption: 70.0,
        timestamp: 5000,
    };

    threat_integration.add_security_metrics(metrics.clone())
        .expect("Failed to add security metrics");

    // Get threat assessment
    let threats = threat_integration.assess_threats()
        .expect("Failed to assess threats");
    assert!(!threats.is_empty(), "Should have threat assessment results");
}

/// Test: AI Integration coordinator
#[test]
fn test_ai_integration_coordinator() {
    let collector = DataCollector::new().expect("Failed to create DataCollector");
    let processor = DataProcessor::new().expect("Failed to create DataProcessor");
    let trainer = ModelTrainer::new().expect("Failed to create ModelTrainer");

    let mut ai_integration = AIIntegration::new(collector, processor, trainer)
        .expect("Failed to create AIIntegration");

    // Initialize AI integration
    ai_integration.initialize().expect("Failed to initialize AI integration");

    // Add system metrics
    let metrics = SystemMetrics {
        cpu_usage: 50.0,
        memory_usage: 55.0,
        disk_usage: 60.0,
        network_throughput: 1200.0,
        power_consumption: 50.0,
        timestamp: 6000,
    };

    ai_integration.add_metrics(metrics.clone())
        .expect("Failed to add metrics");

    // Process data
    ai_integration.process_data().expect("Failed to process data");

    // Get AI insights
    let insights = ai_integration.get_insights().expect("Failed to get insights");
    assert!(!insights.is_empty(), "Should have AI insights");

    // Shutdown
    ai_integration.shutdown().expect("Failed to shutdown AI integration");
}

/// Test: Data collector error handling
#[test]
fn test_data_collector_error_handling() {
    let collector = DataCollector::new().expect("Failed to create DataCollector");

    // Test with invalid metrics
    let invalid_metric = SystemMetrics {
        cpu_usage: -10.0, // Invalid negative value
        memory_usage: 60.0,
        disk_usage: 70.0,
        network_throughput: 1000.0,
        power_consumption: 45.0,
        timestamp: 7000,
    };

    let result = collector.add_metric(invalid_metric);
    assert!(result.is_err(), "Should reject invalid metrics");
}

/// Test: Data processor feature extraction
#[test]
fn test_data_processor_feature_extraction() {
    let processor = DataProcessor::new().expect("Failed to create DataProcessor");

    let metrics = vec![
        SystemMetrics {
            cpu_usage: 50.0,
            memory_usage: 60.0,
            disk_usage: 70.0,
            network_throughput: 1000.0,
            power_consumption: 45.0,
            timestamp: 8000,
        },
        SystemMetrics {
            cpu_usage: 55.0,
            memory_usage: 62.0,
            disk_usage: 71.0,
            network_throughput: 1050.0,
            power_consumption: 47.0,
            timestamp: 8001,
        },
    ];

    let features = processor.extract_features(&metrics)
        .expect("Failed to extract features");
    
    assert!(!features.is_empty(), "Should extract features");
}

/// Test: Model trainer with different algorithms
#[test]
fn test_model_trainer_algorithms() {
    let trainer = ModelTrainer::new().expect("Failed to create ModelTrainer");

    let training_data = vec![
        (vec![1.0, 2.0, 3.0], vec![1.0]),
        (vec![2.0, 3.0, 4.0], vec![2.0]),
        (vec![3.0, 4.0, 5.0], vec![3.0]),
    ];

    let algorithms = vec![
        vantisos_ai::types::TrainingAlgorithm::SGD,
        vantisos_ai::types::TrainingAlgorithm::Adam,
        vantisos_ai::types::TrainingAlgorithm::RMSprop,
        vantisos_ai::types::TrainingAlgorithm::Adagrad,
        vantisos_ai::types::TrainingAlgorithm::LBFGS,
    ];

    for algorithm in algorithms {
        let request = TrainingRequest {
            model_type: vantisos_ai::types::ModelType::Regression,
            algorithm: algorithm.clone(),
            epochs: 5,
            batch_size: 1,
            learning_rate: 0.01,
            validation_split: 0.2,
        };

        let result = trainer.train_model(&training_data, request.clone());
        assert!(result.is_ok(), "Should train with {:?}", algorithm);
    }
}

/// Test: Data pipeline with edge cases
#[test]
fn test_data_pipeline_edge_cases() {
    let collector = DataCollector::new().expect("Failed to create DataCollector");
    let processor = DataProcessor::new().expect("Failed to create DataProcessor");

    // Test with empty data
    let empty_metrics: Vec<SystemMetrics> = vec![];
    let features = processor.extract_features(&empty_metrics);
    assert!(features.is_err(), "Should fail with empty data");

    // Test with single data point
    let single_metric = vec![
        SystemMetrics {
            cpu_usage: 50.0,
            memory_usage: 60.0,
            disk_usage: 70.0,
            network_throughput: 1000.0,
            power_consumption: 45.0,
            timestamp: 9000,
        },
    ];

    let features = processor.extract_features(&single_metric);
    assert!(features.is_ok(), "Should handle single data point");
}

/// Test: Integration performance
#[test]
fn test_integration_performance() {
    let collector = DataCollector::new().expect("Failed to create DataCollector");
    let processor = DataProcessor::new().expect("Failed to create DataProcessor");
    let trainer = ModelTrainer::new().expect("Failed to create ModelTrainer");

    let mut ai_integration = AIIntegration::new(collector, processor, trainer)
        .expect("Failed to create AIIntegration");

    ai_integration.initialize().expect("Failed to initialize");

    // Add multiple metrics rapidly
    for i in 0..100 {
        let metrics = SystemMetrics {
            cpu_usage: 50.0 + (i as f64 * 0.1),
            memory_usage: 55.0 + (i as f64 * 0.1),
            disk_usage: 60.0 + (i as f64 * 0.05),
            network_throughput: 1000.0 + (i as f64 * 10.0),
            power_consumption: 50.0 + (i as f64 * 0.05),
            timestamp: 10000 + i as u64,
        };

        ai_integration.add_metrics(metrics).expect("Failed to add metrics");
    }

    // Process should handle bulk data efficiently
    ai_integration.process_data().expect("Failed to process bulk data");
}

/// Test: Integration state persistence
#[test]
fn test_integration_state_persistence() {
    let collector = DataCollector::new().expect("Failed to create DataCollector");
    let processor = DataProcessor::new().expect("Failed to create DataProcessor");
    let trainer = ModelTrainer::new().expect("Failed to create ModelTrainer");

    let mut ai_integration = AIIntegration::new(collector, processor, trainer)
        .expect("Failed to create AIIntegration");

    ai_integration.initialize().expect("Failed to initialize");

    // Add metrics
    let metrics = SystemMetrics {
        cpu_usage: 50.0,
        memory_usage: 55.0,
        disk_usage: 60.0,
        network_throughput: 1200.0,
        power_consumption: 50.0,
        timestamp: 11000,
    };

    ai_integration.add_metrics(metrics).expect("Failed to add metrics");

    // Get statistics before processing
    let stats_before = ai_integration.get_statistics().expect("Failed to get stats");

    // Process data
    ai_integration.process_data().expect("Failed to process data");

    // Get statistics after processing - should reflect changes
    let stats_after = ai_integration.get_statistics().expect("Failed to get stats");

    assert!(stats_after.total_samples > stats_before.total_samples,
            "Statistics should update after processing");
}