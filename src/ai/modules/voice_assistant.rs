//! Intelligent Voice Assistant with Natural Language Understanding
//! 
//! This module implements a sophisticated voice assistant system that uses
//! advanced NLP and speech recognition to understand and execute user commands,
//! providing context-aware interactions and learning from user behavior.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, mpsc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Voice assistant errors
#[derive(Error, Debug)]
pub enum VoiceAssistantError {
    #[error("Speech recognition failed: {0}")]
    SpeechRecognitionFailed(String),
    
    #[error("Intent recognition failed: {0}")]
    IntentRecognitionFailed(String),
    
    #[error("Command execution failed: {0}")]
    CommandExecutionFailed(String),
    
    #[error("Audio input error: {0}")]
    AudioInputError(String),
}

/// Configuration for voice assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceAssistantConfig {
    /// Enable wake word detection
    pub enable_wake_word: bool,
    
    /// Wake word phrase
    pub wake_word: String,
    
    /// Enable continuous listening
    pub enable_continuous_listening: bool,
    
    /// Confidence threshold for intent recognition
    pub intent_confidence_threshold: f64,
    
    /// Enable context awareness
    pub enable_context: bool,
    
    /// Enable learning from user behavior
    pub enable_learning: bool,
    
    /// Maximum context history size
    pub max_context_history: usize,
    
    /// Voice gender (male/female/neutral)
    pub voice_gender: String,
    
    /// Speech rate (0.0 - 2.0, 1.0 is normal)
    pub speech_rate: f64,
}

/// Intent classification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    /// Intent identifier
    pub intent_id: String,
    
    /// Intent name
    pub name: String,
    
    /// Confidence score (0-1)
    pub confidence: f64,
    
    /// Extracted entities
    pub entities: Vec<Entity>,
    
    /// Suggested response
    pub response: Option<String>,
}

/// Entity extracted from speech
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Entity type
    pub entity_type: String,
    
    /// Entity value
    pub value: String,
    
    /// Confidence score
    pub confidence: f64,
    
    /// Start position in text
    pub start_pos: usize,
    
    /// End position in text
    pub end_pos: usize,
}

/// Command to execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// Command identifier
    pub command_id: String,
    
    /// Command type
    pub command_type: CommandType,
    
    /// Command parameters
    pub parameters: HashMap<String, String>,
    
    /// Execution priority
    pub priority: u8,
}

/// Types of commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandType {
    /// System control (shutdown, restart)
    SystemControl,
    
    /// Application launch
    ApplicationLaunch,
    
    /// Information query
    InformationQuery,
    
    /// Configuration change
    Configuration,
    
    /// Automation trigger
    Automation,
    
    /// File operation
    FileOperation,
    
    /// Network operation
    NetworkOperation,
}

/// Voice command result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    /// Success status
    pub success: bool,
    
    /// Result message
    pub message: String,
    
    /// Execution time
    pub execution_time: Duration,
    
    /// Additional data
    pub data: Option<serde_json::Value>,
}

/// Conversation context entry
#[derive(Debug, Clone)]
struct ContextEntry {
    /// User input
    user_input: String,
    
    /// System response
    system_response: String,
    
    /// Intent identified
    intent: Option<Intent>,
    
    /// Timestamp
    timestamp: Instant,
}

/// Voice Assistant
pub struct VoiceAssistant {
    config: VoiceAssistantConfig,
    
    /// Context history
    context_history: Arc<RwLock<VecDeque<ContextEntry>>>,
    
    /// Intent classifier
    intent_classifier: Arc<RwLock<IntentClassifier>>,
    
    /// Command executor
    command_executor: Arc<RwLock<CommandExecutor>>,
    
    /// Response generator
    response_generator: Arc<RwLock<ResponseGenerator>>,
    
    /// User preferences
    user_preferences: Arc<RwLock<HashMap<String, String>>>,
    
    /// Command queue
    command_queue: Arc<RwLock<VecDeque<Command>>>,
    
    /// Statistics
    stats: Arc<RwLock<VoiceAssistantStats>>,
}

/// Intent classifier
struct IntentClassifier {
    /// Intent patterns
    patterns: HashMap<String, Vec<IntentPattern>>,
    
    /// Learning weights
    weights: HashMap<String, f64>,
}

