//! Optimization Types for VantisOS
//!
//! This module provides types and configurations for automated optimization:
//! - Optimization strategies
//! - Optimization targets and objectives
//! - Constraint definitions
//! - Optimization configurations
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::optimization_types::{OptimizationStrategy, OptimizationTarget, OptimizationConfig};
//!
//! let config = OptimizationConfig::new(OptimizationStrategy::Adaptive)
//!     .with_target(OptimizationTarget::CPU)
//!     .with_max_iterations(100);
//! ```

use crate::ai::error::{AIServiceError, Result};
use std::collections::HashMap;

/// Optimization strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationStrategy {
    /// Gradient-based optimization
    GradientBased,
    /// Evolutionary optimization
    Evolutionary,
    /// Bayesian optimization
    Bayesian,
    /// Adaptive optimization
    Adaptive,
    /// Reinforcement learning based
    ReinforcementLearning,
    /// Ensemble of multiple strategies
    Ensemble,
}

impl OptimizationStrategy {
    pub fn to_string(&self) -> &str {
        match self {
            OptimizationStrategy::GradientBased => "gradient_based",
            OptimizationStrategy::Evolutionary => "evolutionary",
            OptimizationStrategy::Bayesian => "bayesian",
            OptimizationStrategy::Adaptive => "adaptive",
            OptimizationStrategy::ReinforcementLearning => "reinforcement_learning",
            OptimizationStrategy::Ensemble => "ensemble",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "gradient_based" => Some(OptimizationStrategy::GradientBased),
            "evolutionary" => Some(OptimizationStrategy::Evolutionary),
            "bayesian" => Some(OptimizationStrategy::Bayesian),
            "adaptive" => Some(OptimizationStrategy::Adaptive),
            "reinforcement_learning" => Some(OptimizationStrategy::ReinforcementLearning),
            "ensemble" => Some(OptimizationStrategy::Ensemble),
            _ => None,
        }
    }
}

/// Optimization target
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationTarget {
    /// CPU usage optimization
    CPU,
    /// Memory usage optimization
    Memory,
    /// Power consumption optimization
    Power,
    /// Network throughput optimization
    Network,
    /// Disk I/O optimization
    Disk,
    /// Latency optimization
    Latency,
    /// Throughput optimization
    Throughput,
    /// Custom target
    Custom(u32),
}

impl OptimizationTarget {
    pub fn to_string(&self) -> String {
        match self {
            OptimizationTarget::CPU => "cpu".to_string(),
            OptimizationTarget::Memory => "memory".to_string(),
            OptimizationTarget::Power => "power".to_string(),
            OptimizationTarget::Network => "network".to_string(),
            OptimizationTarget::Disk => "disk".to_string(),
            OptimizationTarget::Latency => "latency".to_string(),
            OptimizationTarget::Throughput => "throughput".to_string(),
            OptimizationTarget::Custom(id) => format!("custom_{}", id),
        }
    }
}

/// Optimization objective (minimize or maximize)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationObjective {
    Minimize,
    Maximize,
}

/// Optimization constraint
#[derive(Debug, Clone)]
pub struct OptimizationConstraint {
    pub name: String,
    pub constraint_type: ConstraintType,
    pub threshold: f64,
    pub penalty_weight: f64,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintType {
    UpperBound,
    LowerBound,
    Equality,
    Range { lower: f64, upper: f64 },
}

impl OptimizationConstraint {
    pub fn new(name: String, constraint_type: ConstraintType, threshold: f64) -> Self {
        Self {
            name,
            constraint_type,
            threshold,
            penalty_weight: 1.0,
            description: String::new(),
        }
    }

    pub fn with_penalty_weight(mut self, weight: f64) -> Self {
        self.penalty_weight = weight;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn is_satisfied(&self, value: f64) -> bool {
        match self.constraint_type {
            ConstraintType::UpperBound => value <= self.threshold,
            ConstraintType::LowerBound => value >= self.threshold,
            ConstraintType::Equality => (value - self.threshold).abs() < 1e-6,
            ConstraintType::Range { lower, upper } => value >= lower && value <= upper,
        }
    }

    pub fn penalty(&self, value: f64) -> f64 {
        if self.is_satisfied(value) {
            0.0
        } else {
            let violation = match self.constraint_type {
                ConstraintType::UpperBound => (value - self.threshold).max(0.0),
                ConstraintType::LowerBound => (self.threshold - value).max(0.0),
                ConstraintType::Equality => (value - self.threshold).abs(),
                ConstraintType::Range { lower, upper } => {
                    if value < lower {
                        lower - value
                    } else if value > upper {
                        value - upper
                    } else {
                        0.0
                    }
                }
            };
            violation * self.penalty_weight
        }
    }
}

/// Optimization parameter
#[derive(Debug, Clone)]
pub struct OptimizationParameter {
    pub name: String,
    pub value: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub is_fixed: bool,
    pub description: String,
}

impl OptimizationParameter {
    pub fn new(name: String, value: f64, min_value: f64, max_value: f64) -> Self {
        Self {
            name,
            value,
            min_value,
            max_value,
            is_fixed: false,
            description: String::new(),
        }
    }

