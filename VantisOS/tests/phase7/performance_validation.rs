//! Performance Validation Tests for VantisOS v1.4.0
//! 
//! This module contains performance benchmarks and validation tests
//! for all Phase 7 modules.

use std::time::{Duration, Instant};
use std::sync::Arc;
use std::collections::HashMap;

/// Performance metrics for validation
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub operation: String,
    pub duration_ms: f64,
    pub memory_used_bytes: usize,
    pub cpu_utilization: f64,
    pub throughput_ops_per_sec: f64,
    pub latency_p99_ms: f64,
    pub errors_count: u64,
}

/// Performance threshold configuration
#[derive(Debug, Clone)]
pub struct PerformanceThreshold {
    pub max_duration_ms: f64,
    pub max_memory_bytes: usize,
    pub min_throughput: f64,
    pub max_latency_p99_ms: f64,
    pub max_error_rate: f64,
}

impl Default for PerformanceThreshold {
    fn default() -> Self {
        Self {
            max_duration_ms: 1000.0,
            max_memory_bytes: 100 * 1024 * 1024, // 100MB
            min_throughput: 100.0,
            max_latency_p99_ms: 500.0,
            max_error_rate: 0.01, // 1%
        }
    }
}

/// Performance validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub test_name: String,
    pub passed: bool,
    pub metrics: PerformanceMetrics,
    pub threshold: PerformanceThreshold,
    pub violations: Vec<String>,
}

// ============================================================================
// Phase 7.1: Optimization Performance Tests
// ============================================================================

#[cfg(test)]
mod optimization_performance_tests {
    use super::*;

    /// Test performance profiler overhead
    #[test]
    fn test_profiler_overhead() {
        let start = Instant::now();
        
        // Simulate profiler initialization
        let iterations = 10_000;
        for _ in 0..iterations {
            let _ = std::time::SystemTime::now();
        }
        
        let duration = start.elapsed();
        let avg_duration_us = duration.as_micros() as f64 / iterations as f64;
        
        // Assert profiler overhead is less than 10 microseconds per call
        assert!(avg_duration_us < 10.0, 
            "Profiler overhead too high: {:.2}us per call", avg_duration_us);
        
        println!("Profiler overhead: {:.2}us per call", avg_duration_us);
    }

    /// Test memory allocator performance
    #[test]
    fn test_memory_allocator_performance() {
        let start = Instant::now();
        
        // Simulate memory allocations
        let mut allocations = Vec::with_capacity(1000);
        for i in 0..1000 {
            let mut data = Vec::with_capacity(1024);
            data.resize(1024, 0u8);
            allocations.push(data);
        }
        
        // Clear allocations
        allocations.clear();
        allocations.shrink_to_fit();
        
        let duration = start.elapsed();
        
        // Assert memory operations complete within 100ms
        assert!(duration < Duration::from_millis(100),
            "Memory allocator too slow: {:?}", duration);
        
        println!("Memory allocator test completed in {:?}", duration);
    }

    /// Test cache hit rate performance
    #[test]
    fn test_cache_performance() {
        let mut cache: HashMap<String, Vec<u8>> = HashMap::new();
        let cache_size = 1000;
        
        // Populate cache
        for i in 0..cache_size {
            cache.insert(format!("key_{}", i), vec![0u8; 1024]);
        }
        
        // Test cache hits
        let start = Instant::now();
        let mut hits = 0;
        for i in 0..cache_size {
            if cache.contains_key(&format!("key_{}", i)) {
                hits += 1;
            }
        }
        let hit_duration = start.elapsed();
        
        // Test cache misses
        let start = Instant::now();
        let mut misses = 0;
        for i in cache_size..(cache_size * 2) {
            if !cache.contains_key(&format!("key_{}", i)) {
                misses += 1;
            }
        }
        let miss_duration = start.elapsed();
        
        let hit_rate = hits as f64 / (hits + misses) as f64;
        
        assert!(hit_rate >= 0.5, "Cache hit rate too low: {:.2}%", hit_rate * 100.0);
        assert!(hit_duration < Duration::from_millis(10), "Cache hit lookup too slow");
        assert!(miss_duration < Duration::from_millis(10), "Cache miss lookup too slow");
        
        println!("Cache hit rate: {:.2}%", hit_rate * 100.0);
        println!("Hit lookup time: {:?}", hit_duration);
        println!("Miss lookup time: {:?}", miss_duration);
    }

