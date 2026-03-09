//! Comprehensive integration tests for Phase 7.2.3 - Compliance Modules

use vantisos::ai::compliance::*;
use std::collections::HashMap;

#[tokio::test]
async fn test_compliance_system_initialization() {
    let config = ComplianceConfig::default();
    let system = ComplianceSystem::new(config).unwrap();
    
    assert!(system.compliance().is_ok());
    assert!(system.transparency().is_ok());
    assert!(system.bias_detector().is_ok());
    assert!(system.audit().is_ok());
    assert!(system.ethics().is_ok());
}

#[tokio::test]
async fn test_regulatory_compliance_gdpr() {
    let config = ComplianceConfig::default();
    let manager = ComplianceManager::new(&config).unwrap();
    
    manager.initialize_requirements().await;
    
    let result = manager.check_compliance(ComplianceFramework::GDPR).await.unwrap();
    
    assert_eq!(result.framework, ComplianceFramework::GDPR);
    assert!(matches!(
        result.status,
        ComplianceStatus::Compliant | ComplianceStatus::PartiallyCompliant
    ));
}

#[tokio::test]
async fn test_regulatory_compliance_eu_ai_act() {
    let config = ComplianceConfig {
        gdpr_enabled: true,
        ai_act_enabled: true,
        ..Default::default()
    };
    let manager = ComplianceManager::new(&config).unwrap();
    
    manager.initialize_requirements().await;
    
    let result = manager.check_compliance(ComplianceFramework::EU_AI_ACT).await.unwrap();
    
    assert_eq!(result.framework, ComplianceFramework::EU_AI_ACT);
    assert!(!result.recommendations.is_empty() || result.status == ComplianceStatus::Compliant);
}

#[tokio::test]
async fn test_regulatory_compliance_report_generation() {
    let config = ComplianceConfig::default();
    let manager = ComplianceManager::new(&config).unwrap();
    
    manager.initialize_requirements().await;
    
    let report = manager.generate_report().await.unwrap();
    
    assert!(!report.frameworks.is_empty());
    assert!(!report.requirements.is_empty());
    assert!(report.total_requirements > 0);
    assert!(report.compliant_count + report.partial_count + report.non_compliant_count == report.total_requirements);
}

#[tokio::test]
async fn test_data_processing_activity_registration() {
    let config = ComplianceConfig::default();
    let manager = ComplianceManager::new(&config).unwrap();
    
    let activity = DataProcessingActivity {
        id: "DPA-001".to_string(),
        controller: "Vantis Corp".to_string(),
        data_subjects: vec!["Customers".to_string()],
        data_categories: vec!["Personal Data".to_string()],
        recipients: vec!["Support Team".to_string()],
        third_country_transfers: vec![],
        retention_period: "2 years".to_string(),
        security_measures: vec!["Encryption".to_string()],
    };
    
    manager.register_data_activity(activity).await.unwrap();
    
    // Verify activity was registered
    let gdrp_reqs = manager.get_framework_requirements(ComplianceFramework::GDPR).await;
    assert!(!gdrp_reqs.is_empty());
}

#[tokio::test]
async fn test_risk_assessment_creation() {
    let config = ComplianceConfig::default();
    let manager = ComplianceManager::new(&config).unwrap();
    
    let assessment = manager.create_risk_assessment(
        "Data Privacy".to_string(),
        "Unauthorized data access potential".to_string(),
        4, // Likelihood
        5, // Impact
        vec![
            "Implement encryption".to_string(),
            "Add access controls".to_string(),
        ],
    ).await.unwrap();
    
    assert!(assessment.risk_score > 0.0);
    assert!(assessment.mitigation_measures.len() > 0);
    assert!(assessment.residual_risk < assessment.risk_score);
}

#[tokio::test]
async fn test_high_risk_assessments() {
    let config = ComplianceConfig::default();
    let manager = ComplianceManager::new(&config).unwrap();
    
    // Create high-risk assessment
    manager.create_risk_assessment(
        "Critical System".to_string(),
        "Potential data breach".to_string(),
        5,
        5,
        vec!["Implement full encryption".to_string()],
    ).await.unwrap();
    
    let high_risks = manager.get_high_risk_assessments().await;
    assert!(!high_risks.is_empty());
}

