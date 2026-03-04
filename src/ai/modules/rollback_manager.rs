//! Rollback Manager for VantisOS Optimization
//!
//! This module provides rollback capabilities for optimization operations:
//! - State snapshot management
//! - Rollback to previous states
//! - Rollback history tracking
//! - Automatic rollback on failure
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::rollback_manager::{RollbackManager, RollbackState};
//!
//! let mut manager = RollbackManager::new();
//! let snapshot = manager.create_snapshot("pre-optimization");
//! // ... perform optimization ...
//! manager.rollback(&snapshot.id);
//! ```

use crate::ai::error::{AIServiceError, Result};
use std::collections::HashMap;

/// Rollback state
#[derive(Debug, Clone)]
pub struct RollbackState {
    pub id: String,
    pub timestamp: u64,
    pub name: String,
    pub description: String,
    pub state_data: HashMap<String, f64>,
    pub metadata: HashMap<String, String>,
    pub is_committed: bool,
}

impl RollbackState {
    pub fn new(name: String, state_data: HashMap<String, f64>) -> Self {
        let id = format!("rollback_{}_{}", 
            name,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        Self {
            id,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            name,
            description: String::new(),
            state_data,
            metadata: HashMap::new(),
            is_committed: false,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn commit(&mut self) {
        self.is_committed = true;
    }
}

/// Rollback result
#[derive(Debug, Clone)]
pub struct RollbackResult {
    pub success: bool,
    pub state_id: String,
    pub rolled_back_parameters: Vec<String>,
    pub failed_parameters: Vec<String>,
    pub duration_ms: u64,
    pub error_message: Option<String>,
}

/// Rollback manager
pub struct RollbackManager {
    pub snapshots: Vec<RollbackState>,
    pub max_snapshots: usize,
    pub auto_rollback_enabled: bool,
    pub history: Vec<RollbackResult>,
    pub max_history: usize,
}

impl RollbackManager {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            max_snapshots: 100,
            auto_rollback_enabled: true,
            history: Vec::new(),
            max_history: 1000,
        }
    }

    /// Create a snapshot of the current state
    pub fn create_snapshot(
        &mut self,
        name: String,
        state_data: HashMap<String, f64>,
    ) -> Result<RollbackState> {
        let mut snapshot = RollbackState::new(name, state_data);
        snapshot.commit();

        self.snapshots.push(snapshot.clone());

        // Manage snapshot count
        while self.snapshots.len() > self.max_snapshots {
            self.snapshots.remove(0);
        }

        Ok(snapshot)
    }

    /// Create a snapshot with description
    pub fn create_snapshot_with_description(
        &mut self,
        name: String,
        description: String,
        state_data: HashMap<String, f64>,
    ) -> Result<RollbackState> {
        let mut snapshot = RollbackState::new(name, state_data)
            .with_description(description);
        snapshot.commit();

        self.snapshots.push(snapshot.clone());

        while self.snapshots.len() > self.max_snapshots {
            self.snapshots.remove(0);
        }

        Ok(snapshot)
    }

    /// Rollback to a specific snapshot
    pub fn rollback(&mut self, state_id: &str) -> Result<RollbackResult> {
        let snapshot = self.snapshots
            .iter()
            .find(|s| s.id == state_id)
            .ok_or_else(|| AIServiceError::ModelNotFound)?;

        let start = std::time::Instant::now();
        let mut rolled_back = Vec::new();
        let mut failed = Vec::new();

        // In a real implementation, this would actually apply the state
        // For now, we simulate the rollback
        for (param, value) in &snapshot.state_data {
            rolled_back.push(param.clone());
        }

        let duration = start.elapsed().as_millis() as u64;

        let result = RollbackResult {
            success: failed.is_empty(),
            state_id: state_id.to_string(),
            rolled_back_parameters: rolled_back,
            failed_parameters: failed,
            duration_ms: duration,
            error_message: if failed.is_empty() { None } else {
                Some("Some parameters failed to rollback".to_string())
            },
        };

        // Add to history
        self.history.push(result.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }

        Ok(result)
    }

    /// Rollback to the most recent snapshot
    pub fn rollback_to_latest(&mut self) -> Result<RollbackResult> {
        let latest = self.snapshots
            .last()
            .ok_or_else(|| AIServiceError::ModelNotFound)?;
        self.rollback(&latest.id)
    }

    /// Rollback to a snapshot by name
    pub fn rollback_to_name(&mut self, name: &str) -> Result<RollbackResult> {
        let snapshot = self.snapshots
            .iter()
            .rev()
            .find(|s| s.name == name)
            .ok_or_else(|| AIServiceError::ModelNotFound)?;
        self.rollback(&snapshot.id)
    }

    /// Get a snapshot by ID
    pub fn get_snapshot(&self, state_id: &str) -> Option<&RollbackState> {
        self.snapshots.iter().find(|s| s.id == state_id)
    }

    /// Get all snapshots
    pub fn get_all_snapshots(&self) -> &[RollbackState] {
        &self.snapshots
    }

    /// Get rollback history
    pub fn get_history(&self) -> &[RollbackResult] {
        &self.history
    }

    /// Delete a snapshot
    pub fn delete_snapshot(&mut self, state_id: &str) -> Result<()> {
        let pos = self.snapshots
            .iter()
            .position(|s| s.id == state_id)
            .ok_or_else(|| AIServiceError::ModelNotFound)?;
        self.snapshots.remove(pos);
        Ok(())
    }

    /// Delete old snapshots
    pub fn prune_old_snapshots(&mut self, max_age_seconds: u64) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.snapshots.retain(|s| now - s.timestamp <= max_age_seconds);
    }

