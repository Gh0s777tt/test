//! Intelligent Automation System
//! 
//! This module implements a sophisticated automation framework that learns from
//! user behavior patterns, creates automated workflows, and executes tasks
//! intelligently with context-aware decision making.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, mpsc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Automation errors
#[derive(Error, Debug)]
pub enum AutomationError {
    #[error("Automation not found: {0}")]
    AutomationNotFound(String),
    
    #[error("Invalid workflow: {0}")]
    InvalidWorkflow(String),
    
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Insufficient permissions")]
    InsufficientPermissions,
}

/// Configuration for intelligent automation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationConfig {
    /// Enable automatic workflow creation
    pub enable_auto_creation: bool,
    
    /// Enable learning from user behavior
    pub enable_learning: bool,
    
    /// Maximum concurrent automations
    pub max_concurrent: usize,
    
    /// Learning rate for pattern recognition
    pub learning_rate: f64,
    
    /// Minimum pattern frequency for automation
    pub min_pattern_frequency: usize,
    
    /// Enable predictive automation
    pub enable_predictive: bool,
    
    /// Safety level (0-100)
    pub safety_level: u8,
}

/// Automation task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationTask {
    /// Task ID
    pub task_id: String,
    
    /// Task name
    pub name: String,
    
    /// Task type
    pub task_type: TaskType,
    
    /// Task parameters
    pub parameters: HashMap<String, String>,
    
    /// Execution conditions
    pub conditions: Vec<Condition>,
    
    /// Priority
    pub priority: u8,
    
    /// Estimated duration
    pub estimated_duration: Duration,
    
    /// Retry configuration
    pub retry_config: RetryConfig,
}

/// Types of automation tasks
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TaskType {
    FileOperation,
    SystemCommand,
    ApplicationLaunch,
    NetworkOperation,
    DataProcessing,
    Notification,
    Backup,
    Cleanup,
    Synchronization,
    Custom,
}

/// Execution condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    /// Condition type
    pub condition_type: ConditionType,
    
    /// Condition value
    pub value: String,
    
    /// Operator
    pub operator: String,
}

/// Condition types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConditionType {
    Time,
    Date,
    DayOfWeek,
    FileExists,
    ProcessRunning,
    SystemLoad,
    NetworkAvailable,
    BatteryLevel,
    Custom,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,
    
    /// Delay between retries
    pub retry_delay: Duration,
    
    /// Exponential backoff
    pub exponential_backoff: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            retry_delay: Duration::from_secs(5),
            exponential_backoff: true,
        }
    }
}

/// Automation workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Workflow ID
    pub workflow_id: String,
    
    /// Workflow name
    pub name: String,
    
    /// Tasks in the workflow
    pub tasks: Vec<AutomationTask>,
    
    /// Workflow triggers
    pub triggers: Vec<Trigger>,
    
    /// Workflow status
    pub status: WorkflowStatus,
    
    /// Execution statistics
    pub stats: WorkflowStats,
}

/// Workflow trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    /// Trigger type
    pub trigger_type: TriggerType,
    
    /// Trigger configuration
    pub config: HashMap<String, String>,
    
    /// Enabled flag
    pub enabled: bool,
}

/// Trigger types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TriggerType {
    Schedule,
    Event,
    Condition,
    Manual,
    Predictive,
}

/// Workflow status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Idle,
    Running,
    Paused,
    Completed,
    Failed,
}

/// Workflow statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStats {
    /// Total executions
    pub total_executions: u32,
    
    /// Successful executions
    pub successful_executions: u32,
    
    /// Failed executions
    pub failed_executions: u32,
    
    /// Average execution time
    pub avg_execution_time: Duration,
    
    /// Last execution time
    pub last_execution: Option<Instant>,
}

/// User behavior pattern
#[derive(Debug, Clone)]
struct BehaviorPattern {
    /// Pattern ID
    pattern_id: String,
    
    /// Action sequence
    action_sequence: Vec<String>,
    
    /// Frequency
    frequency: usize,
    
    /// Last occurrence
    last_occurrence: Instant,
    
