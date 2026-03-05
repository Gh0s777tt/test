//! Phase 7 Test Module
//! 
//! This module contains all tests for Phase 7 of VantisOS v1.4.0
//! covering optimization, security, and compliance features.

pub mod optimization_tests;
pub mod security_tests;
pub mod compliance_tests;
pub mod performance_validation;
pub mod user_acceptance_tests;

/// Test utilities and helpers for Phase 7 tests
pub mod utils {
    use std::time::{Duration, Instant};

    /// Measure execution time of a closure
    pub fn measure_time<F, T>(f: F) -> (T, Duration)
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }

    /// Assert that a closure completes within a specified duration
    pub fn assert_completes_within<F>(max_duration: Duration, f: F)
    where
        F: FnOnce(),
    {
        let start = Instant::now();
        f();
        let duration = start.elapsed();
        assert!(
            duration <= max_duration,
            "Operation took {:?}, expected <= {:?}",
            duration,
            max_duration
        );
    }

    /// Generate test data of specified size
    pub fn generate_test_data(size: usize) -> Vec<u8> {
        (0..size).map(|i| (i % 256) as u8).collect()
    }

    /// Calculate throughput (operations per second)
    pub fn calculate_throughput(operations: u64, duration: Duration) -> f64 {
        if duration.as_secs_f64() > 0.0 {
            operations as f64 / duration.as_secs_f64()
        } else {
            0.0
        }
    }

    /// Calculate latency percentiles
    pub fn calculate_percentiles(mut latencies: Vec<f64>) -> (f64, f64, f64) {
        if latencies.is_empty() {
            return (0.0, 0.0, 0.0);
        }
        
        latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let p50 = latencies[latencies.len() / 2];
        let p95 = latencies[(latencies.len() as f64 * 0.95) as usize];
        let p99 = latencies[(latencies.len() as f64 * 0.99) as usize];
        
        (p50, p95, p99)
    }
}

/// Test fixtures for Phase 7 tests
pub mod fixtures {
    use std::collections::HashMap;

    /// Create a sample user record for testing
    pub fn create_sample_user(id: &str) -> HashMap<String, String> {
        let mut user = HashMap::new();
        user.insert("id".to_string(), id.to_string());
        user.insert("email".to_string(), format!("{}@example.com", id));
        user.insert("name".to_string(), format!("User {}", id));
        user.insert("consent".to_string(), "true".to_string());
        user
    }

    /// Create sample feature vector for ML testing
    pub fn create_sample_features(size: usize) -> Vec<f32> {
        (0..size).map(|i| (i as f32 * 0.001) % 1.0).collect()
    }

    /// Create sample security event for testing
    pub fn create_sample_security_event(user_id: &str, event_type: &str) -> HashMap<String, String> {
        let mut event = HashMap::new();
        event.insert("user_id".to_string(), user_id.to_string());
        event.insert("event_type".to_string(), event_type.to_string());
        event.insert("timestamp".to_string(), chrono_lite_timestamp());
        event
    }

    fn chrono_lite_timestamp() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        format!("{}", duration.as_secs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utils_measure_time() {
        let (result, duration) = utils::measure_time(|| {
            std::thread::sleep(std::time::Duration::from_millis(10));
            42
        });
        
        assert_eq!(result, 42);
        assert!(duration >= std::time::Duration::from_millis(10));
    }

    #[test]
    fn test_utils_generate_test_data() {
        let data = utils::generate_test_data(100);
        assert_eq!(data.len(), 100);
    }

    #[test]
    fn test_fixtures_create_sample_user() {
        let user = fixtures::create_sample_user("test123");
        assert_eq!(user.get("id"), Some(&"test123".to_string()));
    }
}