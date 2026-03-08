//! VantisOS AI Benchmarking Module
//!
//! This module provides comprehensive benchmarking capabilities for all AI components,
//! including automated performance measurement, statistical analysis, and comparison tools.

use std::time::{Duration, Instant};

/// Benchmark result for a single measurement
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Name of the benchmark
    pub name: String,
    /// Number of iterations
    pub iterations: u64,
    /// Total time taken
    pub total_duration: Duration,
    /// Average time per iteration
    pub avg_duration: Duration,
    /// Minimum time taken
    pub min_duration: Duration,
    /// Maximum time taken
    pub max_duration: Duration,
    /// Standard deviation
    pub std_dev: Duration,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
}

impl BenchmarkResult {
    /// Calculate throughput in operations per second
    pub fn throughput(&self) -> f64 {
        self.iterations as f64 / self.total_duration.as_secs_f64()
    }

    /// Calculate operations per millisecond
    pub fn ops_per_ms(&self) -> f64 {
        self.iterations as f64 / self.total_duration.as_millis() as f64
    }

    /// Format result as a string
    pub fn format(&self) -> String {
        format!(
            "{}: {} iterations, avg: {:?}, min: {:?}, max: {:?}, std: {:?}, throughput: {:.2} ops/sec",
            self.name,
            self.iterations,
            self.avg_duration,
            self.min_duration,
            self.max_duration,
            self.std_dev,
            self.throughput()
        )
    }
}

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Number of warmup iterations
    pub warmup_iterations: u64,
    /// Number of benchmark iterations
    pub benchmark_iterations: u64,
    /// Memory profiling enabled
    pub memory_profiling: bool,
    /// Detailed timing enabled
    pub detailed_timing: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            warmup_iterations: 10,
            benchmark_iterations: 1000,
            memory_profiling: true,
            detailed_timing: true,
        }
    }
}

/// Benchmark suite runner
pub struct BenchmarkSuite {
    config: BenchmarkConfig,
    results: Vec<BenchmarkResult>,
}

impl BenchmarkSuite {
    /// Create a new benchmark suite
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }

    /// Create a benchmark suite with default configuration
    pub fn with_defaults() -> Self {
        Self::new(BenchmarkConfig::default())
    }

    /// Run a benchmark function
    pub fn run_benchmark<F>(&mut self, name: &str, mut bench_fn: F) -> &BenchmarkResult
    where
        F: FnMut(),
    {
        // Warmup
        for _ in 0..self.config.warmup_iterations {
            bench_fn();
        }

        // Benchmark
        let mut durations = Vec::with_capacity(self.config.benchmark_iterations as usize);
        let start_memory = if self.config.memory_profiling {
            self.measure_memory()
        } else {
            0
        };

        for _ in 0..self.config.benchmark_iterations {
            let start = Instant::now();
            bench_fn();
            durations.push(start.elapsed());
        }

        let end_memory = if self.config.memory_profiling {
            self.measure_memory()
        } else {
            start_memory
        };

        // Calculate statistics
        let total_duration: Duration = durations.iter().sum();
        let avg_duration = total_duration / self.config.benchmark_iterations as u32;
        let min_duration = *durations.iter().min().unwrap();
        let max_duration = *durations.iter().max().unwrap();

        let std_dev = {
            let mean = avg_duration.as_nanos() as f64;
            let variance = durations
                .iter()
                .map(|d| {
                    let diff = d.as_nanos() as f64 - mean;
                    diff * diff
                })
                .sum::<f64>()
                / durations.len() as f64;
            Duration::from_nanos(variance.sqrt() as u64)
        };

        let result = BenchmarkResult {
            name: name.to_string(),
            iterations: self.config.benchmark_iterations,
            total_duration,
            avg_duration,
            min_duration,
            max_duration,
            std_dev,
            memory_usage_bytes: end_memory.saturating_sub(start_memory),
        };

        self.results.push(result);
        self.results.last().unwrap()
    }

    /// Run a benchmark function with setup and teardown
    pub fn run_benchmark_with_setup<Bench, Setup, Teardown>(
        &mut self,
        name: &str,
        setup: Setup,
        mut bench_fn: Bench,
        teardown: Teardown,
    ) -> &BenchmarkResult
    where
        Setup: Fn(),
        Bench: FnMut(),
        Teardown: Fn(),
    {
        self.run_benchmark(name, || {
            setup();
            bench_fn();
            teardown();
        })
    }

    /// Get all benchmark results
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }

    /// Print all benchmark results
    pub fn print_results(&self) {
        println!("\n=== Benchmark Results ===\n");
        for result in &self.results {
            println!("{}", result.format());
        }
        println!("\nTotal benchmarks: {}\n", self.results.len());
    }

    /// Compare two benchmark results
    pub fn compare_results(&self, name1: &str, name2: &str) -> Option<f64> {
        let result1 = self.results.iter().find(|r| r.name == name1)?;
        let result2 = self.results.iter().find(|r| r.name == name2)?;

        Some(result1.avg_duration.as_nanos() as f64 / result2.avg_duration.as_nanos() as f64)
    }

    /// Get benchmark statistics summary
    pub fn summary(&self) -> BenchmarkSummary {
        BenchmarkSummary {
            total_benchmarks: self.results.len(),
            total_iterations: self.results.iter().map(|r| r.iterations).sum(),
            total_duration: self.results.iter().map(|r| r.total_duration).sum(),
            avg_memory_usage: if self.config.memory_profiling {
                let total: u64 = self.results.iter().map(|r| r.memory_usage_bytes).sum();
                total / self.results.len() as u64
            } else {
                0
            },
        }
    }

    /// Measure current memory usage
    #[cfg(target_os = "linux")]
    fn measure_memory(&self) -> u64 {
        use std::fs;
        if let Ok(status) = fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(kb) = parts[1].parse::<u64>() {
                            return kb * 1024; // Convert to bytes
                        }
                    }
                }
            }
        }
        0
    }

    #[cfg(not(target_os = "linux"))]
    fn measure_memory(&self) -> u64 {
        0
    }
}

