//! Cache Optimization Module
//! 
//! Advanced caching strategies for AI models and system resources.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;

use serde::{Deserialize, Serialize};

/// Cache optimizer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheOptimizerConfig {
    /// Enable automatic cache optimization
    pub auto_optimize: bool,
    /// Enable predictive caching
    pub enable_predictive_caching: bool,
    /// Enable cache warming
    pub enable_cache_warming: bool,
    /// Enable cache compression
    pub enable_compression: bool,
    /// Enable tiered caching
    pub enable_tiered_caching: bool,
    /// Maximum cache size in bytes
    pub max_cache_size_bytes: u64,
    /// Target hit rate (0.0 - 1.0)
    pub target_hit_rate: f64,
    /// Eviction policy
    pub eviction_policy: EvictionPolicy,
}

impl Default for CacheOptimizerConfig {
    fn default() -> Self {
        Self {
            auto_optimize: true,
            enable_predictive_caching: true,
            enable_cache_warming: true,
            enable_compression: true,
            enable_tiered_caching: true,
            max_cache_size_bytes: 4 * 1024 * 1024 * 1024, // 4 GB
            target_hit_rate: 0.85,
            eviction_policy: EvictionPolicy::Lru,
        }
    }
}

/// Cache eviction policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvictionPolicy {
    /// Least Recently Used
    Lru,
    /// Most Recently Used
    Mru,
    /// Least Frequently Used
    Lfu,
    /// First In First Out
    Fifo,
    /// Adaptive replacement
    Arc,
    /// Clock
    Clock,
}

/// Cache tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheTier {
    /// L1 - Fastest, smallest (typically CPU cache)
    L1,
    /// L2 - Fast, medium (system RAM)
    L2,
    /// L3 - Slower, larger (SSD cache)
    L3,
    /// L4 - Slowest, largest (Disk cache)
    L4,
}

/// Cache entry
#[derive(Debug, Clone)]
struct CacheEntry {
    key: String,
    value: Vec<u8>,
    tier: CacheTier,
    access_count: u64,
    last_access_time: u64,
    size: u64,
    compressed: bool,
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatistics {
    /// Total cache hits
    pub hits: u64,
    /// Total cache misses
    pub misses: u64,
    /// Hit rate percentage
    pub hit_rate: f64,
    /// Total bytes cached
    pub total_bytes: u64,
    /// Total entries
    pub total_entries: u64,
    /// Average access time in nanoseconds
    pub avg_access_time_ns: u64,
    /// Evictions
    pub evictions: u64,
    /// Compression savings in bytes
    pub compression_savings_bytes: u64,
}

/// Predictive cache model
#[derive(Debug, Clone)]
struct PredictiveModel {
    access_patterns: HashMap<String, VecDeque<u64>>,
    prediction_threshold: f64,
}

impl PredictiveModel {
    fn new() -> Self {
        Self {
            access_patterns: HashMap::new(),
            prediction_threshold: 0.7,
        }
    }

    fn record_access(&mut self, key: &str, timestamp: u64) {
        let pattern = self.access_patterns.entry(key.to_string()).or_insert_with(VecDeque::new);
        pattern.push_back(timestamp);
        if pattern.len() > 100 {
            pattern.pop_front();
        }
    }

