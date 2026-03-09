//! AI Ethics Compliance Module
//!
//! This module provides ethics oversight for AI systems, ensuring that
//! AI decisions align with ethical principles and organizational values.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Ethical principles for AI systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EthicalPrinciple {
    /// Fairness and non-discrimination
    Fairness,
    /// Transparency and explainability
    Transparency,
    /// Accountability and responsibility
    Accountability,
    /// Privacy and data protection
    Privacy,
    /// Safety and reliability
    Safety,
    /// Human oversight and control
    HumanOversight,
    /// Beneficence - do good
    Beneficence,
    /// Non-maleficence - do no harm
    NonMaleficence,
    /// Respect for human autonomy
    Autonomy,
    /// Environmental sustainability
    Sustainability,
}

/// Ethics violation severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Ethics violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicsViolation {
    /// Violation ID
    pub id: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Violated principle
    pub principle: EthicalPrinciple,
    /// Severity
    pub severity: ViolationSeverity,
    /// Description
    pub description: String,
    /// Affected decision ID
    pub decision_id: Option<String>,
    /// Evidence
    pub evidence: Vec<String>,
    /// Affected users
    pub affected_users: Vec<String>,
    /// Recommended actions
    pub recommended_actions: Vec<String>,
    /// Status
    pub status: ViolationStatus,
    /// Resolution notes
    pub resolution_notes: Option<String>,
    /// Resolved by
    pub resolved_by: Option<String>,
    /// Resolution timestamp
    pub resolved_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Violation status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ViolationStatus {
    Detected,
    UnderInvestigation,
    Confirmed,
    Resolved,
    Dismissed,
}

/// Ethics review result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicsReview {
    /// Review ID
    pub id: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Decision ID being reviewed
    pub decision_id: String,
    /// Review type
    pub review_type: ReviewType,
    /// Principle scores
    pub principle_scores: HashMap<EthicalPrinciple, f64>,
    /// Overall ethical score (0.0-1.0)
    pub overall_score: f64,
    /// Passed ethics check
    pub passed: bool,
    /// Concerns identified
    pub concerns: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Reviewer
    pub reviewer: Option<String>,
}

/// Type of ethics review
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReviewType {
    /// Pre-decision review
    PreDecision,
    /// Post-decision review
    PostDecision,
    /// Periodic audit
    PeriodicAudit,
    /// Incident review
    IncidentReview,
}

/// Ethics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicsConfig {
    /// Enable ethics monitoring
    pub enabled: bool,
    /// Principles to enforce
    pub enforced_principles: Vec<EthicalPrinciple>,
    /// Minimum ethical score threshold
    pub min_score_threshold: f64,
    /// Require human review for low scores
    pub require_human_review_below: f64,
    /// Auto-flag violations
    pub auto_flag_violations: bool,
    /// Incident reporting required for
    pub incident_reporting_severity: ViolationSeverity,
}

impl Default for EthicsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            enforced_principles: vec![
                EthicalPrinciple::Fairness,
                EthicalPrinciple::Transparency,
                EthicalPrinciple::Privacy,
                EthicalPrinciple::Safety,
                EthicalPrinciple::Accountability,
            ],
            min_score_threshold: 0.6,
            require_human_review_below: 0.7,
            auto_flag_violations: true,
            incident_reporting_severity: ViolationSeverity::High,
        }
    }
}

/// Ethics statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicsStats {
    /// Total reviews performed
    pub total_reviews: usize,
    /// Reviews passed
    pub reviews_passed: usize,
    /// Reviews failed
    pub reviews_failed: usize,
    /// Violations detected
    pub violations_detected: usize,
    /// By principle
    pub violations_by_principle: HashMap<EthicalPrinciple, usize>,
    /// By severity
    pub violations_by_severity: HashMap<ViolationSeverity, usize>,
    /// Average ethical score
    pub avg_ethical_score: f64,
    /// Human reviews required
    pub human_reviews_required: usize,
}

/// Ethics Manager
pub struct EthicsManager {
    config: EthicsConfig,
    reviews: Arc<RwLock<Vec<EthicsReview>>>,
    violations: Arc<RwLock<Vec<EthicsViolation>>>,
    statistics: Arc<RwLock<EthicsStats>>,
    principle_weights: Arc<RwLock<HashMap<EthicalPrinciple, f64>>>,
}

