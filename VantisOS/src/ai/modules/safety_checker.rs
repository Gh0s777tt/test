//! Safety Checker for VantisOS Optimization
//!
//! This module provides safety validation for optimization operations:
//! - Pre-optimization safety checks
//! - Impact analysis
//! - Risk assessment
//! - Safe operating limits
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::safety_checker::{SafetyChecker, SafetyLevel};
//!
//! let checker = SafetyChecker::new();
//! let result = checker.evaluate_safety(&optimization);
//! if result.is_safe() {
//!     checker.apply(&optimization);
//! }
//! ```

use crate::ai::error::{AIServiceError, Result};
use std::collections::HashMap;

/// Safety level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SafetyLevel {
    Safe,
    Caution,
    Warning,
    Danger,
    Critical,
}

impl SafetyLevel {
    pub fn from_score(score: f64) -> Self {
        if score < 0.2 {
            SafetyLevel::Safe
        } else if score < 0.4 {
            SafetyLevel::Caution
        } else if score < 0.6 {
            SafetyLevel::Warning
        } else if score < 0.8 {
            SafetyLevel::Danger
        } else {
            SafetyLevel::Critical
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            SafetyLevel::Safe => "safe",
            SafetyLevel::Caution => "caution",
            SafetyLevel::Warning => "warning",
            SafetyLevel::Danger => "danger",
            SafetyLevel::Critical => "critical",
        }
    }
}

/// Safety rule
#[derive(Debug, Clone)]
pub struct SafetyRule {
    pub name: String,
    pub description: String,
    pub rule_type: SafetyRuleType,
    pub severity: SafetyLevel,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub enum SafetyRuleType {
    /// Parameter must stay within bounds
    ParameterBounds { min: f64, max: f64 },
    /// Rate of change must not exceed threshold
    RateOfChange { max_rate: f64 },
    /// Cumulative change must not exceed threshold
    CumulativeChange { max_total: f64 },
    /// Custom condition
    Custom { condition: String },
}

impl SafetyRule {
    pub fn new(name: String, rule_type: SafetyRuleType) -> Self {
        Self {
            name,
            description: String::new(),
            rule_type,
            severity: SafetyLevel::Warning,
            enabled: true,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_severity(mut self, severity: SafetyLevel) -> Self {
        self.severity = severity;
        self
    }
}

/// Safety evaluation result
#[derive(Debug, Clone)]
pub struct SafetyEvaluation {
    pub level: SafetyLevel,
    pub score: f64,
    pub passed_rules: usize,
    pub failed_rules: usize,
    pub warnings: Vec<String>,
    pub violations: Vec<SafetyViolation>,
}

impl SafetyEvaluation {
    pub fn is_safe(&self) -> bool {
        matches!(self.level, SafetyLevel::Safe | SafetyLevel::Caution)
    }

    pub fn requires_approval(&self) -> bool {
        matches!(self.level, SafetyLevel::Warning | SafetyLevel::Danger)
    }

    pub fn is_critical(&self) -> bool {
        self.level == SafetyLevel::Critical
    }
}

/// Safety violation
#[derive(Debug, Clone)]
pub struct SafetyViolation {
    pub rule_name: String,
    pub severity: SafetyLevel,
    pub description: String,
    pub current_value: f64,
    pub limit: f64,
    pub violation_magnitude: f64,
}

/// Safety checker
pub struct SafetyChecker {
    pub rules: Vec<SafetyRule>,
    pub safe_operating_limits: HashMap<String, (f64, f64)>,
    pub rate_limits: HashMap<String, f64>,
    pub history: Vec<SafetyEvaluation>,
    pub max_history: usize,
}

impl SafetyChecker {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            safe_operating_limits: HashMap::new(),
            rate_limits: HashMap::new(),
            history: Vec::new(),
            max_history: 1000,
        }
    }

    /// Add a safety rule
    pub fn add_rule(&mut self, rule: SafetyRule) {
        self.rules.push(rule);
    }

    /// Set safe operating limit for a parameter
    pub fn set_safe_limit(&mut self, parameter: String, min: f64, max: f64) {
        self.safe_operating_limits.insert(parameter, (min, max));
    }

    /// Set rate limit for a parameter
    pub fn set_rate_limit(&mut self, parameter: String, max_rate: f64) {
        self.rate_limits.insert(parameter, max_rate);
    }

