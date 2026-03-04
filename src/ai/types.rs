//! AI Module Type Definitions

use crate::ai::error::AIError;

/// Resource usage statistics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceUsage {
    /// CPU usage percentage (0-100)
    pub cpu_usage: f64,
    /// Memory usage percentage (0-100)
    pub memory_usage: f64,
    /// Disk usage percentage (0-100)
    pub disk_usage: f64,
    /// Network usage percentage (0-100)
    pub network_usage: f64,
}

/// Resource usage statistics for AI module monitoring
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AIResourceUsage {
    /// CPU usage percentage (0-100)
    pub cpu_percent: u8,
    /// Memory usage in bytes
    pub memory_bytes: usize,
    /// Inference latency in milliseconds
    pub inference_latency_ms: u64,
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
            network_usage: 0.0,
        }
    }
}

/// AI prediction confidence
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Confidence(pub u8); // 0-255

impl Confidence {
    /// Create a new confidence value
    pub fn new(value: u8) -> Self {
        Self(value.min(255))
    }

    /// Get confidence as percentage (0-100)
    pub fn as_percent(&self) -> u8 {
        ((self.0 as u16) * 100 / 255) as u8
    }

    /// Check if confidence is high enough
    pub fn is_high(&self) -> bool {
        self.as_percent() >= 80
    }

    /// Check if confidence is medium
    pub fn is_medium(&self) -> bool {
        let percent = self.as_percent();
        percent >= 50 && percent < 80
    }

    /// Check if confidence is low
    pub fn is_low(&self) -> bool {
        self.as_percent() < 50
    }
}

impl Default for Confidence {
    fn default() -> Self {
        Self::new(0)
    }
}

// High confidence constant for common use
impl Confidence {
    /// High confidence (80%+)
    pub const HIGH: Self = Self(204); // ~80%
    /// Medium confidence (50-79%)
    pub const MEDIUM: Self = Self(128); // ~50%
    /// Low confidence (<50%)
    pub const LOW: Self = Self(64); // ~25%
}

/// AI prediction result
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prediction<T> {
    /// Predicted value
    pub value: T,
    /// Confidence level
    pub confidence: Confidence,
    /// Prediction timestamp
    pub timestamp: u64,
}

impl<T> Prediction<T> {
    /// Create a new prediction
    pub fn new(value: T, confidence: u8, timestamp: u64) -> Self {
        Self {
            value,
            confidence: Confidence::new(confidence),
            timestamp,
        }
    }
}

/// AI Model type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    /// Reinforcement Learning model
    ReinforcementLearning,
    /// Neural Network model
    NeuralNetwork,
    /// Decision Tree model
    DecisionTree,
    /// Ensemble model
    Ensemble,
    /// Transformer model
    Transformer,
    /// Custom model
    Custom,
}

/// AI Training status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrainingStatus {
    /// Model is not trained
    Untrained,
    /// Training in progress
    Training,
    /// Training completed
    Trained,
    /// Training failed
    Failed,
}

/// AI Model metadata
#[derive(Debug, Clone)]
pub struct ModelMetadata {
    /// Model type
    pub model_type: ModelType,
    /// Model version
    pub version: String,
    /// Training status
    pub training_status: TrainingStatus,
    /// Training accuracy (0-100)
    pub accuracy: Option<u8>,
    /// Last training timestamp
    pub last_trained: Option<u64>,
    /// Model size in bytes
    pub size_bytes: usize,
}

impl ModelMetadata {
    /// Create new model metadata
    pub fn new(model_type: ModelType) -> Self {
        Self {
            model_type,
            version: "1.0.0".to_string(),
            training_status: TrainingStatus::Untrained,
            accuracy: None,
            last_trained: None,
            size_bytes: 0,
        }
    }
}

/// Process scheduling decision
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchedulingDecision {
    /// Process ID
    pub pid: u32,
    /// CPU core to run on
    pub cpu_core: u8,
    /// Priority level (0-255)
    pub priority: u8,
    /// Time slice in milliseconds
    pub time_slice_ms: u64,
}

/// Power management decision
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerDecision {
    /// CPU frequency in MHz
    pub cpu_frequency_mhz: u32,
    /// Power state
    pub power_state: PowerState,
}

/// Power state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    /// Maximum performance
    Performance,
    /// Balanced
    Balanced,
    /// Power save
    PowerSave,
    /// Deep sleep
    DeepSleep,
}

/// Threat level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    /// No threat
    None,
    /// Low threat
    Low,
    /// Medium threat
    Medium,
    /// High threat
    High,
    /// Critical threat
    Critical,
}

/// Security threat detection result
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreatDetection {
    /// Threat level
    pub level: ThreatLevel,
    /// Threat type
    pub threat_type: String,
    /// Confidence
    pub confidence: Confidence,
    /// Detection timestamp
    pub timestamp: u64,
    /// Affected resources
    pub affected_resources: Vec<String>,
}

impl ThreatDetection {
    /// Create a new threat detection
    pub fn new(level: ThreatLevel, threat_type: String) -> Self {
        Self {
            level,
            threat_type,
            confidence: Confidence::new(0),
            timestamp: 0,
            affected_resources: Vec::new(),
        }
    }

    /// Check if threat is severe
    pub fn is_severe(&self) -> bool {
        self.level >= ThreatLevel::High
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence() {
        let conf = Confidence::new(200);
        assert_eq!(conf.as_percent(), 78); // 200 * 100 / 255 = 78
        assert!(conf.is_medium());
    }

    #[test]
    fn test_prediction() {
        let pred = Prediction::new("test", 200, 1234567890);
        assert_eq!(pred.value, "test");
        assert_eq!(pred.confidence.as_percent(), 78);
    }

    #[test]
    fn test_threat_detection() {
        let threat = ThreatDetection::new(ThreatLevel::High, "Malware".to_string());
        assert!(threat.is_severe());
    }
}