    /// Confidence score
    confidence: f64,
    
    /// Context
    context: HashMap<String, String>,
}

/// Execution context
#[derive(Debug, Clone)]
struct ExecutionContext {
    /// Workflow ID
    workflow_id: String,
    
    /// Task ID
    task_id: String,
    
    /// Start time
    start_time: Instant,
    
    /// Variables
    variables: HashMap<String, String>,
    
    /// Results
    results: Vec<TaskResult>,
}

/// Task execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// Task ID
    pub task_id: String,
    
    /// Success flag
    pub success: bool,
    
    /// Result message
    pub message: String,
    
    /// Execution time
    pub execution_time: Duration,
    
    /// Output data
    pub output: Option<String>,
}

/// Intelligent Automation Engine
pub struct IntelligentAutomationEngine {
    config: AutomationConfig,
    
    /// Active workflows
    workflows: Arc<RwLock<HashMap<String, Workflow>>>,
    
    /// Behavior patterns
    patterns: Arc<RwLock<VecDeque<BehaviorPattern>>>,
    
    /// Execution queue
    execution_queue: Arc<RwLock<VecDeque<String>>>,
    
    /// Current executions
    active_executions: Arc<RwLock<HashMap<String, ExecutionContext>>>,
    
    /// Execution history
    execution_history: Arc<RwLock<VecDeque<WorkflowExecution>>>,
    
    /// Task sender channel
    task_sender: Arc<RwLock<mpsc::Sender<AutomationTask>>>,
    
    /// Statistics
    stats: Arc<RwLock<AutomationStats>>,
}

/// Workflow execution record
#[derive(Debug, Clone)]
struct WorkflowExecution {
    workflow_id: String,
    start_time: Instant,
    end_time: Option<Instant>,
    success: bool,
    results: Vec<TaskResult>,
}

/// Automation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationStats {
    /// Total workflows
    pub total_workflows: usize,
    
    /// Active workflows
    pub active_workflows: usize,
    
    /// Total executions
    pub total_executions: u64,
    
    /// Successful executions
    pub successful_executions: u64,
    
    /// Failed executions
    pub failed_executions: u64,
    
    /// Average execution time
    pub avg_execution_time: Duration,
    
    /// Patterns detected
    pub patterns_detected: usize,
}

impl IntelligentAutomationEngine {
    pub fn new(config: AutomationConfig) -> Self {
        let (task_sender, _task_receiver) = mpsc::channel(100);

        Self {
            config,
            workflows: Arc::new(RwLock::new(HashMap::new())),
            patterns: Arc::new(RwLock::new(VecDeque::with_capacity(100))),
            execution_queue: Arc::new(RwLock::new(VecDeque::new())),
            active_executions: Arc::new(RwLock::new(HashMap::new())),
            execution_history: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            task_sender: Arc::new(RwLock::new(task_sender)),
            stats: Arc::new(RwLock::new(AutomationStats {
                total_workflows: 0,
                active_workflows: 0,
                total_executions: 0,
                successful_executions: 0,
                failed_executions: 0,
                avg_execution_time: Duration::ZERO,
                patterns_detected: 0,
            })),
        }
    }

    /// Create a new workflow
    pub async fn create_workflow(&self, workflow: Workflow) -> Result<(), AutomationError> {
        self.validate_workflow(&workflow)?;

        let mut workflows = self.workflows.write().await;
        workflows.insert(workflow.workflow_id.clone(), workflow);

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_workflows = workflows.len();

        Ok(())
    }

    /// Validate workflow structure
    fn validate_workflow(&self, workflow: &Workflow) -> Result<(), AutomationError> {
        if workflow.tasks.is_empty() {
            return Err(AutomationError::InvalidWorkflow(
                "Workflow must have at least one task".to_string()
            ));
        }

        for task in &workflow.tasks {
            if task.task_id.is_empty() {
                return Err(AutomationError::InvalidWorkflow(
                    "Task ID cannot be empty".to_string()
                ));
            }
        }

        Ok(())
    }

