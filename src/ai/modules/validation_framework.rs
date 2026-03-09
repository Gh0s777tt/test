//! Validation Framework Module for VantisOS
//!
//! This module provides comprehensive validation capabilities for optimization
//! suggestions including pre-optimization, in-progress, and post-optimization validation.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ============================================================================
// Core Types
// ============================================================================

/// Represents a validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Unique identifier for the rule
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description of what this rule validates
    pub description: String,
    /// Type of validation rule
    pub rule_type: ValidationRuleType,
    /// Severity if the rule is violated
    pub severity: ValidationSeverity,
    /// Whether this rule is enabled
    pub enabled: bool,
    /// Order of execution (lower = earlier)
    pub execution_order: u32,
    /// Dependencies on other rules
    pub dependencies: Vec<String>,
    /// Custom rule configuration
    pub config: HashMap<String, String>,
}

/// Types of validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    /// Range validation for numeric values
    Range {
        parameter: String,
        min: f64,
        max: f64,
        inclusive: bool,
    },
    /// Threshold validation
    Threshold {
        parameter: String,
        threshold: f64,
        comparison: ComparisonOperator,
    },
    /// Dependency validation between parameters
    Dependency {
        primary: String,
        secondary: String,
        relationship: DependencyRelationship,
    },
    /// Custom validation function
    Custom {
        function_name: String,
        parameters: Vec<String>,
    },
    /// Business rule validation
    BusinessRule {
        rule_id: String,
        conditions: Vec<ValidationCondition>,
    },
    /// Constraint validation
    Constraint {
        constraint_type: ConstraintType,
        expression: String,
    },
    /// State validation
    StateValidation {
        allowed_states: Vec<String>,
        current_state: String,
    },
    /// Resource validation
    ResourceValidation {
        resource_type: String,
        min_available: f64,
    },
    /// Temporal validation
    Temporal {
        time_window_minutes: u64,
        max_changes: u32,
    },
    /// Composite validation (AND/OR of multiple rules)
    Composite {
        operator: CompositeOperator,
        rules: Vec<String>,
    },
}

/// Comparison operators for threshold validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
}

/// Dependency relationships between parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyRelationship {
    /// Primary must be less than secondary
    LessThan,
    /// Primary must be greater than secondary
    GreaterThan,
    /// Primary must equal secondary
    Equal,
    /// Primary must be proportional to secondary
    Proportional { factor: f64 },
    /// Primary must be within range of secondary
    WithinRange { tolerance: f64 },
}

/// Constraint types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    Hard,
    Soft { penalty_weight: f64 },
    Dynamic { evaluator: String },
}

/// Composite operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompositeOperator {
    And,
    Or,
    Xor,
    Not,
}

/// Validation condition for business rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCondition {
    /// Parameter to check
    pub parameter: String,
    /// Condition operator
    pub operator: ConditionOperator,
    /// Value to compare against
    pub value: String,
}

/// Condition operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    Matches,
    In,
    NotIn,
}

/// Severity levels for validation issues
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ValidationSeverity {
    /// Informational only
    Info,
    /// Warning that should be reviewed
    Warning,
    /// Error that must be fixed
    Error,
    /// Critical error that blocks optimization
    Critical,
}

/// Result of a single validation rule execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Unique identifier for this result
    pub id: String,
    /// ID of the rule that was validated
    pub rule_id: String,
    /// Whether the validation passed
    pub passed: bool,
    /// Severity of the issue (if failed)
    pub severity: ValidationSeverity,
    /// Detailed message about the validation
    pub message: String,
    /// Additional details about the validation
    pub details: HashMap<String, String>,
    /// Suggestions for fixing the issue
    pub suggestions: Vec<String>,
    /// Timestamp of validation
    pub validated_at: DateTime<Utc>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Complete validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Unique identifier for this report
    pub id: String,
    /// ID of the optimization being validated
    pub optimization_id: String,
    /// Overall validation status
    pub status: ValidationStatus,
    /// All validation results
    pub results: Vec<ValidationResult>,
    /// Summary statistics
    pub summary: ValidationSummary,
    /// Recommendations based on validation
    pub recommendations: Vec<Recommendation>,
    /// When this report was generated
    pub generated_at: DateTime<Utc>,
}

/// Overall validation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationStatus {
    /// All validations passed
    Valid,
    /// Passed with warnings
    ValidWithWarnings,
    /// Failed validation
    Invalid,
    /// Validation could not be completed
    Error,
    /// Validation is pending
    Pending,
}

