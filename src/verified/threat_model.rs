// Threat Model Module - Threat Model Update Implementation
// VantisOS - Secure Microkernel Operating System

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// Typ zagrożenia STRIDE
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThreatType {
    Spoofing,
    Tampering,
    Repudiation,
    InformationDisclosure,
    DenialOfService,
    ElevationOfPrivilege,
}

/// Poziom wpływu
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Poziom prawdopodobieństwa
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProbabilityLevel {
    VeryLow,
    Low,
    Medium,
    High,
}

/// Ocena DREAD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreadScore {
    pub damage: u8,           // 1-10
    pub reproducibility: u8,  // 1-10
    pub exploitability: u8,   // 1-10
    pub affected_users: u8,   // 1-10
    pub discoverability: u8,  // 1-10
    pub total: f64,           // Average
}

impl DreadScore {
    /// Oblicza całkowity wynik DREAD
    pub fn calculate_total(&mut self) {
        self.total = (self.damage + self.reproducibility + self.exploitability + 
                      self.affected_users + self.discoverability) as f64 / 5.0;
    }
}

/// Środek zaradczy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mitigation {
    pub mitigation_id: Uuid,
    pub description: String,
    pub implemented: bool,
    pub implementation_date: Option<DateTime<Utc>>,
}

/// Zagrożenie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threat {
    pub threat_id: Uuid,
    pub name: String,
    pub description: String,
    pub threat_type: ThreatType,
    pub attack_vector: String,
    pub impact: ImpactLevel,
    pub probability: ProbabilityLevel,
    pub dread_score: DreadScore,
    pub mitigations: Vec<Mitigation>,
    pub discovered_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

/// Raport zagrożeń
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatReport {
    pub report_id: Uuid,
    pub generated_at: DateTime<Utc>,
    pub total_threats: usize,
    pub critical_threats: usize,
    pub high_threats: usize,
    pub medium_threats: usize,
    pub low_threats: usize,
    pub threats_by_type: HashMap<ThreatType, usize>,
    pub mitigations_implemented: usize,
    pub mitigations_pending: usize,
}

/// Menedżer modelu zagrożeń
pub struct ThreatModelManager {
    threats: HashMap<Uuid, Threat>,
    mitigations: HashMap<Uuid, Mitigation>,
}

impl ThreatModelManager {
    /// Tworzy nowy menedżer modelu zagrożeń
    pub fn new() -> Self {
        Self {
            threats: HashMap::new(),
            mitigations: HashMap::new(),
        }
    }

    /// Dodaj zagrożenie
    pub fn add_threat(&mut self, threat: Threat) {
        self.threats.insert(threat.threat_id, threat);
    }

    /// Pobierz zagrożenie
    pub fn get_threat(&self, threat_id: Uuid) -> Option<&Threat> {
        self.threats.get(&threat_id)
    }

    /// Pobierz wszystkie zagrożenia
    pub fn get_all_threats(&self) -> Vec<&Threat> {
        self.threats.values().collect()
    }

    /// Pobierz zagrożenia według typu
    pub fn get_threats_by_type(&self, threat_type: ThreatType) -> Vec<&Threat> {
        self.threats.values()
            .filter(|t| t.threat_type == threat_type)
            .collect()
    }

    /// Pobierz zagrożenia według wpływu
    pub fn get_threats_by_impact(&self, impact: ImpactLevel) -> Vec<&Threat> {
        self.threats.values()
            .filter(|t| t.impact == impact)
            .collect()
    }

    /// Aktualizuj zagrożenie
    pub fn update_threat(&mut self, threat_id: Uuid, updated_threat: Threat) -> Result<(), String> {
        if !self.threats.contains_key(&threat_id) {
            return Err("Threat not found".to_string());
        }
        self.threats.insert(threat_id, updated_threat);
        Ok(())
    }