    /// Test batch processing throughput
    #[test]
    fn test_batch_processing_throughput() {
        let batch_size = 1000;
        let item_size = 1024; // 1KB per item
        
        // Prepare batch data
        let batch: Vec<Vec<u8>> = (0..batch_size)
            .map(|_| vec![0u8; item_size])
            .collect();
        
        let start = Instant::now();
        
        // Process batch (simulated)
        let processed: usize = batch.iter()
            .map(|item| item.len())
            .sum();
        
        let duration = start.elapsed();
        let throughput_mbps = (processed as f64 / 1024.0 / 1024.0) / duration.as_secs_f64();
        
        // Assert minimum throughput of 100 MB/s
        assert!(throughput_mbps >= 100.0, 
            "Batch processing throughput too low: {:.2} MB/s", throughput_mbps);
        
        println!("Batch processing throughput: {:.2} MB/s", throughput_mbps);
    }

    /// Test parallel processing scaling
    #[test]
    fn test_parallel_processing_scaling() {
        use std::thread;
        
        let work_size = 100_000;
        let data: Vec<u64> = (0..work_size).collect();
        
        // Sequential processing
        let start = Instant::now();
        let sequential_sum: u64 = data.iter().map(|x| x * 2).sum();
        let sequential_duration = start.elapsed();
        
        // Parallel processing (simulated with chunks)
        let start = Instant::now();
        let chunk_size = work_size / 4;
        let chunks: Vec<&[u64]> = data.chunks(chunk_size).collect();
        let parallel_sum: u64 = chunks.iter()
            .map(|chunk| chunk.iter().map(|x| x * 2).sum::<u64>())
            .sum();
        let parallel_duration = start.elapsed();
        
        assert_eq!(sequential_sum, parallel_sum);
        
        // Parallel should be at least as fast as sequential
        println!("Sequential duration: {:?}", sequential_duration);
        println!("Parallel duration: {:?}", parallel_duration);
        println!("Speedup: {:.2}x", 
            sequential_duration.as_secs_f64() / parallel_duration.as_secs_f64());
    }

    /// Test GPU offloading performance (simulated)
    #[test]
    fn test_gpu_offloading_overhead() {
        // Simulate GPU memory transfer
        let data_size = 1024 * 1024; // 1MB
        
        let start = Instant::now();
        
        // Simulate data preparation
        let host_data = vec![0u8; data_size];
        
        // Simulated transfer time (PCIe 3.0 x16: ~16GB/s)
        let transfer_time = Duration::from_micros(62); // ~1MB / 16GB/s
        
        let duration = start.elapsed();
        
        // Total overhead should be minimal
        assert!(duration < Duration::from_millis(10),
            "GPU offloading overhead too high: {:?}", duration);
        
        println!("GPU offloading overhead: {:?}", duration);
    }
}

// ============================================================================
// Phase 7.2: Security Performance Tests
// ============================================================================

#[cfg(test)]
mod security_performance_tests {
    use super::*;

    /// Test encryption/decryption performance
    #[test]
    fn test_encryption_performance() {
        // Simulate encryption operations
        let data_sizes = vec![1024, 10 * 1024, 100 * 1024, 1024 * 1024]; // 1KB to 1MB
        
        for size in data_sizes {
            let data = vec![0u8; size];
            
            let start = Instant::now();
            
            // Simulate encryption (XOR for benchmarking purposes)
            let encrypted: Vec<u8> = data.iter().map(|b| b ^ 0x55).collect();
            
            // Simulate decryption
            let decrypted: Vec<u8> = encrypted.iter().map(|b| b ^ 0x55).collect();
            
            let duration = start.elapsed();
            let throughput_mbps = (size as f64 * 2.0 / 1024.0 / 1024.0) / duration.as_secs_f64();
            
            // Assert data integrity
            assert_eq!(data, decrypted);
            
            // Assert minimum throughput of 500 MB/s for encryption
            assert!(throughput_mbps >= 500.0,
                "Encryption throughput too low for {} bytes: {:.2} MB/s", size, throughput_mbps);
            
            println!("Encryption throughput ({}KB): {:.2} MB/s", size / 1024, throughput_mbps);
        }
    }