    /// Evaluate safety of an optimization change
    pub fn evaluate_safety(
        &mut self,
        changes: &HashMap<String, f64>,
        current_state: &HashMap<String, f64>,
    ) -> SafetyEvaluation {
        let mut warnings = Vec::new();
        let mut violations = Vec::new();
        let mut passed_rules = 0;
        let mut failed_rules = 0;

        // Check safe operating limits
        for (param, &(min, max)) in &self.safe_operating_limits {
            if let Some(&new_value) = changes.get(param) {
                if new_value < min || new_value > max {
                    let severity = if new_value < min * 0.5 || new_value > max * 1.5 {
                        SafetyLevel::Danger
                    } else {
                        SafetyLevel::Warning
                    };

                    violations.push(SafetyViolation {
                        rule_name: format!("{}_bounds", param),
                        severity,
                        description: format!("Parameter {} out of safe bounds", param),
                        current_value: new_value,
                        limit: if new_value < min { min } else { max },
                        violation_magnitude: if new_value < min {
                            min - new_value
                        } else {
                            new_value - max
                        },
                    });
                    failed_rules += 1;
                } else {
                    passed_rules += 1;
                }
            }
        }

        // Check rate limits
        for (param, &max_rate) in &self.rate_limits {
            if let (Some(&new_value), Some(&current_value)) = (changes.get(param), current_state.get(param)) {
                let rate = (new_value - current_value).abs();
                if rate > max_rate {
                    let severity = if rate > max_rate * 2.0 {
                        SafetyLevel::Critical
                    } else if rate > max_rate * 1.5 {
                        SafetyLevel::Danger
                    } else {
                        SafetyLevel::Warning
                    };

                    violations.push(SafetyViolation {
                        rule_name: format!("{}_rate_limit", param),
                        severity,
                        description: format!("Parameter {} rate change exceeds limit", param),
                        current_value: rate,
                        limit: max_rate,
                        violation_magnitude: rate - max_rate,
                    });
                    failed_rules += 1;
                } else {
                    passed_rules += 1;
                }
            }
        }

        // Check custom rules
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }

            let violation = self.check_rule(rule, changes, current_state);
            if let Some(violation) = violation {
                violations.push(violation);
                failed_rules += 1;
            } else {
                passed_rules += 1;
            }
        }

        // Add warnings for potentially risky changes
        for (param, &new_value) in changes {
            if let Some(&current_value) = current_state.get(param) {
                let change_percent = if current_value.abs() > 1e-10 {
                    ((new_value - current_value) / current_value).abs() * 100.0
                } else {
                    0.0
                };

                if change_percent > 50.0 {
                    warnings.push(format!(
                        "Parameter {} changing by {:.1}%",
                        param, change_percent
                    ));
                }
            }
        }

        // Calculate overall safety score
        let total_rules = passed_rules + failed_rules;
        let safety_score = if total_rules > 0 {
            failed_rules as f64 / total_rules as f64
        } else {
            0.0
        };

        // Determine safety level
        let level = if violations.iter().any(|v| v.severity >= SafetyLevel::Danger) {
            SafetyLevel::Danger
        } else if violations.iter().any(|v| v.severity >= SafetyLevel::Warning) {
            SafetyLevel::Warning
        } else if !warnings.is_empty() {
            SafetyLevel::Caution
        } else {
            SafetyLevel::Safe
        };

        let evaluation = SafetyEvaluation {
            level,
            score: safety_score,
            passed_rules,
            failed_rules,
            warnings,
            violations,
        };

        // Add to history
        self.history.push(evaluation.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }

        evaluation
    }

    /// Check a specific safety rule
    fn check_rule(
        &self,
        rule: &SafetyRule,
        changes: &HashMap<String, f64>,
        current_state: &HashMap<String, f64>,
    ) -> Option<SafetyViolation> {
        match &rule.rule_type {
            SafetyRuleType::ParameterBounds { min, max } => {
                if let Some(&value) = changes.get(&rule.name) {
                    if value < *min || value > *max {
                        return Some(SafetyViolation {
                            rule_name: rule.name.clone(),
                            severity: rule.severity,
                            description: format!("Parameter {} out of bounds", rule.name),
                            current_value: value,
                            limit: if value < *min { *min } else { *max },
                            violation_magnitude: if value < *min {
                                min - value
                            } else {
                                value - max
                            },
                        });
                    }
                }
            }
            SafetyRuleType::RateOfChange { max_rate } => {
                if let (Some(&new_value), Some(&current_value)) =
                    (changes.get(&rule.name), current_state.get(&rule.name))
                {
                    let rate = (new_value - current_value).abs();
                    if rate > *max_rate {
                        return Some(SafetyViolation {
                            rule_name: rule.name.clone(),
                            severity: rule.severity,
                            description: format!("Rate of change exceeded for {}", rule.name),
                            current_value: rate,
                            limit: *max_rate,
                            violation_magnitude: rate - max_rate,
                        });
                    }
                }
            }
            SafetyRuleType::CumulativeChange { max_total } => {
                // This would require tracking cumulative changes
                // Simplified: check if change is within limit
                if let (Some(&new_value), Some(&current_value)) =
                    (changes.get(&rule.name), current_state.get(&rule.name))
                {
                    let change = (new_value - current_value).abs();
                    if change > *max_total {
                        return Some(SafetyViolation {
                            rule_name: rule.name.clone(),
                            severity: rule.severity,
                            description: format!("Cumulative change exceeded for {}", rule.name),
                            current_value: change,
                            limit: *max_total,
                            violation_magnitude: change - max_total,
                        });
                    }
                }
            }
            SafetyRuleType::Custom { .. } => {
                // Custom rules would be evaluated via external logic
            }
        }

        None
    }

    /// Get safety statistics
    pub fn get_statistics(&self) -> SafetyStatistics {
        if self.history.is_empty() {
            return SafetyStatistics::default();
        }

        let total_evaluations = self.history.len();
        let safe_evaluations = self.history
            .iter()
            .filter(|e| e.is_safe())
            .count();
        let critical_evaluations = self.history
            .iter()
            .filter(|e| e.is_critical())
            .count();
        let total_violations: usize = self.history
            .iter()
            .map(|e| e.violations.len())
            .sum();

        SafetyStatistics {
            total_evaluations,
            safe_evaluations,
            critical_evaluations,
            total_violations,
            safe_rate: safe_evaluations as f64 / total_evaluations as f64,
        }
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

impl Default for SafetyChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Safety statistics
#[derive(Debug, Clone, Default)]
pub struct SafetyStatistics {
    pub total_evaluations: usize,
    pub safe_evaluations: usize,
    pub critical_evaluations: usize,
    pub total_violations: usize,
    pub safe_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_level() {
        assert_eq!(SafetyLevel::from_score(0.1), SafetyLevel::Safe);
        assert_eq!(SafetyLevel::from_score(0.3), SafetyLevel::Caution);
        assert_eq!(SafetyLevel::from_score(0.5), SafetyLevel::Warning);
        assert_eq!(SafetyLevel::from_score(0.7), SafetyLevel::Danger);
        assert_eq!(SafetyLevel::from_score(0.9), SafetyLevel::Critical);
    }

    #[test]
    fn test_safety_checker() {
        let mut checker = SafetyChecker::new();
        
        checker.set_safe_limit("cpu_usage".to_string(), 0.0, 100.0);
        checker.set_rate_limit("cpu_usage".to_string(), 10.0);

        let mut changes = HashMap::new();
        changes.insert("cpu_usage".to_string(), 50.0);

        let mut current = HashMap::new();
        current.insert("cpu_usage".to_string(), 45.0);

        let result = checker.evaluate_safety(&changes, &current);
        assert!(result.is_safe());
    }

    #[test]
    fn test_safety_violation() {
        let mut checker = SafetyChecker::new();
        
        checker.set_safe_limit("cpu_usage".to_string(), 0.0, 80.0);

        let mut changes = HashMap::new();
        changes.insert("cpu_usage".to_string(), 90.0);

        let current = HashMap::new();

        let result = checker.evaluate_safety(&changes, &current);
        assert!(!result.is_safe());
        assert_eq!(result.violations.len(), 1);
    }

    #[test]
    fn test_rate_limit() {
        let mut checker = SafetyChecker::new();
        
        checker.set_rate_limit("cpu_usage".to_string(), 10.0);

        let mut changes = HashMap::new();
        changes.insert("cpu_usage".to_string(), 50.0);

        let mut current = HashMap::new();
        current.insert("cpu_usage".to_string(), 30.0);

        let result = checker.evaluate_safety(&changes, &current);
        assert!(!result.violations.is_empty());
    }

    #[test]
    fn test_safety_rules() {
        let mut checker = SafetyChecker::new();
        
        checker.add_rule(SafetyRule::new(
            "memory_usage".to_string(),
            SafetyRuleType::ParameterBounds { min: 0.0, max: 1024.0 },
        ));

        let mut changes = HashMap::new();
        changes.insert("memory_usage".to_string(), 1500.0);

        let current = HashMap::new();

        let result = checker.evaluate_safety(&changes, &current);
        assert!(!result.violations.is_empty());
    }

    #[test]
    fn test_safety_statistics() {
        let mut checker = SafetyChecker::new();
        
        checker.set_safe_limit("cpu".to_string(), 0.0, 100.0);

        for i in 0..10 {
            let mut changes = HashMap::new();
            changes.insert("cpu".to_string(), i as f64 * 10.0);
            let current = HashMap::new();
            checker.evaluate_safety(&changes, &current);
        }

        let stats = checker.get_statistics();
        assert_eq!(stats.total_evaluations, 10);
    }
}