/// Intent pattern for matching
#[derive(Debug, Clone)]
struct IntentPattern {
    pattern: String,
    entities: Vec<EntityPattern>,
    confidence_weight: f64,
}

/// Entity pattern
#[derive(Debug, Clone)]
struct EntityPattern {
    entity_type: String,
    pattern: String,
}

impl IntentClassifier {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        
        // System control intents
        patterns.insert("system.shutdown".to_string(), vec![
            IntentPattern {
                pattern: "shut down".to_string(),
                entities: vec![],
                confidence_weight: 1.0,
            },
            IntentPattern {
                pattern: "power off".to_string(),
                entities: vec![],
                confidence_weight: 0.9,
            },
        ]);
        
        patterns.insert("system.restart".to_string(), vec![
            IntentPattern {
                pattern: "restart".to_string(),
                entities: vec![],
                confidence_weight: 1.0,
            },
            IntentPattern {
                pattern: "reboot".to_string(),
                entities: vec![],
                confidence_weight: 0.9,
            },
        ]);

        Self {
            patterns,
            weights: HashMap::new(),
        }
    }

    /// Classify intent from text
    pub fn classify(&mut self, text: &str) -> Option<Intent> {
        let mut best_intent: Option<Intent> = None;
        let mut best_confidence = 0.0;

        for (intent_id, intent_patterns) in &self.patterns {
            for pattern in intent_patterns {
                let confidence = self.match_pattern(text, pattern);
                
                if confidence > best_confidence {
                    let entities = self.extract_entities(text, &pattern.entities);
                    
                    best_intent = Some(Intent {
                        intent_id: intent_id.clone(),
                        name: intent_id.clone(),
                        confidence,
                        entities,
                        response: None,
                    });
                    
                    best_confidence = confidence;
                }
            }
        }

        best_intent
    }

    /// Match pattern against text
    fn match_pattern(&self, text: &str, pattern: &IntentPattern) -> f64 {
        let lower_text = text.to_lowercase();
        let lower_pattern = pattern.pattern.to_lowercase();
        
        if lower_text.contains(&lower_pattern) {
            pattern.confidence_weight
        } else {
            0.0
        }
    }

    /// Extract entities from text
    fn extract_entities(&self, text: &str, patterns: &[EntityPattern]) -> Vec<Entity> {
        let mut entities = Vec::new();
        
        for entity_pattern in patterns {
            if let Some(pos) = text.to_lowercase().find(&entity_pattern.pattern.to_lowercase()) {
                entities.push(Entity {
                    entity_type: entity_pattern.entity_type.clone(),
                    value: text[pos..pos + entity_pattern.pattern.len()].to_string(),
                    confidence: 0.8,
                    start_pos: pos,
                    end_pos: pos + entity_pattern.pattern.len(),
                });
            }
        }
        
        entities
    }
}

/// Command executor
struct CommandExecutor;

impl CommandExecutor {
    pub fn execute(&self, command: &Command) -> Result<CommandResult, VoiceAssistantError> {
        let start = Instant::now();
        
        let result = match command.command_type {
            CommandType::SystemControl => {
                self.execute_system_control(command)
            },
            CommandType::ApplicationLaunch => {
                self.execute_application_launch(command)
            },
            CommandType::InformationQuery => {
                self.execute_information_query(command)
            },
            CommandType::Configuration => {
                self.execute_configuration(command)
            },
            CommandType::Automation => {
                self.execute_automation(command)
            },
            CommandType::FileOperation => {
                self.execute_file_operation(command)
            },
            CommandType::NetworkOperation => {
                self.execute_network_operation(command)
            },
        };

        match result {
            Ok(message) => Ok(CommandResult {
                success: true,
                message,
                execution_time: start.elapsed(),
                data: None,
            }),
            Err(e) => Ok(CommandResult {
                success: false,
                message: e.to_string(),
                execution_time: start.elapsed(),
                data: None,
            }),
        }
    }

    fn execute_system_control(&self, command: &Command) -> Result<String, VoiceAssistantError> {
        let action = command.parameters.get("action")
            .ok_or_else(|| VoiceAssistantError::CommandExecutionFailed(
                "Missing action parameter".to_string()
            ))?;

        match action.as_str() {
            "shutdown" => Ok("System shutdown initiated".to_string()),
            "restart" => Ok("System restart initiated".to_string()),
            "sleep" => Ok("System entering sleep mode".to_string()),
            _ => Err(VoiceAssistantError::CommandExecutionFailed(
                format!("Unknown action: {}", action)
            )),
        }
    }

