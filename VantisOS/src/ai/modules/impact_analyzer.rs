//! Impact Analyzer Module for VantisOS
//!
//! This module provides comprehensive impact analysis for proposed optimization
//! changes, including dependency tracking, risk assessment, and effect prediction.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, BTreeMap};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ============================================================================
// Core Types
// ============================================================================

/// Represents a proposed change for impact analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedChange {
    /// Unique identifier
    pub id: String,
    /// Parameter being changed
    pub parameter: String,
    /// Current value
    pub current_value: f64,
    /// Proposed new value
    pub proposed_value: f64,
    /// Type of change
    pub change_type: ChangeType,
    /// Reason for the change
    pub reason: String,
    /// Source of the change suggestion
    pub source: String,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// When this change was proposed
    pub proposed_at: DateTime<Utc>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Types of changes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    /// Configuration parameter change
    Configuration,
    /// Resource allocation change
    ResourceAllocation,
    /// Performance threshold change
    PerformanceThreshold,
    /// Scaling parameter change
    Scaling,
    /// Feature flag toggle
    FeatureFlag,
    /// Policy update
    Policy,
    /// Schedule change
    Schedule,
    /// Dependency update
    Dependency,
}

/// Result of impact analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysis {
    /// Unique identifier
    pub id: String,
    /// ID of the analyzed change
    pub change_id: String,
    /// Overall impact level
    pub overall_impact: ImpactLevel,
    /// Overall risk level
    pub overall_risk: RiskLevel,
    /// Detailed impact assessment
    pub impacts: Vec<Impact>,
    /// Risk assessment details
    pub risks: Vec<RiskAssessment>,
    /// Affected components
    pub affected_components: Vec<AffectedComponent>,
    /// Affected dependencies
    pub affected_dependencies: Vec<DependencyImpact>,
    /// Predicted effects
    pub predicted_effects: Vec<PredictedEffect>,
    /// Mitigation recommendations
    pub mitigations: Vec<MitigationRecommendation>,
    /// Cost-benefit analysis
    pub cost_benefit: CostBenefitAnalysis,
    /// When this analysis was performed
    pub analyzed_at: DateTime<Utc>,
    /// Duration of analysis in milliseconds
    pub analysis_duration_ms: u64,
}

/// Impact levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ImpactLevel {
    /// Negligible impact
    Negligible,
    /// Low impact
    Low,
    /// Medium impact
    Medium,
    /// High impact
    High,
    /// Critical impact
    Critical,
}

/// Risk levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Minimal risk
    Minimal,
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Critical risk
    Critical,
}

/// Individual impact assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Impact {
    /// Impact category
    pub category: ImpactCategory,
    /// Impact description
    pub description: String,
    /// Impact level
    pub level: ImpactLevel,
    /// Direction of impact
    pub direction: ImpactDirection,
    /// Magnitude of impact (0.0 to 1.0)
    pub magnitude: f64,
    /// Duration of impact
    pub duration: ImpactDuration,
    /// Reversibility
    pub reversible: bool,
    /// Confidence in the assessment
    pub confidence: f64,
}

/// Categories of impact
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactCategory {
    Performance,
    Availability,
    Cost,
    Security,
    Compliance,
    UserExperience,
    DataIntegrity,
    SystemStability,
    ResourceUtilization,
    Operational,
    Business,
    Technical,
}

/// Direction of impact
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactDirection {
    Positive,
    Negative,
    Neutral,
    Mixed,
}

/// Duration of impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactDuration {
    Immediate,
    ShortTerm { minutes: u64 },
    MediumTerm { hours: u64 },
    LongTerm { days: u64 },
    Permanent,
    Unknown,
}

/// Risk assessment details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Risk identifier
    pub id: String,
    /// Risk name
    pub name: String,
    /// Risk description
    pub description: String,
    /// Risk category
    pub category: RiskCategory,
    /// Risk level
    pub level: RiskLevel,
    /// Probability of risk materializing (0.0 to 1.0)
    pub probability: f64,
    /// Impact if risk materializes
    pub impact: f64,
    /// Risk score (probability * impact)
    pub score: f64,
    /// Mitigation strategies
    pub mitigations: Vec<String>,
    /// Early warning indicators
    pub indicators: Vec<String>,
}

/// Categories of risk
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskCategory {
    Operational,
    Financial,
    Security,
    Compliance,
    Performance,
    Reputational,
    Technical,
    Business,
}