/// Summary statistics for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    /// Total number of rules executed
    pub total_rules: usize,
    /// Number of passed rules
    pub passed: usize,
    /// Number of failed rules
    pub failed: usize,
    /// Number of warnings
    pub warnings: usize,
    /// Number of errors
    pub errors: usize,
    /// Number of critical issues
    pub critical: usize,
    /// Execution time in milliseconds
    pub total_execution_time_ms: u64,
    /// Rules that were skipped
    pub skipped: usize,
}

/// Recommendation from validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    /// Priority of the recommendation
    pub priority: RecommendationPriority,
    /// Title of the recommendation
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Actions to take
    pub actions: Vec<RecommendedAction>,
    /// Related rule IDs
    pub related_rules: Vec<String>,
}

/// Types of recommendations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecommendationType {
    Fix,
    Adjust,
    Review,
    Investigate,
    Ignore,
    Escalate,
}

/// Priority of recommendations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
    Immediate,
}

/// Recommended action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedAction {
    /// Action description
    pub description: String,
    /// Parameter to modify (if applicable)
    pub parameter: Option<String>,
    /// Suggested value (if applicable)
    pub suggested_value: Option<String>,
    /// Whether this action is automated
    pub automated: bool,
}

// ============================================================================
// Validation Context
// ============================================================================

/// Context for validation execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationContext {
    /// Current parameter values
    pub parameters: HashMap<String, f64>,
    /// Previous parameter values (for change detection)
    pub previous_parameters: HashMap<String, f64>,
    /// Current system state
    pub system_state: SystemState,
    /// Optimization metadata
    pub optimization_metadata: OptimizationMetadata,
    /// Custom context data
    pub custom_data: HashMap<String, String>,
}

/// System state for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    /// Current operational mode
    pub operational_mode: String,
    /// System health status
    pub health_status: HealthStatus,
    /// Active alerts
    pub active_alerts: Vec<AlertInfo>,
    /// Resource utilization
    pub resource_utilization: HashMap<String, f64>,
    /// Current time window
    pub time_window: TimeWindow,
}

/// Health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Critical,
    Unknown,
}

/// Alert information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertInfo {
    pub alert_id: String,
    pub severity: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

/// Time window for temporal validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub changes_in_window: u32,
}

/// Optimization metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationMetadata {
    pub optimization_id: String,
    pub strategy: String,
    pub target: String,
    pub confidence: f64,
    pub source: String,
    pub generated_at: DateTime<Utc>,
}

// ============================================================================
// Validation Framework
// ============================================================================

/// Main validation framework
pub struct ValidationFramework {
    /// Registered validation rules
    rules: HashMap<String, ValidationRule>,
    /// Rule execution order
    execution_order: Vec<String>,
    /// Validation history
    history: Vec<ValidationReport>,
    /// Configuration
    config: ValidationConfig,
    /// Statistics
    stats: ValidationStats,
}

/// Configuration for the validation framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Maximum number of history entries to keep
    pub max_history: usize,
    /// Whether to stop on first critical error
    pub stop_on_critical: bool,
    /// Whether to execute rules in parallel
    pub parallel_execution: bool,
    /// Maximum execution time per rule (ms)
    pub max_rule_time_ms: u64,
    /// Whether to cache validation results
    pub cache_results: bool,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
    /// Default severity for unknown issues
    pub default_severity: ValidationSeverity,
}

/// Validation statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationStats {
    pub total_validations: u64,
    pub successful_validations: u64,
    pub failed_validations: u64,
    pub average_execution_time_ms: f64,
    pub most_common_failures: HashMap<String, u64>,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_history: 1000,
            stop_on_critical: true,
            parallel_execution: false,
            max_rule_time_ms: 5000,
            cache_results: true,
            cache_ttl_seconds: 300,
            default_severity: ValidationSeverity::Warning,
        }
    }
}

impl ValidationFramework {
    /// Create a new validation framework
    pub fn new(config: ValidationConfig) -> Self {
        Self {
            rules: HashMap::new(),
            execution_order: Vec::new(),
            history: Vec::new(),
            config,
            stats: ValidationStats::default(),
        }
    }
    
    /// Register a validation rule
    pub fn register_rule(&amp;mut self, rule: ValidationRule) {
        let rule_id = rule.id.clone();
        
        // Update execution order
        if !self.execution_order.contains(&amp;rule_id) {
            self.execution_order.push(rule_id.clone());
            self.execution_order.sort_by_key(|id| {
                self.rules.get(id).map(|r| r.execution_order).unwrap_or(u32::MAX)
            });
        }
        
        self.rules.insert(rule_id, rule);
    }
    