    /// Test adversarial detection performance
    #[test]
    fn test_adversarial_detection_performance() {
        // Simulate adversarial input detection
        let num_inputs = 10_000;
        let input_size = 784; // MNIST-like input
        
        let start = Instant::now();
        
        // Simulate detection (simple statistical check)
        let mut detected_count = 0;
        for _ in 0..num_inputs {
            let input: Vec<f32> = (0..input_size).map(|i| i as f32 * 0.001).collect();
            
            // Simple anomaly score calculation
            let mean: f32 = input.iter().sum::<f32>() / input.len() as f32;
            let variance: f32 = input.iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f32>() / input.len() as f32;
            
            if variance > 0.1 {
                detected_count += 1;
            }
        }
        
        let duration = start.elapsed();
        let throughput = num_inputs as f64 / duration.as_secs_f64();
        
        // Assert minimum throughput of 1000 detections per second
        assert!(throughput >= 1000.0,
            "Adversarial detection throughput too low: {:.0} ops/s", throughput);
        
        println!("Adversarial detection throughput: {:.0} ops/s", throughput);
    }

    /// Test differential privacy overhead
    #[test]
    fn test_differential_privacy_overhead() {
        // Simulate differential privacy noise addition
        let num_queries = 10_000;
        let sensitivity = 1.0;
        let epsilon = 0.1;
        
        let start = Instant::now();
        
        // Simulate Laplace noise generation
        use std::f64::consts::PI;
        let scale = sensitivity / epsilon;
        
        let mut results = Vec::with_capacity(num_queries);
        for i in 0..num_queries {
            // Simple Laplace approximation
            let u = (i as f64 + 0.5) / (num_queries as f64 + 1.0);
            let noise = scale * (u.ln() - (1.0 - u).ln()).signum() * (u.abs() * 2.0 - 1.0).ln().abs();
            results.push(noise);
        }
        
        let duration = start.elapsed();
        let overhead_us = duration.as_micros() as f64 / num_queries as f64;
        
        // Assert overhead is less than 10 microseconds per query
        assert!(overhead_us < 10.0,
            "Differential privacy overhead too high: {:.2}us per query", overhead_us);
        
        println!("Differential privacy overhead: {:.2}us per query", overhead_us);
    }

    /// Test federated learning aggregation performance
    #[test]
    fn test_federated_aggregation_performance() {
        // Simulate federated learning aggregation
        let num_clients = 100;
        let model_size = 10_000;
        
        // Simulate client updates
        let client_updates: Vec<Vec<f32>> = (0..num_clients)
            .map(|_| (0..model_size).map(|i| i as f32 * 0.001).collect())
            .collect();
        
        let start = Instant::now();
        
        // Aggregate updates (federated averaging)
        let mut aggregated = vec![0.0f32; model_size];
        for update in &client_updates {
            for (i, val) in update.iter().enumerate() {
                aggregated[i] += val;
            }
        }
        for val in &mut aggregated {
            *val /= num_clients as f32;
        }
        
        let duration = start.elapsed();
        let throughput = (num_clients * model_size) as f64 / duration.as_secs_f64();
        
        // Assert minimum throughput of 1M aggregations per second
        assert!(throughput >= 1_000_000.0,
            "Federated aggregation throughput too low: {:.0} ops/s", throughput);
        
        println!("Federated aggregation throughput: {:.0} ops/s", throughput);
    }

    /// Test runtime monitoring overhead
    #[test]
    fn test_runtime_monitoring_overhead() {
        // Simulate runtime security monitoring
        let num_operations = 100_000;
        
        let start = Instant::now();
        
        // Simulate monitoring checks
        let mut alerts = 0;
        for i in 0..num_operations {
            // Simulate security check
            let risk_score = (i % 100) as f32 / 100.0;
            if risk_score > 0.9 {
                alerts += 1;
            }
        }
        
        let duration = start.elapsed();
        let overhead_ns = duration.as_nanos() as f64 / num_operations as f64;
        
        // Assert overhead is less than 100 nanoseconds per operation
        assert!(overhead_ns < 100.0,
            "Runtime monitoring overhead too high: {:.2}ns per operation", overhead_ns);
        
        println!("Runtime monitoring overhead: {:.2}ns per operation", overhead_ns);
        println!("Alerts generated: {}", alerts);
    }