/// Affected component information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedComponent {
    /// Component identifier
    pub id: String,
    /// Component name
    pub name: String,
    /// Component type
    pub component_type: ComponentType,
    /// How the component is affected
    pub effect: String,
    /// Severity of effect
    pub severity: ImpactLevel,
    /// Whether the component is critical
    pub is_critical: bool,
    /// Dependencies on this component
    pub dependents: Vec<String>,
}

/// Types of components
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentType {
    Service,
    Database,
    Cache,
    Queue,
    External,
    Infrastructure,
    Application,
    Storage,
    Network,
    Security,
}

/// Dependency impact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyImpact {
    /// Dependency identifier
    pub id: String,
    /// Dependency name
    pub name: String,
    /// Type of dependency
    pub dependency_type: DependencyType,
    /// Impact on the dependency
    pub impact: String,
    /// Whether the dependency is upstream or downstream
    pub direction: DependencyDirection,
    /// Severity of impact
    pub severity: ImpactLevel,
}

/// Types of dependencies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyType {
    Service,
    Data,
    Configuration,
    Infrastructure,
    External,
}

/// Direction of dependency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyDirection {
    Upstream,
    Downstream,
    Bidirectional,
}

/// Predicted effect of a change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedEffect {
    /// Effect identifier
    pub id: String,
    /// What will be affected
    pub target: String,
    /// Description of the effect
    pub description: String,
    /// Metric that will change
    pub metric: String,
    /// Current value
    pub current_value: f64,
    /// Predicted value after change
    pub predicted_value: f64,
    /// Change amount
    pub change_amount: f64,
    /// Change percentage
    pub change_percentage: f64,
    /// Time until effect manifests
    pub time_to_effect: TimeToEffect,
    /// Confidence in prediction
    pub confidence: f64,
    /// Historical accuracy for similar predictions
    pub historical_accuracy: Option<f64>,
}

/// Time until effect manifests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeToEffect {
    Immediate,
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
    Unknown,
}

/// Mitigation recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationRecommendation {
    /// Recommendation identifier
    pub id: String,
    /// What risk or impact this mitigates
    pub target: String,
    /// Recommendation description
    pub description: String,
    /// Type of mitigation
    pub mitigation_type: MitigationType,
    /// Priority of implementing this mitigation
    pub priority: MitigationPriority,
    /// Estimated effort to implement
    pub effort: EffortLevel,
    /// Estimated effectiveness (0.0 to 1.0)
    pub effectiveness: f64,
    /// Steps to implement
    pub steps: Vec<String>,
    /// Preconditions
    pub preconditions: Vec<String>,
}

/// Types of mitigation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MitigationType {
    Preventive,
    Detective,
    Corrective,
    Compensating,
    Deterrent,
}

/// Priority of mitigation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MitigationPriority {
    Low,
    Medium,
    High,
    Critical,
    Immediate,
}

/// Effort levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EffortLevel {
    Trivial,
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Cost-benefit analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostBenefitAnalysis {
    /// Estimated costs
    pub costs: CostBreakdown,
    /// Estimated benefits
    pub benefits: BenefitBreakdown,
    /// Net value
    pub net_value: f64,
    /// Return on investment percentage
    pub roi_percentage: f64,
    /// Payback period in days
    pub payback_period_days: Option<f64>,
    /// Break-even point
    pub break_even: Option<BreakEvenPoint>,
    /// Overall recommendation
    pub recommendation: CostBenefitRecommendation,
}

/// Cost breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostBreakdown {
    /// Implementation costs
    pub implementation: f64,
    /// Operational costs
    pub operational: f64,
    /// Risk costs (expected value)
    pub risk: f64,
    /// Opportunity costs
    pub opportunity: f64,
    /// Total costs
    pub total: f64,
}

/// Benefit breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitBreakdown {
    /// Performance improvements value
    pub performance: f64,
    /// Cost savings
    pub cost_savings: f64,
    /// Revenue increase
    pub revenue_increase: f64,
    /// Risk reduction value
    pub risk_reduction: f64,
    /// Operational efficiency
    pub operational_efficiency: f64,
    /// Total benefits
    pub total: f64,
}

/// Break-even point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakEvenPoint {
    /// Time to break-even
    pub time_to_break_even: TimeToEffect,
    /// Conditions for break-even
    pub conditions: Vec<String>,
}

/// Cost-benefit recommendation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CostBenefitRecommendation {
    StronglyRecommend,
    Recommend,
    Neutral,
    NotRecommended,
    StronglyNotRecommended,
}

// ============================================================================
// Impact Analyzer
// ============================================================================