    /// Dodaj środek zaradczy
    pub fn add_mitigation(&mut self, threat_id: Uuid, mitigation: Mitigation) -> Result<(), String> {
        if !self.threats.contains_key(&threat_id) {
            return Err("Threat not found".to_string());
        }
        
        self.mitigations.insert(mitigation.mitigation_id, mitigation.clone());
        
        if let Some(threat) = self.threats.get_mut(&threat_id) {
            threat.mitigations.push(mitigation);
            threat.last_updated = Utc::now();
        }
        
        Ok(())
    }

    /// Oznacz środek zaradczy jako zaimplementowany
    pub fn implement_mitigation(&mut self, mitigation_id: Uuid) -> Result<(), String> {
        if let Some(mitigation) = self.mitigations.get_mut(&mitigation_id) {
            mitigation.implemented = true;
            mitigation.implementation_date = Some(Utc::now());
            Ok(())
        } else {
            Err("Mitigation not found".to_string())
        }
    }

    /// Generuj raport zagrożeń
    pub fn generate_report(&self) -> ThreatReport {
        let threats = self.get_all_threats();
        
        let total_threats = threats.len();
        let critical_threats = threats.iter().filter(|t| t.impact == ImpactLevel::Critical).count();
        let high_threats = threats.iter().filter(|t| t.impact == ImpactLevel::High).count();
        let medium_threats = threats.iter().filter(|t| t.impact == ImpactLevel::Medium).count();
        let low_threats = threats.iter().filter(|t| t.impact == ImpactLevel::Low).count();
        
        let mut threats_by_type: HashMap<ThreatType, usize> = HashMap::new();
        for threat in &threats {
            *threats_by_type.entry(threat.threat_type.clone()).or_insert(0) += 1;
        }
        
        let mitigations_implemented = self.mitigations.values().filter(|m| m.implemented).count();
        let mitigations_pending = self.mitigations.values().filter(|m| !m.implemented).count();
        
        ThreatReport {
            report_id: Uuid::new_v4(),
            generated_at: Utc::now(),
            total_threats,
            critical_threats,
            high_threats,
            medium_threats,
            low_threats,
            threats_by_type,
            mitigations_implemented,
            mitigations_pending,
        }
    }

    /// Oblicz priorytet zagrożenia
    pub fn calculate_priority(&self, threat: &Threat) -> String {
        let dread = threat.dread_score.total;
        
        if dread >= 7.5 {
            "Critical".to_string()
        } else if dread >= 6.5 {
            "High".to_string()
        } else if dread >= 5.0 {
            "Medium".to_string()
        } else {
            "Low".to_string()
        }
    }

    /// Pobierz zagrożenia krytyczne
    pub fn get_critical_threats(&self) -> Vec<&Threat> {
        self.threats.values()
            .filter(|t| self.calculate_priority(t) == "Critical")
            .collect()
    }

    /// Pobierz zagrożenia wysokiego priorytetu
    pub fn get_high_priority_threats(&self) -> Vec<&Threat> {
        self.threats.values()
            .filter(|t| self.calculate_priority(t) == "High")
            .collect()
    }
}

impl Default for ThreatModelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_threat() {
        let mut manager = ThreatModelManager::new();
        
        let threat = Threat {
            threat_id: Uuid::new_v4(),
            name: "Test Threat".to_string(),
            description: "Test description".to_string(),
            threat_type: ThreatType::DenialOfService,
            attack_vector: "Flooding".to_string(),
            impact: ImpactLevel::High,
            probability: ProbabilityLevel::High,
            dread_score: DreadScore {
                damage: 8,
                reproducibility: 9,
                exploitability: 8,
                affected_users: 9,
                discoverability: 9,
                total: 8.6,
            },
            mitigations: Vec::new(),
            discovered_at: Utc::now(),
            last_updated: Utc::now(),
        };
        