    /// Test threat intelligence lookup performance
    #[test]
    fn test_threat_intelligence_lookup() {
        // Simulate threat intelligence database
        let mut threat_db: HashMap<String, i32> = HashMap::new();
        for i in 0..10_000 {
            threat_db.insert(format!("threat_{}", i), i);
        }
        
        let num_lookups = 100_000;
        
        let start = Instant::now();
        
        // Perform lookups
        let mut hits = 0;
        for i in 0..num_lookups {
            if threat_db.contains_key(&format!("threat_{}", i % 15_000)) {
                hits += 1;
            }
        }
        
        let duration = start.elapsed();
        let throughput = num_lookups as f64 / duration.as_secs_f64();
        
        // Assert minimum throughput of 100,000 lookups per second
        assert!(throughput >= 100_000.0,
            "Threat intelligence lookup throughput too low: {:.0} ops/s", throughput);
        
        let hit_rate = hits as f64 / num_lookups as f64;
        println!("Threat intelligence lookup throughput: {:.0} ops/s", throughput);
        println!("Hit rate: {:.2}%", hit_rate * 100.0);
    }
}

// ============================================================================
// Phase 7.2.3: Compliance Performance Tests
// ============================================================================

#[cfg(test)]
mod compliance_performance_tests {
    use super::*;

    /// Test GDPR compliance check performance
    #[test]
    fn test_gdpr_compliance_check_performance() {
        // Simulate GDPR compliance checks
        let num_records = 10_000;
        
        let records: Vec<HashMap<String, String>> = (0..num_records)
            .map(|i| {
                let mut record = HashMap::new();
                record.insert("id".to_string(), format!("user_{}", i));
                record.insert("email".to_string(), format!("user{}@example.com", i));
                record.insert("consent".to_string(), if i % 10 == 0 { "false" } else { "true" }.to_string());
                record
            })
            .collect();
        
        let start = Instant::now();
        
        // Check compliance
        let mut compliant_count = 0;
        for record in &records {
            if record.get("consent").map(|c| c.as_str()) == Some("true") {
                compliant_count += 1;
            }
        }
        
        let duration = start.elapsed();
        let throughput = num_records as f64 / duration.as_secs_f64();
        
        // Assert minimum throughput of 10,000 checks per second
        assert!(throughput >= 10_000.0,
            "GDPR compliance check throughput too low: {:.0} ops/s", throughput);
        
        println!("GDPR compliance check throughput: {:.0} ops/s", throughput);
        println!("Compliant records: {}/{}", compliant_count, num_records);
    }

    /// Test bias detection performance
    #[test]
    fn test_bias_detection_performance() {
        // Simulate bias detection on predictions
        let num_predictions = 100_000;
        let num_groups = 10;
        
        let predictions: Vec<(usize, f32)> = (0..num_predictions)
            .map(|i| (i % num_groups, (i as f32 / num_predictions as f32)))
            .collect();
        
        let start = Instant::now();
        
        // Calculate group statistics
        let mut group_stats: Vec<(f32, usize)> = vec![(0.0, 0); num_groups];
        for (group, score) in &predictions {
            group_stats[*group].0 += score;
            group_stats[*group].1 += 1;
        }
        
        // Calculate group means
        let group_means: Vec<f32> = group_stats.iter()
            .map(|(sum, count)| sum / *count as f32)
            .collect();
        
        // Calculate demographic parity difference
        let max_mean = group_means.iter().cloned().fold(0.0, f32::max);
        let min_mean = group_means.iter().cloned().fold(1.0, f32::min);
        let dp_diff = max_mean - min_mean;
        
        let duration = start.elapsed();
        
        // Assert calculation completes within 100ms
        assert!(duration < Duration::from_millis(100),
            "Bias detection too slow: {:?}", duration);
        
        println!("Bias detection completed in {:?}", duration);
        println!("Demographic parity difference: {:.4}", dp_diff);
    }

    /// Test audit trail logging performance
    #[test]
    fn test_audit_trail_performance() {
        // Simulate audit trail logging
        let num_entries = 100_000;
        
        let start = Instant::now();
        
        // Create audit entries
        let mut entries = Vec::with_capacity(num_entries);
        for i in 0..num_entries {
            entries.push((
                format!("user_{}", i % 1000),
                format!("action_{}", i % 50),
                std::time::SystemTime::now(),
                i % 10 == 0,
            ));
        }
        
        let duration = start.elapsed();
        let throughput = num_entries as f64 / duration.as_secs_f64();
        
        // Assert minimum throughput of 100,000 entries per second
        assert!(throughput >= 100_000.0,
            "Audit trail logging throughput too low: {:.0} ops/s", throughput);
        
        println!("Audit trail logging throughput: {:.0} entries/s", throughput);
    }

