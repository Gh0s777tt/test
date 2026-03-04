//! Rollout Controller for VantisOS Optimization
//!
//! This module provides progressive rollout control:
//! - Staged rollout strategies
//! - Canary deployments
//! - Traffic splitting
//! - Automatic rollback on failure
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::rollout_controller::{RolloutController, RolloutStrategy};
//!
//! let mut controller = RolloutController::new(RolloutStrategy::Progressive);
//! let plan = controller.create_plan("optimization_1");
//! controller.start(&plan);
//! ```

use crate::ai::error::{AIServiceError, Result};
use std::collections::{HashMap, VecDeque};

/// Rollout strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RolloutStrategy {
    /// Roll out to all targets at once
    Immediate,
    /// Progressive rollout in stages
    Progressive,
    /// Canary rollout to a small subset first
    Canary,
    /// Blue-green deployment
    BlueGreen,
    /// Feature flag based
    FeatureFlag,
}

/// Rollout status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RolloutStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    RolledBack,
}

/// Rollout plan
#[derive(Debug, Clone)]
pub struct RolloutPlan {
    pub id: String,
    pub name: String,
    pub strategy: RolloutStrategy,
    pub status: RolloutStatus,
    pub stages: Vec<RolloutStage>,
    pub current_stage: usize,
    pub progress_percent: f64,
    pub created_at: u64,
    pub started_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub success_criteria: Vec<SuccessCriterion>,
    pub auto_rollback_enabled: bool,
}

impl RolloutPlan {
    pub fn new(name: String, strategy: RolloutStrategy) -> Self {
        let id = format!("rollout_{}_{}", 
            name,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        Self {
            id,
            name,
            strategy,
            status: RolloutStatus::Pending,
            stages: Vec::new(),
            current_stage: 0,
            progress_percent: 0.0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            started_at: None,
            completed_at: None,
            success_criteria: Vec::new(),
            auto_rollback_enabled: true,
        }
    }

    pub fn with_stages(mut self, stages: Vec<RolloutStage>) -> Self {
        self.stages = stages;
        self
    }

    pub fn with_success_criteria(mut self, criteria: Vec<SuccessCriterion>) -> Self {
        self.success_criteria = criteria;
        self
    }

    pub fn with_auto_rollback(mut self, enabled: bool) -> Self {
        self.auto_rollback_enabled = enabled;
        self
    }

    pub fn progress(&self) -> f64 {
        if self.stages.is_empty() {
            return 0.0;
        }

        let completed_stages = self.current_stage as f64;
        let total_stages = self.stages.len() as f64;
        (completed_stages / total_stages) * 100.0
    }
}

/// Rollout stage
#[derive(Debug, Clone)]
pub struct RolloutStage {
    pub id: String,
    pub name: String,
    pub target_percent: f64,
    pub duration_seconds: Option<u64>,
    pub wait_for_approval: bool,
    pub approval_timeout: Option<u64>,
    pub approved: bool,
    pub started_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub status: StageStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageStatus {
    Pending,
    Running,
    WaitingForApproval,
    Completed,
    Failed,
}

impl RolloutStage {
    pub fn new(name: String, target_percent: f64) -> Self {
        Self {
            id: format!("stage_{}_{}", 
                name,
                uuid::Uuid::new_v4().to_string()[..8].to_string()
            ),
            name,
            target_percent,
            duration_seconds: None,
            wait_for_approval: false,
            approval_timeout: None,
            approved: false,
            started_at: None,
            completed_at: None,
            status: StageStatus::Pending,
        }
    }

    pub fn with_duration(mut self, seconds: u64) -> Self {
        self.duration_seconds = Some(seconds);
        self
    }

    pub fn with_approval(mut self, timeout: u64) -> Self {
        self.wait_for_approval = true;
        self.approval_timeout = Some(timeout);
        self
    }

    pub fn approve(&mut self) {
        self.approved = true;
    }

    pub fn start(&mut self) {
        self.status = StageStatus::Running;
        self.started_at = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    }