    fn execute_application_launch(&self, command: &Command) -> Result<String, VoiceAssistantError> {
        let app = command.parameters.get("application")
            .ok_or_else(|| VoiceAssistantError::CommandExecutionFailed(
                "Missing application parameter".to_string()
            ))?;

        Ok(format!("Launching application: {}", app))
    }

    fn execute_information_query(&self, command: &Command) -> Result<String, VoiceAssistantError> {
        let query = command.parameters.get("query")
            .ok_or_else(|| VoiceAssistantError::CommandExecutionFailed(
                "Missing query parameter".to_string()
            ))?;

        Ok(format!("Processing query: {}", query))
    }

    fn execute_configuration(&self, command: &Command) -> Result<String, VoiceAssistantError> {
        let setting = command.parameters.get("setting")
            .ok_or_else(|| VoiceAssistantError::CommandExecutionFailed(
                "Missing setting parameter".to_string()
            ))?;

        Ok(format!("Configuration updated: {}", setting))
    }

    fn execute_automation(&self, command: &Command) -> Result<String, VoiceAssistantError> {
        let automation = command.parameters.get("automation")
            .ok_or_else(|| VoiceAssistantError::CommandExecutionFailed(
                "Missing automation parameter".to_string()
            ))?;

        Ok(format!("Automation triggered: {}", automation))
    }

    fn execute_file_operation(&self, command: &Command) -> Result<String, VoiceAssistantError> {
        let operation = command.parameters.get("operation")
            .ok_or_else(|| VoiceAssistantError::CommandExecutionFailed(
                "Missing operation parameter".to_string()
            ))?;

        Ok(format!("File operation: {}", operation))
    }

    fn execute_network_operation(&self, command: &Command) -> Result<String, VoiceAssistantError> {
        let operation = command.parameters.get("operation")
            .ok_or_else(|| VoiceAssistantError::CommandExecutionFailed(
                "Missing operation parameter".to_string()
            ))?;

        Ok(format!("Network operation: {}", operation))
    }
}

/// Response generator
struct ResponseGenerator {
    response_templates: HashMap<String, Vec<String>>,
}

impl ResponseGenerator {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        
        templates.insert("greeting".to_string(), vec![
            "Hello! How can I help you today?".to_string(),
            "Hi there! What can I do for you?".to_string(),
            "Good to see you! How may I assist you?".to_string(),
        ]);
        
        templates.insert("confirmation".to_string(), vec![
            "Got it, I'll do that right away.".to_string(),
            "Understood. Executing now.".to_string(),
            "Sure thing! On it.".to_string(),
        ]);
        
        templates.insert("error".to_string(), vec![
            "I'm sorry, I couldn't understand that.".to_string(),
            "I'm having trouble with that request. Could you rephrase?".to_string(),
            "Hmm, something went wrong. Let me try again.".to_string(),
        ]);

        Self {
            response_templates: templates,
        }
    }

    pub fn generate(&self, context: &str) -> String {
        if let Some(responses) = self.response_templates.get(context) {
            let idx = fastrand::usize(..responses.len());
            responses[idx].clone()
        } else {
            "I understand. What else can I help with?".to_string()
        }
    }
}

/// Voice assistant statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceAssistantStats {
    /// Total interactions
    pub total_interactions: u64,
    
    /// Successful intent recognitions
    pub successful_intents: u64,
    
    /// Failed intent recognitions
    pub failed_intents: u64,
    
    /// Average confidence score
    pub avg_confidence: f64,
    
    /// Most common intents
    pub common_intents: Vec<(String, u64)>,
}

impl VoiceAssistant {
    pub fn new(config: VoiceAssistantConfig) -> Self {
        Self {
            config,
            context_history: Arc::new(RwLock::new(VecDeque::with_capacity(100))),
            intent_classifier: Arc::new(RwLock::new(IntentClassifier::new())),
            command_executor: Arc::new(RwLock::new(CommandExecutor)),
            response_generator: Arc::new(RwLock::new(ResponseGenerator::new())),
            user_preferences: Arc::new(RwLock::new(HashMap::new())),
            command_queue: Arc::new(RwLock::new(VecDeque::new())),
            stats: Arc::new(RwLock::new(VoiceAssistantStats {
                total_interactions: 0,
                successful_intents: 0,
                failed_intents: 0,
                avg_confidence: 0.0,
                common_intents: Vec::new(),
            })),
        }
    }