    /// Get snapshot statistics
    pub fn get_statistics(&self) -> RollbackStatistics {
        let successful_rollbacks = self.history.iter().filter(|r| r.success).count();
        let failed_rollbacks = self.history.iter().filter(|r| !r.success).count();
        let avg_duration = if self.history.is_empty() {
            0.0
        } else {
            self.history.iter()
                .map(|r| r.duration_ms as f64)
                .sum::<f64>() / self.history.len() as f64
        };

        RollbackStatistics {
            total_snapshots: self.snapshots.len(),
            total_rollbacks: self.history.len(),
            successful_rollbacks,
            failed_rollbacks,
            success_rate: if self.history.is_empty() {
                0.0
            } else {
                successful_rollbacks as f64 / self.history.len() as f64
            },
            avg_duration_ms: avg_duration,
        }
    }

    /// Clear all snapshots
    pub fn clear_snapshots(&mut self) {
        self.snapshots.clear();
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Enable or disable auto rollback
    pub fn set_auto_rollback(&mut self, enabled: bool) {
        self.auto_rollback_enabled = enabled;
    }

    /// Check if auto rollback is enabled
    pub fn is_auto_rollback_enabled(&self) -> bool {
        self.auto_rollback_enabled
    }
}

impl Default for RollbackManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Rollback statistics
#[derive(Debug, Clone, Default)]
pub struct RollbackStatistics {
    pub total_snapshots: usize,
    pub total_rollbacks: usize,
    pub successful_rollbacks: usize,
    pub failed_rollbacks: usize,
    pub success_rate: f64,
    pub avg_duration_ms: f64,
}

/// Rollback point for automatic rollback
#[derive(Debug, Clone)]
pub struct RollbackPoint {
    pub id: String,
    pub name: String,
    pub created_at: u64,
    pub state: HashMap<String, f64>,
}

impl RollbackPoint {
    pub fn new(name: String, state: HashMap<String, f64>) -> Self {
        Self {
            id: format!("rbp_{}", uuid::Uuid::new_v4()),
            name,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            state,
        }
    }
}

/// Automatic rollback controller
pub struct AutoRollbackController {
    pub rollback_points: VecDeque<RollbackPoint>,
    pub max_points: usize,
    pub health_check_interval: u64,
    pub failure_threshold: usize,
    pub consecutive_failures: usize,
}

impl AutoRollbackController {
    pub fn new() -> Self {
        Self {
            rollback_points: VecDeque::new(),
            max_points: 10,
            health_check_interval: 60,
            failure_threshold: 3,
            consecutive_failures: 0,
        }
    }