/// Main impact analyzer
pub struct ImpactAnalyzer {
    /// Component dependency graph
    dependency_graph: HashMap<String, Vec<String>>,
    /// Historical impact data
    impact_history: Vec<ImpactAnalysis>,
    /// Impact rules
    impact_rules: Vec<ImpactRule>,
    /// Configuration
    config: ImpactAnalyzerConfig,
    /// Statistics
    stats: AnalyzerStats,
}

/// Impact rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactRule {
    /// Rule identifier
    pub id: String,
    /// Rule name
    pub name: String,
    /// Parameter pattern this rule applies to
    pub parameter_pattern: String,
    /// Conditions for the rule to apply
    pub conditions: Vec<RuleCondition>,
    /// Impacts when rule matches
    pub impacts: Vec<ImpactTemplate>,
    /// Risk factors
    pub risk_factors: Vec<RiskFactor>,
    /// Whether the rule is enabled
    pub enabled: bool,
}

/// Rule condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    /// Condition type
    pub condition_type: ConditionType,
    /// Parameter or metric to check
    pub target: String,
    /// Operator
    pub operator: ConditionOperator,
    /// Value to compare against
    pub value: f64,
}

/// Condition types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionType {
    ParameterValue,
    ChangeMagnitude,
    ChangePercentage,
    SystemLoad,
    TimeOfDay,
    DayOfWeek,
}

/// Condition operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    Between,
    In,
}

/// Impact template for rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactTemplate {
    /// Impact category
    pub category: ImpactCategory,
    /// Impact level
    pub level: ImpactLevel,
    /// Impact description template
    pub description_template: String,
    /// Confidence level
    pub confidence: f64,
}

/// Risk factor definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Risk factor name
    pub name: String,
    /// Risk category
    pub category: RiskCategory,
    /// Base probability
    pub base_probability: f64,
    /// Modifiers based on conditions
    pub modifiers: Vec<ProbabilityModifier>,
}

/// Probability modifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbabilityModifier {
    /// Condition for the modifier
    pub condition: String,
    /// Probability adjustment
    pub adjustment: f64,
}

/// Configuration for impact analyzer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalyzerConfig {
    /// Maximum analysis history to keep
    pub max_history: usize,
    /// Enable predictive modeling
    pub enable_predictions: bool,
    /// Confidence threshold for predictions
    pub prediction_confidence_threshold: f64,
    /// Enable cost-benefit analysis
    pub enable_cost_benefit: bool,
    /// Default cost unit (e.g., USD)
    pub cost_unit: String,
    /// Enable dependency analysis
    pub enable_dependency_analysis: bool,
    /// Maximum dependency depth
    pub max_dependency_depth: u32,
}

/// Analyzer statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnalyzerStats {
    /// Total analyses performed
    pub total_analyses: u64,
    /// Average analysis duration
    pub avg_duration_ms: f64,
    /// Prediction accuracy rate
    pub prediction_accuracy: f64,
    /// Most common impact categories
    pub common_impacts: HashMap<String, u64>,
}

impl Default for ImpactAnalyzerConfig {
    fn default() -> Self {
        Self {
            max_history: 1000,
            enable_predictions: true,
            prediction_confidence_threshold: 0.7,
            enable_cost_benefit: true,
            cost_unit: "USD".to_string(),
            enable_dependency_analysis: true,
            max_dependency_depth: 5,
        }
    }
}

impl ImpactAnalyzer {
    /// Create a new impact analyzer
    pub fn new(config: ImpactAnalyzerConfig) -> Self {
        Self {
            dependency_graph: HashMap::new(),
            impact_history: Vec::new(),
            impact_rules: Vec::new(),
            config,
            stats: AnalyzerStats::default(),
        }
    }
    
    /// Register a dependency relationship
    pub fn register_dependency(&mut self, component: String, depends_on: String) {
        self.dependency_graph
            .entry(component)
            .or_insert_with(Vec::new)
            .push(depends_on);
    }
    
    /// Add an impact rule
    pub fn add_rule(&mut self, rule: ImpactRule) {
        self.impact_rules.push(rule);
    }
    
