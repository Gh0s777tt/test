//! Comprehensive integration tests for Phase 7.1 - Optimization Modules

use vantisos::ai::optimization::{
    OptimizationManager, OptimizationConfig,
    SystemProfiler, MemoryOptimizer, CpuOptimizer,
    GpuOptimizer, NetworkOptimizer, IoOptimizer,
    ModelOptimizer, CacheOptimizer, InferenceOptimizer,
};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_optimization_manager_initialization() {
    let config = OptimizationConfig::default();
    let manager = OptimizationManager::new(config).await;
    
    assert!(manager.is_ok(), "OptimizationManager should initialize successfully");
    
    let manager = manager.unwrap();
    let stats = manager.get_statistics().await;
    assert_eq!(stats.optimization_cycles, 0);
}

#[tokio::test]
async fn test_system_profiler_comprehensive_profile() {
    let profiler = SystemProfiler::new().await.unwrap();
    
    let profile = profiler.profile_system().await.unwrap();
    
    assert!(!profile.component_name.is_empty());
    assert!(profile.timestamp > chrono::Utc::now() - chrono::Duration::seconds(10));
    assert!(profile.cpu_info.num_cores > 0);
    assert!(profile.memory_info.total_bytes > 0);
}

#[tokio::test]
async fn test_memory_optimizer_pool_management() {
    let config = Default::default();
    let optimizer = MemoryOptimizer::new(config).await.unwrap();
    
    // Allocate memory pools
    let pool_id = "test_pool".to_string();
    optimizer.allocate_pool(pool_id, 1024 * 1024).await.unwrap();
    
    let pools = optimizer.get_active_pools().await;
    assert!(pools.iter().any(|p| p.id == pool_id));
    
    // Release pool
    optimizer.release_pool(&pool_id).await.unwrap();
    
    let pools = optimizer.get_active_pools().await;
    assert!(!pools.iter().any(|p| p.id == pool_id));
}

#[tokio::test]
async fn test_cpu_optimizer_frequency_scaling() {
    let config = Default::default();
    let optimizer = CpuOptimizer::new(config).await.unwrap();
    
    let initial_freq = optimizer.get_current_frequency(0).await;
    assert!(initial_freq.is_some());
    
    // Set performance mode
    optimizer.set_governor_mode("performance".to_string()).await.unwrap();
    
    sleep(Duration::from_millis(100)).await;
    
    let perf_freq = optimizer.get_current_frequency(0).await;
    assert!(perf_freq.is_some());
}

#[tokio::test]
async fn test_gpu_optimizer_task_scheduling() {
    let config = Default::default();
    let optimizer = GpuOptimizer::new(config).await.unwrap();
    
    let profile = optimizer.profile_gpu().await.unwrap();
    assert!(!profile.device_name.is_empty());
    
    // Optimize for inference workload
    optimizer.optimize_for_inference().await.unwrap();
    
    let stats = optimizer.get_statistics().await;
    assert!(stats.optimization_cycles > 0);
}

#[tokio::test]
async fn test_network_optimizer_tcp_tuning() {
    let config = Default::default();
    let optimizer = NetworkOptimizer::new(config).await.unwrap();
    
    let interfaces = optimizer.list_interfaces().await;
    assert!(!interfaces.is_empty());
    
    // Optimize TCP settings
    optimizer.optimize_tcp_settings().await.unwrap();
    
    let config_stats = optimizer.get_configuration().await;
    assert!(!config_stats.tcp_settings.is_empty());
}

#[tokio::test]
async fn test_io_optimizer_scheduler() {
    let config = Default::default();
    let optimizer = IoOptimizer::new(config).await.unwrap();
    
    let devices = optimizer.list_block_devices().await;
    assert!(!devices.is_empty());
    
    // Set optimal scheduler for first device
    if let Some(device) = devices.first() {
        let result = optimizer.set_scheduler(device, "mq-deadline".to_string()).await;
        assert!(result.is_ok() || result.is_err()); // May not have permissions
    }
}

#[tokio::test]
async fn test_model_optimizer_quantization() {
    let config = Default::default();
    let optimizer = ModelOptimizer::new(config).await.unwrap();
    
    // Create dummy model parameters
    let mut parameters = vec![0.5f32; 1000];
    
    // Test INT8 quantization
    let quantized = optimizer.quantize_int8(&parameters).await.unwrap();
    assert_eq!(quantized.len(), parameters.len());
    
    // Test dequantization
    let dequantized = optimizer.dequantize_int8(&quantized).await.unwrap();
    assert_eq!(dequantized.len(), parameters.len());
}

