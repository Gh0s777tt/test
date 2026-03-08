//! Natural Language Interface for System Commands
//!
//! Advanced NLP system that allows users to interact with the OS
//! using natural language, converting commands to system actions.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Configuration for natural language interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NLIConfig {
    /// Enable context awareness
    pub enable_context: bool,
    
    /// Number of previous commands to remember
    pub context_history_size: usize,
    
    /// Confidence threshold for command execution
    pub confidence_threshold: f64,
    
    /// Enable auto-completion
    pub enable_auto_completion: bool,
    
    /// Enable command suggestions
    pub enable_suggestions: bool,
    
    /// Maximum number of suggestions
    pub max_suggestions: usize,
}

impl Default for NLIConfig {
    fn default() -> Self {
        Self {
            enable_context: true,
            context_history_size: 10,
            confidence_threshold: 0.7,
            enable_auto_completion: true,
            enable_suggestions: true,
            max_suggestions: 5,
        }
    }
}

/// Intent classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Intent {
    /// Get system information
    GetInfo,
    /// Execute a command
    Execute,
    /// Create a file/directory
    Create,
    /// Delete a file/directory
    Delete,
    /// Modify a file
    Modify,
    /// Search for files
    Search,
    /// Copy files
    Copy,
    /// Move files
    Move,
    /// Open an application
    Open,
    /// Close an application
    Close,
    /// Get help
    Help,
    /// Unknown intent
    Unknown,
}

/// Command entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandEntity {
    pub entity_type: EntityType,
    pub value: String,
    pub confidence: f64,
}

/// Entity type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    /// File path
    FilePath,
    /// Directory path
    DirectoryPath,
    /// Command name
    CommandName,
    /// Application name
    ApplicationName,
    /// Number/quantity
    Number,
    /// Option/flag
    Option,
    /// User
    User,
    /// Process ID
    ProcessId,
}

/// Parsed command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCommand {
    pub intent: Intent,
    pub entities: Vec<CommandEntity>,
    pub original_text: String,
    pub confidence: f64,
    pub system_command: Option<String>,
}

/// Command suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandSuggestion {
    pub text: String,
    pub description: String,
    pub confidence: f64,
}

/// Context entry
#[derive(Debug, Clone)]
struct ContextEntry {
    timestamp: std::time::Instant,
    original_text: String,
    parsed_command: ParsedCommand,
}

/// Natural language interface
pub struct NaturalLanguageInterface {
    config: NLIConfig,
    intent_classifier: Arc<RwLock<IntentClassifier>>,
    context_history: Arc<RwLock<VecDeque<ContextEntry>>>,
    command_templates: Arc<RwLock<HashMap<Intent, Vec<String>>>>,
}

/// Intent classifier
struct IntentClassifier {
    patterns: HashMap<Intent, Vec<String>>,
}

impl IntentClassifier {
    fn new() -> Self {
        let mut patterns = HashMap::new();
        
        patterns.insert(Intent::GetInfo, vec![
            "what is", "show me", "get information", "list", "status",
            "how much", "how many", "tell me about",
        ]);
        
        patterns.insert(Intent::Execute, vec![
            "run", "execute", "start", "launch", "perform",
        ]);
        
        patterns.insert(Intent::Create, vec![
            "create", "make", "new", "add", "generate",
        ]);
        
        patterns.insert(Intent::Delete, vec![
            "delete", "remove", "erase", "clear", "destroy",
        ]);
        
        patterns.insert(Intent::Modify, vec![
            "modify", "change", "update", "edit", "alter",
        ]);
        
        patterns.insert(Intent::Search, vec![
            "search", "find", "look for", "locate", "where is",
        ]);
        
        patterns.insert(Intent::Copy, vec![
            "copy", "duplicate", "clone",
        ]);
        
        patterns.insert(Intent::Move, vec![
            "move", "rename", "transfer",
        ]);
        
        patterns.insert(Intent::Open, vec![
            "open", "launch app", "start app",
        ]);
        
        patterns.insert(Intent::Close, vec![
            "close", "quit", "exit", "stop",
        ]);
        
        patterns.insert(Intent::Help, vec![
            "help", "how to", "what can you do", "assist",
        ]);
        
        Self { patterns }
    }
    
