//! Integration benchmarks for the complete AI data pipeline
//!
//! These benchmarks measure end-to-end performance of the complete data pipeline
//! from collection through processing to training.

use crate::ai::modules::{DataCollector, DataProcessor, ModelTrainer};
use crate::ai::integration::AIIntegration;
use crate::ai::benchmarks::{BenchmarkSuite, PerformanceTarget};
use crate::ai::types::{SystemMetrics, NormalizationMethod, TrainingRequest, TrainingAlgorithm, ModelType};

/// Generate sample metrics for integration benchmarks
pub fn generate_integration_metrics(count: usize) -> Vec<SystemMetrics> {
    (0..count)
        .map(|i| SystemMetrics {
            cpu_usage: 50.0 + (i as f64 * 0.1 % 50.0),
            memory_usage: 60.0 + (i as f64 * 0.1 % 40.0),
            disk_usage: 70.0 + (i as f64 * 0.05 % 30.0),
            network_throughput: 1000.0 + (i as f64 * 10.0 % 2000.0),
            power_consumption: 45.0 + (i as f64 * 0.05 % 20.0),
            timestamp: 1000 + i as u64,
        })
        .collect()
}

/// Run all integration benchmarks
pub fn run_integration_benchmarks() {
    println!("\n=== Integration Benchmarks ===\n");

    let mut suite = BenchmarkSuite::with_defaults();

    // Benchmark 1: End-to-end pipeline (10 metrics)
    suite.run_benchmark("integration_e2e_10", || {
        let collector = DataCollector::new().unwrap();
        let processor = DataProcessor::new().unwrap();
        let trainer = ModelTrainer::new().unwrap();

        let metrics = generate_integration_metrics(10);
        
        // Collect
        for metric in &metrics {
            let _ = collector.add_metric(*metric);
        }
        
        // Process
        let features = processor.extract_features(&metrics).unwrap();
        let normalized = processor.normalize(&features, NormalizationMethod::ZScore).unwrap();
        
        // Train
        let training_data = processor.prepare_training_data(&normalized).unwrap();
        let request = TrainingRequest {
            model_type: ModelType::Regression,
            algorithm: TrainingAlgorithm::SGD,
            epochs: 5,
            batch_size: 32,
            learning_rate: 0.01,
            validation_split: 0.2,
        };
        let _ = trainer.train_model(&training_data, request);
    });

    // Benchmark 2: End-to-end pipeline (100 metrics)
    suite.run_benchmark("integration_e2e_100", || {
        let collector = DataCollector::new().unwrap();
        let processor = DataProcessor::new().unwrap();
        let trainer = ModelTrainer::new().unwrap();

        let metrics = generate_integration_metrics(100);
        
        // Collect
        for metric in &metrics {
            let _ = collector.add_metric(*metric);
        }
        
        // Process
        let features = processor.extract_features(&metrics).unwrap();
        let normalized = processor.normalize(&features, NormalizationMethod::ZScore).unwrap();
        
        // Train
        let training_data = processor.prepare_training_data(&normalized).unwrap();
        let request = TrainingRequest {
            model_type: ModelType::Regression,
            algorithm: TrainingAlgorithm::Adam,
            epochs: 10,
            batch_size: 32,
            learning_rate: 0.01,
            validation_split: 0.2,
        };
        let _ = trainer.train_model(&training_data, request);
    });

    // Benchmark 3: AIIntegration complete flow
    suite.run_benchmark("integration_ai_flow", || {
        let collector = DataCollector::new().unwrap();
        let processor = DataProcessor::new().unwrap();
        let trainer = ModelTrainer::new().unwrap();

        let mut ai_integration = AIIntegration::new(collector, processor, trainer).unwrap();
        ai_integration.initialize().unwrap();

        // Add metrics
        let metrics = generate_integration_metrics(50);
        for metric in &metrics {
            let _ = ai_integration.add_metrics(*metric);
        }

        // Process
        let _ = ai_integration.process_data();

        // Get insights
        let _ = ai_integration.get_insights();

        // Shutdown
        let _ = ai_integration.shutdown();
    });

    // Benchmark 4: Scheduler integration
    suite.run_benchmark("integration_scheduler", || {
        use crate::ai::integration::SchedulerIntegration;

        let collector = DataCollector::new().unwrap();
        let processor = DataProcessor::new().unwrap();
        let trainer = ModelTrainer::new().unwrap();

        let mut scheduler_integration = SchedulerIntegration::new(collector, processor, trainer).unwrap();

        // Add scheduler metrics
        let metrics = generate_integration_metrics(50);
        for metric in &metrics {
            let _ = scheduler_integration.add_scheduler_metrics(*metric);
        }

        // Get optimization suggestions
        let _ = scheduler_integration.get_optimization_suggestions();
    });

    // Benchmark 5: Power manager integration
    suite.run_benchmark("integration_power_manager", || {
        use crate::ai::integration::PowerManagerIntegration;

        let collector = DataCollector::new().unwrap();
        let processor = DataProcessor::new().unwrap();
        let trainer = ModelTrainer::new().unwrap();

        let mut power_integration = PowerManagerIntegration::new(collector, processor, trainer).unwrap();

        // Add power metrics
        let metrics = generate_integration_metrics(50);
        for metric in &metrics {
            let _ = power_integration.add_power_metrics(*metric);
        }

        // Get power optimizations
        let _ = power_integration.get_power_optimizations();
    });

    // Benchmark 6: Load balancer integration
    suite.run_benchmark("integration_load_balancer", || {
        use crate::ai::integration::LoadBalancerIntegration;

        let collector = DataCollector::new().unwrap();
        let processor = DataProcessor::new().unwrap();
        let trainer = ModelTrainer::new().unwrap();

        let mut lb_integration = LoadBalancerIntegration::new(collector, processor, trainer).unwrap();

        // Add load balancer metrics
        let metrics = generate_integration_metrics(50);
        for metric in &metrics {
            let _ = lb_integration.add_load_balancer_metrics(*metric);
        }

        // Get load balancing decisions
        let _ = lb_integration.get_load_balancing_decisions();
    });

    // Benchmark 7: Threat detection integration
    suite.run_benchmark("integration_threat_detection", || {
        use crate::ai::integration::ThreatDetectionIntegration;

        let collector = DataCollector::new().unwrap();
        let processor = DataProcessor::new().unwrap();
        let trainer = ModelTrainer::new().unwrap();

        let mut threat_integration = ThreatDetectionIntegration::new(collector, processor, trainer).unwrap();

        // Add security metrics
        let metrics = generate_integration_metrics(50);
        for metric in &metrics {
            let _ = threat_integration.add_security_metrics(*metric);
        }

        // Assess threats
        let _ = threat_integration.assess_threats();
    });

    // Benchmark 8: Full system integration with all components
    suite.run_benchmark("integration_full_system", || {
        // Initialize all components
        let collector = DataCollector::new().unwrap();
        let processor = DataProcessor::new().unwrap();
        let trainer = ModelTrainer::new().unwrap();

        let mut ai_integration = AIIntegration::new(
            collector,
            processor,
            trainer,
        ).unwrap();
        ai_integration.initialize().unwrap();

        // Simulate multiple metric cycles
        for cycle in 0..5 {
            let metrics = generate_integration_metrics(20);
            for metric in &metrics {
                let _ = ai_integration.add_metrics(*metric);
            }
            let _ = ai_integration.process_data();
        }

        // Final insights
        let _ = ai_integration.get_insights();
        let _ = ai_integration.shutdown();
    });

    // Print results
    suite.print_results();

    // Check against performance targets
    println!("\n=== Performance Targets ===\n");
    let targets = vec![
        PerformanceTarget {
            name: "integration_e2e_10".to_string(),
            target_avg: Duration::from_millis(10), // <10ms end-to-end
            target_max: Duration::from_millis(50), // <50ms
            target_min_throughput: Some(100.0), // >100 pipelines/sec
        },
        PerformanceTarget {
            name: "integration_e2e_100".to_string(),
            target_avg: Duration::from_millis(50), // <50ms end-to-end
            target_max: Duration::from_millis(100), // <100ms
            target_min_throughput: Some(20.0), // >20 pipelines/sec
        },
        PerformanceTarget {
            name: "integration_ai_flow".to_string(),
            target_avg: Duration::from_millis(20), // <20ms
            target_max: Duration::from_millis(100), // <100ms
            target_min_throughput: Some(50.0), // >50 flows/sec
        },
        PerformanceTarget {
            name: "integration_full_system".to_string(),
            target_avg: Duration::from_millis(100), // <100ms
            target_max: Duration::from_millis(500), // <500ms
            target_min_throughput: Some(10.0), // >10 full cycles/sec
        },
    ];

    for target in &targets {
        if let Some(result) = suite.results().iter().find(|r| r.name == target.name) {
            let comparison = target.compare(result);
            println!("{}", comparison.format());
        }
    }
}