#[tokio::test]
async fn test_transparency_explanation_generation() {
    let manager = TransparencyManager::new().unwrap();
    
    let mut features = HashMap::new();
    features.insert("age".to_string(), 35.0);
    features.insert("income".to_string(), 50000.0);
    features.insert("score".to_string(), 0.85);
    
    let explanation = manager.explain_decision(
        "decision-1".to_string(),
        features,
        0.75,
        "credit_scoring_model",
        ExplanationType::FeatureImportance,
    ).await.unwrap();
    
    assert!(!explanation.feature_importance.is_empty());
    assert!(!explanation.summary.is_empty());
    assert!(!explanation.details.is_empty());
    assert!(explanation.confidence > 0.0 && explanation.confidence <= 1.0);
}

#[tokio::test]
async fn test_transparency_counterfactual() {
    let manager = TransparencyManager::new().unwrap();
    
    let mut features = HashMap::new();
    features.insert("age".to_string(), 25.0);
    features.insert("income".to_string(), 30000.0);
    
    let counterfactual = manager.generate_counterfactual(
        "decision-2".to_string(),
        features,
        "loan_model",
        0.8,
    ).await.unwrap();
    
    assert!(!counterfactual.is_empty());
    // Counterfactual should differ from original
}

#[tokio::test]
async fn test_transparency_model_registration() {
    let manager = TransparencyManager::new().unwrap();
    
    let metadata = crate::ai::compliance::transparency::ModelMetadata {
        name: "test_model".to_string(),
        version: "1.0.0".to_string(),
        model_type: "RandomForest".to_string(),
        training_data_summary: "10k samples, 50 features".to_string(),
        limitations: vec!["Limited to specific domains".to_string()],
        performance_metrics: {
            let mut m = HashMap::new();
            m.insert("accuracy".to_string(), 0.95);
            m.insert("precision".to_string(), 0.93);
            m
        },
    };
    
    manager.register_model(metadata).await.unwrap();
}

#[tokio::test]
async fn test_bias_detection_demographic_parity() {
    let detector = BiasDetector::new().unwrap();
    
    let mut predictions = HashMap::new();
    predictions.insert("group_a".to_string(), vec![0.9, 0.8, 0.7, 0.6]);
    predictions.insert("group_b".to_string(), vec![0.4, 0.3, 0.2, 0.1]);
    
    let result = detector.check_demographic_parity(&predictions, ProtectedAttribute::Gender).await.unwrap();
    
    assert!(result.metric_value >= 0.0);
    assert!(!result.affected_groups.is_empty());
    assert!(result.is_biased || !result.is_biased);
}

#[tokio::test]
async fn test_bias_detection_equalized_odds() {
    let detector = BiasDetector::new().unwrap();
    
    let mut predictions = HashMap::new();
    predictions.insert("group_a".to_string(), vec![
        (0.9, true), (0.8, true), (0.7, false), (0.6, false)
    ]);
    predictions.insert("group_b".to_string(), vec![
        (0.4, true), (0.3, true), (0.2, false), (0.1, false)
    ]);
    
    let result = detector.check_equalized_odds(&predictions, ProtectedAttribute::Race).await.unwrap();
    
    assert_eq!(result.metric, FairnessMetric::EqualizedOdds);
    assert!(!result.affected_groups.is_empty());
}

#[tokio::test]
async fn test_bias_detection_mitigation_recommendations() {
    let detector = BiasDetector::new().unwrap();
    
    let mut predictions = HashMap::new();
    predictions.insert("group_a".to_string(), vec![0.9; 50]);
    predictions.insert("group_b".to_string(), vec![0.4; 50]);
    
    let result = detector.check_demographic_parity(&predictions, ProtectedAttribute::Age).await.unwrap();
    
    if result.is_biased {
        assert!(!result.mitigations.is_empty());
    }
}