impl EthicsManager {
    /// Create a new ethics manager
    pub fn new() -> Result<Self> {
        info!("Initializing Ethics Manager");

        let mut principle_weights = HashMap::new();
        principle_weights.insert(EthicalPrinciple::Fairness, 0.2);
        principle_weights.insert(EthicalPrinciple::Transparency, 0.15);
        principle_weights.insert(EthicalPrinciple::Accountability, 0.15);
        principle_weights.insert(EthicalPrinciple::Privacy, 0.15);
        principle_weights.insert(EthicalPrinciple::Safety, 0.15);
        principle_weights.insert(EthicalPrinciple::HumanOversight, 0.1);
        principle_weights.insert(EthicalPrinciple::Beneficence, 0.05);
        principle_weights.insert(EthicalPrinciple::NonMaleficence, 0.05);

        Ok(Self {
            config: EthicsConfig::default(),
            reviews: Arc::new(RwLock::new(Vec::new())),
            violations: Arc::new(RwLock::new(Vec::new())),
            statistics: Arc::new(RwLock::new(EthicsStats {
                total_reviews: 0,
                reviews_passed: 0,
                reviews_failed: 0,
                violations_detected: 0,
                violations_by_principle: HashMap::new(),
                violations_by_severity: HashMap::new(),
                avg_ethical_score: 0.0,
                human_reviews_required: 0,
            })),
            principle_weights: Arc::new(RwLock::new(principle_weights)),
        })
    }

    /// Review a decision for ethical compliance
    pub async fn review_decision(
        &self,
        decision_id: String,
        review_type: ReviewType,
        context: &EthicsContext,
    ) -> Result<EthicsReview> {
        let mut principle_scores = HashMap::new();
        let mut concerns = Vec::new();
        let mut recommendations = Vec::new();

        // Evaluate each principle
        for &principle in &self.config.enforced_principles {
            let (score, principle_concerns, principle_recommendations) = 
                self.evaluate_principle(principle, context).await?;
            
            principle_scores.insert(principle, score);
            concerns.extend(principle_concerns);
            recommendations.extend(principle_recommendations);

            // Check for violations
            if score < self.config.min_score_threshold && self.config.auto_flag_violations {
                let severity = self.determine_severity(score, principle);
                self.create_violation(
                    principle,
                    severity,
                    format!(
                        "Ethical principle {:?} violated with score {:.2}",
                        principle, score
                    ),
                    Some(decision_id.clone()),
                ).await?;
            }
        }

        // Calculate overall weighted score
        let weights = self.principle_weights.read().await;
        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for (principle, score) in &principle_scores {
            let weight = weights.get(principle).unwrap_or(&0.1);
            weighted_sum += score * weight;
            total_weight += weight;
        }

        let overall_score = if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.5
        };

        let passed = overall_score >= self.config.min_score_threshold;

        let review = EthicsReview {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            decision_id,
            review_type,
            principle_scores,
            overall_score,
            passed,
            concerns,
            recommendations,
            reviewer: None,
        };

        // Store and update statistics
        {
            let mut reviews = self.reviews.write().await;
            reviews.push(review.clone());
        }

        self.update_review_statistics(&review).await;

        // Check if human review is needed
        if overall_score < self.config.require_human_review_below {
            let mut stats = self.statistics.write().await;
            stats.human_reviews_required += 1;
            warn!(
                "Decision {} requires human review (ethical score: {:.2})",
                review.decision_id, overall_score
            );
        }

        info!(
            "Ethics review completed for {}: score={:.2}, passed={}",
            review.decision_id, overall_score, passed
        );