    /// Classify intent from text
    fn classify(&self, text: &str) -> (Intent, f64) {
        let text_lower = text.to_lowercase();
        let mut best_intent = Intent::Unknown;
        let mut best_confidence = 0.0;
        
        for (intent, patterns) in &self.patterns {
            for pattern in patterns {
                if text_lower.contains(pattern) {
                    let confidence = 0.8 + (pattern.len() as f64 / text.len() as f64) * 0.2;
                    if confidence > best_confidence {
                        best_intent = *intent;
                        best_confidence = confidence;
                    }
                }
            }
        }
        
        (best_intent, best_confidence)
    }
}

impl NaturalLanguageInterface {
    /// Create a new natural language interface
    pub fn new(config: NLIConfig) -> Self {
        Self {
            config,
            intent_classifier: Arc::new(RwLock::new(IntentClassifier::new())),
            context_history: Arc::new(RwLock::new(VecDeque::with_capacity(
                config.context_history_size,
            ))),
            command_templates: Arc::new(RwLock::new(Self::init_templates())),
        }
    }
    
    /// Initialize command templates
    fn init_templates() -> HashMap<Intent, Vec<String>> {
        let mut templates = HashMap::new();
        
        templates.insert(Intent::GetInfo, vec![
            "ls -la",
            "df -h",
            "free -h",
            "ps aux",
            "uname -a",
        ]);
        
        templates.insert(Intent::Create, vec![
            "touch {path}",
            "mkdir {path}",
        ]);
        
        templates.insert(Intent::Delete, vec![
            "rm {path}",
            "rmdir {path}",
        ]);
        
        templates.insert(Intent::Search, vec![
            "find . -name '{query}'",
            "grep -r '{query}' .",
        ]);
        
        templates.insert(Intent::Copy, vec![
            "cp {source} {destination}",
        ]);
        
        templates.insert(Intent::Move, vec![
            "mv {source} {destination}",
        ]);
        
        templates.insert(Intent::Open, vec![
            "{application}",
        ]);
        
        templates.insert(Intent::Close, vec![
            "killall {application}",
        ]);
        
        templates
    }
    
    /// Process natural language input
    pub async fn process(&self, text: String) -> ParsedCommand {
        let classifier = self.intent_classifier.read().await;
        let (intent, confidence) = classifier.classify(&text);
        drop(classifier);
        
        let entities = self.extract_entities(&text, intent).await;
        let system_command = self.generate_command(intent, &entities).await;
        
        let parsed = ParsedCommand {
            intent,
            entities,
            original_text: text.clone(),
            confidence,
            system_command,
        };
        
        // Add to context
        if self.config.enable_context {
            self.add_to_context(text.clone(), parsed.clone()).await;
        }
        
        parsed
    }
    
    /// Extract entities from text
    async fn extract_entities(&self, text: &str, intent: Intent) -> Vec<CommandEntity> {
        let mut entities = Vec::new();
        
        // Extract file paths
        if text.contains("/") {
            let parts: Vec<&str> = text.split_whitespace().collect();
            for part in parts {
                if part.starts_with("/") {
                    entities.push(CommandEntity {
                        entity_type: EntityType::FilePath,
                        value: part.to_string(),
                        confidence: 0.9,
                    });
                }
            }
        }
        
        // Extract numbers
        for (pos, part) in text.split_whitespace().enumerate() {
            if part.parse::<usize>().is_ok() {
                entities.push(CommandEntity {
                    entity_type: EntityType::Number,
                    value: part.to_string(),
                    confidence: 0.85,
                });
            }
        }
        
        // Extract application names for open/close intents
        if intent == Intent::Open || intent == Intent::Close {
            let words: Vec<&str> = text.split_whitespace().collect();
            for word in words {
                if !["open", "close", "the", "app", "application"].contains(&word) {
                    entities.push(CommandEntity {
                        entity_type: EntityType::ApplicationName,
                        value: word.to_string(),
                        confidence: 0.7,
                    });
                }
            }
        }
        
        entities
    }
    
