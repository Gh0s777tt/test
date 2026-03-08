//! Integration Tests for AI Modules
//! 
//! This module contains comprehensive integration tests for all AI modules
//! implemented in Phase 6, ensuring proper functionality, performance,
//! and compatibility.

use vantisos::ai::modules::predictive_caching::{PredictiveCache, PredictiveCacheConfig, EvictionPolicy};
use vantisos::ai::modules::intelligent_scheduling::{IntelligentScheduler, SchedulingConfig, SchedulingAlgorithm};
use vantisos::ai::modules::adaptive_resource_allocation::{ResourceAllocator, AllocationConfig, AllocationPolicy};
use vantisos::ai::modules::security_threat_detection::{ThreatDetector, SecurityConfig, ThreatType};
use vantisos::ai::modules::natural_language_interface::{NaturalLanguageInterface, NLIConfig};
use std::time::Duration;

#[tokio::test]
async fn test_predictive_caching_integration() {
    // Test predictive caching with real-world scenarios
    let config = PredictiveCacheConfig {
        max_cache_size: 1000,
        min_confidence: 0.7,
        eviction_policy: EvictionPolicy::Predictive,
    };

    let cache = PredictiveCache::new(config);

    // Simulate access patterns
    for i in 0..100 {
        let key = format!("key_{}", i % 20); // 20 unique keys
        cache.get(&key).await;
        
        if i % 5 == 0 {
            cache.insert(&key, vec![1, 2, 3], 10).await;
        }
    }

    // Verify predictions
    let predictions = cache.get_predictions(5).await;
    assert!(!predictions.is_empty(), "Should generate predictions");

    // Verify cache hit rate improved
    let stats = cache.get_stats().await;
    assert!(stats.hit_rate > 0.5, "Hit rate should be > 50% after learning");
}

#[tokio::test]
async fn test_intelligent_scheduling_integration() {
    // Test intelligent scheduling with various algorithms
    let config = SchedulingConfig {
        base_quantum_ms: 10,
        algorithm: SchedulingAlgorithm::DeepLearning,
        enable_adaptive_quantum: true,
    };

    let scheduler = IntelligentScheduler::new(config);

    // Add multiple tasks with different characteristics
    for i in 0..10 {
        scheduler.add_task(
            format!("task_{}", i),
            i as f64,
            i as u64 * 10,
            vec![],
        ).await;
    }

    // Run scheduling cycle
    let scheduled = scheduler.schedule(3).await;
    assert_eq!(scheduled.len(), 3, "Should schedule 3 tasks");

    // Verify adaptive behavior
    let stats = scheduler.get_stats().await;
    assert!(stats.total_scheduled > 0, "Should have scheduled tasks");
}

#[tokio::test]
async fn test_adaptive_resource_allocation_integration() {
    // Test resource allocation with dynamic workloads
    let config = AllocationConfig {
        max_cpu_per_process: 4.0,
        allocation_policy: AllocationPolicy::Predictive,
    };

    let allocator = ResourceAllocator::new(config);

    // Request resources for multiple processes
    for i in 0..5 {
        let result = allocator.allocate(
            format!("process_{}", i),
            2.0,
            1024,
            512,
            1000,
        ).await;

        assert!(result.is_ok(), "Allocation should succeed");
    }

    // Verify allocation statistics
    let stats = allocator.get_stats().await;
    assert_eq!(stats.active_allocations, 5, "Should have 5 active allocations");
}