    /// Analyze a proposed change
    pub fn analyze(&mut self, change: &ProposedChange) -> ImpactAnalysis {
        let start_time = std::time::Instant::now();
        
        // Assess direct impacts
        let impacts = self.assess_impacts(change);
        
        // Assess risks
        let risks = self.assess_risks(change, &impacts);
        
        // Identify affected components
        let affected_components = self.identify_affected_components(change);
        
        // Analyze dependencies
        let affected_dependencies = self.analyze_dependencies(change);
        
        // Predict effects
        let predicted_effects = self.predict_effects(change);
        
        // Generate mitigations
        let mitigations = self.generate_mitigations(&risks, &impacts);
        
        // Calculate cost-benefit
        let cost_benefit = self.calculate_cost_benefit(change, &impacts, &risks);
        
        // Determine overall impact and risk
        let overall_impact = self.calculate_overall_impact(&impacts);
        let overall_risk = self.calculate_overall_risk(&risks);
        
        let analysis = ImpactAnalysis {
            id: Uuid::new_v4().to_string(),
            change_id: change.id.clone(),
            overall_impact,
            overall_risk,
            impacts,
            risks,
            affected_components,
            affected_dependencies,
            predicted_effects,
            mitigations,
            cost_benefit,
            analyzed_at: Utc::now(),
            analysis_duration_ms: start_time.elapsed().as_millis() as u64,
        };
        
        // Update statistics
        self.update_stats(&analysis);
        
        // Store in history
        self.impact_history.push(analysis.clone());
        if self.impact_history.len() > self.config.max_history {
            self.impact_history.remove(0);
        }
        
        analysis
    }
    
    /// Assess impacts of a change
    fn assess_impacts(&self, change: &ProposedChange) -> Vec<Impact> {
        let mut impacts = Vec::new();
        
        // Calculate change magnitude
        let change_magnitude = (change.proposed_value - change.current_value).abs();
        let change_percentage = if change.current_value.abs() > 1e-10 {
            (change.proposed_value - change.current_value) / change.current_value.abs() * 100.0
        } else {
            0.0
        };
        
        // Apply impact rules
        for rule in &self.impact_rules {
            if !rule.enabled {
                continue;
            }
            
            // Check if rule applies
            if self.rule_applies(rule, change) {
                for template in &rule.impacts {
                    let impact = Impact {
                        category: template.category,
                        description: template.description_template
                            .replace("{parameter}", &change.parameter)
                            .replace("{change}", &format!("{:.2}", change_magnitude)),
                        level: template.level,
                        direction: self.determine_impact_direction(change),
                        magnitude: change_magnitude.min(1.0),
                        duration: ImpactDuration::MediumTerm { hours: 1 },
                        reversible: true,
                        confidence: template.confidence,
                    };
                    impacts.push(impact);
                }
            }
        }
        
        // Add default impacts based on change magnitude
        if change_magnitude > 0.1 {
            impacts.push(Impact {
                category: ImpactCategory::Performance,
                description: format!("Performance impact from {} change", change.parameter),
                level: self.magnitude_to_impact_level(change_magnitude),
                direction: ImpactDirection::Mixed,
                magnitude: change_magnitude.min(1.0),
                duration: ImpactDuration::ShortTerm { minutes: 30 },
                reversible: true,
                confidence: 0.8,
            });
        }
        
        impacts
    }
    
    /// Check if a rule applies to a change
    fn rule_applies(&self, rule: &ImpactRule, change: &ProposedChange) -> bool {
        // Simple pattern matching on parameter name
        if !change.parameter.contains(&rule.parameter_pattern) && 
           !rule.parameter_pattern.contains(&change.parameter) {
            return false;
        }
        
        // Check conditions
        for condition in &rule.conditions {
            if !self.evaluate_condition(condition, change) {
                return false;
            }
        }
        
        true
    }
    
    /// Evaluate a rule condition
    fn evaluate_condition(&self, condition: &RuleCondition, change: &ProposedChange) -> bool {
        let value = match condition.condition_type {
            ConditionType::ParameterValue => change.proposed_value,
            ConditionType::ChangeMagnitude => (change.proposed_value - change.current_value).abs(),
            ConditionType::ChangePercentage => {
                if change.current_value.abs() > 1e-10 {
                    ((change.proposed_value - change.current_value) / change.current_value).abs() * 100.0
                } else {
                    0.0
                }
            }
            _ => return true, // Skip unimplemented conditions
        };
        
        match condition.operator {
            ConditionOperator::LessThan => value < condition.value,
            ConditionOperator::LessThanOrEqual => value <= condition.value,
            ConditionOperator::GreaterThan => value > condition.value,
            ConditionOperator::GreaterThanOrEqual => value >= condition.value,
            ConditionOperator::Equal => (value - condition.value).abs() < 1e-10,
            ConditionOperator::NotEqual => (value - condition.value).abs() >= 1e-10,
            ConditionOperator::Between => false, // Would need two values
            ConditionOperator::In => false, // Would need multiple values
        }
    }
    