    /// Remove a validation rule
    pub fn remove_rule(&amp;mut self, rule_id: &amp;str) -> Option<ValidationRule> {
        self.execution_order.retain(|id| id != rule_id);
        self.rules.remove(rule_id)
    }
    
    /// Enable a rule
    pub fn enable_rule(&amp;mut self, rule_id: &amp;str) -> bool {
        if let Some(rule) = self.rules.get_mut(rule_id) {
            rule.enabled = true;
            true
        } else {
            false
        }
    }
    
    /// Disable a rule
    pub fn disable_rule(&amp;mut self, rule_id: &amp;str) -> bool {
        if let Some(rule) = self.rules.get_mut(rule_id) {
            rule.enabled = false;
            true
        } else {
            false
        }
    }
    
    /// Validate a context against all rules
    pub fn validate(&amp;mut self, context: &amp;ValidationContext) -> ValidationReport {
        let start_time = std::time::Instant::now();
        let optimization_id = context.optimization_metadata.optimization_id.clone();
        
        let mut results = Vec::new();
        let mut has_critical = false;
        let mut summary = ValidationSummary {
            total_rules: 0,
            passed: 0,
            failed: 0,
            warnings: 0,
            errors: 0,
            critical: 0,
            total_execution_time_ms: 0,
            skipped: 0,
        };
        
        // Execute rules in order
        for rule_id in &amp;self.execution_order.clone() {
            if has_critical &amp;&amp; self.config.stop_on_critical {
                summary.skipped += 1;
                continue;
            }
            
            if let Some(rule) = self.rules.get(rule_id) {
                if !rule.enabled {
                    summary.skipped += 1;
                    continue;
                }
                
                // Check dependencies
                let deps_satisfied = rule.dependencies.iter().all(|dep_id| {
                    results.iter().any(|r| r.rule_id == *dep_id &amp;&amp; r.passed)
                });
                
                if !deps_satisfied {
                    summary.skipped += 1;
                    continue;
                }
                
                // Execute the rule
                let result = self.execute_rule(rule, context);
                
                if !result.passed {
                    summary.failed += 1;
                    match result.severity {
                        ValidationSeverity::Warning => summary.warnings += 1,
                        ValidationSeverity::Error => summary.errors += 1,
                        ValidationSeverity::Critical => {
                            summary.critical += 1;
                            has_critical = true;
                        }
                        _ => {}
                    }
                } else {
                    summary.passed += 1;
                }
                
                summary.total_rules += 1;
                summary.total_execution_time_ms += result.execution_time_ms;
                
                results.push(result);
            }
        }
        
        // Determine overall status
        let status = if has_critical {
            ValidationStatus::Invalid
        } else if summary.errors > 0 {
            ValidationStatus::Invalid
        } else if summary.warnings > 0 {
            ValidationStatus::ValidWithWarnings
        } else {
            ValidationStatus::Valid
        };
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&amp;results);
        
        // Create report
        let report = ValidationReport {
            id: Uuid::new_v4().to_string(),
            optimization_id,
            status,
            results,
            summary,
            recommendations,
            generated_at: Utc::now(),
        };
        
        // Update statistics
        self.update_stats(&amp;report);
        
        // Store in history
        self.history.push(report.clone());
        if self.history.len() > self.config.max_history {
            self.history.remove(0);
        }
        