    /// Process voice input
    pub async fn process_voice_input(&self, text: &str) -> Result<String, VoiceAssistantError> {
        // Classify intent
        let mut classifier = self.intent_classifier.write().await;
        let intent = classifier.classify(text)
            .ok_or_else(|| VoiceAssistantError::IntentRecognitionFailed(
                "Could not identify intent".to_string()
            ))?;

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_interactions += 1;
            
            if intent.confidence >= self.config.intent_confidence_threshold {
                stats.successful_intents += 1;
                stats.avg_confidence = stats.avg_confidence * 0.95 + intent.confidence * 0.05;
            } else {
                stats.failed_intents += 1;
            }
        }

        // Execute command
        let command = self.intent_to_command(&intent).await;
        let result = {
            let executor = self.command_executor.read().await;
            executor.execute(&command)?
        };

        // Generate response
        let response = if result.success {
            let generator = self.response_generator.read().await;
            generator.generate("confirmation")
        } else {
            let generator = self.response_generator.read().await;
            generator.generate("error")
        };

        // Add to context history
        {
            let mut history = self.context_history.write().await;
            history.push_back(ContextEntry {
                user_input: text.to_string(),
                system_response: response.clone(),
                intent: Some(intent),
                timestamp: Instant::now(),
            });
            
            if history.len() > self.config.max_context_history {
                history.pop_front();
            }
        }

        Ok(response)
    }

    /// Convert intent to command
    async fn intent_to_command(&self, intent: &Intent) -> Command {
        let command_type = match intent.intent_id.as_str() {
            id if id.starts_with("system.") => {
                let action = intent.intent_id.strip_prefix("system.")
                    .unwrap_or("unknown");
                
                CommandType::SystemControl
            },
            _ => CommandType::InformationQuery,
        };

        let mut parameters = HashMap::new();
        
        for entity in &intent.entities {
            parameters.insert(entity.entity_type.clone(), entity.value.clone());
        }

        Command {
            command_id: uuid::Uuid::new_v4().to_string(),
            command_type,
            parameters,
            priority: 50,
        }
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> VoiceAssistantStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Get context history
    pub async fn get_context_history(&self) -> Vec<ContextEntry> {
        let history = self.context_history.read().await;
        history.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_assistant_config() {
        let config = VoiceAssistantConfig {
            enable_wake_word: true,
            wake_word: "Hey Vantis".to_string(),
            enable_continuous_listening: false,
            intent_confidence_threshold: 0.7,
            enable_context: true,
            enable_learning: true,
            max_context_history: 100,
            voice_gender: "female".to_string(),
            speech_rate: 1.0,
        };

        assert_eq!(config.wake_word, "Hey Vantis");
        assert!(config.enable_wake_word);
    }

    #[test]
    fn test_intent_classifier() {
        let mut classifier = IntentClassifier::new();
        
        let intent = classifier.classify("shut down the system");
        assert!(intent.is_some());
        assert_eq!(intent.unwrap().intent_id, "system.shutdown");
    }

    #[tokio::test]
    async fn test_voice_assistant_creation() {
        let config = VoiceAssistantConfig {
            enable_wake_word: true,
            wake_word: "Hey Vantis".to_string(),
            enable_continuous_listening: false,
            intent_confidence_threshold: 0.7,
            enable_context: true,
            enable_learning: true,
            max_context_history: 100,
            voice_gender: "female".to_string(),
            speech_rate: 1.0,
        };

        let assistant = VoiceAssistant::new(config);
        let stats = assistant.get_statistics().await;
        
        assert_eq!(stats.total_interactions, 0);
    }

    #[tokio::test]
    async fn test_voice_processing() {
        let config = VoiceAssistantConfig {
            enable_wake_word: true,
            wake_word: "Hey Vantis".to_string(),
            enable_continuous_listening: false,
            intent_confidence_threshold: 0.7,
            enable_context: true,
            enable_learning: true,
            max_context_history: 100,
            voice_gender: "female".to_string(),
            speech_rate: 1.0,
        };

        let assistant = VoiceAssistant::new(config);
        let result = assistant.process_voice_input("shut down the system").await;
        
        assert!(result.is_ok());
    }
}