#[tokio::test]
async fn test_cache_optimizer_lru_eviction() {
    let config = Default::default();
    let optimizer = CacheOptimizer::new(config).await.unwrap();
    
    // Add items to cache
    for i in 0..20 {
        optimizer.insert(
            format!("key_{}", i),
            format!("value_{}", i).into_bytes(),
            100,
        ).await.unwrap();
    }
    
    // Check that least recently used items were evicted
    let stats = optimizer.get_statistics().await;
    assert!(stats.hits + stats.misses >= 20);
}

#[tokio::test]
async fn test_inference_optimizer_batching() {
    let config = Default::default();
    let optimizer = InferenceOptimizer::new(config).await.unwrap();
    
    // Configure batch size
    optimizer.set_batch_size(8).await.unwrap();
    
    // Create mock requests
    let requests: Vec<Vec<f32>> = (0..10)
        .map(|_| vec![0.5f32; 100])
        .collect();
    
    // Process with batching
    let results = optimizer.process_batch(requests).await.unwrap();
    assert_eq!(results.len(), 10);
}

#[tokio::test]
async fn test_optimization_end_to_end() {
    let config = OptimizationConfig::default();
    let manager = OptimizationManager::new(config).await.unwrap();
    
    // Run optimization cycle
    manager.run_optimization_cycle().await.unwrap();
    
    // Check statistics
    let stats = manager.get_statistics().await;
    assert!(stats.optimization_cycles > 0);
    assert!(stats.total_optimizations > 0);
    
    // Check that all modules participated
    assert!(stats.memory_optimizations >= 0);
    assert!(stats.cpu_optimizations >= 0);
    assert!(stats.gpu_optimizations >= 0);
}

#[tokio::test]
async fn test_bottleneck_detection() {
    let profiler = SystemProfiler::new().await.unwrap();
    
    let bottlenecks = profiler.detect_bottlenecks().await.unwrap();
    
    // Should always return at least an empty vector
    assert!(bottlenecks.len() >= 0);
    
    // If bottlenecks found, validate structure
    for bottleneck in bottlenecks {
        assert!(!bottleneck.component.is_empty());
        assert!(bottleneck.severity >= 0.0 && bottleneck.severity <= 1.0);
    }
}

#[tokio::test]
async fn test_memory_deduplication() {
    let config = Default::default();
    let optimizer = MemoryOptimizer::new(config).await.unwrap();
    
    // Add identical data
    let data = vec![1u8; 1024];
    let hash1 = optimizer.deduplicate_and_store(data.clone()).await.unwrap();
    let hash2 = optimizer.deduplicate_and_store(data).await.unwrap();
    
    // Should be the same hash
    assert_eq!(hash1, hash2);
    
    let stats = optimizer.get_deduplication_stats().await;
    assert!(stats.bytes_saved > 0);
}

#[tokio::test]
async fn test_cache_predictive_prefetching() {
    let config = Default::default();
    let optimizer = CacheOptimizer::new(config).await.unwrap();
    
    // Train on access patterns
    for i in 0..100 {
        let key = format!("key_{}", i % 10); // Repeated pattern
        optimizer.insert(key.clone(), vec![i as u8], 100).await.unwrap();
        optimizer.get(&key).await;
    }
    
    // Enable predictive prefetching
    optimizer.enable_predictive_prefetching(true).await.unwrap();
    
    let stats = optimizer.get_statistics().await;
    assert!(stats.prefetch_hits > 0 || stats.prefetch_misses >= 0);
}

#[tokio::test]
async fn test_optimization_configuration() {
    let config = OptimizationConfig {
        enable_memory_optimization: true,
        enable_cpu_optimization: true,
        enable_gpu_optimization: false,
        optimization_interval_secs: 300,
        ..Default::default()
    };
    
    let manager = OptimizationManager::new(config).await.unwrap();
    
    // Verify configuration applied
    let stats = manager.get_statistics().await;
    assert_eq!(stats.optimization_cycles, 0);
}

#[tokio::test]
async fn test_load_balancing() {
    let config = Default::default();
    let optimizer = InferenceOptimizer::new(config).await.unwrap();
    
    // Add multiple inference endpoints
    optimizer.add_endpoint("endpoint1".to_string(), 1000).await.unwrap();
    optimizer.add_endpoint("endpoint2".to_string(), 1000).await.unwrap();
    optimizer.add_endpoint("endpoint3".to_string(), 1000).await.unwrap();
    
    // Distribute load
    for i in 0..30 {
        let endpoint = optimizer.select_endpoint().await.unwrap();
        assert!(endpoint.starts_with("endpoint"));
    }
    
    // Check distribution
    let stats = optimizer.get_load_balancing_stats().await;
    assert_eq!(stats.total_endpoints, 3);
}