        manager.add_threat(threat);
        assert_eq!(manager.get_all_threats().len(), 1);
    }

    #[test]
    fn test_add_mitigation() {
        let mut manager = ThreatModelManager::new();
        
        let threat_id = Uuid::new_v4();
        let threat = Threat {
            threat_id,
            name: "Test Threat".to_string(),
            description: "Test description".to_string(),
            threat_type: ThreatType::DenialOfService,
            attack_vector: "Flooding".to_string(),
            impact: ImpactLevel::High,
            probability: ProbabilityLevel::High,
            dread_score: DreadScore {
                damage: 8,
                reproducibility: 9,
                exploitability: 8,
                affected_users: 9,
                discoverability: 9,
                total: 8.6,
            },
            mitigations: Vec::new(),
            discovered_at: Utc::now(),
            last_updated: Utc::now(),
        };
        
        manager.add_threat(threat);
        
        let mitigation = Mitigation {
            mitigation_id: Uuid::new_v4(),
            description: "Test mitigation".to_string(),
            implemented: false,
            implementation_date: None,
        };
        
        manager.add_mitigation(threat_id, mitigation).unwrap();
        assert_eq!(manager.get_threat(threat_id).unwrap().mitigations.len(), 1);
    }

    #[test]
    fn test_implement_mitigation() {
        let mut manager = ThreatModelManager::new();
        
        let threat_id = Uuid::new_v4();
        let threat = Threat {
            threat_id,
            name: "Test Threat".to_string(),
            description: "Test description".to_string(),
            threat_type: ThreatType::DenialOfService,
            attack_vector: "Flooding".to_string(),
            impact: ImpactLevel::High,
            probability: ProbabilityLevel::High,
            dread_score: DreadScore {
                damage: 8,
                reproducibility: 9,
                exploitability: 8,
                affected_users: 9,
                discoverability: 9,
                total: 8.6,
            },
            mitigations: Vec::new(),
            discovered_at: Utc::now(),
            last_updated: Utc::now(),
        };
        
        manager.add_threat(threat);
        
        let mitigation_id = Uuid::new_v4();
        let mitigation = Mitigation {
            mitigation_id,
            description: "Test mitigation".to_string(),
            implemented: false,
            implementation_date: None,
        };
        
        manager.add_mitigation(threat_id, mitigation).unwrap();
        manager.implement_mitigation(mitigation_id).unwrap();
        
        assert!(manager.mitigations.get(&mitigation_id).unwrap().implemented);
    }

    #[test]
    fn test_generate_report() {
        let mut manager = ThreatModelManager::new();
        
        let threat = Threat {
            threat_id: Uuid::new_v4(),
            name: "Test Threat".to_string(),
            description: "Test description".to_string(),
            threat_type: ThreatType::DenialOfService,
            attack_vector: "Flooding".to_string(),
            impact: ImpactLevel::High,
            probability: ProbabilityLevel::High,
            dread_score: DreadScore {
                damage: 8,
                reproducibility: 9,
                exploitability: 8,
                affected_users: 9,
                discoverability: 9,
                total: 8.6,
            },
            mitigations: Vec::new(),
            discovered_at: Utc::now(),
            last_updated: Utc::now(),
        };
        
        manager.add_threat(threat);
        
        let report = manager.generate_report();
        assert_eq!(report.total_threats, 1);
        assert_eq!(report.high_threats, 1);
    }

    #[test]
    fn test_calculate_priority() {
        let manager = ThreatModelManager::new();
        
        let threat = Threat {
            threat_id: Uuid::new_v4(),
            name: "Test Threat".to_string(),
            description: "Test description".to_string(),
            threat_type: ThreatType::DenialOfService,
            attack_vector: "Flooding".to_string(),
            impact: ImpactLevel::High,
            probability: ProbabilityLevel::High,
            dread_score: DreadScore {
                damage: 8,
                reproducibility: 9,
                exploitability: 8,
                affected_users: 9,
                discoverability: 9,
                total: 8.6,
            },
            mitigations: Vec::new(),
            discovered_at: Utc::now(),
            last_updated: Utc::now(),
        };
        
        let priority = manager.calculate_priority(&threat);
        assert_eq!(priority, "Critical");
    }
}