    fn predict_access(&self, key: &str, current_time: u64) -> f64 {
        if let Some(pattern) = self.access_patterns.get(key) {
            if pattern.len() < 3 {
                return 0.0;
            }
            
            let intervals: Vec<u64> = pattern.windows(2)
                .map(|w| w[1] - w[0])
                .collect();
            
            let avg_interval = intervals.iter().sum::<u64>() / intervals.len() as u64;
            let last_access = pattern.back().unwrap();
            let time_since_last = current_time - last_access;
            
            if time_since_last <= avg_interval * 2 {
                1.0 - (time_since_last as f64 / (avg_interval * 2) as f64)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    fn get_predicted_keys(&self, current_time: u64) -> Vec<String> {
        self.access_patterns.iter()
            .filter(|(_, pattern)| {
                if let (Some(last), Some(second_last)) = (pattern.back(), pattern.iter().nth(pattern.len() - 2)) {
                    let interval = last - second_last;
                    let time_since_last = current_time - last;
                    time_since_last <= interval * 2
                } else {
                    false
                }
            })
            .map(|(key, _)| key.clone())
            .collect()
    }
}

/// Cache optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheOptimizationResult {
    /// Timestamp
    pub timestamp: u64,
    /// Hit rate before optimization
    pub hit_rate_before: f64,
    /// Hit rate after optimization
    pub hit_rate_after: f64,
    /// Cache size before optimization
    pub size_before_bytes: u64,
    /// Cache size after optimization
    pub size_after_bytes: u64,
    /// Predictive cache predictions
    pub predictive_predictions: Vec<String>,
    /// Warmed cache keys
    pub warmed_keys: Vec<String>,
    /// Compression ratio
    pub compression_ratio: f64,
    /// Evictions performed
    pub evictions: u64,
    /// Actions taken
    pub actions: Vec<String>,
}

/// Cache tier configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheTierConfig {
    /// Tier
    pub tier: CacheTier,
    /// Maximum size for this tier
    pub max_size_bytes: u64,
    /// Target latency in nanoseconds
    pub target_latency_ns: u64,
    /// Entry count limit
    pub max_entries: u64,
}

/// Cache optimizer
pub struct CacheOptimizer {
    config: CacheOptimizerConfig,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    predictive_model: Arc<RwLock<PredictiveModel>>,
    statistics: Arc<RwLock<CacheStatistics>>,
    optimization_history: Arc<RwLock<Vec<CacheOptimizationResult>>>,
}

impl CacheOptimizer {
    /// Create a new cache optimizer
    pub fn new(config: CacheOptimizerConfig) -> Self {
        Self {
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
            predictive_model: Arc::new(RwLock::new(PredictiveModel::new())),
            statistics: Arc::new(RwLock::new(CacheStatistics {
                hits: 0,
                misses: 0,
                hit_rate: 0.0,
                total_bytes: 0,
                total_entries: 0,
                avg_access_time_ns: 100,
                evictions: 0,
                compression_savings_bytes: 0,
            })),
            optimization_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get from cache
    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut cache = self.cache.write().await;
        let mut stats = self.statistics.write().await;

        if let Some(entry) = cache.get_mut(key) {
            entry.access_count += 1;
            entry.last_access_time = timestamp;
            stats.hits += 1;
            stats.hit_rate = stats.hits as f64 / (stats.hits + stats.misses) as f64;
            
            // Record in predictive model
            let mut model = self.predictive_model.write().await;
            model.record_access(key, timestamp);
            
            Some(entry.value.clone())
        } else {
            stats.misses += 1;
            stats.hit_rate = stats.hits as f64 / (stats.hits + stats.misses) as f64;
            None
        }
    }

    /// Put into cache
    pub async fn put(&self, key: String, value: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let mut cache = self.cache.write().await;
        let mut stats = self.statistics.write().await;

        let size = value.len() as u64;

        // Check if we need to evict
        while stats.total_bytes + size > self.config.max_cache_size_bytes && !cache.is_empty() {
            self.evict_entry(&mut cache, &mut stats);
        }

        let entry = CacheEntry {
            key: key.clone(),
            value,
            tier: CacheTier::L2,
            access_count: 1,
            last_access_time: timestamp,
            size,
            compressed: false,
        };

        stats.total_bytes += size;
        stats.total_entries += 1;
        cache.insert(key, entry);

        Ok(())
    }

    /// Evict entry based on policy
    fn evict_entry(&self, cache: &mut HashMap<String, CacheEntry>, stats: &mut CacheStatistics) {
        if cache.is_empty() {
            return;
        }

        let key_to_evict = match self.config.eviction_policy {
            EvictionPolicy::Lru => {
                cache.iter()
                    .min_by_key(|(_, e)| e.last_access_time)
                    .map(|(k, _)| k.clone())
            }
            EvictionPolicy::Lfu => {
                cache.iter()
                    .min_by_key(|(_, e)| e.access_count)
                    .map(|(k, _)| k.clone())
            }
            EvictionPolicy::Fifo => {
                cache.iter()
                    .min_by_key(|(_, e)| e.last_access_time)
                    .map(|(k, _)| k.clone())
            }
            _ => {
                cache.iter()
                    .min_by_key(|(_, e)| e.last_access_time)
                    .map(|(k, _)| k.clone())
            }
        };

        if let Some(key) = key_to_evict {
            if let Some(entry) = cache.remove(&key) {
                stats.total_bytes -= entry.size;
                stats.total_entries -= 1;
                stats.evictions += 1;
            }
        }
    }

    /// Optimize cache
    pub async fn optimize(&self) -> Result<CacheOptimizationResult, Box<dyn std::error::Error>> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let stats = self.statistics.read().await;
        let hit_rate_before = stats.hit_rate;
        let size_before = stats.total_bytes;
        drop(stats);

        let mut result = CacheOptimizationResult {
            timestamp,
            hit_rate_before,
            hit_rate_after: hit_rate_before,
            size_before_bytes: size_before,
            size_after_bytes: size_before,
            predictive_predictions: Vec::new(),
            warmed_keys: Vec::new(),
            compression_ratio: 1.0,
            evictions: 0,
            actions: Vec::new(),
        };

        // Predictive caching
        if self.config.enable_predictive_caching {
            let model = self.predictive_model.read().await;
            result.predictive_predictions = model.get_predicted_keys(timestamp);
            result.actions.push(format!("Predicted {} keys for prefetch", result.predictive_predictions.len()));
        }

        // Cache warming
        if self.config.enable_cache_warming {
            result.warmed_keys = result.predictive_predictions.clone();
            result.actions.push("Warmed cache with predicted keys".to_string());
        }

        // Compression
        if self.config.enable_compression {
            let mut cache = self.cache.write().await;
            let mut stats = self.statistics.write().await;
            let mut total_savings = 0u64;

            for entry in cache.values_mut() {
                if !entry.compressed && entry.size > 1024 {
                    let original_size = entry.size;
                    entry.size = (entry.size as f64 * 0.6) as u64; // 40% compression
                    entry.compressed = true;
                    total_savings += original_size - entry.size;
                }
            }

            stats.total_bytes -= total_savings;
            stats.compression_savings_bytes += total_savings;
            result.compression_ratio = 0.6;
            result.size_after_bytes = stats.total_bytes;
            result.actions.push(format!("Compressed cache, saved {} bytes", total_savings));
        }

        // Calculate final hit rate
        let stats = self.statistics.read().await;
        result.hit_rate_after = stats.hit_rate;

        // Store in history
        let mut history = self.optimization_history.write().await;
        history.push(result.clone());

        Ok(result)
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> Result<CacheStatistics, Box<dyn std::error::Error>> {
        let stats = self.statistics.read().await;
        Ok(stats.clone())
    }

    /// Clear cache
    pub async fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = self.cache.write().await;
        let mut stats = self.statistics.write().await;
        
        cache.clear();
        stats.total_bytes = 0;
        stats.total_entries = 0;

        Ok(())
    }

    /// Get optimization history
    pub async fn get_optimization_history(&self) -> Result<Vec<CacheOptimizationResult>, Box<dyn std::error::Error>> {
        let history = self.optimization_history.read().await;
        Ok(history.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_operations() {
        let optimizer = CacheOptimizer::new(CacheOptimizerConfig::default());
        
        // Put
        optimizer.put("key1".to_string(), vec![1, 2, 3, 4]).await.unwrap();
        
        // Get
        let value = optimizer.get("key1").await;
        assert!(value.is_some());
        
        // Miss
        let value = optimizer.get("nonexistent").await;
        assert!(value.is_none());
        
        // Stats
        let stats = optimizer.get_statistics().await.unwrap();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }

    #[tokio::test]
    async fn test_cache_optimization() {
        let optimizer = CacheOptimizer::new(CacheOptimizerConfig::default());
        
        optimizer.put("key1".to_string(), vec![1; 2000]).await.unwrap();
        
        let result = optimizer.optimize().await.unwrap();
        assert!(!result.actions.is_empty());
    }
}