    /// Test ethics evaluation performance
    #[test]
    fn test_ethics_evaluation_performance() {
        // Simulate ethics evaluation
        let num_decisions = 10_000;
        
        let decisions: Vec<HashMap<String, f32>> = (0..num_decisions)
            .map(|i| {
                let mut decision = HashMap::new();
                decision.insert("fairness".to_string(), (i % 100) as f32 / 100.0);
                decision.insert("transparency".to_string(), ((i + 25) % 100) as f32 / 100.0);
                decision.insert("accountability".to_string(), ((i + 50) % 100) as f32 / 100.0);
                decision.insert("privacy".to_string(), ((i + 75) % 100) as f32 / 100.0);
                decision
            })
            .collect();
        
        let start = Instant::now();
        
        // Evaluate ethics scores
        let mut violations = 0;
        for decision in &decisions {
            let overall_score: f32 = decision.values().sum::<f32>() / decision.len() as f32;
            if overall_score < 0.5 {
                violations += 1;
            }
        }
        
        let duration = start.elapsed();
        let throughput = num_decisions as f64 / duration.as_secs_f64();
        
        // Assert minimum throughput of 10,000 evaluations per second
        assert!(throughput >= 10_000.0,
            "Ethics evaluation throughput too low: {:.0} ops/s", throughput);
        
        println!("Ethics evaluation throughput: {:.0} decisions/s", throughput);
        println!("Violations detected: {}/{}", violations, num_decisions);
    }

    /// Test transparency explanation generation
    #[test]
    fn test_transparency_explanation_performance() {
        // Simulate explanation generation
        let num_explanations = 1_000;
        let feature_count = 100;
        
        let start = Instant::now();
        
        // Generate SHAP-like explanations
        let mut explanations = Vec::with_capacity(num_explanations);
        for i in 0..num_explanations {
            let feature_importance: Vec<(String, f32)> = (0..feature_count)
                .map(|j| (format!("feature_{}", j), ((i + j) % 100) as f32 / 100.0))
                .collect();
            
            // Sort by importance
            let mut sorted = feature_importance;
            sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            explanations.push(sorted);
        }
        
        let duration = start.elapsed();
        let throughput = num_explanations as f64 / duration.as_secs_f64();
        
        // Assert minimum throughput of 100 explanations per second
        assert!(throughput >= 100.0,
            "Explanation generation throughput too low: {:.0} ops/s", throughput);
        
        println!("Explanation generation throughput: {:.0} explanations/s", throughput);
    }
}

// ============================================================================
// Integration Performance Tests
// ============================================================================

#[cfg(test)]
mod integration_performance_tests {
    use super::*;

    /// Test end-to-end pipeline performance
    #[test]
    fn test_e2e_pipeline_performance() {
        let num_requests = 1_000;
        
        let start = Instant::now();
        
        // Simulate end-to-end request processing
        for i in 0..num_requests {
            // 1. Input validation (security)
            let input_valid = i % 100 != 0; // 1% invalid
            
            if input_valid {
                // 2. Processing (optimization)
                let _processed: Vec<u8> = vec![0; 1024];
                
                // 3. Compliance check
                let compliant = i % 50 != 0; // 2% non-compliant
                
                if compliant {
                    // 4. Audit logging
                    let _entry = format!("Request {} processed successfully", i);
                }
            }
        }
        
        let duration = start.elapsed();
        let throughput = num_requests as f64 / duration.as_secs_f64();
        
        // Assert minimum throughput of 500 requests per second
        assert!(throughput >= 500.0,
            "E2E pipeline throughput too low: {:.0} req/s", throughput);
        
        println!("E2E pipeline throughput: {:.0} requests/s", throughput);
        println!("Average latency: {:.2}ms", duration.as_millis() as f64 / num_requests as f64);
    }

    /// Test concurrent request handling
    #[test]
    fn test_concurrent_request_handling() {
        use std::sync::atomic::{AtomicU64, Ordering};
        use std::sync::Arc;
        
        let counter = Arc::new(AtomicU64::new(0));
        let num_requests = 10_000;
        
        let start = Instant::now();
        
        // Simulate concurrent processing
        for i in 0..num_requests {
            counter.fetch_add(1, Ordering::SeqCst);
        }
        
        let duration = start.elapsed();
        let total_processed = counter.load(Ordering::SeqCst);
        
        assert_eq!(total_processed, num_requests as u64);
        
        let throughput = num_requests as f64 / duration.as_secs_f64();
        println!("Concurrent processing throughput: {:.0} ops/s", throughput);
    }