    /// Determine impact direction
    fn determine_impact_direction(&self, change: &ProposedChange) -> ImpactDirection {
        let change_delta = change.proposed_value - change.current_value;
        
        // Heuristic: assume increases are positive for performance, negative for cost
        if change.parameter.to_lowercase().contains("cost") ||
           change.parameter.to_lowercase().contains("price") {
            if change_delta > 0.0 {
                ImpactDirection::Negative
            } else {
                ImpactDirection::Positive
            }
        } else if change.parameter.to_lowercase().contains("performance") ||
                  change.parameter.to_lowercase().contains("throughput") {
            if change_delta > 0.0 {
                ImpactDirection::Positive
            } else {
                ImpactDirection::Negative
            }
        } else {
            ImpactDirection::Neutral
        }
    }
    
    /// Assess risks of a change
    fn assess_risks(&self, change: &ProposedChange, impacts: &[Impact]) -> Vec<RiskAssessment> {
        let mut risks = Vec::new();
        
        // Assess risk based on change magnitude
        let change_magnitude = (change.proposed_value - change.current_value).abs();
        
        if change_magnitude > 0.5 {
            risks.push(RiskAssessment {
                id: Uuid::new_v4().to_string(),
                name: "Large Change Risk".to_string(),
                description: format!("Large change magnitude ({:.2}) increases risk of instability", change_magnitude),
                category: RiskCategory::Operational,
                level: RiskLevel::Medium,
                probability: 0.3 + change_magnitude * 0.2,
                impact: 0.5,
                score: (0.3 + change_magnitude * 0.2) * 0.5,
                mitigations: vec![
                    "Consider breaking change into smaller steps".to_string(),
                    "Implement monitoring for early warning signs".to_string(),
                ],
                indicators: vec![
                    "Increased error rates".to_string(),
                    "Latency spikes".to_string(),
                    "Resource contention".to_string(),
                ],
            });
        }
        
        // Assess risk based on low confidence
        if change.confidence < 0.7 {
            risks.push(RiskAssessment {
                id: Uuid::new_v4().to_string(),
                name: "Low Confidence Risk".to_string(),
                description: format!("Low confidence ({:.2}) in change suggestion", change.confidence),
                category: RiskCategory::Technical,
                level: RiskLevel::Low,
                probability: 1.0 - change.confidence,
                impact: 0.4,
                score: (1.0 - change.confidence) * 0.4,
                mitigations: vec![
                    "Gather more data before applying change".to_string(),
                    "Start with smaller rollout".to_string(),
                ],
                indicators: vec![
                    "Unexpected behavior".to_string(),
                    "Inconsistent results".to_string(),
                ],
            });
        }
        
        // Add risks from impact assessment
        for impact in impacts {
            if impact.level >= ImpactLevel::High {
                risks.push(RiskAssessment {
                    id: Uuid::new_v4().to_string(),
                    name: format!("{:?} Impact Risk", impact.category),
                    description: impact.description.clone(),
                    category: RiskCategory::Operational,
                    level: self.impact_to_risk_level(impact.level),
                    probability: 0.5,
                    impact: impact.magnitude,
                    score: 0.5 * impact.magnitude,
                    mitigations: vec!["Monitor closely during and after change".to_string()],
                    indicators: vec![format!("Changes in {:?} metrics", impact.category)],
                });
            }
        }
        
        risks
    }
    
    /// Identify affected components
    fn identify_affected_components(&self, change: &ProposedChange) -> Vec<AffectedComponent> {
        let mut components = Vec::new();
        
        // Parse parameter to identify component
        let parts: Vec<&str> = change.parameter.split('.').collect();
        
        if !parts.is_empty() {
            let component_name = parts[0].to_string();
            let effect = if parts.len() > 1 {
                format!("Configuration change in {}", parts[1..].join("."))
            } else {
                "Direct configuration change".to_string()
            };
            
            components.push(AffectedComponent {
                id: format!("comp_{}", component_name),
                name: component_name.clone(),
                component_type: ComponentType::Service,
                effect,
                severity: self.magnitude_to_impact_level(
                    (change.proposed_value - change.current_value).abs()
                ),
                is_critical: false,
                dependents: self.dependency_graph
                    .iter()
                    .filter(|(_, deps)| deps.contains(&component_name))
                    .map(|(name, _)| name.clone())
                    .collect(),
            });
        }
        
        components
    }
    
