//! Benchmarks for ModelTrainer module
//!
//! These benchmarks measure the performance of model training operations
//! including different algorithms, hyperparameter tuning, and model compression.

use crate::ai::modules::ModelTrainer;
use crate::ai::benchmarks::{BenchmarkSuite, PerformanceTarget};
use crate::ai::types::{TrainingRequest, TrainingAlgorithm, ModelType, CompressionMethod};

/// Generate sample training data
pub fn generate_training_data(samples: usize, features: usize) -> Vec<(Vec<f64>, Vec<f64>)> {
    (0..samples)
        .map(|_| {
            let input = (0..features)
                .map(|i| (i as f64 + 1.0) / (features as f64))
                .collect();
            let output = vec![input.iter().sum::<f64>() / features as f64];
            (input, output)
        })
        .collect()
}

/// Run all ModelTrainer benchmarks
pub fn run_trainer_benchmarks() {
    println!("\n=== ModelTrainer Benchmarks ===\n");

    let mut suite = BenchmarkSuite::with_defaults();

    // Benchmark 1: Small model training (SGD)
    suite.run_benchmark("trainer_sgd_small", || {
        let trainer = ModelTrainer::new().unwrap();
        let data = generate_training_data(100, 10);
        let request = TrainingRequest {
            model_type: ModelType::Regression,
            algorithm: TrainingAlgorithm::SGD,
            epochs: 10,
            batch_size: 32,
            learning_rate: 0.01,
            validation_split: 0.2,
        };
        let _ = trainer.train_model(&data, request);
    });

    // Benchmark 2: Medium model training (Adam)
    suite.run_benchmark("trainer_adam_medium", || {
        let trainer = ModelTrainer::new().unwrap();
        let data = generate_training_data(500, 20);
        let request = TrainingRequest {
            model_type: ModelType::Regression,
            algorithm: TrainingAlgorithm::Adam,
            epochs: 20,
            batch_size: 32,
            learning_rate: 0.001,
            validation_split: 0.2,
        };
        let _ = trainer.train_model(&data, request);
    });

    // Benchmark 3: Large model training (RMSprop)
    suite.run_benchmark("trainer_rmsprop_large", || {
        let trainer = ModelTrainer::new().unwrap();
        let data = generate_training_data(1000, 50);
        let request = TrainingRequest {
            model_type: ModelType::Regression,
            algorithm: TrainingAlgorithm::RMSprop,
            epochs: 30,
            batch_size: 64,
            learning_rate: 0.001,
            validation_split: 0.2,
        };
        let _ = trainer.train_model(&data, request);
    });

    // Benchmark 4: Model inference latency
    suite.run_benchmark_with_setup(
        "trainer_inference",
        || {
            let trainer = ModelTrainer::new().unwrap();
            let data = generate_training_data(100, 10);
            let request = TrainingRequest {
                model_type: ModelType::Regression,
                algorithm: TrainingAlgorithm::Adam,
                epochs: 10,
                batch_size: 32,
                learning_rate: 0.01,
                validation_split: 0.2,
            };
            let result = trainer.train_model(&data, request).unwrap();
            (trainer, result.model)
        },
        |(_, model)| {
            let input = vec![0.5; 10];
            let _ = model.predict(&input);
        },
        |_| {},
    );

    // Benchmark 5: Model quantization
    suite.run_benchmark_with_setup(
        "trainer_quantization",
        || {
            let trainer = ModelTrainer::new().unwrap();
            let data = generate_training_data(100, 10);
            let request = TrainingRequest {
                model_type: ModelType::Regression,
                algorithm: TrainingAlgorithm::Adam,
                epochs: 10,
                batch_size: 32,
                learning_rate: 0.01,
                validation_split: 0.2,
            };
            let result = trainer.train_model(&data, request).unwrap();
            trainer
        },
        |trainer| {
            let _ = trainer.compress_model(
                &crate::ai::types::AIModel::default(),
                CompressionMethod::Quantization(8),
            );
        },
        |_| {},
    );

    // Benchmark 6: Hyperparameter tuning (Grid Search)
    suite.run_benchmark("trainer_hyperopt_grid", || {
        let trainer = ModelTrainer::new().unwrap();
        let data = generate_training_data(100, 10);
        let _ = trainer.tune_hyperparameters(&data, crate::ai::types::TuningConfig {
            method: crate::ai::types::TuningMethod::Grid,
            iterations: 5,
            parallel: false,
        });
    });

    // Benchmark 7: Cross-validation (K-fold)
    suite.run_benchmark("trainer_cross_validation", || {
        let trainer = ModelTrainer::new().unwrap();
        let data = generate_training_data(100, 10);
        let _ = trainer.cross_validate(&data, crate::ai::types::CrossValidationConfig {
            folds: 5,
            stratified: false,
        });
    });

    // Benchmark 8: Model evaluation
    suite.run_benchmark_with_setup(
        "trainer_evaluation",
        || {
            let trainer = ModelTrainer::new().unwrap();
            let data = generate_training_data(100, 10);
            let request = TrainingRequest {
                model_type: ModelType::Regression,
                algorithm: TrainingAlgorithm::Adam,
                epochs: 10,
                batch_size: 32,
                learning_rate: 0.01,
                validation_split: 0.2,
            };
            let result = trainer.train_model(&data, request).unwrap();
            (trainer, result.model, data)
        },
        |(trainer, model, data)| {
            let _ = trainer.evaluate_model(&model, &data);
        },
        |_| {},
    );

    // Print results
    suite.print_results();

    // Check against performance targets
    println!("\n=== Performance Targets ===\n");
    let targets = vec![
        PerformanceTarget {
            name: "trainer_sgd_small".to_string(),
            target_avg: Duration::from_secs(5), // <5 minutes for small models
            target_max: Duration::from_secs(300), // <5 minutes
            target_min_throughput: None,
        },
        PerformanceTarget {
            name: "trainer_inference".to_string(),
            target_avg: Duration::from_millis(5), // <5ms inference latency
            target_max: Duration::from_millis(10), // <10ms
            target_min_throughput: Some(200.0), // >200 predictions/sec
        },
    ];

    for target in &targets {
        if let Some(result) = suite.results().iter().find(|r| r.name == target.name) {
            let comparison = target.compare(result);
            println!("{}", comparison.format());
        }
    }
}