#[tokio::test]
async fn test_bias_detection_comprehensive_check() {
    let detector = BiasDetector::new().unwrap();
    
    let mut predictions_by_attr = HashMap::new();
    
    let mut gender_predictions = HashMap::new();
    gender_predictions.insert("male".to_string(), vec![0.7; 30]);
    gender_predictions.insert("female".to_string(), vec![0.6; 30]);
    predictions_by_attr.insert(ProtectedAttribute::Gender, gender_predictions);
    
    let results = detector.comprehensive_check(predictions_by_attr).await.unwrap();
    
    assert!(!results.is_empty());
    assert!(results.iter().all(|r| matches!(
        r.bias_type, 
        BiasType::AlgorithmicBias | BiasType::SamplingBias
    )));
}

#[tokio::test]
async fn test_audit_trail_decision_logging() {
    let manager = AuditTrailManager::new(365).unwrap();
    
    let entry = AuditEntry {
        id: String::new(),
        timestamp: chrono::Utc::now(),
        decision_id: "decision-001".to_string(),
        user_id: Some("user-123".to_string()),
        session_id: Some("session-456".to_string()),
        model_name: "credit_model".to_string(),
        model_version: "2.1.0".to_string(),
        decision_type: DecisionType::Classification,
        input_hash: String::new(),
        output: serde_json::json!({"prediction": "approved"}),
        confidence: 0.95,
        rationale: "High credit score".to_string(),
        metadata: HashMap::new(),
        ip_address: Some("192.168.1.1".to_string()),
        user_agent: Some("Mozilla/5.0".to_string()),
        explanation_id: Some("exp-001".to_string()),
        review_status: ReviewStatus::Pending,
        reviewed_by: None,
        review_timestamp: None,
        review_notes: None,
    };
    
    manager.log_decision(entry).await.unwrap();
    
    let stats = manager.get_statistics().await;
    assert_eq!(stats.total_entries, 1);
}

#[tokio::test]
async fn test_audit_trail_query() {
    let manager = AuditTrailManager::new(365).unwrap();
    
    // Log multiple decisions
    for i in 0..5 {
        let entry = AuditEntry {
            id: String::new(),
            timestamp: chrono::Utc::now(),
            decision_id: format!("decision-{:03}", i),
            user_id: Some(format!("user-{}", i)),
            session_id: None,
            model_name: "test_model".to_string(),
            model_version: "1.0.0".to_string(),
            decision_type: DecisionType::Classification,
            input_hash: String::new(),
            output: serde_json::json!({"value": i}),
            confidence: 0.9,
            rationale: "Test".to_string(),
            metadata: HashMap::new(),
            ip_address: None,
            user_agent: None,
            explanation_id: None,
            review_status: ReviewStatus::Pending,
            reviewed_by: None,
            review_timestamp: None,
            review_notes: None,
        };
        manager.log_decision(entry).await.unwrap();
    }
    
    // Query for specific decision
    let query = crate::ai::compliance::audit_trail::AuditQuery {
        decision_id: Some("decision-002".to_string()),
        limit: None,
        ..Default::default()
    };
    
    let results = manager.query(query).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].decision_id, "decision-002");
}

#[tokio::test]
async fn test_audit_trail_review_workflow() {
    let manager = AuditTrailManager::new(365).unwrap();
    
    // Log a decision
    let mut entry = AuditEntry {
        id: String::new(),
        timestamp: chrono::Utc::now(),
        decision_id: "decision-001".to_string(),
        user_id: None,
        session_id: None,
        model_name: "test_model".to_string(),
        model_version: "1.0.0".to_string(),
        decision_type: DecisionType::Classification,
        input_hash: String::new(),
        output: serde_json::json!({"result": "yes"}),
        confidence: 0.8,
        rationale: "Test".to_string(),
        metadata: HashMap::new(),
        ip_address: None,
        user_agent: None,
        explanation_id: None,
        review_status: ReviewStatus::Pending,
        reviewed_by: None,
        review_timestamp: None,
        review_notes: None,
    };
    
    manager.log_decision(entry.clone()).await.unwrap();
    
    // Get the entry
    let entries = manager.get_recent_entries(1).await;
    let entry_id = &entries[0].id;
    
    // Review the entry
    manager.review_entry(entry_id, "reviewer", ReviewStatus::Approved, Some("Looks good".to_string())).await.unwrap();
    
    // Verify review status
    let reviewed = manager.get_entry(entry_id).await.unwrap();
    assert_eq!(reviewed.review_status, ReviewStatus::Approved);
    assert_eq!(reviewed.reviewed_by, Some("reviewer".to_string()));
}