    /// Analyze dependency impacts
    fn analyze_dependencies(&self, change: &ProposedChange) -> Vec<DependencyImpact> {
        let mut dependencies = Vec::new();
        
        // Find components that depend on the changed parameter's component
        let parts: Vec<&str> = change.parameter.split('.').collect();
        if !parts.is_empty() {
            let component = parts[0];
            
            if let Some(deps) = self.dependency_graph.get(component) {
                for dep in deps {
                    dependencies.push(DependencyImpact {
                        id: Uuid::new_v4().to_string(),
                        name: dep.clone(),
                        dependency_type: DependencyType::Service,
                        impact: format!("May be affected by {} change", change.parameter),
                        direction: DependencyDirection::Downstream,
                        severity: ImpactLevel::Low,
                    });
                }
            }
        }
        
        dependencies
    }
    
    /// Predict effects of a change
    fn predict_effects(&self, change: &ProposedChange) -> Vec<PredictedEffect> {
        if !self.config.enable_predictions {
            return Vec::new();
        }
        
        let mut effects = Vec::new();
        
        // Predict performance effect
        effects.push(PredictedEffect {
            id: Uuid::new_v4().to_string(),
            target: change.parameter.clone(),
            description: format!("Predicted change in {}", change.parameter),
            metric: change.parameter.clone(),
            current_value: change.current_value,
            predicted_value: change.proposed_value,
            change_amount: change.proposed_value - change.current_value,
            change_percentage: if change.current_value.abs() > 1e-10 {
                (change.proposed_value - change.current_value) / change.current_value * 100.0
            } else {
                0.0
            },
            time_to_effect: TimeToEffect::Minutes(5),
            confidence: change.confidence,
            historical_accuracy: None,
        });
        
        // Predict secondary effects based on change type
        match change.change_type {
            ChangeType::Configuration => {
                effects.push(PredictedEffect {
                    id: Uuid::new_v4().to_string(),
                    target: "system.stability".to_string(),
                    description: "Predicted effect on system stability".to_string(),
                    metric: "stability_score".to_string(),
                    current_value: 0.95,
                    predicted_value: 0.95 - (change.proposed_value - change.current_value).abs() * 0.01,
                    change_amount: -(change.proposed_value - change.current_value).abs() * 0.01,
                    change_percentage: -1.0,
                    time_to_effect: TimeToEffect::Immediate,
                    confidence: 0.7,
                    historical_accuracy: Some(0.85),
                });
            }
            ChangeType::ResourceAllocation => {
                effects.push(PredictedEffect {
                    id: Uuid::new_v4().to_string(),
                    target: "resource.utilization".to_string(),
                    description: "Predicted change in resource utilization".to_string(),
                    metric: "utilization_percentage".to_string(),
                    current_value: 70.0,
                    predicted_value: 70.0 + (change.proposed_value - change.current_value) * 10.0,
                    change_amount: (change.proposed_value - change.current_value) * 10.0,
                    change_percentage: (change.proposed_value - change.current_value) * 10.0 / 70.0 * 100.0,
                    time_to_effect: TimeToEffect::Minutes(1),
                    confidence: 0.9,
                    historical_accuracy: Some(0.92),
                });
            }
            _ => {}
        }
        
        effects
    }
    
    /// Generate mitigation recommendations
    fn generate_mitigations(&self, risks: &[RiskAssessment], impacts: &[Impact]) -> Vec<MitigationRecommendation> {
        let mut mitigations = Vec::new();
        
        for risk in risks {
            if risk.level >= RiskLevel::Medium {
                mitigations.push(MitigationRecommendation {
                    id: Uuid::new_v4().to_string(),
                    target: risk.id.clone(),
                    description: format!("Mitigate {} risk", risk.name),
                    mitigation_type: MitigationType::Preventive,
                    priority: self.risk_to_mitigation_priority(risk.level),
                    effort: EffortLevel::Medium,
                    effectiveness: 0.7,
                    steps: risk.mitigations.clone(),
                    preconditions: vec!["Monitoring in place".to_string()],
                });
            }
        }
        
        for impact in impacts {
            if impact.level >= ImpactLevel::High {
                mitigations.push(MitigationRecommendation {
                    id: Uuid::new_v4().to_string(),
                    target: format!("{:?}_impact", impact.category),
                    description: format!("Mitigate {:?} impact", impact.category),
                    mitigation_type: MitigationType::Corrective,
                    priority: self.impact_to_mitigation_priority(impact.level),
                    effort: EffortLevel::Low,
                    effectiveness: 0.6,
                    steps: vec![
                        "Monitor impact metrics".to_string(),
                        "Prepare rollback plan".to_string(),
                        "Notify affected stakeholders".to_string(),
                    ],
                    preconditions: vec![],
                });
            }
        }
        
        mitigations.sort_by(|a, b| b.priority.cmp(&a.priority));
        mitigations
    }
    