/// Benchmark: Training time for small model
pub fn benchmark_training_small() -> Duration {
    let mut suite = BenchmarkSuite::with_defaults();

    let result = suite.run_benchmark("training_small", || {
        let trainer = ModelTrainer::new().unwrap();
        let data = generate_training_data(100, 10);
        let request = TrainingRequest {
            model_type: ModelType::Regression,
            algorithm: TrainingAlgorithm::SGD,
            epochs: 10,
            batch_size: 32,
            learning_rate: 0.01,
            validation_split: 0.2,
        };
        let _ = trainer.train_model(&data, request);
    });

    result.avg_duration
}

/// Benchmark: Training time for medium model
pub fn benchmark_training_medium() -> Duration {
    let mut suite = BenchmarkSuite::with_defaults();

    let result = suite.run_benchmark("training_medium", || {
        let trainer = ModelTrainer::new().unwrap();
        let data = generate_training_data(500, 20);
        let request = TrainingRequest {
            model_type: ModelType::Regression,
            algorithm: TrainingAlgorithm::Adam,
            epochs: 20,
            batch_size: 32,
            learning_rate: 0.001,
            validation_split: 0.2,
        };
        let _ = trainer.train_model(&data, request);
    });

    result.avg_duration
}

/// Benchmark: Inference latency
pub fn benchmark_inference_latency() -> Duration {
    let mut suite = BenchmarkSuite::with_defaults();

    let result = suite.run_benchmark("inference_latency", || {
        let trainer = ModelTrainer::new().unwrap();
        let data = generate_training_data(100, 10);
        let request = TrainingRequest {
            model_type: ModelType::Regression,
            algorithm: TrainingAlgorithm::Adam,
            epochs: 10,
            batch_size: 32,
            learning_rate: 0.01,
            validation_split: 0.2,
        };
        let result = trainer.train_model(&data, request).unwrap();
        let input = vec![0.5; 10];
        let _ = result.model.predict(&input);
    });

    result.avg_duration
}

/// Benchmark: Inference throughput
pub fn benchmark_inference_throughput() -> f64 {
    let mut suite = BenchmarkSuite::with_defaults();

    suite.run_benchmark("inference_throughput", || {
        let trainer = ModelTrainer::new().unwrap();
        let data = generate_training_data(100, 10);
        let request = TrainingRequest {
            model_type: ModelType::Regression,
            algorithm: TrainingAlgorithm::Adam,
            epochs: 10,
            batch_size: 32,
            learning_rate: 0.01,
            validation_split: 0.2,
        };
        let result = trainer.train_model(&data, request).unwrap();
        for _ in 0..100 {
            let input = vec![0.5; 10];
            let _ = result.model.predict(&input);
        }
    });

    // Calculate predictions per second
    let result = suite.results().last().unwrap();
    100.0 / result.avg_duration.as_secs_f64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_trainer_benchmarks() {
        run_trainer_benchmarks();
    }

    #[test]
    fn test_training_small_performance() {
        let training_time = benchmark_training_small();
        assert!(
            training_time < Duration::from_secs(300),
            "Small model training should be <5 minutes"
        );
    }

    #[test]
    fn test_inference_latency() {
        let latency = benchmark_inference_latency();
        assert!(
            latency < Duration::from_millis(5),
            "Inference latency should be <5ms"
        );
    }

    #[test]
    fn test_inference_throughput() {
        let throughput = benchmark_inference_throughput();
        assert!(
            throughput > 200.0,
            "Inference throughput should be >200 predictions/sec"
        );
    }
}