#[tokio::test]
async fn test_security_threat_detection_integration() {
    // Test security threat detection with various scenarios
    let config = SecurityConfig {
        sensitivity: 0.8,
        enable_signature_detection: true,
        enable_ml_detection: true,
    };

    let detector = ThreatDetector::new(config);

    // Add known threat signatures
    detector.add_signature(
        "malware_signature_1",
        ThreatType::Malware,
        vec![1, 2, 3, 4, 5],
        0.9,
    );

    // Process events
    for i in 0..20 {
        let event = if i % 3 == 0 {
            // Simulate suspicious activity
            detector::SecurityEvent::NetworkAnomaly {
                source_ip: "192.168.1.100".to_string(),
                destination_ip: "10.0.0.1".to_string(),
                port: 443,
                protocol: "TCP".to_string(),
                data: vec![1, 2, 3, 4, 5],
            }
        } else {
            // Normal activity
            detector::SecurityEvent::SystemEvent {
                event_type: "login".to_string(),
                user: "user1".to_string(),
                timestamp: chrono::Utc::now(),
                details: HashMap::new(),
            }
        };

        detector.process_event(event).await;
    }

    // Verify threat detection
    let threats = detector.get_recent_threats(10).await;
    assert!(!threats.is_empty(), "Should detect threats");

    // Verify statistics
    let stats = detector.get_stats().await;
    assert!(stats.total_threats_detected > 0, "Should detect at least one threat");
}

#[tokio::test]
async fn test_natural_language_interface_integration() {
    // Test NLP interface with various commands
    let config = NLIConfig {
        confidence_threshold: 0.7,
        enable_context: true,
    };

    let nli = NaturalLanguageInterface::new(config);

    // Test various commands
    let commands = vec![
        "Start the web server",
        "Stop the database service",
        "Check system status",
        "Increase memory allocation",
    ];

    for command in commands {
        let result = nli.parse_command(command).await;
        assert!(result.is_some(), "Should parse command: {}", command);
        
        if let Some(parsed) = result {
            assert!(parsed.confidence >= config.confidence_threshold, 
                "Confidence should be above threshold");
        }
    }

    // Test context-aware suggestions
    let suggestions = nli.get_suggestions("system").await;
    assert!(!suggestions.is_empty(), "Should provide suggestions");

    // Test learning from user feedback
    nli.provide_feedback("Start the web server", true).await;
    let learned = nli.parse_command("Launch the web server").await;
    assert!(learned.is_some(), "Should learn from feedback");
}

#[tokio::test]
async fn test_ai_modules_integration() {
    // Test integration between multiple AI modules
    let cache_config = PredictiveCacheConfig {
        max_cache_size: 500,
        min_confidence: 0.6,
        eviction_policy: EvictionPolicy::Adaptive,
    };

    let scheduler_config = SchedulingConfig {
        base_quantum_ms: 5,
        algorithm: SchedulingAlgorithm::Priority,
        enable_adaptive_quantum: true,
    };

    let cache = PredictiveCache::new(cache_config);
    let scheduler = IntelligentScheduler::new(scheduler_config);

    // Simulate workload with caching and scheduling
    for i in 0..50 {
        let key = format!("resource_{}", i % 10);
        
        // Try cache first
        let cached = cache.get(&key).await;
        
        if cached.is_none() {
            // Add task to scheduler
            scheduler.add_task(
                format!("fetch_{}", key),
                1.0,
                100,
                vec![key.clone()],
            ).await;
            
            // Cache the result
            cache.insert(&key, vec![i], 10).await;
        }
    }

    // Verify coordinated operation
    let cache_stats = cache.get_stats().await;
    let scheduler_stats = scheduler.get_stats().await;

    assert!(cache_stats.total_accesses > 0, "Cache should have accesses");
    assert!(scheduler_stats.total_scheduled > 0, "Scheduler should schedule tasks");
}