    /// Calculate cost-benefit analysis
    fn calculate_cost_benefit(
        &self,
        change: &ProposedChange,
        impacts: &[Impact],
        risks: &[RiskAssessment],
    ) -> CostBenefitAnalysis {
        if !self.config.enable_cost_benefit {
            return CostBenefitAnalysis {
                costs: CostBreakdown {
                    implementation: 0.0,
                    operational: 0.0,
                    risk: 0.0,
                    opportunity: 0.0,
                    total: 0.0,
                },
                benefits: BenefitBreakdown {
                    performance: 0.0,
                    cost_savings: 0.0,
                    revenue_increase: 0.0,
                    risk_reduction: 0.0,
                    operational_efficiency: 0.0,
                    total: 0.0,
                },
                net_value: 0.0,
                roi_percentage: 0.0,
                payback_period_days: None,
                break_even: None,
                recommendation: CostBenefitRecommendation::Neutral,
            };
        }
        
        // Calculate costs
        let implementation = 50.0; // Base implementation cost
        let operational = 10.0; // Ongoing operational cost
        let risk = risks.iter().map(|r| r.score * 100.0).sum();
        let opportunity = 20.0; // Opportunity cost
        
        let total_costs = implementation + operational + risk + opportunity;
        
        // Calculate benefits
        let performance = impacts.iter()
            .filter(|i| i.category == ImpactCategory::Performance && i.direction == ImpactDirection::Positive)
            .map(|i| i.magnitude * 100.0)
            .sum();
        let cost_savings = if change.parameter.to_lowercase().contains("cost") {
            (change.current_value - change.proposed_value).max(0.0) * 50.0
        } else {
            0.0
        };
        let risk_reduction = risks.iter()
            .filter(|r| r.level <= RiskLevel::Low)
            .map(|r| r.score * 20.0)
            .sum();
        let operational_efficiency = change.confidence * 30.0;
        
        let total_benefits = performance + cost_savings + risk_reduction + operational_efficiency;
        
        // Calculate net value and ROI
        let net_value = total_benefits - total_costs;
        let roi_percentage = if total_costs > 0.0 {
            (net_value / total_costs) * 100.0
        } else {
            0.0
        };
        
        // Determine recommendation
        let recommendation = if roi_percentage > 50.0 {
            CostBenefitRecommendation::StronglyRecommend
        } else if roi_percentage > 20.0 {
            CostBenefitRecommendation::Recommend
        } else if roi_percentage > -20.0 {
            CostBenefitRecommendation::Neutral
        } else if roi_percentage > -50.0 {
            CostBenefitRecommendation::NotRecommended
        } else {
            CostBenefitRecommendation::StronglyNotRecommended
        };
        
        CostBenefitAnalysis {
            costs: CostBreakdown {
                implementation,
                operational,
                risk,
                opportunity,
                total: total_costs,
            },
            benefits: BenefitBreakdown {
                performance,
                cost_savings,
                revenue_increase: 0.0,
                risk_reduction,
                operational_efficiency,
                total: total_benefits,
            },
            net_value,
            roi_percentage,
            payback_period_days: if net_value > 0.0 { Some(total_costs / net_value * 30.0) } else { None },
            break_even: None,
            recommendation,
        }
    }
    
    /// Calculate overall impact level
    fn calculate_overall_impact(&self, impacts: &[Impact]) -> ImpactLevel {
        impacts.iter()
            .map(|i| i.level)
            .max()
            .unwrap_or(ImpactLevel::Negligible)
    }
    
    /// Calculate overall risk level
    fn calculate_overall_risk(&self, risks: &[RiskAssessment]) -> RiskLevel {
        risks.iter()
            .map(|r| r.level)
            .max()
            .unwrap_or(RiskLevel::Minimal)
    }
    
    /// Convert magnitude to impact level
    fn magnitude_to_impact_level(&self, magnitude: f64) -> ImpactLevel {
        if magnitude > 0.8 {
            ImpactLevel::Critical
        } else if magnitude > 0.6 {
            ImpactLevel::High
        } else if magnitude > 0.4 {
            ImpactLevel::Medium
        } else if magnitude > 0.2 {
            ImpactLevel::Low
        } else {
            ImpactLevel::Negligible
        }
    }
    
    /// Convert impact level to risk level
    fn impact_to_risk_level(&self, level: ImpactLevel) -> RiskLevel {
        match level {
            ImpactLevel::Critical => RiskLevel::Critical,
            ImpactLevel::High => RiskLevel::High,
            ImpactLevel::Medium => RiskLevel::Medium,
            ImpactLevel::Low => RiskLevel::Low,
            ImpactLevel::Negligible => RiskLevel::Minimal,
        }
    }
    
