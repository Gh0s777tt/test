//! Benchmarks for DataProcessor module
//!
//! These benchmarks measure the performance of data processing operations
//! including feature extraction, normalization, and outlier detection.

use crate::ai::modules::DataProcessor;
use crate::ai::benchmarks::{BenchmarkSuite, PerformanceTarget};
use crate::ai::types::{SystemMetrics, NormalizationMethod, OutlierMethod};

/// Generate sample metrics for benchmarking
pub fn generate_sample_metrics(count: usize) -> Vec<SystemMetrics> {
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

/// Run all DataProcessor benchmarks
pub fn run_processor_benchmarks() {
    println!("\n=== DataProcessor Benchmarks ===\n");

    let mut suite = BenchmarkSuite::with_defaults();

    // Benchmark 1: Feature extraction (10 metrics)
    suite.run_benchmark("processor_extract_features_10", || {
        let processor = DataProcessor::new().unwrap();
        let metrics = generate_sample_metrics(10);
        let _ = processor.extract_features(&metrics);
    });

    // Benchmark 2: Feature extraction (100 metrics)
    suite.run_benchmark("processor_extract_features_100", || {
        let processor = DataProcessor::new().unwrap();
        let metrics = generate_sample_metrics(100);
        let _ = processor.extract_features(&metrics);
    });

    // Benchmark 3: Feature extraction (1000 metrics)
    suite.run_benchmark("processor_extract_features_1000", || {
        let processor = DataProcessor::new().unwrap();
        let metrics = generate_sample_metrics(1000);
        let _ = processor.extract_features(&metrics);
    });

    // Benchmark 4: MinMax normalization
    suite.run_benchmark("processor_normalize_minmax", || {
        let processor = DataProcessor::new().unwrap();
        let metrics = generate_sample_metrics(100);
        let features = processor.extract_features(&metrics).unwrap();
        let _ = processor.normalize(&features, NormalizationMethod::MinMax);
    });

    // Benchmark 5: Z-score normalization
    suite.run_benchmark("processor_normalize_zscore", || {
        let processor = DataProcessor::new().unwrap();
        let metrics = generate_sample_metrics(100);
        let features = processor.extract_features(&metrics).unwrap();
        let _ = processor.normalize(&features, NormalizationMethod::ZScore);
    });

    // Benchmark 6: Robust normalization
    suite.run_benchmark("processor_normalize_robust", || {
        let processor = DataProcessor::new().unwrap();
        let metrics = generate_sample_metrics(100);
        let features = processor.extract_features(&metrics).unwrap();
        let _ = processor.normalize(&features, NormalizationMethod::Robust);
    });

    // Benchmark 7: Outlier detection (IQR)
    suite.run_benchmark("processor_outlier_iqr", || {
        let processor = DataProcessor::new().unwrap();
        let metrics = generate_sample_metrics(100);
        let features = processor.extract_features(&metrics).unwrap();
        let _ = processor.detect_outliers(&features, OutlierMethod::IQR);
    });

    // Benchmark 8: Outlier detection (Z-score)
    suite.run_benchmark("processor_outlier_zscore", || {
        let processor = DataProcessor::new().unwrap();
        let metrics = generate_sample_metrics(100);
        let features = processor.extract_features(&metrics).unwrap();
        let _ = processor.detect_outliers(&features, OutlierMethod::ZScore);
    });

    // Benchmark 9: Prepare training data
    suite.run_benchmark("processor_prepare_training_data", || {
        let processor = DataProcessor::new().unwrap();
        let metrics = generate_sample_metrics(100);
        let features = processor.extract_features(&metrics).unwrap();
        let _ = processor.prepare_training_data(&features);
    });

    // Benchmark 10: Complete processing pipeline
    suite.run_benchmark("processor_complete_pipeline", || {
        let processor = DataProcessor::new().unwrap();
        let metrics = generate_sample_metrics(100);
        
        let features = processor.extract_features(&metrics).unwrap();
        let normalized = processor.normalize(&features, NormalizationMethod::ZScore).unwrap();
        let outliers = processor.detect_outliers(&normalized, OutlierMethod::IQR).unwrap();
        let _ = processor.prepare_training_data(&normalized);
    });

    // Print results
    suite.print_results();

    // Check against performance targets
    println!("\n=== Performance Targets ===\n");
    let targets = vec![
        PerformanceTarget {
            name: "processor_extract_features_10".to_string(),
            target_avg: Duration::from_micros(100), // <100μs per sample = <1ms total
            target_max: Duration::from_millis(2), // <2ms
            target_min_throughput: Some(10_000.0), // >10,000 ops/sec
        },
        PerformanceTarget {
            name: "processor_extract_features_100".to_string(),
            target_avg: Duration::from_millis(10), // <10ms = <100μs per sample
            target_max: Duration::from_millis(20), // <20ms
            target_min_throughput: Some(10_000.0), // >10,000 samples/sec
        },
        PerformanceTarget {
            name: "processor_extract_features_1000".to_string(),
            target_avg: Duration::from_millis(100), // <100ms = <100μs per sample
            target_max: Duration::from_millis(200), // <200ms
            target_min_throughput: Some(10_000.0), // >10,000 samples/sec
        },
    ];

    for target in &targets {
        if let Some(result) = suite.results().iter().find(|r| r.name == target.name) {
            let comparison = target.compare(result);
            println!("{}", comparison.format());
        }
    }
}

/// Benchmark: Feature extraction time
pub fn benchmark_feature_extraction() -> Duration {
    let mut suite = BenchmarkSuite::with_defaults();

    let result = suite.run_benchmark("feature_extraction", || {
        let processor = DataProcessor::new().unwrap();
        let metrics = generate_sample_metrics(100);
        let _ = processor.extract_features(&metrics);
    });

    result.avg_duration
}

/// Benchmark: Normalization time
pub fn benchmark_normalization() -> Duration {
    let mut suite = BenchmarkSuite::with_defaults();

    let result = suite.run_benchmark("normalization", || {
        let processor = DataProcessor::new().unwrap();
        let metrics = generate_sample_metrics(100);
        let features = processor.extract_features(&metrics).unwrap();
        let _ = processor.normalize(&features, NormalizationMethod::ZScore);
    });

    result.avg_duration
}

/// Benchmark: Processing throughput
pub fn benchmark_processing_throughput() -> f64 {
    let mut suite = BenchmarkSuite::with_defaults();

    let result = suite.run_benchmark("processing_throughput", || {
        let processor = DataProcessor::new().unwrap();
        let metrics = generate_sample_metrics(100);
        let _ = processor.extract_features(&metrics);
    });

    // Calculate samples per second
    100.0 / result.avg_duration.as_secs_f64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_processor_benchmarks() {
        run_processor_benchmarks();
    }

    #[test]
    fn test_feature_extraction_performance() {
        let extraction_time = benchmark_feature_extraction();
        let per_sample = extraction_time / 100; // 100 samples
        assert!(
            per_sample < Duration::from_micros(100),
            "Feature extraction should be <100μs per sample"
        );
    }

    #[test]
    fn test_normalization_performance() {
        let norm_time = benchmark_normalization();
        let per_sample = norm_time / 100; // 100 samples
        assert!(
            per_sample < Duration::from_micros(50),
            "Normalization should be <50μs per sample"
        );
    }

    #[test]
    fn test_processing_throughput() {
        let throughput = benchmark_processing_throughput();
        assert!(
            throughput > 10_000.0,
            "Processing throughput should be >10,000 samples/sec"
        );
    }
}