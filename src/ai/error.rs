//! AI Error Types

/// AI Module errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AIError {
    /// Initialization failed
    InitializationError,
    /// Configuration error
    ConfigurationError,
    /// Insufficient resources
    InsufficientResources,
    /// Model not found
    ModelNotFound,
    /// Inference failed
    InferenceError,
    /// Training failed
    TrainingError,
    /// Invalid input
    InvalidInput,
    /// Timeout
    Timeout,
    /// Module not ready
    ModuleNotReady,
    /// Memory allocation failed
    MemoryAllocationError,
    /// Hardware not supported
    HardwareNotSupported,
    /// Insufficient data for training/inference
    InsufficientData,
    /// Alert deduplicated
    AlertDeduplicated,
    /// Too many alerts
    TooManyAlerts,
    /// Alert not found
    AlertNotFound(String),
    /// Queue full
    QueueFull,
    /// Template not found
    TemplateNotFound(String),
}

impl core::fmt::Display for AIError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AIError::InitializationError => write!(f, "AI initialization failed"),
            AIError::ConfigurationError => write!(f, "AI configuration error"),
            AIError::InsufficientResources => write!(f, "Insufficient resources for AI operation"),
            AIError::ModelNotFound => write!(f, "AI model not found"),
            AIError::InferenceError => write!(f, "AI inference failed"),
            AIError::TrainingError => write!(f, "AI training failed"),
            AIError::InvalidInput => write!(f, "Invalid input for AI operation"),
            AIError::Timeout => write!(f, "AI operation timed out"),
            AIError::ModuleNotReady => write!(f, "AI module not ready"),
            AIError::MemoryAllocationError => write!(f, "AI memory allocation failed"),
            AIError::HardwareNotSupported => write!(f, "Hardware not supported for AI"),
            AIError::InsufficientData => write!(f, "Insufficient data for AI operation"),
            AIError::AlertDeduplicated => write!(f, "Alert was deduplicated"),
            AIError::TooManyAlerts => write!(f, "Too many active alerts"),
            AIError::AlertNotFound(id) => write!(f, "Alert not found: {}", id),
            AIError::QueueFull => write!(f, "Notification queue is full"),
            AIError::TemplateNotFound(name) => write!(f, "Template not found: {}", name),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AIError {}

/// Type alias for AI service errors
pub type AIServiceError = AIError;

/// Result type for AI operations
pub type Result<T> = core::result::Result<T, AIServiceError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            AIError::InitializationError.to_string(),
            "AI initialization failed"
        );
        assert_eq!(
            AIError::ModuleNotReady.to_string(),
            "AI module not ready"
        );
    }
}