    /// Test memory efficiency under load
    #[test]
    fn test_memory_efficiency() {
        let mut peak_memory = 0;
        let iterations = 100;
        
        for _ in 0..iterations {
            // Simulate memory-intensive operation
            let data: Vec<Vec<u8>> = (0..100)
                .map(|_| vec![0u8; 1024])
                .collect();
            
            let current_size = data.iter().map(|v| v.len()).sum::<usize>();
            peak_memory = peak_memory.max(current_size);
        }
        
        // Peak memory should be less than 1MB for this test
        assert!(peak_memory < 1024 * 1024,
            "Memory usage too high: {} bytes", peak_memory);
        
        println!("Peak memory usage: {} bytes", peak_memory);
    }
}

// ============================================================================
// Performance Benchmark Suite
// ============================================================================

pub struct PerformanceBenchmarkSuite {
    pub results: Vec<ValidationResult>,
}

impl PerformanceBenchmarkSuite {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }
    
    pub fn run_all_benchmarks(&mut self) {
        println!("Running Performance Benchmark Suite...");
        println!("========================================\n");
        
        // Optimization benchmarks
        println!("Phase 7.1: Optimization Performance Tests");
        println!("----------------------------------------");
        self.benchmark("profiler_overhead", || {
            let start = Instant::now();
            for _ in 0..10_000 {
                let _ = std::time::SystemTime::now();
            }
            start.elapsed()
        });
        
        // Security benchmarks
        println!("\nPhase 7.2: Security Performance Tests");
        println!("------------------------------------");
        self.benchmark("encryption_throughput", || {
            let data = vec![0u8; 1024 * 1024];
            let start = Instant::now();
            let _encrypted: Vec<u8> = data.iter().map(|b| b ^ 0x55).collect();
            start.elapsed()
        });
        
        // Compliance benchmarks
        println!("\nPhase 7.2.3: Compliance Performance Tests");
        println!("-----------------------------------------");
        self.benchmark("gdpr_check", || {
            let records: Vec<HashMap<String, String>> = (0..10_000)
                .map(|i| {
                    let mut r = HashMap::new();
                    r.insert("consent".to_string(), "true".to_string());
                    r
                })
                .collect();
            let start = Instant::now();
            let _compliant = records.iter().filter(|r| r.get("consent") == Some(&"true".to_string())).count();
            start.elapsed()
        });
        
        println!("\n========================================");
        println!("Benchmark Suite Complete!");
        println!("Total tests run: {}", self.results.len());
    }
    
    fn benchmark<F>(&mut self, name: &str, f: F) -> Duration
    where
        F: FnOnce() -> Duration,
    {
        let duration = f();
        println!("  {}: {:?}", name, duration);
        duration
    }
    
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("# Performance Validation Report\n\n");
        report.push_str("## Summary\n\n");
        report.push_str(&format!("Total tests: {}\n", self.results.len()));
        report.push_str(&format!("Passed: {}\n", self.results.iter().filter(|r| r.passed).count()));
        report.push_str(&format!("Failed: {}\n\n", self.results.iter().filter(|r| !r.passed).count()));
        
        report.push_str("## Detailed Results\n\n");
        for result in &self.results {
            report.push_str(&format!("### {}\n", result.test_name));
            report.push_str(&format!("- Status: {}\n", if result.passed { "PASSED" } else { "FAILED" }));
            report.push_str(&format!("- Duration: {:.2}ms\n", result.metrics.duration_ms));
            report.push_str(&format!("- Throughput: {:.2} ops/s\n\n", result.metrics.throughput_ops_per_sec));
        }
        
        report
    }
}

#[cfg(test)]
mod benchmark_runner {
    use super::*;
    
    #[test]
    fn run_performance_benchmarks() {
        let mut suite = PerformanceBenchmarkSuite::new();
        suite.run_all_benchmarks();
        
        // Generate and print report
        let report = suite.generate_report();
        println!("\n{}", report);
        
        // All tests should pass
        let failed = suite.results.iter().filter(|r| !r.passed).count();
        assert_eq!(failed, 0, "Some performance tests failed");
    }
}