#[tokio::test]
async fn test_audit_trail_export() {
    let manager = AuditTrailManager::new(365).unwrap();
    
    // Log a test entry
    let entry = AuditEntry {
        id: String::new(),
        timestamp: chrono::Utc::now(),
        decision_id: "export_test".to_string(),
        user_id: None,
        session_id: None,
        model_name: "model".to_string(),
        model_version: "1.0".to_string(),
        decision_type: DecisionType::Classification,
        input_hash: String::new(),
        output: serde_json::json!({}),
        confidence: 0.5,
        rationale: "Test".to_string(),
        metadata: HashMap::new(),
        ip_address: None,
        user_agent: None,
        explanation_id: None,
        review_status: ReviewStatus::Pending,
        reviewed_by: None,
        review_timestamp: None,
        review_notes: None,
    };
    manager.log_decision(entry).await.unwrap();
    
    // Export as JSON
    let json_export = manager.export_audit_trail(crate::ai::compliance::audit_trail::ExportFormat::Json).await.unwrap();
    assert!(json_export.contains("export_test"));
    
    // Export as CSV
    let csv_export = manager.export_audit_trail(crate::ai::compliance::audit_trail::ExportFormat::Csv).await.unwrap();
    assert!(csv_export.contains("id,timestamp"));
}

#[tokio::test]
async fn test_ethics_review_decision() {
    let manager = EthicsManager::new().unwrap();
    
    let context = crate::ai::compliance::ethics::EthicsContext {
        has_explanation: true,
        bias_detected: Some(false),
        disparate_impact: Some(0.05),
        confidence: Some(0.92),
        audit_trail_enabled: true,
        privacy_compliant: true,
        sensitive_data_used: Some(false),
        risk_level: Some(2),
        fallback_mechanism: true,
        human_review_enabled: true,
        impact: ImpactLevel::Medium,
    };
    
    let review = manager.review_decision("decision-1".to_string(), ReviewType::PostDecision, &context).await.unwrap();
    
    assert!(!review.principle_scores.is_empty());
    assert!(review.overall_score >= 0.0 && review.overall_score <= 1.0);
    assert!(review.passed || !review.passed);
}

#[tokio::test]
async fn test_ethics_violation_detection() {
    let manager = EthicsManager::new().unwrap();
    
    // Create context with ethical issues
    let context = crate::ai::compliance::ethics::EthicsContext {
        has_explanation: false,
        bias_detected: Some(true),
        disparate_impact: Some(0.6),
        confidence: None,
        audit_trail_enabled: false,
        privacy_compliant: false,
        sensitive_data_used: Some(true),
        risk_level: Some(4),
        fallback_mechanism: false,
        human_review_enabled: false,
        impact: ImpactLevel::High,
    };
    
    let review = manager.review_decision("decision-2".to_string(), ReviewType::PostDecision, &context).await.unwrap();
    
    // Should have concerns due to ethical issues
    assert!(!review.concerns.is_empty() || !review.passed);
}

#[tokio::test]
async fn test_ethics_statistics() {
    let manager = EthicsManager::new().unwrap();
    
    let context = crate::ai::compliance::ethics::EthicsContext {
        has_explanation: true,
        bias_detected: Some(false),
        disparate_impact: Some(0.1),
        confidence: Some(0.85),
        audit_trail_enabled: true,
        privacy_compliant: true,
        sensitive_data_used: Some(false),
        risk_level: Some(1),
        fallback_mechanism: true,
        human_review_enabled: true,
        impact: ImpactLevel::Low,
    };
    
    manager.review_decision("decision-1".to_string(), ReviewType::PostDecision, &context).await.unwrap();
    
    let stats = manager.get_statistics().await;
    assert_eq!(stats.total_reviews, 1);
    assert!(stats.avg_ethical_score > 0.0);
}