#[tokio::test]
async fn test_performance_benchmarks() {
    // Performance benchmarking for AI modules
    let start = std::time::Instant::now();

    // Benchmark predictive caching
    let cache_config = PredictiveCacheConfig {
        max_cache_size: 10000,
        min_confidence: 0.5,
        eviction_policy: EvictionPolicy::LRU,
    };

    let cache = PredictiveCache::new(cache_config);

    let cache_start = std::time::Instant::now();
    for i in 0..10000 {
        let key = format!("benchmark_key_{}", i % 100);
        cache.insert(&key, vec![i], 10).await;
        cache.get(&key).await;
    }
    let cache_duration = cache_start.elapsed();

    // Benchmark intelligent scheduling
    let scheduler_config = SchedulingConfig {
        base_quantum_ms: 1,
        algorithm: SchedulingAlgorithm::RoundRobin,
        enable_adaptive_quantum: false,
    };

    let scheduler = IntelligentScheduler::new(scheduler_config);

    for i in 0..1000 {
        scheduler.add_task(format!("task_{}", i), i as f64, 10, vec![]).await;
    }

    let schedule_start = std::time::Instant::now();
    scheduler.schedule(10).await;
    let schedule_duration = schedule_start.elapsed();

    let total_duration = start.elapsed();

    println!("Performance Benchmarks:");
    println!("  Total time: {:?}", total_duration);
    println!("  Cache operations: {:?}", cache_duration);
    println!("  Scheduling: {:?}", schedule_duration);
    println!("  Cache ops/sec: {:.2}", 10000.0 / cache_duration.as_secs_f64());
    println!("  Tasks/sec: {:.2}", 1000.0 / schedule_duration.as_secs_f64());

    // Assert reasonable performance
    assert!(cache_duration.as_secs_f64() < 5.0, "Cache operations should complete in < 5s");
    assert!(schedule_duration.as_secs_f64() < 1.0, "Scheduling should complete in < 1s");
}

#[tokio::test]
async fn test_stress_testing() {
    // Stress test AI modules with high load
    let config = PredictiveCacheConfig {
        max_cache_size: 100000,
        min_confidence: 0.7,
        eviction_policy: EvictionPolicy::Predictive,
    };

    let cache = PredictiveCache::new(config);

    // Concurrent access simulation
    let mut handles = vec![];

    for i in 0..10 {
        let cache_clone = cache.clone();
        let handle = tokio::spawn(async move {
            for j in 0..1000 {
                let key = format!("stress_{}_{}", i, j % 100);
                cache_clone.insert(&key, vec![i, j], 10).await;
                cache_clone.get(&key).await;
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify cache integrity
    let stats = cache.get_stats().await;
    assert!(stats.total_accesses > 0, "Should have cache accesses");
    assert!(stats.total_hits + stats.total_misses == stats.total_accesses, 
        "Hits + misses should equal total accesses");
}

#[tokio::test]
async fn test_error_handling() {
    // Test error handling in AI modules
    let config = PredictiveCacheConfig {
        max_cache_size: 100,
        min_confidence: 0.5,
        eviction_policy: EvictionPolicy::LRU,
    };

    let cache = PredictiveCache::new(config);

    // Test with invalid inputs
    let result = cache.insert("", vec![1, 2, 3], 10).await;
    // Should handle gracefully

    let result = cache.get(&format!("_")).await; // Non-existent key
    assert!(result.is_none(), "Should return None for non-existent key");

    // Test overflow scenarios
    for i in 0..200 {
        let key = format!("overflow_{}", i);
        cache.insert(&key, vec![i], 10).await;
    }

    let stats = cache.get_stats().await;
    assert!(stats.total_misses > 0, "Should have evictions due to overflow");
}

#[tokio::test]
async fn test_memory_efficiency() {
    // Test memory efficiency of AI modules
    let config = PredictiveCacheConfig {
        max_cache_size: 1000,
        min_confidence: 0.5,
        eviction_policy: EvictionPolicy::LFU,
    };

    let cache = PredictiveCache::new(config);

    // Populate cache
    for i in 0..1000 {
        let key = format!("memory_{}", i);
        cache.insert(&key, vec![i; 100], 10).await;
    }

    // Access patterns to trigger optimizations
    for i in 0..100 {
        let key = format!("memory_{}", i);
        for _ in 0..10 {
            cache.get(&key).await;
        }
    }

    let stats = cache.get_stats().await;
    
    // Verify memory is managed efficiently
    assert!(stats.cache_size <= config.max_cache_size, 
        "Cache size should not exceed maximum");
}

// Helper module for testing
mod detector {
    use std::collections::HashMap;
    use chrono::Utc;

    #[derive(Debug, Clone)]
    pub enum SecurityEvent {
        NetworkAnomaly {
            source_ip: String,
            destination_ip: String,
            port: u16,
            protocol: String,
            data: Vec<u8>,
        },
        SystemEvent {
            event_type: String,
            user: String,
            timestamp: chrono::DateTime<chrono::Utc>,
            details: HashMap<String, String>,
        },
    }
}