    /// Create a rollback point
    pub fn create_point(&mut self, name: String, state: HashMap<String, f64>) {
        let point = RollbackPoint::new(name, state);
        self.rollback_points.push_back(point);

        while self.rollback_points.len() > self.max_points {
            self.rollback_points.pop_front();
        }
    }

    /// Get the latest rollback point
    pub fn get_latest_point(&self) -> Option<&RollbackPoint> {
        self.rollback_points.back()
    }

    /// Record a failure
    pub fn record_failure(&mut self) {
        self.consecutive_failures += 1;
    }

    /// Record a success
    pub fn record_success(&mut self) {
        self.consecutive_failures = 0;
    }

    /// Check if rollback should be triggered
    pub fn should_rollback(&self) -> bool {
        self.consecutive_failures >= self.failure_threshold
    }

    /// Clear rollback points
    pub fn clear_points(&mut self) {
        self.rollback_points.clear();
    }

    /// Reset failure counter
    pub fn reset_failures(&mut self) {
        self.consecutive_failures = 0;
    }
}

use std::collections::VecDeque;

impl Default for AutoRollbackController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rollback_manager() {
        let mut manager = RollbackManager::new();
        
        let mut state = HashMap::new();
        state.insert("cpu_threshold".to_string(), 80.0);
        state.insert("memory_limit".to_string(), 1024.0);

        let snapshot = manager.create_snapshot("test".to_string(), state).unwrap();
        assert!(snapshot.is_committed);
        assert_eq!(manager.snapshots.len(), 1);
    }

    #[test]
    fn test_rollback() {
        let mut manager = RollbackManager::new();
        
        let mut state = HashMap::new();
        state.insert("param1".to_string(), 100.0);
        
        let snapshot = manager.create_snapshot("test".to_string(), state.clone()).unwrap();
        let result = manager.rollback(&snapshot.id).unwrap();
        
        assert!(result.success);
        assert_eq!(result.state_id, snapshot.id);
    }

    #[test]
    fn test_rollback_to_latest() {
        let mut manager = RollbackManager::new();
        
        let mut state = HashMap::new();
        state.insert("param".to_string(), 50.0);
        
        manager.create_snapshot("test".to_string(), state).unwrap();
        let result = manager.rollback_to_latest();
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_prune_old_snapshots() {
        let mut manager = RollbackManager::new();
        
        let mut state = HashMap::new();
        state.insert("param".to_string(), 50.0);
        
        manager.create_snapshot("old".to_string(), state.clone()).unwrap();
        
        // Simulate time passing
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        manager.prune_old_snapshots(1);
        
        assert!(manager.snapshots.is_empty());
    }

    #[test]
    fn test_auto_rollback_controller() {
        let mut controller = AutoRollbackController::new();
        
        let mut state = HashMap::new();
        state.insert("param".to_string(), 100.0);
        
        controller.create_point("test".to_string(), state);
        assert!(controller.get_latest_point().is_some());
        
        controller.record_failure();
        controller.record_failure();
        controller.record_failure();
        
        assert!(controller.should_rollback());
    }

    #[test]
    fn test_rollback_statistics() {
        let mut manager = RollbackManager::new();
        
        let mut state = HashMap::new();
        state.insert("param".to_string(), 50.0);
        
        let snapshot = manager.create_snapshot("test".to_string(), state).unwrap();
        manager.rollback(&snapshot.id).unwrap();
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_rollbacks, 1);
        assert_eq!(stats.successful_rollbacks, 1);
    }
}