impl Default for BenchmarkSuite {
    fn default() -> Self {
        Self::with_defaults()
    }
}

/// Benchmark summary statistics
#[derive(Debug, Clone)]
pub struct BenchmarkSummary {
    /// Total number of benchmarks run
    pub total_benchmarks: usize,
    /// Total iterations across all benchmarks
    pub total_iterations: u64,
    /// Total duration across all benchmarks
    pub total_duration: Duration,
    /// Average memory usage across all benchmarks
    pub avg_memory_usage: u64,
}

/// Performance target for a benchmark
#[derive(Debug, Clone)]
pub struct PerformanceTarget {
    /// Name of the benchmark
    pub name: String,
    /// Target average duration
    pub target_avg: Duration,
    /// Target maximum duration
    pub target_max: Duration,
    /// Target minimum throughput (ops/sec)
    pub target_min_throughput: Option<f64>,
}

impl PerformanceTarget {
    /// Check if a benchmark result meets the target
    pub fn meets_target(&self, result: &BenchmarkResult) -> bool {
        let avg_ok = result.avg_duration <= self.target_avg;
        let max_ok = result.max_duration <= self.target_max;
        let throughput_ok = self
            .target_min_throughput
            .map(|target| result.throughput() >= target)
            .unwrap_or(true);

        avg_ok && max_ok && throughput_ok
    }

    /// Generate a report comparing result to target
    pub fn compare(&self, result: &BenchmarkResult) -> TargetComparison {
        TargetComparison {
            name: self.name.clone(),
            avg_diff: result.avg_duration.as_nanos() as f64 - self.target_avg.as_nanos() as f64,
            max_diff: result.max_duration.as_nanos() as f64 - self.target_max.as_nanos() as f64,
            meets_target: self.meets_target(result),
        }
    }
}

/// Target comparison result
#[derive(Debug, Clone)]
pub struct TargetComparison {
    /// Name of the benchmark
    pub name: String,
    /// Difference from target average (positive = slower)
    pub avg_diff: f64,
    /// Difference from target maximum (positive = exceeds)
    pub max_diff: f64,
    /// Whether the target is met
    pub meets_target: bool,
}

impl TargetComparison {
    /// Format comparison result
    pub fn format(&self) -> String {
        let avg_pct = (self.avg_diff / 1_000_000.0) * 100.0; // Convert ns to ms percentage
        let status = if self.meets_target { "✓" } else { "✗" };
        format!(
            "{} {}: avg diff: {:.2}ms, max diff: {:.2}ms",
            status, self.name, self.avg_diff / 1_000_000.0, self.max_diff / 1_000_000.0
        )
    }
}

// Benchmark submodules
pub mod collector_bench;
pub mod processor_bench;
pub mod trainer_bench;

// Integration benchmarks
pub mod integration_bench;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_result() {
        let result = BenchmarkResult {
            name: "test".to_string(),
            iterations: 1000,
            total_duration: Duration::from_secs(1),
            avg_duration: Duration::from_millis(1),
            min_duration: Duration::from_millis(1),
            max_duration: Duration::from_millis(2),
            std_dev: Duration::from_micros(100),
            memory_usage_bytes: 1024,
        };

        assert_eq!(result.name, "test");
        assert_eq!(result.iterations, 1000);
        assert_eq!(result.throughput(), 1000.0);
    }

    #[test]
    fn test_performance_target() {
        let target = PerformanceTarget {
            name: "test".to_string(),
            target_avg: Duration::from_millis(1),
            target_max: Duration::from_millis(2),
            target_min_throughput: Some(1000.0),
        };

        let result = BenchmarkResult {
            name: "test".to_string(),
            iterations: 1000,
            total_duration: Duration::from_secs(1),
            avg_duration: Duration::from_millis(1),
            min_duration: Duration::from_millis(1),
            max_duration: Duration::from_millis(1),
            std_dev: Duration::from_micros(0),
            memory_usage_bytes: 0,
        };

        assert!(target.meets_target(&result));
    }

    #[test]
    fn test_benchmark_suite() {
        let mut suite = BenchmarkSuite::with_defaults();

        suite.run_benchmark("simple", || {
            // Simple operation
            let _ = 1 + 1;
        });

        assert_eq!(suite.results().len(), 1);
    }
}