    /// Generate system command from intent and entities
    async fn generate_command(&self, intent: Intent, entities: &[CommandEntity]) -> Option<String> {
        let templates = self.command_templates.read().await;
        
        if let Some(cmd_templates) = templates.get(&intent) {
            if cmd_templates.is_empty() {
                return None;
            }
            
            let template = &cmd_templates[0];
            let mut command = template.clone();
            
            // Replace placeholders with entity values
            for entity in entities {
                match entity.entity_type {
                    EntityType::FilePath | EntityType::DirectoryPath => {
                        command = command.replace("{path}", &entity.value);
                    }
                    EntityType::ApplicationName => {
                        command = command.replace("{application}", &entity.value);
                    }
                    EntityType::Number => {
                        command = command.replace("{number}", &entity.value);
                    }
                    _ => {}
                }
            }
            
            Some(command)
        } else {
            None
        }
    }
    
    /// Add command to context history
    async fn add_to_context(&self, original_text: String, parsed: ParsedCommand) {
        let entry = ContextEntry {
            timestamp: std::time::Instant::now(),
            original_text,
            parsed_command: parsed,
        };
        
        let mut history = self.context_history.write().await;
        history.push_back(entry);
        
        while history.len() > self.config.context_history_size {
            history.pop_front();
        }
    }
    
    /// Get command suggestions based on input
    pub async fn get_suggestions(&self, partial_text: &str) -> Vec<CommandSuggestion> {
        if !self.config.enable_suggestions {
            return Vec::new();
        }
        
        let mut suggestions = Vec::new();
        
        let example_commands = vec![
            ("Show me system information", "Display CPU, memory, and disk usage"),
            ("Create a file named test.txt", "Create new file in current directory"),
            ("List all files", "Show files in current directory"),
            ("Open Firefox", "Launch the Firefox browser"),
            ("Search for files with .log extension", "Find all log files"),
            ("Delete file test.txt", "Remove the specified file"),
            ("Copy file1.txt to file2.txt", "Duplicate the file"),
        ];
        
        for (text, description) in example_commands {
            if text.to_lowercase().contains(&partial_text.to_lowercase()) {
                suggestions.push(CommandSuggestion {
                    text: text.to_string(),
                    description: description.to_string(),
                    confidence: 0.8,
                });
            }
            
            if suggestions.len() >= self.config.max_suggestions {
                break;
            }
        }
        
        suggestions
    }
    
    /// Auto-complete partial input
    pub async fn auto_complete(&self, partial: &str) -> Vec<String> {
        if !self.config.enable_auto_completion {
            return Vec::new();
        }
        
        let mut completions = Vec::new();
        
        let common_completions = vec![
            "show me system information",
            "list all files",
            "create a file",
            "delete a file",
            "search for",
            "open application",
            "close application",
        ];
        
        for completion in common_completions {
            if completion.starts_with(partial) {
                completions.push(completion.to_string());
            }
        }
        
        completions
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
    
    #[tokio::test]
    async fn test_process_simple_command() {
        let config = NLIConfig::default();
        let nli = NaturalLanguageInterface::new(config);
        
        let result = nli.process("show me system information".to_string()).await;
        assert_eq!(result.intent, Intent::GetInfo);
        assert!(result.confidence > 0.7);
    }
    
    #[tokio::test]
    async fn test_process_file_creation() {
        let config = NLIConfig::default();
        let nli = NaturalLanguageInterface::new(config);
        
        let result = nli.process("create a file named test.txt".to_string()).await;
        assert_eq!(result.intent, Intent::Create);
    }
    
    #[tokio::test]
    async fn test_extract_entities() {
        let config = NLIConfig::default();
        let nli = NaturalLanguageInterface::new(config);
        
        let entities = nli.extract_entities("open /home/user/file.txt", Intent::Open).await;
        assert!(!entities.is_empty());
        assert_eq!(entities[0].entity_type, EntityType::FilePath);
    }
    
    #[tokio::test]
    async fn test_get_suggestions() {
        let config = NLIConfig::default();
        let nli = NaturalLanguageInterface::new(config);
        
        let suggestions = nli.get_suggestions("sho").await;
        assert!(!suggestions.is_empty());
    }
}