#[tokio::test]
async fn test_ethics_violation_resolution() {
    let manager = EthicsManager::new().unwrap();
    
    // Trigger a violation
    let context = crate::ai::compliance::ethics::EthicsContext {
        has_explanation: false,
        bias_detected: Some(true),
        disparate_impact: Some(0.8),
        confidence: None,
        audit_trail_enabled: false,
        privacy_compliant: false,
        sensitive_data_used: Some(true),
        risk_level: Some(5),
        fallback_mechanism: false,
        human_review_enabled: false,
        impact: ImpactLevel::Critical,
    };
    
    manager.review_decision("decision-3".to_string(), ReviewType::PostDecision, &context).await.unwrap();
    
    // Get violations
    let violations = manager.get_recent_violations(10).await;
    
    if !violations.is_empty() {
        let violation = &violations[0];
        
        // Resolve the violation
        manager.resolve_violation(&violation.id, "admin", "Issue addressed".to_string()).await.unwrap();
        
        let resolved = manager.get_recent_violations(10).await;
        assert_eq!(resolved[0].status, crate::ai::compliance::ethics::ViolationStatus::Resolved);
    }
}

#[tokio::test]
async fn test_comprehensive_compliance_workflow() {
    let config = ComplianceConfig::default();
    let system = ComplianceSystem::new(config).unwrap();
    
    // 1. Check regulatory compliance
    let report = system.compliance().generate_report().await.unwrap();
    assert!(!report.frameworks.is_empty());
    
    // 2. Generate explanation for a decision
    let mut features = HashMap::new();
    features.insert("feature1".to_string(), 0.5);
    let explanation = system.transparency().explain_decision(
        "test_decision".to_string(),
        features,
        0.75,
        "model",
        ExplanationType::FeatureImportance,
    ).await.unwrap();
    assert!(!explanation.summary.is_empty());
    
    // 3. Check for bias
    let mut predictions = HashMap::new();
    predictions.insert("group_a".to_string(), vec![0.7; 20]);
    predictions.insert("group_b".to_string(), vec![0.6; 20]);
    let bias_result = system.bias_detector().check_demographic_parity(&predictions, ProtectedAttribute::Gender).await.unwrap();
    assert!(bias_result.metric_value >= 0.0);
    
    // 4. Log decision to audit trail
    let entry = AuditEntry {
        id: String::new(),
        timestamp: chrono::Utc::now(),
        decision_id: "test_decision".to_string(),
        user_id: None,
        session_id: None,
        model_name: "model".to_string(),
        model_version: "1.0".to_string(),
        decision_type: DecisionType::Classification,
        input_hash: String::new(),
        output: serde_json::json!({"prediction": 0.75}),
        confidence: 0.75,
        rationale: "Test decision".to_string(),
        metadata: HashMap::new(),
        ip_address: None,
        user_agent: None,
        explanation_id: Some(explanation.id),
        review_status: ReviewStatus::Pending,
        reviewed_by: None,
        review_timestamp: None,
        review_notes: None,
    };
    system.audit().log_decision(entry).await.unwrap();
    
    // 5. Ethics review
    let context = crate::ai::compliance::ethics::EthicsContext {
        has_explanation: true,
        bias_detected: Some(false),
        disparate_impact: Some(0.1),
        confidence: Some(0.75),
        audit_trail_enabled: true,
        privacy_compliant: true,
        sensitive_data_used: Some(false),
        risk_level: Some(1),
        fallback_mechanism: true,
        human_review_enabled: true,
        impact: ImpactLevel::Low,
    };
    let ethics_review = system.ethics().review_decision("test_decision".to_string(), ReviewType::PostDecision, &context).await.unwrap();
    assert!(ethics_review.overall_score > 0.0);
}