/// Run all benchmarks and print summary
pub fn run_all_benchmarks() {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║       VantisOS AI Pipeline Benchmark Suite               ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    collector_bench::run_collector_benchmarks();
    processor_bench::run_processor_benchmarks();
    trainer_bench::run_trainer_benchmarks();
    run_integration_benchmarks();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║                  Benchmark Summary                        ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
}

/// Benchmark: End-to-end latency
pub fn benchmark_e2e_latency() -> Duration {
    let mut suite = BenchmarkSuite::with_defaults();

    let result = suite.run_benchmark("e2e_latency", || {
        let collector = DataCollector::new().unwrap();
        let processor = DataProcessor::new().unwrap();
        let trainer = ModelTrainer::new().unwrap();

        let metrics = generate_integration_metrics(100);
        
        for metric in &metrics {
            let _ = collector.add_metric(*metric);
        }
        
        let features = processor.extract_features(&metrics).unwrap();
        let normalized = processor.normalize(&features, NormalizationMethod::ZScore).unwrap();
        
        let training_data = processor.prepare_training_data(&normalized).unwrap();
        let request = TrainingRequest {
            model_type: ModelType::Regression,
            algorithm: TrainingAlgorithm::Adam,
            epochs: 10,
            batch_size: 32,
            learning_rate: 0.01,
            validation_split: 0.2,
        };
        let _ = trainer.train_model(&training_data, request);
    });

    result.avg_duration
}

/// Benchmark: System throughput
pub fn benchmark_system_throughput() -> f64 {
    let mut suite = BenchmarkSuite::with_defaults();

    suite.run_benchmark("system_throughput", || {
        let collector = DataCollector::new().unwrap();
        let processor = DataProcessor::new().unwrap();

        let metrics = generate_integration_metrics(100);
        
        for metric in &metrics {
            let _ = collector.add_metric(*metric);
        }
        
        let features = processor.extract_features(&metrics).unwrap();
        let _ = processor.normalize(&features, NormalizationMethod::ZScore);
    });

    // Calculate samples per second
    let result = suite.results().last().unwrap();
    100.0 / result.avg_duration.as_secs_f64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_integration_benchmarks() {
        run_integration_benchmarks();
    }

    #[test]
    fn test_e2e_latency() {
        let latency = benchmark_e2e_latency();
        assert!(
            latency < Duration::from_millis(50),
            "End-to-end latency should be <50ms"
        );
    }

    #[test]
    fn test_system_throughput() {
        let throughput = benchmark_system_throughput();
        assert!(
            throughput > 1000.0,
            "System throughput should be >1000 samples/sec"
        );
    }
}