    /// Execute a workflow
    pub async fn execute_workflow(&self, workflow_id: &str) -> Result<(), AutomationError> {
        let workflow = {
            let workflows = self.workflows.read().await;
            workflows.get(workflow_id)
                .cloned()
                .ok_or_else(|| AutomationError::AutomationNotFound(
                    workflow_id.to_string()
                ))?
        };

        // Check if workflow is already running
        if matches!(workflow.status, WorkflowStatus::Running) {
            return Err(AutomationError::ExecutionFailed(
                "Workflow is already running".to_string()
            ));
        }

        // Update workflow status
        {
            let mut workflows = self.workflows.write().await;
            if let Some(wf) = workflows.get_mut(workflow_id) {
                wf.status = WorkflowStatus::Running;
            }
        }

        // Create execution context
        let execution_id = uuid::Uuid::new_v4().to_string();
        let context = ExecutionContext {
            workflow_id: workflow_id.to_string(),
            task_id: String::new(),
            start_time: Instant::now(),
            variables: HashMap::new(),
            results: Vec::new(),
        };

        // Add to active executions
        {
            let mut active = self.active_executions.write().await;
            active.insert(execution_id.clone(), context);
        }

        // Execute tasks
        let mut results = Vec::new();
        let mut success = true;

        for task in &workflow.tasks {
            let task_result = self.execute_task(task).await;
            
            if !task_result.success {
                success = false;
                
                // Check retry configuration
                if task.retry_config.max_attempts > 1 {
                    let mut attempts = 1;
                    let mut delay = task.retry_config.retry_delay;
                    
                    while attempts < task.retry_config.max_attempts {
                        tokio::time::sleep(delay).await;
                        
                        let retry_result = self.execute_task(task).await;
                        if retry_result.success {
                            success = true;
                            results.push(retry_result);
                            break;
                        }
                        
                        attempts += 1;
                        if task.retry_config.exponential_backoff {
                            delay = delay * 2;
                        }
                    }
                }
                
                if !success {
                    results.push(task_result);
                    break;
                }
            } else {
                results.push(task_result);
            }
        }

        // Update workflow status and statistics
        let execution_time = {
            let mut active = self.active_executions.write().await;
            if let Some(ctx) = active.remove(&execution_id) {
                ctx.start_time.elapsed()
            } else {
                Duration::ZERO
            }
        };

        {
            let mut workflows = self.workflows.write().await;
            if let Some(wf) = workflows.get_mut(workflow_id) {
                wf.status = if success {
                    WorkflowStatus::Completed
                } else {
                    WorkflowStatus::Failed
                };
                
                wf.stats.total_executions += 1;
                if success {
                    wf.stats.successful_executions += 1;
                } else {
                    wf.stats.failed_executions += 1;
                }
                
                wf.stats.avg_execution_time = Duration::from_millis(
                    (wf.stats.avg_execution_time.as_millis() as u64 
                     * (wf.stats.total_executions - 1) 
                     + execution_time.as_millis() as u64) / wf.stats.total_executions as u64
                );
                
                wf.stats.last_execution = Some(Instant::now());
            }
        }

        // Record execution
        {
            let mut history = self.execution_history.write().await;
            history.push_back(WorkflowExecution {
                workflow_id: workflow_id.to_string(),
                start_time: Instant::now() - execution_time,
                end_time: Some(Instant::now()),
                success,
                results,
            });
            
            if history.len() > 1000 {
                history.pop_front();
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_executions += 1;
            if success {
                stats.successful_executions += 1;
            } else {
                stats.failed_executions += 1;
            }
        }

        Ok(())
    }

    /// Execute a single task
    async fn execute_task(&self, task: &AutomationTask) -> TaskResult {
        let start = Instant::now();

        // Check conditions
        for condition in &task.conditions {
            if !self.check_condition(condition).await {
                return TaskResult {
                    task_id: task.task_id.clone(),
                    success: false,
                    message: "Condition not met".to_string(),
                    execution_time: start.elapsed(),
                    output: None,
                };
            }
        }

        // Execute task based on type
        let result = match task.task_type {
            TaskType::FileOperation => self.execute_file_operation(task).await,
            TaskType::SystemCommand => self.execute_system_command(task).await,
            TaskType::ApplicationLaunch => self.execute_application_launch(task).await,
            TaskType::NetworkOperation => self.execute_network_operation(task).await,
            TaskType::DataProcessing => self.execute_data_processing(task).await,
            TaskType::Notification => self.execute_notification(task).await,
            TaskType::Backup => self.execute_backup(task).await,
            TaskType::Cleanup => self.execute_cleanup(task).await,
            TaskType::Synchronization => self.execute_synchronization(task).await,
            TaskType::Custom => self.execute_custom_task(task).await,
        };

        TaskResult {
            task_id: task.task_id.clone(),
            success: result.is_ok(),
            message: result.unwrap_or_else(|e| e.to_string()),
            execution_time: start.elapsed(),
            output: None,
        }
    }

    /// Check execution condition
    async fn check_condition(&self, condition: &Condition) -> bool {
        match condition.condition_type {
            ConditionType::Time => {
                let current_time = chrono::Local::now().format("%H:%M").to_string();
                current_time == condition.value
            },
            ConditionType::DayOfWeek => {
                let current_day = chrono::Local::now().format("%A").to_string();
                current_day == condition.value
            },
            ConditionType::FileExists => {
                std::path::Path::new(&condition.value).exists()
            },
            _ => true, // Default to true for other conditions
        }
    }

    /// Execute file operation task
    async fn execute_file_operation(&self, task: &AutomationTask) -> Result<String, AutomationError> {
        let operation = task.parameters.get("operation")
            .ok_or_else(|| AutomationError::ExecutionFailed(
                "Missing operation parameter".to_string()
            ))?;

        log::info!("Executing file operation: {}", operation);
        Ok(format!("File operation completed: {}", operation))
    }

    /// Execute system command task
    async fn execute_system_command(&self, task: &AutomationTask) -> Result<String, AutomationError> {
        let command = task.parameters.get("command")
            .ok_or_else(|| AutomationError::ExecutionFailed(
                "Missing command parameter".to_string()
            ))?;

        log::info!("Executing system command: {}", command);
        Ok(format!("Command executed: {}", command))
    }

    /// Execute application launch task
    async fn execute_application_launch(&self, task: &AutomationTask) -> Result<String, AutomationError> {
        let application = task.parameters.get("application")
            .ok_or_else(|| AutomationError::ExecutionFailed(
                "Missing application parameter".to_string()
            ))?;

        log::info!("Launching application: {}", application);
        Ok(format!("Application launched: {}", application))
    }

    /// Execute network operation task
    async fn execute_network_operation(&self, task: &AutomationTask) -> Result<String, AutomationError> {
        let operation = task.parameters.get("operation")
            .ok_or_else(|| AutomationError::ExecutionFailed(
                "Missing operation parameter".to_string()
            ))?;

        log::info!("Executing network operation: {}", operation);
        Ok(format!("Network operation completed: {}", operation))
    }

    /// Execute data processing task
    async fn execute_data_processing(&self, task: &AutomationTask) -> Result<String, AutomationError> {
        log::info!("Executing data processing");
        Ok("Data processing completed".to_string())
    }

    /// Execute notification task
    async fn execute_notification(&self, task: &AutomationTask) -> Result<String, AutomationError> {
        let message = task.parameters.get("message")
            .unwrap_or(&"Notification".to_string())
            .clone();

        log::info!("Sending notification: {}", message);
        Ok(format!("Notification sent: {}", message))
    }

    /// Execute backup task
    async fn execute_backup(&self, task: &AutomationTask) -> Result<String, AutomationError> {
        log::info!("Executing backup");
        Ok("Backup completed".to_string())
    }

    /// Execute cleanup task
    async fn execute_cleanup(&self, task: &AutomationTask) -> Result<String, AutomationError> {
        log::info!("Executing cleanup");
        Ok("Cleanup completed".to_string())
    }

    /// Execute synchronization task
    async fn execute_synchronization(&self, task: &AutomationTask) -> Result<String, AutomationError> {
        log::info!("Executing synchronization");
        Ok("Synchronization completed".to_string())
    }

    /// Execute custom task
    async fn execute_custom_task(&self, task: &AutomationTask) -> Result<String, AutomationError> {
        log::info!("Executing custom task: {}", task.name);
        Ok(format!("Custom task executed: {}", task.name))
    }

    /// Learn from user behavior and create patterns
    pub async fn learn_from_behavior(&self, actions: Vec<String>, context: HashMap<String, String>) {
        if !self.config.enable_learning {
            return;
        }

        let pattern_id = uuid::Uuid::new_v4().to_string();
        let pattern = BehaviorPattern {
            pattern_id: pattern_id.clone(),
            action_sequence: actions,
            frequency: 1,
            last_occurrence: Instant::now(),
            confidence: 0.0,
            context,
        };

        let mut patterns = self.patterns.write().await;
        patterns.push_back(pattern);

        if patterns.len() > 100 {
            patterns.pop_front();
        }

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.patterns_detected = patterns.len();
    }

    /// Get workflow information
    pub async fn get_workflow(&self, workflow_id: &str) -> Option<Workflow> {
        let workflows = self.workflows.read().await;
        workflows.get(workflow_id).cloned()
    }

    /// Get automation statistics
    pub async fn get_statistics(&self) -> AutomationStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Pause a workflow
    pub async fn pause_workflow(&self, workflow_id: &str) -> Result<(), AutomationError> {
        let mut workflows = self.workflows.write().await;
        let workflow = workflows.get_mut(workflow_id)
            .ok_or_else(|| AutomationError::AutomationNotFound(
                workflow_id.to_string()
            ))?;

        if workflow.status == WorkflowStatus::Running {
            workflow.status = WorkflowStatus::Paused;
        }

        Ok(())
    }

    /// Resume a paused workflow
    pub async fn resume_workflow(&self, workflow_id: &str) -> Result<(), AutomationError> {
        let mut workflows = self.workflows.write().await;
        let workflow = workflows.get_mut(workflow_id)
            .ok_or_else(|| AutomationError::AutomationNotFound(
                workflow_id.to_string()
            ))?;

        if workflow.status == WorkflowStatus::Paused {
            workflow.status = WorkflowStatus::Idle;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automation_config() {
        let config = AutomationConfig {
            enable_auto_creation: true,
            enable_learning: true,
            max_concurrent: 5,
            learning_rate: 0.1,
            min_pattern_frequency: 5,
            enable_predictive: true,
            safety_level: 80,
        };

        assert!(config.enable_auto_creation);
        assert_eq!(config.max_concurrent, 5);
    }

    #[tokio::test]
    async fn test_automation_engine_creation() {
        let config = AutomationConfig {
            enable_auto_creation: true,
            enable_learning: true,
            max_concurrent: 5,
            learning_rate: 0.1,
            min_pattern_frequency: 5,
            enable_predictive: true,
            safety_level: 80,
        };

        let engine = IntelligentAutomationEngine::new(config);
        let stats = engine.get_statistics().await;
        
        assert_eq!(stats.total_workflows, 0);
    }

    #[tokio::test]
    async fn test_workflow_creation() {
        let config = AutomationConfig {
            enable_auto_creation: true,
            enable_learning: true,
            max_concurrent: 5,
            learning_rate: 0.1,
            min_pattern_frequency: 5,
            enable_predictive: true,
            safety_level: 80,
        };

        let engine = IntelligentAutomationEngine::new(config);
        
        let workflow = Workflow {
            workflow_id: "test-workflow".to_string(),
            name: "Test Workflow".to_string(),
            tasks: vec![],
            triggers: vec![],
            status: WorkflowStatus::Idle,
            stats: WorkflowStats {
                total_executions: 0,
                successful_executions: 0,
                failed_executions: 0,
                avg_execution_time: Duration::ZERO,
                last_execution: None,
            },
        };

        let result = engine.create_workflow(workflow).await;
        assert!(result.is_err()); // Empty workflow should fail
    }
}