    pub fn fixed(mut self) -> Self {
        self.is_fixed = true;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn set_value(&mut self, value: f64) -> Result<()> {
        if self.is_fixed {
            return Err(AIServiceError::ConfigurationError);
        }
        if value < self.min_value || value > self.max_value {
            return Err(AIServiceError::InvalidInput);
        }
        self.value = value;
        Ok(())
    }
}

/// Optimization suggestion
#[derive(Debug, Clone)]
pub struct OptimizationSuggestion {
    pub id: String,
    pub target: OptimizationTarget,
    pub parameters: HashMap<String, f64>,
    pub expected_improvement: f64,
    pub confidence: f64,
    pub risk_level: RiskLevel,
    pub estimated_duration: u64,
    pub created_at: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn from_score(score: f64) -> Self {
        if score < 0.25 {
            RiskLevel::Low
        } else if score < 0.5 {
            RiskLevel::Medium
        } else if score < 0.75 {
            RiskLevel::High
        } else {
            RiskLevel::Critical
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            RiskLevel::Low => "low",
            RiskLevel::Medium => "medium",
            RiskLevel::High => "high",
            RiskLevel::Critical => "critical",
        }
    }
}

impl OptimizationSuggestion {
    pub fn new(target: OptimizationTarget) -> Self {
        let id = format!("opt_{}_{}", 
            target.to_string(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        Self {
            id,
            target,
            parameters: HashMap::new(),
            expected_improvement: 0.0,
            confidence: 0.0,
            risk_level: RiskLevel::Medium,
            estimated_duration: 0,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn with_parameters(mut self, parameters: HashMap<String, f64>) -> Self {
        self.parameters = parameters;
        self
    }

    pub fn with_improvement(mut self, improvement: f64, confidence: f64) -> Self {
        self.expected_improvement = improvement;
        self.confidence = confidence;
        self.risk_level = RiskLevel::from_score(1.0 - confidence);
        self
    }

    pub fn with_duration(mut self, duration: u64) -> Self {
        self.estimated_duration = duration;
        self
    }
}

/// Optimization configuration
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    pub strategy: OptimizationStrategy,
    pub target: OptimizationTarget,
    pub objective: OptimizationObjective,
    pub max_iterations: usize,
    pub convergence_threshold: f64,
    pub timeout_seconds: Option<u64>,
    pub constraints: Vec<OptimizationConstraint>,
    pub parameters: Vec<OptimizationParameter>,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            strategy: OptimizationStrategy::Adaptive,
            target: OptimizationTarget::CPU,
            objective: OptimizationObjective::Minimize,
            max_iterations: 100,
            convergence_threshold: 1e-6,
            timeout_seconds: None,
            constraints: Vec::new(),
            parameters: Vec::new(),
        }
    }
}

impl OptimizationConfig {
    pub fn new(strategy: OptimizationStrategy) -> Self {
        Self::default().with_strategy(strategy)
    }