    pub fn complete(&mut self) {
        self.status = StageStatus::Completed;
        self.completed_at = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    }

    pub fn fail(&mut self) {
        self.status = StageStatus::Failed;
    }
}

/// Success criterion
#[derive(Debug, Clone)]
pub struct SuccessCriterion {
    pub metric_name: String,
    pub metric_type: MetricType,
    pub threshold: f64,
    pub comparison: Comparison,
    pub time_window_seconds: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    ErrorRate,
    Latency,
    Throughput,
    ResourceUsage,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
}

impl SuccessCriterion {
    pub fn new(metric_name: String, metric_type: MetricType, threshold: f64, comparison: Comparison) -> Self {
        Self {
            metric_name,
            metric_type,
            threshold,
            comparison,
            time_window_seconds: 300,
        }
    }

    pub fn with_time_window(mut self, seconds: u64) -> Self {
        self.time_window_seconds = seconds;
        self
    }

    pub fn evaluate(&self, current_value: f64) -> bool {
        match self.comparison {
            Comparison::LessThan => current_value < self.threshold,
            Comparison::LessThanOrEqual => current_value <= self.threshold,
            Comparison::GreaterThan => current_value > self.threshold,
            Comparison::GreaterThanOrEqual => current_value >= self.threshold,
            Comparison::Equal => (current_value - self.threshold).abs() < 1e-6,
        }
    }
}

/// Rollout controller
pub struct RolloutController {
    pub strategy: RolloutStrategy,
    pub active_plans: HashMap<String, RolloutPlan>,
    pub history: VecDeque<RolloutPlan>,
    pub max_history: usize,
    pub metrics: HashMap<String, f64>,
}

impl RolloutController {
    pub fn new(strategy: RolloutStrategy) -> Self {
        Self {
            strategy,
            active_plans: HashMap::new(),
            history: VecDeque::new(),
            max_history: 100,
            metrics: HashMap::new(),
        }
    }

    /// Create a new rollout plan
    pub fn create_plan(&self, name: String) -> RolloutPlan {
        RolloutPlan::new(name, self.strategy)
    }

    /// Create default stages based on strategy
    pub fn create_default_stages(&self, strategy: RolloutStrategy) -> Vec<RolloutStage> {
        match strategy {
            RolloutStrategy::Immediate => {
                vec![RolloutStage::new("full_rollout".to_string(), 100.0)]
            }
            RolloutStrategy::Progressive => {
                vec![
                    RolloutStage::new("canary".to_string(), 5.0)
                        .with_duration(300),
                    RolloutStage::new("small".to_string(), 25.0)
                        .with_duration(300),
                    RolloutStage::new("medium".to_string(), 50.0)
                        .with_duration(300),
                    RolloutStage::new("large".to_string(), 100.0)
                        .with_duration(300),
                ]
            }
            RolloutStrategy::Canary => {
                vec![
                    RolloutStage::new("canary".to_string(), 1.0)
                        .with_duration(600)
                        .with_approval(300),
                    RolloutStage::new("ramp_up".to_string(), 10.0)
                        .with_duration(300),
                    RolloutStage::new("full".to_string(), 100.0),
                ]
            }
            RolloutStrategy::BlueGreen => {
                vec![
                    RolloutStage::new("blue_active".to_string(), 100.0)
                        .with_duration(0),
                    RolloutStage::new("green_prepare".to_string(), 0.0)
                        .with_duration(60),
                    RolloutStage::new("switch_traffic".to_string(), 100.0)
                        .with_approval(300),
                ]
            }
            RolloutStrategy::FeatureFlag => {
                vec![
                    RolloutStage::new("internal".to_string(), 0.0)
                        .with_approval(0),
                    RolloutStage::new("beta".to_string(), 10.0)
                        .with_approval(300),
                    RolloutStage::new("general".to_string(), 100.0),
                ]
            }
        }
    }