    /// Convert risk level to mitigation priority
    fn risk_to_mitigation_priority(&self, level: RiskLevel) -> MitigationPriority {
        match level {
            RiskLevel::Critical => MitigationPriority::Immediate,
            RiskLevel::High => MitigationPriority::Critical,
            RiskLevel::Medium => MitigationPriority::High,
            RiskLevel::Low => MitigationPriority::Medium,
            RiskLevel::Minimal => MitigationPriority::Low,
        }
    }
    
    /// Convert impact level to mitigation priority
    fn impact_to_mitigation_priority(&self, level: ImpactLevel) -> MitigationPriority {
        match level {
            ImpactLevel::Critical => MitigationPriority::Immediate,
            ImpactLevel::High => MitigationPriority::Critical,
            ImpactLevel::Medium => MitigationPriority::High,
            ImpactLevel::Low => MitigationPriority::Medium,
            ImpactLevel::Negligible => MitigationPriority::Low,
        }
    }
    
    /// Update statistics
    fn update_stats(&mut self, analysis: &ImpactAnalysis) {
        self.stats.total_analyses += 1;
        self.stats.avg_duration_ms = 
            (self.stats.avg_duration_ms * (self.stats.total_analyses - 1) as f64 
                + analysis.analysis_duration_ms as f64) 
            / self.stats.total_analyses as f64;
        
        for impact in &analysis.impacts {
            let category = format!("{:?}", impact.category);
            *self.stats.common_impacts.entry(category).or_insert(0) += 1;
        }
    }
    
    /// Get analysis history
    pub fn get_history(&self) -> &[ImpactAnalysis] {
        &self.impact_history
    }
    
    /// Get statistics
    pub fn get_stats(&self) -> &AnalyzerStats {
        &self.stats
    }
    
    /// Clear history
    pub fn clear_history(&mut self) {
        self.impact_history.clear();
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_impact_analyzer_creation() {
        let config = ImpactAnalyzerConfig::default();
        let analyzer = ImpactAnalyzer::new(config);
        assert_eq!(analyzer.impact_history.len(), 0);
    }
    
    #[test]
    fn test_proposed_change() {
        let change = ProposedChange {
            id: "change1".to_string(),
            parameter: "service.timeout".to_string(),
            current_value: 30.0,
            proposed_value: 60.0,
            change_type: ChangeType::Configuration,
            reason: "Reduce timeout errors".to_string(),
            source: "optimization".to_string(),
            confidence: 0.85,
            proposed_at: Utc::now(),
            metadata: HashMap::new(),
        };
        
        assert_eq!(change.parameter, "service.timeout");
        assert_eq!(change.confidence, 0.85);
    }
    
    #[test]
    fn test_impact_analysis() {
        let config = ImpactAnalyzerConfig::default();
        let mut analyzer = ImpactAnalyzer::new(config);
        
        let change = ProposedChange {
            id: "change1".to_string(),
            parameter: "service.timeout".to_string(),
            current_value: 30.0,
            proposed_value: 60.0,
            change_type: ChangeType::Configuration,
            reason: "Reduce timeout errors".to_string(),
            source: "optimization".to_string(),
            confidence: 0.85,
            proposed_at: Utc::now(),
            metadata: HashMap::new(),
        };
        
        let analysis = analyzer.analyze(&change);
        
        assert!(!analysis.impacts.is_empty() || !analysis.risks.is_empty());
        assert_eq!(analysis.change_id, "change1");
    }
    
    #[test]
    fn test_dependency_registration() {
        let mut analyzer = ImpactAnalyzer::new(ImpactAnalyzerConfig::default());
        
        analyzer.register_dependency("service_a".to_string(), "service_b".to_string());
        analyzer.register_dependency("service_a".to_string(), "service_c".to_string());
        
        assert!(analyzer.dependency_graph.contains_key("service_a"));
        assert_eq!(analyzer.dependency_graph.get("service_a").unwrap().len(), 2);
    }
    
    #[test]
    fn test_impact_level_ordering() {
        assert!(ImpactLevel::Critical > ImpactLevel::High);
        assert!(ImpactLevel::High > ImpactLevel::Medium);
        assert!(ImpactLevel::Medium > ImpactLevel::Low);
        assert!(ImpactLevel::Low > ImpactLevel::Negligible);
    }
    
    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::Critical > RiskLevel::High);
        assert!(RiskLevel::High > RiskLevel::Medium);
        assert!(RiskLevel::Medium > RiskLevel::Low);
        assert!(RiskLevel::Low > RiskLevel::Minimal);
    }
}