    pub fn with_strategy(mut self, strategy: OptimizationStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn with_target(mut self, target: OptimizationTarget) -> Self {
        self.target = target;
        self
    }

    pub fn with_objective(mut self, objective: OptimizationObjective) -> Self {
        self.objective = objective;
        self
    }

    pub fn with_max_iterations(mut self, iterations: usize) -> Self {
        self.max_iterations = iterations;
        self
    }

    pub fn with_constraint(mut self, constraint: OptimizationConstraint) -> Self {
        self.constraints.push(constraint);
        self
    }

    pub fn with_parameter(mut self, parameter: OptimizationParameter) -> Self {
        self.parameters.push(parameter);
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout_seconds = Some(timeout);
        self
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.max_iterations == 0 {
            return Err(AIServiceError::ConfigurationError);
        }
        
        if self.convergence_threshold <= 0.0 {
            return Err(AIServiceError::ConfigurationError);
        }

        for param in &self.parameters {
            if param.value < param.min_value || param.value > param.max_value {
                return Err(AIServiceError::ConfigurationError);
            }
        }

        Ok(())
    }
}

/// Optimization result
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub suggestion: OptimizationSuggestion,
    pub iterations: usize,
    pub convergence_value: f64,
    pub optimization_time: u64,
    pub best_parameters: HashMap<String, f64>,
    pub best_objective_value: f64,
    pub constraint_violations: Vec<String>,
    pub success: bool,
}

impl OptimizationResult {
    pub fn new(suggestion: OptimizationSuggestion) -> Self {
        Self {
            suggestion,
            iterations: 0,
            convergence_value: f64::INFINITY,
            optimization_time: 0,
            best_parameters: HashMap::new(),
            best_objective_value: 0.0,
            constraint_violations: Vec::new(),
            success: false,
        }
    }

    pub fn with_metrics(
        mut self,
        iterations: usize,
        convergence: f64,
        time: u64,
    ) -> Self {
        self.iterations = iterations;
        self.convergence_value = convergence;
        self.optimization_time = time;
        self
    }

    pub fn with_solution(
        mut self,
        parameters: HashMap<String, f64>,
        objective_value: f64,
    ) -> Self {
        self.best_parameters = parameters;
        self.best_objective_value = objective_value;
        self.success = true;
        self
    }

    pub fn with_violations(mut self, violations: Vec<String>) -> Self {
        self.constraint_violations = violations;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_constraint() {
        let constraint = OptimizationConstraint::new(
            "cpu_limit".to_string(),
            ConstraintType::UpperBound,
            80.0,
        );

        assert!(constraint.is_satisfied(70.0));
        assert!(!constraint.is_satisfied(90.0));
        assert_eq!(constraint.penalty(70.0), 0.0);
        assert!(constraint.penalty(90.0) > 0.0);
    }

    #[test]
    fn test_optimization_parameter() {
        let mut param = OptimizationParameter::new("threshold".to_string(), 0.5, 0.0, 1.0);
        
        assert_eq!(param.value, 0.5);
        assert!(param.set_value(0.8).is_ok());
        assert_eq!(param.value, 0.8);
        
        assert!(param.set_value(1.5).is_err());
    }

    #[test]
    fn test_optimization_suggestion() {
        let suggestion = OptimizationSuggestion::new(OptimizationTarget::CPU)
            .with_improvement(0.15, 0.85)
            .with_duration(300);

        assert_eq!(suggestion.target, OptimizationTarget::CPU);
        assert_eq!(suggestion.expected_improvement, 0.15);
        assert_eq!(suggestion.confidence, 0.85);
        assert_eq!(suggestion.estimated_duration, 300);
    }

    #[test]
    fn test_risk_level() {
        assert_eq!(RiskLevel::from_score(0.1), RiskLevel::Low);
        assert_eq!(RiskLevel::from_score(0.4), RiskLevel::Medium);
        assert_eq!(RiskLevel::from_score(0.6), RiskLevel::High);
        assert_eq!(RiskLevel::from_score(0.9), RiskLevel::Critical);
    }

    #[test]
    fn test_optimization_config() {
        let config = OptimizationConfig::new(OptimizationStrategy::Bayesian)
            .with_target(OptimizationTarget::Memory)
            .with_max_iterations(200)
            .with_constraint(OptimizationConstraint::new(
                "memory_limit".to_string(),
                ConstraintType::UpperBound,
                1024.0,
            ));

        assert_eq!(config.strategy, OptimizationStrategy::Bayesian);
        assert_eq!(config.target, OptimizationTarget::Memory);
        assert_eq!(config.max_iterations, 200);
        assert_eq!(config.constraints.len(), 1);
    }

    #[test]
    fn test_optimization_config_validation() {
        let mut config = OptimizationConfig::default();
        
        assert!(config.validate().is_ok());
        
        config.max_iterations = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_optimization_result() {
        let suggestion = OptimizationSuggestion::new(OptimizationTarget::CPU);
        let result = OptimizationResult::new(suggestion)
            .with_metrics(50, 1e-7, 1000)
            .with_solution(
                [("threshold".to_string(), 0.7)].iter().cloned().collect(),
                0.85,
            );

        assert_eq!(result.iterations, 50);
        assert!(result.success);
        assert_eq!(result.best_parameters.len(), 1);
    }
}