        Ok(review)
    }

    /// Evaluate a single ethical principle
    async fn evaluate_principle(
        &self,
        principle: EthicalPrinciple,
        context: &EthicsContext,
    ) -> Result<(f64, Vec<String>, Vec<String>)> {
        let mut concerns = Vec::new();
        let mut recommendations = Vec::new();
        let mut score = 1.0;

        match principle {
            EthicalPrinciple::Fairness => {
                // Check for fairness indicators
                if let Some(bias_detected) = &context.bias_detected {
                    if *bias_detected {
                        score -= 0.5;
                        concerns.push("Potential bias detected in decision".to_string());
                        recommendations.push("Implement bias mitigation strategies".to_string());
                    }
                }
                
                if let Some(disparate_impact) = context.disparate_impact {
                    if disparate_impact > 0.2 {
                        score -= 0.3;
                        concerns.push("High disparate impact detected".to_string());
                        recommendations.push("Review and adjust decision criteria".to_string());
                    }
                }
            }
            EthicalPrinciple::Transparency => {
                if !context.has_explanation {
                    score -= 0.4;
                    concerns.push("No explanation provided for decision".to_string());
                    recommendations.push("Generate and provide decision explanation".to_string());
                }

                if context.confidence.is_none() {
                    score -= 0.2;
                    concerns.push("Confidence score not provided".to_string());
                    recommendations.push("Include confidence scores in decisions".to_string());
                }
            }
            EthicalPrinciple::Accountability => {
                if !context.audit_trail_enabled {
                    score -= 0.5;
                    concerns.push("Audit trail not enabled".to_string());
                    recommendations.push("Enable audit trail for all decisions".to_string());
                }
            }
            EthicalPrinciple::Privacy => {
                if !context.privacy_compliant {
                    score -= 0.6;
                    concerns.push("Privacy compliance issues detected".to_string());
                    recommendations.push("Ensure proper data handling and consent".to_string());
                }

                if let Some(sensitive_data_used) = &context.sensitive_data_used {
                    if *sensitive_data_used {
                        score -= 0.2;
                        concerns.push("Sensitive data in use - ensure proper handling".to_string());
                        recommendations.push("Apply data minimization and encryption".to_string());
                    }
                }
            }
            EthicalPrinciple::Safety => {
                if let Some(risk_level) = context.risk_level {
                    if risk_level > 3 {
                        score -= 0.3 * (risk_level as f64);
                        concerns.push(format!("High risk level detected: {}", risk_level));
                        recommendations.push("Implement additional safety measures".to_string());
                    }
                }

                if !context.fallback_mechanism {
                    score -= 0.2;
                    concerns.push("No fallback mechanism available".to_string());
                    recommendations.push("Implement human-in-the-loop fallback".to_string());
                }
            }
            EthicalPrinciple::HumanOversight => {
                if !context.human_review_enabled && context.impact == ImpactLevel::High {
                    score -= 0.4;
                    concerns.push("High-impact decision without human review".to_string());
                    recommendations.push("Enable human review for high-impact decisions".to_string());
                }
            }
            _ => {
                // Default evaluation
                score = 0.8;
            }
        }

        score = score.max(0.0).min(1.0);
        Ok((score, concerns, recommendations))
    }

    /// Determine violation severity
    fn determine_severity(&self, score: f64, principle: EthicalPrinciple) -> ViolationSeverity {
        let base_severity = if score < 0.3 {
            ViolationSeverity::Critical
        } else if score < 0.5 {
            ViolationSeverity::High
        } else if score < 0.7 {
            ViolationSeverity::Medium
        } else {
            ViolationSeverity::Low
        };

        // Adjust severity based on principle
        match principle {
            EthicalPrinciple::Safety | EthicalPrinciple::NonMaleficence => {
                // Safety violations are more severe
                match base_severity {
                    ViolationSeverity::Low => ViolationSeverity::Medium,
                    ViolationSeverity::Medium => ViolationSeverity::High,
                    _ => base_severity,
                }
            }
            _ => base_severity,
        }
    }

    /// Create ethics violation
    async fn create_violation(
        &self,
        principle: EthicalPrinciple,
        severity: ViolationSeverity,
        description: String,
        decision_id: Option<String>,
    ) -> Result<()> {
        let violation = EthicsViolation {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            principle,
            severity,
            description,
            decision_id,
            evidence: Vec::new(),
            affected_users: Vec::new(),
            recommended_actions: vec![
                "Investigate the decision immediately".to_string(),
                "Review and mitigate the violation".to_string(),
                "Document resolution steps".to_string(),
            ],
            status: ViolationStatus::Detected,
            resolution_notes: None,
            resolved_by: None,
            resolved_at: None,
        };

        let mut violations = self.violations.write().await;
        violations.push(violation.clone());

        // Update statistics
        let mut stats = self.statistics.write().await;
        stats.violations_detected += 1;
        *stats.violations_by_principle.entry(principle).or_insert(0) += 1;
        *stats.violations_by_severity.entry(severity).or_insert(0) += 1;

        warn!("Ethics violation detected: {:?}", principle);

        Ok(())
    }

    /// Update review statistics
    async fn update_review_statistics(&self, review: &EthicsReview) {
        let mut stats = self.statistics.write().await;
        stats.total_reviews += 1;
        
        if review.passed {
            stats.reviews_passed += 1;
        } else {
            stats.reviews_failed += 1;
        }

        stats.avg_ethical_score = (stats.avg_ethical_score * (stats.total_reviews - 1) as f64
            + review.overall_score)
            / stats.total_reviews as f64;
    }

    /// Resolve a violation
    pub async fn resolve_violation(
        &self,
        violation_id: &str,
        resolver: &str,
        notes: String,
    ) -> Result<()> {
        let mut violations = self.violations.write().await;
        
        if let Some(violation) = violations.iter_mut().find(|v| v.id == violation_id) {
            violation.status = ViolationStatus::Resolved;
            violation.resolved_by = Some(resolver.to_string());
            violation.resolved_at = Some(chrono::Utc::now());
            violation.resolution_notes = Some(notes);

            info!("Resolved ethics violation: {}", violation_id);
            return Ok(());
        }

        Err(anyhow::anyhow!("Violation not found: {}", violation_id))
    }

    /// Get recent violations
    pub async fn get_recent_violations(&self, limit: usize) -> Vec<EthicsViolation> {
        let violations = self.violations.read().await;
        violations.iter().rev().take(limit).cloned().collect()
    }

    /// Get violations by severity
    pub async fn get_violations_by_severity(&self, severity: ViolationSeverity) -> Vec<EthicsViolation> {
        let violations = self.violations.read().await;
        violations
            .iter()
            .filter(|v| v.severity == severity)
            .cloned()
            .collect()
    }

    /// Get recent reviews
    pub async fn get_recent_reviews(&self, limit: usize) -> Vec<EthicsReview> {
        let reviews = self.reviews.read().await;
        reviews.iter().rev().take(limit).cloned().collect()
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> EthicsStats {
        self.statistics.read().await.clone()
    }

    /// Generate ethics report
    pub async fn generate_report(&self) -> Result<String> {
        let reviews = self.reviews.read().await;
        let violations = self.violations.read().await;
        let stats = self.statistics.read().await;

        let report = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "summary": {
                "total_reviews": stats.total_reviews,
                "pass_rate": if stats.total_reviews > 0 {
                    stats.reviews_passed as f64 / stats.total_reviews as f64
                } else {
                    0.0
                },
                "avg_ethical_score": stats.avg_ethical_score,
                "violations_detected": stats.violations_detected,
            },
            "principles": {
                "enforced": self.config.enforced_principles.iter().map(|p| format!("{:?}", p)).collect::<Vec<_>>(),
                "violations": stats.violations_by_principle,
            },
            "severity_breakdown": stats.violations_by_severity,
        });

        Ok(serde_json::to_string_pretty(&report)?)
    }
}

