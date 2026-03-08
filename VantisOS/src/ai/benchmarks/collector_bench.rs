//! Benchmarks for DataCollector module
//!
//! These benchmarks measure the performance of data collection operations
//! including metric collection, buffer operations, and sampling.

use crate::ai::modules::DataCollector;
use crate::ai::benchmarks::{BenchmarkSuite, PerformanceTarget};
use crate::ai::types::SystemMetrics;

/// Run all DataCollector benchmarks
pub fn run_collector_benchmarks() {
    println!("\n=== DataCollector Benchmarks ===\n");

    let mut suite = BenchmarkSuite::with_defaults();

    // Benchmark 1: Single metric collection
    suite.run_benchmark("collector_single_metric", || {
        let mut collector = DataCollector::new().unwrap();
        let metric = SystemMetrics {
            cpu_usage: 50.0,
            memory_usage: 60.0,
            disk_usage: 70.0,
            network_throughput: 1000.0,
            power_consumption: 45.0,
            timestamp: 1000,
        };
        let _ = collector.add_metric(metric);
    });

    // Benchmark 2: Batch metric collection (100 metrics)
    suite.run_benchmark("collector_batch_100", || {
        let mut collector = DataCollector::new().unwrap();
        for i in 0..100 {
            let metric = SystemMetrics {
                cpu_usage: 50.0 + (i as f64 * 0.1),
                memory_usage: 60.0 + (i as f64 * 0.1),
                disk_usage: 70.0 + (i as f64 * 0.05),
                network_throughput: 1000.0 + (i as f64 * 10.0),
                power_consumption: 45.0 + (i as f64 * 0.05),
                timestamp: 1000 + i as u64,
            };
            let _ = collector.add_metric(metric);
        }
    });

    // Benchmark 3: Collect all metrics from buffer
    suite.run_benchmark_with_setup(
        "collector_collect_all",
        || {
            let mut collector = DataCollector::new().unwrap();
            // Populate buffer
            for i in 0..100 {
                let metric = SystemMetrics {
                    cpu_usage: 50.0 + (i as f64 * 0.1),
                    memory_usage: 60.0 + (i as f64 * 0.1),
                    disk_usage: 70.0 + (i as f64 * 0.05),
                    network_throughput: 1000.0 + (i as f64 * 10.0),
                    power_consumption: 45.0 + (i as f64 * 0.05),
                    timestamp: 1000 + i as u64,
                };
                let _ = collector.add_metric(metric);
            }
            collector
        },
        |collector| {
            let _ = collector.collect_all_metrics();
        },
        |_collector| {},
    );

    // Benchmark 4: Get statistics
    suite.run_benchmark_with_setup(
        "collector_get_stats",
        || {
            let mut collector = DataCollector::new().unwrap();
            for i in 0..100 {
                let metric = SystemMetrics {
                    cpu_usage: 50.0 + (i as f64 * 0.1),
                    memory_usage: 60.0 + (i as f64 * 0.1),
                    disk_usage: 70.0 + (i as f64 * 0.05),
                    network_throughput: 1000.0 + (i as f64 * 10.0),
                    power_consumption: 45.0 + (i as f64 * 0.05),
                    timestamp: 1000 + i as u64,
                };
                let _ = collector.add_metric(metric);
            }
            collector
        },
        |collector| {
            let _ = collector.get_statistics();
        },
        |_collector| {},
    );

    // Benchmark 5: Buffer operations (add and remove)
    suite.run_benchmark("collector_buffer_ops", || {
        let mut collector = DataCollector::new().unwrap();
        for i in 0..50 {
            let metric = SystemMetrics {
                cpu_usage: 50.0,
                memory_usage: 60.0,
                disk_usage: 70.0,
                network_throughput: 1000.0,
                power_consumption: 45.0,
                timestamp: 1000 + i as u64,
            };
            let _ = collector.add_metric(metric);
        }
        let _ = collector.collect_all_metrics();
    });

    // Print results
    suite.print_results();

    // Check against performance targets
    println!("\n=== Performance Targets ===\n");
    let targets = vec![
        PerformanceTarget {
            name: "collector_single_metric".to_string(),
            target_avg: Duration::from_micros(100), // <100μs
            target_max: Duration::from_micros(500), // <500μs
            target_min_throughput: Some(10_000.0), // >10,000 ops/sec
        },
        PerformanceTarget {
            name: "collector_batch_100".to_string(),
            target_avg: Duration::from_millis(1), // <1ms
            target_max: Duration::from_millis(5), // <5ms
            target_min_throughput: Some(1000.0), // >1,000 ops/sec
        },
        PerformanceTarget {
            name: "collector_collect_all".to_string(),
            target_avg: Duration::from_micros(100), // <100μs
            target_max: Duration::from_millis(1), // <1ms
            target_min_throughput: Some(10_000.0), // >10,000 ops/sec
        },
    ];

    for target in &targets {
        if let Some(result) = suite.results().iter().find(|r| r.name == target.name) {
            let comparison = target.compare(result);
            println!("{}", comparison.format());
        }
    }
}

/// Benchmark: Data collection latency
pub fn benchmark_collection_latency() -> Duration {
    let mut suite = BenchmarkSuite::with_defaults();

    let result = suite.run_benchmark("collection_latency", || {
        let mut collector = DataCollector::new().unwrap();
        let metric = SystemMetrics {
            cpu_usage: 50.0,
            memory_usage: 60.0,
            disk_usage: 70.0,
            network_throughput: 1000.0,
            power_consumption: 45.0,
            timestamp: 1000,
        };
        let _ = collector.add_metric(metric);
    });

    result.avg_duration
}

/// Benchmark: Data collection throughput
pub fn benchmark_collection_throughput() -> f64 {
    let mut suite = BenchmarkSuite::with_defaults();

    let result = suite.run_benchmark("collection_throughput", || {
        let mut collector = DataCollector::new().unwrap();
        let metric = SystemMetrics {
            cpu_usage: 50.0,
            memory_usage: 60.0,
            disk_usage: 70.0,
            network_throughput: 1000.0,
            power_consumption: 45.0,
            timestamp: 1000,
        };
        let _ = collector.add_metric(metric);
    });

    result.throughput()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_collector_benchmarks() {
        run_collector_benchmarks();
    }

    #[test]
    fn test_collection_latency() {
        let latency = benchmark_collection_latency();
        assert!(latency < Duration::from_millis(1), "Collection latency should be <1ms");
    }

    #[test]
    fn test_collection_throughput() {
        let throughput = benchmark_collection_throughput();
        assert!(
            throughput > 10_000.0,
            "Collection throughput should be >10,000 ops/sec"
        );
    }
}