    /// Start a rollout
    pub fn start(&mut self, plan: &RolloutPlan) -> Result<()> {
        let mut plan = plan.clone();
        plan.status = RolloutStatus::Running;
        plan.started_at = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        // Start first stage
        if !plan.stages.is_empty() {
            plan.stages[0].start();
        }

        self.active_plans.insert(plan.id.clone(), plan);
        Ok(())
    }

    /// Pause a rollout
    pub fn pause(&mut self, plan_id: &str) -> Result<()> {
        let plan = self.active_plans.get_mut(plan_id)
            .ok_or_else(|| AIServiceError::ModelNotFound)?;
        plan.status = RolloutStatus::Paused;
        Ok(())
    }

    /// Resume a rollout
    pub fn resume(&mut self, plan_id: &str) -> Result<()> {
        let plan = self.active_plans.get_mut(plan_id)
            .ok_or_else(|| AIServiceError::ModelNotFound)?;
        plan.status = RolloutStatus::Running;
        Ok(())
    }

    /// Advance to the next stage
    pub fn advance(&mut self, plan_id: &str) -> Result<()> {
        let plan = self.active_plans.get_mut(plan_id)
            .ok_or_else(|| AIServiceError::ModelNotFound)?;

        if plan.current_stage < plan.stages.len() - 1 {
            // Complete current stage
            plan.stages[plan.current_stage].complete();
            plan.current_stage += 1;

            // Start next stage
            plan.stages[plan.current_stage].start();
        } else {
            // Rollout complete
            plan.stages[plan.current_stage].complete();
            plan.status = RolloutStatus::Completed;
            plan.completed_at = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
        }

        plan.progress_percent = plan.progress();
        Ok(())
    }

    /// Rollback a rollout
    pub fn rollback(&mut self, plan_id: &str) -> Result<()> {
        let plan = self.active_plans.get_mut(plan_id)
            .ok_or_else(|| AIServiceError::ModelNotFound)?;

        plan.status = RolloutStatus::RolledBack;
        plan.completed_at = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        Ok(())
    }

    /// Approve a stage
    pub fn approve_stage(&mut self, plan_id: &str, stage_id: &str) -> Result<()> {
        let plan = self.active_plans.get_mut(plan_id)
            .ok_or_else(|| AIServiceError::ModelNotFound)?;

        let stage = plan.stages.iter_mut().find(|s| s.id == stage_id)
            .ok_or_else(|| AIServiceError::ModelNotFound)?;

        stage.approve();
        Ok(())
    }

    /// Check success criteria
    pub fn check_success_criteria(&self, plan: &RolloutPlan) -> Vec<CriterionResult> {
        let mut results = Vec::new();

        for criterion in &plan.success_criteria {
            let current_value = self.metrics.get(&criterion.metric_name).copied().unwrap_or(0.0);
            let passed = criterion.evaluate(current_value);

            results.push(CriterionResult {
                metric_name: criterion.metric_name.clone(),
                threshold: criterion.threshold,
                current_value,
                passed,
            });
        }

        results
    }

    /// Update metrics
    pub fn update_metrics(&mut self, metrics: HashMap<String, f64>) {
        for (name, value) in metrics {
            self.metrics.insert(name, value);
        }
    }

    /// Get active plan
    pub fn get_plan(&self, plan_id: &str) -> Option<&RolloutPlan> {
        self.active_plans.get(plan_id)
    }

    /// Get all active plans
    pub fn get_active_plans(&self) -> Vec<&RolloutPlan> {
        self.active_plans.values().collect()
    }

    /// Get history
    pub fn get_history(&self) -> &VecDeque<RolloutPlan> {
        &self.history
    }