/// Context for ethics evaluation
#[derive(Debug, Clone)]
pub struct EthicsContext {
    /// Has explanation for the decision
    pub has_explanation: bool,
    /// Bias detected
    pub bias_detected: Option<bool>,
    /// Disparate impact score
    pub disparate_impact: Option<f64>,
    /// Confidence score
    pub confidence: Option<f64>,
    /// Audit trail enabled
    pub audit_trail_enabled: bool,
    /// Privacy compliant
    pub privacy_compliant: bool,
    /// Sensitive data used
    pub sensitive_data_used: Option<bool>,
    /// Risk level (1-5)
    pub risk_level: Option<u8>,
    /// Fallback mechanism available
    pub fallback_mechanism: bool,
    /// Human review enabled
    pub human_review_enabled: bool,
    /// Impact level of decision
    pub impact: ImpactLevel,
}

/// Impact level of decisions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethics_manager_creation() {
        let manager = EthicsManager::new();
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_review_decision() {
        let manager = EthicsManager::new().unwrap();

        let context = EthicsContext {
            has_explanation: true,
            bias_detected: Some(false),
            disparate_impact: Some(0.1),
            confidence: Some(0.95),
            audit_trail_enabled: true,
            privacy_compliant: true,
            sensitive_data_used: Some(false),
            risk_level: Some(2),
            fallback_mechanism: true,
            human_review_enabled: true,
            impact: ImpactLevel::Medium,
        };

        let review = manager
            .review_decision("decision-1".to_string(), ReviewType::PostDecision, &context)
            .await
            .unwrap();

        assert!(review.overall_score > 0.0);
    }

    #[tokio::test]
    async fn test_violation_creation() {
        let manager = EthicsManager::new().unwrap();

        let context = EthicsContext {
            has_explanation: false,
            bias_detected: Some(true),
            disparate_impact: Some(0.5),
            confidence: None,
            audit_trail_enabled: false,
            privacy_compliant: false,
            sensitive_data_used: Some(true),
            risk_level: Some(4),
            fallback_mechanism: false,
            human_review_enabled: false,
            impact: ImpactLevel::High,
        };

        let review = manager
            .review_decision("decision-1".to_string(), ReviewType::PostDecision, &context)
            .await
            .unwrap();

        // Should have violations due to low ethical score
        let stats = manager.get_statistics().await;
        assert!(stats.violations_detected > 0 || !review.passed);
    }
}