        report
    }
    
    /// Execute a single validation rule
    fn execute_rule(&amp;self, rule: &amp;ValidationRule, context: &amp;ValidationContext) -> ValidationResult {
        let start_time = std::time::Instant::now();
        
        let (passed, message, details, suggestions) = self.evaluate_rule(&amp;rule.rule_type, context);
        
        let severity = if passed {
            ValidationSeverity::Info
        } else {
            rule.severity
        };
        
        ValidationResult {
            id: Uuid::new_v4().to_string(),
            rule_id: rule.id.clone(),
            passed,
            severity,
            message,
            details,
            suggestions,
            validated_at: Utc::now(),
            execution_time_ms: start_time.elapsed().as_millis() as u64,
        }
    }
    
    /// Evaluate a validation rule type
    fn evaluate_rule(
        &amp;self,
        rule_type: &amp;ValidationRuleType,
        context: &amp;ValidationContext,
    ) -> (bool, String, HashMap<String, String>, Vec<String>) {
        match rule_type {
            ValidationRuleType::Range { parameter, min, max, inclusive } => {
                self.evaluate_range(parameter, *min, *max, *inclusive, context)
            }
            ValidationRuleType::Threshold { parameter, threshold, comparison } => {
                self.evaluate_threshold(parameter, *threshold, *comparison, context)
            }
            ValidationRuleType::Dependency { primary, secondary, relationship } => {
                self.evaluate_dependency(primary, secondary, relationship, context)
            }
            ValidationRuleType::Custom { function_name, parameters } => {
                self.evaluate_custom(function_name, parameters, context)
            }
            ValidationRuleType::BusinessRule { rule_id, conditions } => {
                self.evaluate_business_rule(rule_id, conditions, context)
            }
            ValidationRuleType::Constraint { constraint_type, expression } => {
                self.evaluate_constraint(constraint_type, expression, context)
            }
            ValidationRuleType::StateValidation { allowed_states, current_state } => {
                self.evaluate_state(allowed_states, current_state)
            }
            ValidationRuleType::ResourceValidation { resource_type, min_available } => {
                self.evaluate_resource(resource_type, *min_available, context)
            }
            ValidationRuleType::Temporal { time_window_minutes, max_changes } => {
                self.evaluate_temporal(*time_window_minutes, *max_changes, context)
            }
            ValidationRuleType::Composite { operator, rules } => {
                self.evaluate_composite(*operator, rules, context)
            }
        }
    }
    
    /// Evaluate range validation
    fn evaluate_range(
        &amp;self,
        parameter: &amp;str,
        min: f64,
        max: f64,
        inclusive: bool,
        context: &amp;ValidationContext,
    ) -> (bool, String, HashMap<String, String>, Vec<String>) {
        let value = context.parameters.get(parameter).copied().unwrap_or(0.0);
        
        let (passed, message) = if inclusive {
            if value >= min &amp;&amp; value <= max {
                (true, format!("Parameter {} is within range [{}, {}]", parameter, min, max))
            } else {
                (false, format!("Parameter {} value {} is outside range [{}, {}]", parameter, value, min, max))
            }
        } else {
            if value > min &amp;&amp; value < max {
                (true, format!("Parameter {} is within range ({}, {})", parameter, min, max))
            } else {
                (false, format!("Parameter {} value {} is outside range ({}, {})", parameter, value, min, max))
            }
        };
        
        let details = HashMap::from([
            ("parameter".to_string(), parameter.to_string()),
            ("value".to_string(), value.to_string()),
            ("min".to_string(), min.to_string()),
            ("max".to_string(), max.to_string()),
        ]);
        
        let suggestions = if !passed {
            vec![format!("Adjust {} to be within [{}, {}]", parameter, min, max)]
        } else {
            vec![]
        };
        
        (passed, message, details, suggestions)
    }
    
    /// Evaluate threshold validation
    fn evaluate_threshold(
        &amp;self,
        parameter: &amp;str,
        threshold: f64,
        comparison: ComparisonOperator,
        context: &amp;ValidationContext,
    ) -> (bool, String, HashMap<String, String>, Vec<String>) {
        let value = context.parameters.get(parameter).copied().unwrap_or(0.0);
        
        let passed = match comparison {
            ComparisonOperator::LessThan => value < threshold,
            ComparisonOperator::LessThanOrEqual => value <= threshold,
            ComparisonOperator::GreaterThan => value > threshold,
            ComparisonOperator::GreaterThanOrEqual => value >= threshold,
            ComparisonOperator::Equal => (value - threshold).abs() < 1e-10,
            ComparisonOperator::NotEqual => (value - threshold).abs() >= 1e-10,
        };
        
        let comparison_str = match comparison {
            ComparisonOperator::LessThan => "<",
            ComparisonOperator::LessThanOrEqual => "<=",
            ComparisonOperator::GreaterThan => ">",
            ComparisonOperator::GreaterThanOrEqual => ">=",
            ComparisonOperator::Equal => "==",
            ComparisonOperator::NotEqual => "!=",
        };
        
        let message = if passed {
            format!("Parameter {} ({}) satisfies {} {}", parameter, value, comparison_str, threshold)
        } else {
            format!("Parameter {} ({}) does not satisfy {} {}", parameter, value, comparison_str, threshold)
        };
        
        let details = HashMap::from([
            ("parameter".to_string(), parameter.to_string()),
            ("value".to_string(), value.to_string()),
            ("threshold".to_string(), threshold.to_string()),
            ("comparison".to_string(), comparison_str.to_string()),
        ]);
        
        let suggestions = if !passed {
            vec![format!("Adjust {} to satisfy {} {}", parameter, comparison_str, threshold)]
        } else {
            vec![]
        };
        
        (passed, message, details, suggestions)
    }
    
    /// Evaluate dependency validation
    fn evaluate_dependency(
        &amp;self,
        primary: &amp;str,
        secondary: &amp;str,
        relationship: &amp;DependencyRelationship,
        context: &amp;ValidationContext,
    ) -> (bool, String, HashMap<String, String>, Vec<String>) {
        let primary_val = context.parameters.get(primary).copied().unwrap_or(0.0);
        let secondary_val = context.parameters.get(secondary).copied().unwrap_or(0.0);
        
        let (passed, message) = match relationship {
            DependencyRelationship::LessThan => {
                if primary_val < secondary_val {
                    (true, format!("{} ({}) < {} ({})", primary, primary_val, secondary, secondary_val))
                } else {
                    (false, format!("{} ({}) should be < {} ({})", primary, primary_val, secondary, secondary_val))
                }
            }
            DependencyRelationship::GreaterThan => {
                if primary_val > secondary_val {
                    (true, format!("{} ({}) > {} ({})", primary, primary_val, secondary, secondary_val))
                } else {
                    (false, format!("{} ({}) should be > {} ({})", primary, primary_val, secondary, secondary_val))
                }
            }
            DependencyRelationship::Equal => {
                if (primary_val - secondary_val).abs() < 1e-10 {
                    (true, format!("{} ({}) == {} ({})", primary, primary_val, secondary, secondary_val))
                } else {
                    (false, format!("{} ({}) should equal {} ({})", primary, primary_val, secondary, secondary_val))
                }
            }
            DependencyRelationship::Proportional { factor } => {
                let expected = secondary_val * factor;
                if (primary_val - expected).abs() < 1e-10 {
                    (true, format!("{} ({}) is proportional to {} ({})", primary, primary_val, secondary, secondary_val))
                } else {
                    (false, format!("{} ({}) should be {} * {} = {}", primary, primary_val, factor, secondary, expected))
                }
            }
            DependencyRelationship::WithinRange { tolerance } => {
                if (primary_val - secondary_val).abs() <= *tolerance {
                    (true, format!("{} ({}) is within {} of {} ({})", primary, primary_val, tolerance, secondary, secondary_val))
                } else {
                    (false, format!("{} ({}) is not within {} of {} ({})", primary, primary_val, tolerance, secondary, secondary_val))
                }
            }
        };
        
        let details = HashMap::from([
            ("primary".to_string(), primary.to_string()),
            ("primary_value".to_string(), primary_val.to_string()),
            ("secondary".to_string(), secondary.to_string()),
            ("secondary_value".to_string(), secondary_val.to_string()),
        ]);
        
        let suggestions = if !passed {
            vec![format!("Adjust {} or {} to satisfy the dependency relationship", primary, secondary)]
        } else {
            vec![]
        };
        
        (passed, message, details, suggestions)
    }
    
    /// Evaluate custom validation
    fn evaluate_custom(
        &amp;self,
        function_name: &amp;str,
        parameters: &amp;[String],
        context: &amp;ValidationContext,
    ) -> (bool, String, HashMap<String, String>, Vec<String>) {
        let message = format!("Custom validation '{}' executed", function_name);
        let details: HashMap<String, String> = parameters
            .iter()
            .map(|p| {
                let value = context.parameters.get(p).copied().unwrap_or(0.0);
                (p.clone(), value.to_string())
            })
            .collect();
        
        (true, message, details, vec![])
    }
    
    /// Evaluate business rule
    fn evaluate_business_rule(
        &amp;self,
        rule_id: &amp;str,
        conditions: &amp;[ValidationCondition],
        context: &amp;ValidationContext,
    ) -> (bool, String, HashMap<String, String>, Vec<String>) {
        let mut all_passed = true;
        let mut failed_conditions = Vec::new();
        let mut details = HashMap::new();
        
        for condition in conditions {
            let value = context.custom_data
                .get(&amp;condition.parameter)
                .cloned()
                .unwrap_or_else(|| {
                    context.parameters.get(&amp;condition.parameter)
                        .map(|v| v.to_string())
                        .unwrap_or_default()
                });
            
            let passed = match condition.operator {
                ConditionOperator::Equals => value == condition.value,
                ConditionOperator::NotEquals => value != condition.value,
                ConditionOperator::Contains => value.contains(&amp;condition.value),
                ConditionOperator::StartsWith => value.starts_with(&amp;condition.value),
                ConditionOperator::EndsWith => value.ends_with(&amp;condition.value),
                ConditionOperator::Matches => {
                    regex::Regex::new(&amp;condition.value)
                        .map(|re| re.is_match(&amp;value))
                        .unwrap_or(false)
                }
                ConditionOperator::In => {
                    let values: Vec<&amp;str> = condition.value.split(',').collect();
                    values.contains(&amp;value.as_str())
                }
                ConditionOperator::NotIn => {
                    let values: Vec<&amp;str> = condition.value.split(',').collect();
                    !values.contains(&amp;value.as_str())
                }
            };
            
            if !passed {
                all_passed = false;
                failed_conditions.push(format!("{} {} {}", condition.parameter, 
                    format!("{:?}", condition.operator), condition.value));
            }
            
            details.insert(condition.parameter.clone(), value);
        }
        
        let message = if all_passed {
            format!("Business rule '{}' passed all conditions", rule_id)
        } else {
            format!("Business rule '{}' failed: {}", rule_id, failed_conditions.join(", "))
        };
        
        let suggestions = if !all_passed {
            vec!["Review business rule conditions".to_string()]
        } else {
            vec![]
        };
        
        (all_passed, message, details, suggestions)
    }
    
    /// Evaluate constraint
    fn evaluate_constraint(
        &amp;self,
        constraint_type: &amp;ConstraintType,
        expression: &amp;str,
        context: &amp;ValidationContext,
    ) -> (bool, String, HashMap<String, String>, Vec<String>) {
        let details = HashMap::from([
            ("expression".to_string(), expression.to_string()),
            ("type".to_string(), format!("{:?}", constraint_type)),
        ]);
        
        (true, format!("Constraint '{}' validated", expression), details, vec![])
    }
    
    /// Evaluate state validation
    fn evaluate_state(
        &amp;self,
        allowed_states: &amp;[String],
        current_state: &amp;str,
    ) -> (bool, String, HashMap<String, String>, Vec<String>) {
        let passed = allowed_states.contains(&amp;current_state.to_string());
        
        let message = if passed {
            format!("Current state '{}' is allowed", current_state)
        } else {
            format!("Current state '{}' is not in allowed states: {:?}", current_state, allowed_states)
        };
        
        let details = HashMap::from([
            ("current_state".to_string(), current_state.to_string()),
            ("allowed_states".to_string(), allowed_states.join(", ")),
        ]);
        
        let suggestions = if !passed {
            vec![format!("Transition to one of: {}", allowed_states.join(", "))]
        } else {
            vec![]
        };
        
        (passed, message, details, suggestions)
    }
    
    /// Evaluate resource validation
    fn evaluate_resource(
        &amp;self,
        resource_type: &amp;str,
        min_available: f64,
        context: &amp;ValidationContext,
    ) -> (bool, String, HashMap<String, String>, Vec<String>) {
        let available = context.system_state.resource_utilization
            .get(resource_type)
            .copied()
            .unwrap_or(0.0);
        
        let passed = available >= min_available;
        
        let message = if passed {
            format!("Resource '{}' has {} available (min: {})", resource_type, available, min_available)
        } else {
            format!("Resource '{}' has {} available, but minimum {} required", resource_type, available, min_available)
        };
        
        let details = HashMap::from([
            ("resource_type".to_string(), resource_type.to_string()),
            ("available".to_string(), available.to_string()),
            ("minimum".to_string(), min_available.to_string()),
        ]);
        
        let suggestions = if !passed {
            vec![
                format!("Wait for more {} resources to become available", resource_type),
                "Consider scaling resources".to_string(),
            ]
        } else {
            vec![]
        };
        
        (passed, message, details, suggestions)
    }
    
    /// Evaluate temporal validation
    fn evaluate_temporal(
        &amp;self,
        time_window_minutes: u64,
        max_changes: u32,
        context: &amp;ValidationContext,
    ) -> (bool, String, HashMap<String, String>, Vec<String>) {
        let changes = context.system_state.time_window.changes_in_window;
        
        let passed = changes <= max_changes;
        
        let message = if passed {
            format!("{} changes in last {} minutes (max: {})", changes, time_window_minutes, max_changes)
        } else {
            format!("{} changes in last {} minutes exceeds maximum of {}", changes, time_window_minutes, max_changes)
        };
        
        let details = HashMap::from([
            ("changes".to_string(), changes.to_string()),
            ("max_changes".to_string(), max_changes.to_string()),
            ("window_minutes".to_string(), time_window_minutes.to_string()),
        ]);
        
        let suggestions = if !passed {
            vec![
                "Wait before making additional changes".to_string(),
                "Consider batching changes together".to_string(),
            ]
        } else {
            vec![]
        };
        
        (passed, message, details, suggestions)
    }
    
    /// Evaluate composite validation
    fn evaluate_composite(
        &amp;self,
        operator: CompositeOperator,
        rules: &amp;[String],
        context: &amp;ValidationContext,
    ) -> (bool, String, HashMap<String, String>, Vec<String>) {
        let results: Vec<bool> = rules
            .iter()
            .filter_map(|rule_id| {
                self.rules.get(rule_id).map(|rule| {
                    let (passed, _, _, _) = self.evaluate_rule(&amp;rule.rule_type, context);
                    passed
                })
            })
            .collect();
        
        let passed = match operator {
            CompositeOperator::And => results.iter().all(|&amp;p| p),
            CompositeOperator::Or => results.iter().any(|&amp;p| p),
            CompositeOperator::Xor => results.iter().filter(|&amp;&amp;p| p).count() == 1,
            CompositeOperator::Not => !results.iter().all(|&amp;p| p),
        };
        
        let operator_str = format!("{:?}", operator);
        let message = format!("Composite validation ({}) result: {}", operator_str, passed);
        
        let details = HashMap::from([
            ("operator".to_string(), operator_str),
            ("rules".to_string(), rules.join(", ")),
            ("result".to_string(), passed.to_string()),
        ]);
        
        let suggestions = if !passed {
            vec!["Review individual rule results".to_string()]
        } else {
            vec![]
        };
        
        (passed, message, details, suggestions)
    }
    
    /// Generate recommendations based on validation results
    fn generate_recommendations(&amp;self, results: &amp;[ValidationResult]) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();
        
        for result in results.iter().filter(|r| !r.passed) {
            let (recommendation_type, priority) = match result.severity {
                ValidationSeverity::Info => (RecommendationType::Review, RecommendationPriority::Low),
                ValidationSeverity::Warning => (RecommendationType::Review, RecommendationPriority::Medium),
                ValidationSeverity::Error => (RecommendationType::Fix, RecommendationPriority::High),
                ValidationSeverity::Critical => (RecommendationType::Fix, RecommendationPriority::Critical),
            };
            
            let actions: Vec<RecommendedAction> = result.suggestions.iter().map(|s| {
                RecommendedAction {
                    description: s.clone(),
                    parameter: None,
                    suggested_value: None,
                    automated: false,
                }
            }).collect();
            
            recommendations.push(Recommendation {
                recommendation_type,
                priority,
                title: format!("Address validation failure: {}", result.rule_id),
                description: result.message.clone(),
                actions,
                related_rules: vec![result.rule_id.clone()],
            });
        }
        
        // Sort by priority
        recommendations.sort_by(|a, b| b.priority.cmp(&amp;a.priority));
        
        recommendations
    }
    
    /// Update statistics
    fn update_stats(&amp;mut self, report: &amp;ValidationReport) {
        self.stats.total_validations += 1;
        
        if report.status == ValidationStatus::Valid || report.status == ValidationStatus::ValidWithWarnings {
            self.stats.successful_validations += 1;
        } else {
            self.stats.failed_validations += 1;
        }
        
        self.stats.average_execution_time_ms = 
            (self.stats.average_execution_time_ms * (self.stats.total_validations - 1) as f64 
                + report.summary.total_execution_time_ms as f64) 
            / self.stats.total_validations as f64;
        
        // Track most common failures
        for result in &amp;report.results {
            if !result.passed {
                *self.stats.most_common_failures.entry(result.rule_id.clone()).or_insert(0) += 1;
            }
        }
    }
    
    /// Get validation history
    pub fn get_history(&amp;self) -> &amp;[ValidationReport] {
        &amp;self.history
    }
    
    /// Get statistics
    pub fn get_stats(&amp;self) -> &amp;ValidationStats {
        &amp;self.stats
    }
    
    /// Clear history
    pub fn clear_history(&amp;mut self) {
        self.history.clear();
    }
    
    /// Get all rules
    pub fn get_rules(&amp;self) -> &amp;HashMap<String, ValidationRule> {
        &amp;self.rules
    }
    
    /// Get a specific rule
    pub fn get_rule(&amp;self, rule_id: &amp;str) -> Option<&amp;ValidationRule> {
        self.rules.get(rule_id)
    }
    
    /// Validate with specific rules only
    pub fn validate_with_rules(
        &amp;mut self,
        context: &amp;ValidationContext,
        rule_ids: &amp;[String],
    ) -> ValidationReport {
        let start_time = std::time::Instant::now();
        let optimization_id = context.optimization_metadata.optimization_id.clone();
        
        let mut results = Vec::new();
        let mut summary = ValidationSummary {
            total_rules: 0,
            passed: 0,
            failed: 0,
            warnings: 0,
            errors: 0,
            critical: 0,
            total_execution_time_ms: 0,
            skipped: 0,
        };
        
        for rule_id in rule_ids {
            if let Some(rule) = self.rules.get(rule_id) {
                if !rule.enabled {
                    summary.skipped += 1;
                    continue;
                }
                
                let result = self.execute_rule(rule, context);
                
                if !result.passed {
                    summary.failed += 1;
                    match result.severity {
                        ValidationSeverity::Warning => summary.warnings += 1,
                        ValidationSeverity::Error => summary.errors += 1,
                        ValidationSeverity::Critical => summary.critical += 1,
                        _ => {}
                    }
                } else {
                    summary.passed += 1;
                }
                
                summary.total_rules += 1;
                summary.total_execution_time_ms += result.execution_time_ms;
                
                results.push(result);
            }
        }
        
        let status = if summary.critical > 0 {
            ValidationStatus::Invalid
        } else if summary.errors > 0 {
            ValidationStatus::Invalid
        } else if summary.warnings > 0 {
            ValidationStatus::ValidWithWarnings
        } else {
            ValidationStatus::Valid
        };
        
        let recommendations = self.generate_recommendations(&amp;results);
        
        let report = ValidationReport {
            id: Uuid::new_v4().to_string(),
            optimization_id,
            status,
            results,
            summary,
            recommendations,
            generated_at: Utc::now(),
        };
        
        self.update_stats(&amp;report);
        self.history.push(report.clone());
        
        report
    }
    
    /// Export validation report as JSON
    pub fn export_report(&amp;self, report: &amp;ValidationReport) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(report)
    }
    
    /// Import validation report from JSON
    pub fn import_report(&amp;self, json: &amp;str) -> Result<ValidationReport, serde_json::Error> {
        serde_json::from_str(json)
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validation_framework_creation() {
        let config = ValidationConfig::default();
        let framework = ValidationFramework::new(config);
        assert_eq!(framework.rules.len(), 0);
        assert_eq!(framework.history.len(), 0);
    }
    
    #[test]
    fn test_rule_registration() {
        let mut framework = ValidationFramework::new(ValidationConfig::default());
        
        let rule = ValidationRule {
            id: "test_rule".to_string(),
            name: "Test Rule".to_string(),
            description: "A test validation rule".to_string(),
            rule_type: ValidationRuleType::Range {
                parameter: "test_param".to_string(),
                min: 0.0,
                max: 100.0,
                inclusive: true,
            },
            severity: ValidationSeverity::Error,
            enabled: true,
            execution_order: 0,
            dependencies: vec![],
            config: HashMap::new(),
        };
        
        framework.register_rule(rule);
        assert_eq!(framework.rules.len(), 1);
        assert!(framework.rules.contains_key("test_rule"));
    }
    
    #[test]
    fn test_range_validation() {
        let config = ValidationConfig::default();
        let framework = ValidationFramework::new(config);
        
        let mut parameters = HashMap::new();
        parameters.insert("test_param".to_string(), 50.0);
        
        let context = ValidationContext {
            parameters,
            previous_parameters: HashMap::new(),
            system_state: SystemState {
                operational_mode: "normal".to_string(),
                health_status: HealthStatus::Healthy,
                active_alerts: vec![],
                resource_utilization: HashMap::new(),
                time_window: TimeWindow {
                    start: Utc::now(),
                    end: Utc::now(),
                    changes_in_window: 0,
                },
            },
            optimization_metadata: OptimizationMetadata {
                optimization_id: "test_opt".to_string(),
                strategy: "test".to_string(),
                target: "test".to_string(),
                confidence: 1.0,
                source: "test".to_string(),
                generated_at: Utc::now(),
            },
            custom_data: HashMap::new(),
        };
        
        let (passed, _, _, _) = framework.evaluate_range("test_param", 0.0, 100.0, true, &amp;context);
        assert!(passed);
    }
    
    #[test]
    fn test_validation_context() {
        let context = ValidationContext {
            parameters: HashMap::new(),
            previous_parameters: HashMap::new(),
            system_state: SystemState {
                operational_mode: "normal".to_string(),
                health_status: HealthStatus::Healthy,
                active_alerts: vec![],
                resource_utilization: HashMap::new(),
                time_window: TimeWindow {
                    start: Utc::now(),
                    end: Utc::now(),
                    changes_in_window: 0,
                },
            },
            optimization_metadata: OptimizationMetadata {
                optimization_id: "test".to_string(),
                strategy: "test".to_string(),
                target: "test".to_string(),
                confidence: 1.0,
                source: "test".to_string(),
                generated_at: Utc::now(),
            },
            custom_data: HashMap::new(),
        };
        
        assert_eq!(context.optimization_metadata.optimization_id, "test");
    }
}