    /// Get statistics
    pub fn get_statistics(&self) -> RolloutStatistics {
        let total_plans = self.history.len() + self.active_plans.len();
        let completed = self.history.iter().filter(|p| p.status == RolloutStatus::Completed).count();
        let failed = self.history.iter().filter(|p| p.status == RolloutStatus::Failed).count();
        let rolled_back = self.history.iter().filter(|p| p.status == RolloutStatus::RolledBack).count();

        RolloutStatistics {
            total_plans,
            active_plans: self.active_plans.len(),
            completed,
            failed,
            rolled_back,
        }
    }
}

/// Criterion result
#[derive(Debug, Clone)]
pub struct CriterionResult {
    pub metric_name: String,
    pub threshold: f64,
    pub current_value: f64,
    pub passed: bool,
}

/// Rollout statistics
#[derive(Debug, Clone, Default)]
pub struct RolloutStatistics {
    pub total_plans: usize,
    pub active_plans: usize,
    pub completed: usize,
    pub failed: usize,
    pub rolled_back: usize,
}

impl Default for RolloutController {
    fn default() -> Self {
        Self::new(RolloutStrategy::Progressive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rollout_plan() {
        let plan = RolloutPlan::new("test".to_string(), RolloutStrategy::Progressive);
        assert_eq!(plan.status, RolloutStatus::Pending);
        assert_eq!(plan.progress_percent, 0.0);
    }

    #[test]
    fn test_rollout_stage() {
        let stage = RolloutStage::new("test".to_string(), 50.0)
            .with_duration(300)
            .with_approval(60);

        assert_eq!(stage.target_percent, 50.0);
        assert_eq!(stage.duration_seconds, Some(300));
        assert!(stage.wait_for_approval);
    }

    #[test]
    fn test_success_criterion() {
        let criterion = SuccessCriterion::new(
            "error_rate".to_string(),
            MetricType::ErrorRate,
            0.01,
            Comparison::LessThan,
        );

        assert!(criterion.evaluate(0.005));
        assert!(!criterion.evaluate(0.02));
    }

    #[test]
    fn test_rollout_controller() {
        let mut controller = RolloutController::new(RolloutStrategy::Progressive);
        
        let plan = controller.create_plan("test".to_string());
        assert!(controller.start(&plan).is_ok());
        assert_eq!(controller.active_plans.len(), 1);
    }

    #[test]
    fn test_advance_stage() {
        let mut controller = RolloutController::new(RolloutStrategy::Progressive);
        
        let stages = controller.create_default_stages(RolloutStrategy::Progressive);
        let plan = RolloutPlan::new("test".to_string(), RolloutStrategy::Progressive)
            .with_stages(stages);

        controller.start(&plan).unwrap();
        let plan_id = controller.get_active_plans()[0].id.clone();
        
        assert!(controller.advance(&plan_id).is_ok());
    }

    #[test]
    fn test_pause_resume() {
        let mut controller = RolloutController::new(RolloutStrategy::Progressive);
        
        let plan = controller.create_plan("test".to_string());
        controller.start(&plan).unwrap();
        
        let plan_id = controller.get_active_plans()[0].id.clone();
        
        assert!(controller.pause(&plan_id).is_ok());
        assert!(controller.resume(&plan_id).is_ok());
    }

    #[test]
    fn test_rollback() {
        let mut controller = RolloutController::new(RolloutStrategy::Progressive);
        
        let plan = controller.create_plan("test".to_string());
        controller.start(&plan).unwrap();
        
        let plan_id = controller.get_active_plans()[0].id.clone();
        
        assert!(controller.rollback(&plan_id).is_ok());
        
        let plan = controller.get_plan(&plan_id).unwrap();
        assert_eq!(plan.status, RolloutStatus::RolledBack);
    }

    #[test]
    fn test_check_success_criteria() {
        let controller = RolloutController::new(RolloutStrategy::Progressive);
        
        let criterion = SuccessCriterion::new(
            "cpu_usage".to_string(),
            MetricType::ResourceUsage,
            80.0,
            Comparison::LessThan,
        );
        
        let plan = RolloutPlan::new("test".to_string(), RolloutStrategy::Progressive)
            .with_success_criteria(vec![criterion]);

        let results = controller.check_success_criteria(&plan);
        assert_eq!(results.len(), 1);
    }
}