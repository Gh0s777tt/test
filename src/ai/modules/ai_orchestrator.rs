<content>
//! AI Orchestrator Module
//! Provides orchestration of AI workflows, managing complex multi-step
//! AI operations, dependencies, and execution pipelines.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// AI Orchestrator Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorConfig {
    /// Enable parallel execution
    pub enable_parallel_execution: bool,
    
    /// Enable caching of workflow results
    pub enable_workflow_caching: bool,
    
    /// Enable automatic retries
    pub enable_auto_retry: bool,
    
    /// Maximum concurrent tasks
    pub max_concurrent_tasks: usize,
    
    /// Maximum retry attempts
    pub max_retry_attempts: u32,
    
    /// Task timeout in milliseconds
    pub task_timeout_ms: u64,
    
    /// Workflow timeout in milliseconds
    pub workflow_timeout_ms: u64,
    
    /// Enable progress tracking
    pub enable_progress_tracking: bool,
    
    /// Enable checkpoint recovery
    pub enable_checkpointing: bool,
    
    /// Checkpoint interval in milliseconds
    pub checkpoint_interval_ms: u64,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            enable_parallel_execution: true,
            enable_workflow_caching: true,
            enable_auto_retry: true,
            max_concurrent_tasks: 10,
            max_retry_attempts: 3,
            task_timeout_ms: 60000,
            workflow_timeout_ms: 300000,
            enable_progress_tracking: true,
            enable_checkpointing: true,
            checkpoint_interval_ms: 10000,
        }
    }
}

/// Workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Workflow ID
    pub id: String,
    
    /// Workflow name
    pub name: String,
    
    /// Workflow description
    pub description: String,
    
    /// Workflow version
    pub version: String,
    
    /// Tasks in the workflow
    pub tasks: Vec<WorkflowTask>,
    
    /// Task dependencies (task_id -> depends_on_task_ids)
    pub dependencies: HashMap<String, Vec<String>>,
    
    /// Workflow parameters
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Workflow tags
    pub tags: Vec<String>,
    
    /// Created at
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Workflow task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTask {
    /// Task ID
    pub id: String,
    
    /// Task name
    pub name: String,
    
    /// Task type
    pub task_type: TaskType,
    
    /// Task configuration
    pub config: TaskConfig,
    
    /// Task inputs
    pub inputs: TaskInputs,
    
    /// Task outputs
    pub outputs: TaskOutputs,
    
    /// Retry configuration
    pub retry_config: Option<RetryConfig>,
    
    /// Timeout override
    pub timeout_ms: Option<u64>,
    
    /// Priority
    pub priority: u8,
}

/// Task types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    /// AI model inference
    Inference,
    
    /// Data preprocessing
    Preprocessing,
    
    /// Data postprocessing
    Postprocessing,
    
    /// Feature extraction
    FeatureExtraction,
    
    /// Model training
    Training,
    
    /// Evaluation
    Evaluation,
    
    /// Validation
    Validation,
    
    /// Custom task
    Custom(String),
}

/// Task configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    /// Model to use
    pub model: Option<String>,
    
    /// Parameters
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
    
    /// Execution environment
    pub environment: ExecutionEnvironment,
}

/// Resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// CPU cores needed
    pub cpu_cores: u32,
    
    /// Memory in MB
    pub memory_mb: u64,
    
    /// GPU needed
    pub gpu_required: bool,
    
    /// GPU memory in MB
    pub gpu_memory_mb: u64,
    
    /// Disk space in MB
    pub disk_mb: u64,
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self {
            cpu_cores: 1,
            memory_mb: 512,
            gpu_required: false,
            gpu_memory_mb: 0,
            disk_mb: 100,
        }
    }
}

/// Execution environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionEnvironment {
    /// Environment type
    pub env_type: EnvironmentType,
    
    /// Docker image (if containerized)
    pub docker_image: Option<String>,
    
    /// Python version (if applicable)
    pub python_version: Option<String>,
    
    /// Required packages
    pub packages: Vec<String>,
    
    /// Environment variables
    pub env_vars: HashMap<String, String>,
}

/// Environment types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentType {
    /// Native execution
    Native,
    
    /// Docker container
    Docker,
    
    /// Kubernetes pod
    Kubernetes,
    
    /// Virtual machine
    VirtualMachine,
    
    /// Serverless
    Serverless,
}

/// Task inputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskInputs {
    /// Direct value
    Value(serde_json::Value),
    
    /// Reference to previous task output
    Reference(String),
    
    /// Multiple inputs
    Multiple(Vec<TaskInputs>),
    
    /// No inputs
    None,
}

/// Task outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskOutputs {
    /// Expected output schema
    Schema(serde_json::Value),
    
    /// No output
    None,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,
    
    /// Retry delay in milliseconds
    pub delay_ms: u64,
    
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    
    /// Retry on errors
    pub retry_on_errors: Vec<String>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            delay_ms: 1000,
            backoff_multiplier: 2.0,
            retry_on_errors: vec!["timeout".to_string(), "network_error".to_string()],
        }
    }
}

/// Workflow execution instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    /// Execution ID
    pub execution_id: String,
    
    /// Workflow ID being executed
    pub workflow_id: String,
    
    /// Execution status
    pub status: ExecutionStatus,
    
    /// Task executions
    pub task_executions: HashMap<String, TaskExecution>,
    
    /// Execution parameters
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Start time
    pub started_at: chrono::DateTime<chrono::Utc>,
    
    /// End time
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Error message if failed
    pub error: Option<String>,
    
    /// Progress percentage
    pub progress: f64,
    
    /// Output data
    pub outputs: HashMap<String, serde_json::Value>,
}

/// Execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// Task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskExecution {
    /// Task ID
    pub task_id: String,
    
    /// Execution status
    pub status: ExecutionStatus,
    
    /// Number of attempts
    pub attempts: u32,
    
    /// Start time
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    
    /// End time
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Duration in milliseconds
    pub duration_ms: Option<f64>,
    
    /// Output data
    pub output: Option<serde_json::Value>,
    
    /// Error message if failed
    pub error: Option<String>,
    
    /// Progress percentage
    pub progress: f64,
}

/// Workflow checkpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowCheckpoint {
    /// Checkpoint ID
    pub checkpoint_id: String,
    
    /// Execution ID
    pub execution_id: String,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Completed task IDs
    pub completed_tasks: Vec<String>,
    
    /// Task outputs
    pub task_outputs: HashMap<String, serde_json::Value>,
    
    /// Execution parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// AI Orchestrator
pub struct AiOrchestrator {
    config: OrchestratorConfig,
    workflows: HashMap<String, Workflow>,
    executions: HashMap<String, WorkflowExecution>,
    checkpoints: HashMap<String, WorkflowCheckpoint>,
    task_queue: VecDeque<TaskExecutionRequest>,
    active_tasks: HashMap<String, ActiveTask>,
}

/// Task execution request
#[derive(Debug, Clone)]
struct TaskExecutionRequest {
    execution_id: String,
    task_id: String,
    task: WorkflowTask,
    priority: u8,
    retry_count: u32,
}

/// Active task
#[derive(Debug, Clone)]
struct ActiveTask {
    execution_id: String,
    task_id: String,
    started_at: Instant,
}

impl AiOrchestrator {
    /// Create a new AI orchestrator
    pub fn new(config: OrchestratorConfig) -> Self {
        Self {
            config,
            workflows: HashMap::new(),
            executions: HashMap::new(),
            checkpoints: HashMap::new(),
            task_queue: VecDeque::new(),
            active_tasks: HashMap::new(),
        }
    }
    
    /// Create with default configuration
    pub fn default_orchestrator() -> Self {
        Self::new(OrchestratorConfig::default())
    }
    
    /// Register a workflow
    pub fn register_workflow(&mut self, workflow: Workflow) {
        self.workflows.insert(workflow.id.clone(), workflow);
    }
    
    /// Unregister a workflow
    pub fn unregister_workflow(&mut self, workflow_id: &str) {
        self.workflows.remove(workflow_id);
    }
    
    /// Start a workflow execution
    pub fn start_workflow(
        &mut self,
        workflow_id: &str,
        parameters: HashMap<String, serde_json::Value>,
    ) -> Result<String, OrchestratorError> {
        let workflow = self.workflows.get(workflow_id)
            .ok_or_else(|| OrchestratorError::WorkflowNotFound(workflow_id.to_string()))?;
        
        let execution_id = uuid::Uuid::new_v4().to_string();
        
        // Initialize task executions
        let mut task_executions = HashMap::new();
        for task in &workflow.tasks {
            task_executions.insert(task.id.clone(), TaskExecution {
                task_id: task.id.clone(),
                status: ExecutionStatus::Pending,
                attempts: 0,
                started_at: None,
                ended_at: None,
                duration_ms: None,
                output: None,
                error: None,
                progress: 0.0,
            });
        }
        
        // Create execution
        let execution = WorkflowExecution {
            execution_id: execution_id.clone(),
            workflow_id: workflow_id.to_string(),
            status: ExecutionStatus::Running,
            task_executions,
            parameters,
            started_at: chrono::Utc::now(),
            ended_at: None,
            error: None,
            progress: 0.0,
            outputs: HashMap::new(),
        };
        
        self.executions.insert(execution_id.clone(), execution);
        
        // Queue initial tasks (those with no dependencies)
        self.queue_initial_tasks(&execution_id, workflow)?;
        
        Ok(execution_id)
    }
    
    /// Queue initial tasks (no dependencies)
    fn queue_initial_tasks(&mut self, execution_id: &str, workflow: &Workflow) -> Result<(), OrchestratorError> {
        for task in &workflow.tasks {
            let deps = workflow.dependencies.get(&task.id).cloned().unwrap_or_default();
            if deps.is_empty() {
                self.task_queue.push_back(TaskExecutionRequest {
                    execution_id: execution_id.to_string(),
                    task_id: task.id.clone(),
                    task: task.clone(),
                    priority: task.priority,
                    retry_count: 0,
                });
            }
        }
        Ok(())
    }
    
    /// Execute workflow
    pub fn execute_workflow(&mut self, execution_id: &str) -> Result<WorkflowExecution, OrchestratorError> {
        let execution = self.executions.get_mut(execution_id)
            .ok_or_else(|| OrchestratorError::ExecutionNotFound(execution_id.to_string()))?;
        
        let workflow_id = execution.workflow_id.clone();
        let workflow = self.workflows.get(&workflow_id)
            .ok_or_else(|| OrchestratorError::WorkflowNotFound(workflow_id))?;
        
        while !self.task_queue.is_empty() && self.active_tasks.len() < self.config.max_concurrent_tasks {
            if let Some(request) = self.task_queue.pop_front() {
                self.execute_task(&request)?;
            }
        }
        
        // Wait for active tasks to complete
        while !self.active_tasks.is_empty() {
            self.process_active_tasks();
        }
        
        // Update execution status
        self.update_execution_status(execution_id)?;
        
        let result = self.executions.get(execution_id).unwrap().clone();
        Ok(result)
    }
    
    /// Execute a single task
    fn execute_task(&mut self, request: &TaskExecutionRequest) -> Result<(), OrchestratorError> {
        let execution = self.executions.get_mut(&request.execution_id).unwrap();
        
        // Update task status
        let task_execution = execution.task_executions.get_mut(&request.task_id).unwrap();
        task_execution.status = ExecutionStatus::Running;
        task_execution.started_at = Some(chrono::Utc::now());
        task_execution.attempts += 1;
        
        // Add to active tasks
        let active_task = ActiveTask {
            execution_id: request.execution_id.clone(),
            task_id: request.task_id.clone(),
            started_at: Instant::now(),
        };
        self.active_tasks.insert(request.task_id.clone(), active_task);
        
        // Simulate task execution
        let output = self.simulate_task_execution(&request.task)?;
        
        // Mark task as complete
        let task_execution = execution.task_executions.get_mut(&request.task_id).unwrap();
        task_execution.status = ExecutionStatus::Completed;
        task_execution.ended_at = Some(chrono::Utc::now());
        task_execution.duration_ms = Some(Instant::now().duration_since(active_task.started_at).as_millis() as f64);
        task_execution.output = Some(output);
        task_execution.progress = 1.0;
        
        // Remove from active tasks
        self.active_tasks.remove(&request.task_id);
        
        // Queue dependent tasks
        self.queue_dependent_tasks(&request.execution_id, &request.task_id)?;
        
        // Save checkpoint if enabled
        if self.config.enable_checkpointing {
            self.save_checkpoint(&request.execution_id)?;
        }
        
        Ok(())
    }
    
    /// Simulate task execution
    fn simulate_task_execution(&self, task: &WorkflowTask) -> Result<serde_json::Value, OrchestratorError> {
        // Simulate processing time
        std::thread::sleep(Duration::from_millis(100));
        
        // Return mock output based on task type
        Ok(match task.task_type {
            TaskType::Inference => serde_json::json!({
                "predictions": [0.85, 0.12, 0.03],
                "model": task.config.model.unwrap_or_default()
            }),
            TaskType::Preprocessing => serde_json::json!({
                "processed": true,
                "records": 100
            }),
            TaskType::Postprocessing => serde_json::json!({
                "finalized": true,
                "output": "result"
            }),
            TaskType::FeatureExtraction => serde_json::json!({
                "features": ["f1", "f2", "f3"],
                "vectors": [[0.1, 0.2, 0.3]]
            }),
            TaskType::Training => serde_json::json!({
                "loss": 0.15,
                "accuracy": 0.92,
                "epochs": 10
            }),
            TaskType::Evaluation => serde_json::json!({
                "metrics": {"precision": 0.9, "recall": 0.88, "f1": 0.89}
            }),
            TaskType::Validation => serde_json::json!({
                "valid": true,
                "warnings": []
            }),
            TaskType::Custom(_) => serde_json::json!({"result": "completed"}),
        })
    }
    
    /// Queue dependent tasks
    fn queue_dependent_tasks(&mut self, execution_id: &str, completed_task_id: &str) -> Result<(), OrchestratorError> {
        let execution = self.executions.get(execution_id).unwrap();
        let workflow = self.workflows.get(&execution.workflow_id).unwrap();
        
        // Find tasks that depend on the completed task
        for task in &workflow.tasks {
            if let Some(deps) = workflow.dependencies.get(&task.id) {
                if deps.contains(completed_task_id) {
                    // Check if all dependencies are complete
                    let all_complete = deps.iter().all(|dep_id| {
                        execution.task_executions.get(dep_id)
                            .map(|te| te.status == ExecutionStatus::Completed)
                            .unwrap_or(false)
                    });
                    
                    if all_complete {
                        // Check if task is already queued
                        let already_queued = self.task_queue.iter()
                            .any(|t| t.task_id == task.id);
                        let already_complete = execution.task_executions.get(&task.id)
                            .map(|te| te.status == ExecutionStatus::Completed)
                            .unwrap_or(false);
                        
                        if !already_queued && !already_complete {
                            self.task_queue.push_back(TaskExecutionRequest {
                                execution_id: execution_id.to_string(),
                                task_id: task.id.clone(),
                                task: task.clone(),
                                priority: task.priority,
                                retry_count: 0,
                            });
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Process active tasks
    fn process_active_tasks(&mut self) {
        // In a real implementation, this would poll or use callbacks
        // For now, assume all tasks complete immediately
        let task_ids: Vec<String> = self.active_tasks.keys().cloned().collect();
        for task_id in task_ids {
            // Task already completed in execute_task
            self.active_tasks.remove(&task_id);
        }
    }
    
    /// Update execution status
    fn update_execution_status(&mut self, execution_id: &str) -> Result<(), OrchestratorError> {
        let execution = self.executions.get_mut(execution_id).unwrap();
        let workflow = self.workflows.get(&execution.workflow_id).unwrap();
        
        // Calculate progress
        let total_tasks = workflow.tasks.len();
        let completed_tasks = execution.task_executions.values()
            .filter(|te| te.status == ExecutionStatus::Completed)
            .count();
        execution.progress = completed_tasks as f64 / total_tasks as f64;
        
        // Check if all tasks are complete
        if completed_tasks == total_tasks {
            execution.status = ExecutionStatus::Completed;
            execution.ended_at = Some(chrono::Utc::now());
        } else {
            // Check for failed tasks
            let has_failed = execution.task_executions.values()
                .any(|te| te.status == ExecutionStatus::Failed);
            if has_failed {
                execution.status = ExecutionStatus::Failed;
                execution.ended_at = Some(chrono::Utc::now());
                execution.error = Some("One or more tasks failed".to_string());
            }
        }
        
        Ok(())
    }
    
    /// Save checkpoint
    fn save_checkpoint(&mut self, execution_id: &str) -> Result<(), OrchestratorError> {
        let execution = self.executions.get(execution_id).unwrap();
        
        let checkpoint = WorkflowCheckpoint {
            checkpoint_id: uuid::Uuid::new_v4().to_string(),
            execution_id: execution_id.to_string(),
            timestamp: chrono::Utc::now(),
            completed_tasks: execution.task_executions.values()
                .filter(|te| te.status == ExecutionStatus::Completed)
                .map(|te| te.task_id.clone())
                .collect(),
            task_outputs: execution.task_executions.values()
                .filter_map(|te| te.output.as_ref().map(|o| (te.task_id.clone(), o.clone())))
                .collect(),
            parameters: execution.parameters.clone(),
        };
        
        self.checkpoints.insert(execution_id.to_string(), checkpoint);
        Ok(())
    }
    
    /// Restore from checkpoint
    pub fn restore_from_checkpoint(&mut self, execution_id: &str) -> Result<(), OrchestratorError> {
        let checkpoint = self.checkpoints.get(execution_id)
            .ok_or_else(|| OrchestratorError::CheckpointNotFound(execution_id.to_string()))?;
        
        let execution = self.executions.get_mut(execution_id).unwrap();
        
        // Restore task executions
        for (task_id, output) in &checkpoint.task_outputs {
            if let Some(task_execution) = execution.task_executions.get_mut(task_id) {
                task_execution.status = ExecutionStatus::Completed;
                task_execution.output = Some(output.clone());
                task_execution.progress = 1.0;
            }
        }
        
        // Queue remaining tasks
        let workflow = self.workflows.get(&execution.workflow_id).unwrap();
        for task in &workflow.tasks {
            if checkpoint.completed_tasks.contains(&task.id) {
                continue;
            }
            
            // Check if dependencies are complete
            let deps = workflow.dependencies.get(&task.id).cloned().unwrap_or_default();
            let all_complete = deps.iter().all(|dep_id| checkpoint.completed_tasks.contains(dep_id));
            
            if all_complete {
                self.task_queue.push_back(TaskExecutionRequest {
                    execution_id: execution_id.to_string(),
                    task_id: task.id.clone(),
                    task: task.clone(),
                    priority: task.priority,
                    retry_count: 0,
                });
            }
        }
        
        Ok(())
    }
    
    /// Pause execution
    pub fn pause_execution(&mut self, execution_id: &str) -> Result<(), OrchestratorError> {
        let execution = self.executions.get_mut(execution_id)
            .ok_or_else(|| OrchestratorError::ExecutionNotFound(execution_id.to_string()))?;
        
        execution.status = ExecutionStatus::Paused;
        Ok(())
    }
    
    /// Resume execution
    pub fn resume_execution(&mut self, execution_id: &str) -> Result<(), OrchestratorError> {
        let execution = self.executions.get_mut(execution_id)
            .ok_or_else(|| OrchestratorError::ExecutionNotFound(execution_id.to_string()))?;
        
        execution.status = ExecutionStatus::Running;
        Ok(())
    }
    
    /// Cancel execution
    pub fn cancel_execution(&mut self, execution_id: &str) -> Result<(), OrchestratorError> {
        let execution = self.executions.get_mut(execution_id)
            .ok_or_else(|| OrchestratorError::ExecutionNotFound(execution_id.to_string()))?;
        
        execution.status = ExecutionStatus::Cancelled;
        execution.ended_at = Some(chrono::Utc::now());
        
        // Clear queued tasks for this execution
        self.task_queue.retain(|req| req.execution_id != execution_id);
        
        Ok(())
    }
    
    /// Get execution status
    pub fn get_execution(&self, execution_id: &str) -> Option<&WorkflowExecution> {
        self.executions.get(execution_id)
    }
    
    /// Get workflow
    pub fn get_workflow(&self, workflow_id: &str) -> Option<&Workflow> {
        self.workflows.get(workflow_id)
    }
    
    /// Get all executions
    pub fn get_all_executions(&self) -> Vec<&WorkflowExecution> {
        self.executions.values().collect()
    }
    
    /// Get all workflows
    pub fn get_all_workflows(&self) -> Vec<&Workflow> {
        self.workflows.values().collect()
    }
    
    /// Retry failed task
    pub fn retry_task(&mut self, execution_id: &str, task_id: &str) -> Result<(), OrchestratorError> {
        let execution = self.executions.get(execution_id)
            .ok_or_else(|| OrchestratorError::ExecutionNotFound(execution_id.to_string()))?;
        
        let workflow = self.workflows.get(&execution.workflow_id).unwrap();
        let task = workflow.tasks.iter()
            .find(|t| t.id == task_id)
            .ok_or_else(|| OrchestratorError::TaskNotFound(task_id.to_string()))?;
        
        // Re-queue the task
        self.task_queue.push_back(TaskExecutionRequest {
            execution_id: execution_id.to_string(),
            task_id: task_id.to_string(),
            task: task.clone(),
            priority: task.priority,
            retry_count: execution.task_executions.get(task_id).map(|te| te.attempts).unwrap_or(0) + 1,
        });
        
        Ok(())
    }
}

/// Orchestrator Error
#[derive(Debug, thiserror::Error)]
pub enum OrchestratorError {
    #[error("Workflow not found: {0}")]
    WorkflowNotFound(String),
    
    #[error("Task not found: {0}")]
    TaskNotFound(String),
    
    #[error("Execution not found: {0}")]
    ExecutionNotFound(String),
    
    #[error("Checkpoint not found: {0}")]
    CheckpointNotFound(String),
    
    #[error("Workflow execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Task execution failed: {0}")]
    TaskExecutionFailed(String),
    
    #[error("Dependency cycle detected")]
    DependencyCycle,
    
    #[error("Maximum retries exceeded")]
    MaxRetriesExceeded,
    
    #[error("Timeout exceeded")]
    Timeout,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orchestrator_creation() {
        let config = OrchestratorConfig::default();
        let orchestrator = AiOrchestrator::new(config);
        
        assert!(orchestrator.config.enable_parallel_execution);
        assert!(orchestrator.workflows.is_empty());
    }

    #[test]
    fn test_workflow_registration() {
        let mut orchestrator = AiOrchestrator::default_orchestrator();
        
        let workflow = Workflow {
            id: "wf_001".to_string(),
            name: "Test Workflow".to_string(),
            description: "A test workflow".to_string(),
            version: "1.0.0".to_string(),
            tasks: vec![
                WorkflowTask {
                    id: "task_1".to_string(),
                    name: "First Task".to_string(),
                    task_type: TaskType::Inference,
                    config: TaskConfig {
                        model: Some("model_v1".to_string()),
                        parameters: HashMap::new(),
                        resource_requirements: ResourceRequirements::default(),
                        environment: ExecutionEnvironment {
                            env_type: EnvironmentType::Native,
                            docker_image: None,
                            python_version: None,
                            packages: vec![],
                            env_vars: HashMap::new(),
                        },
                    },
                    inputs: TaskInputs::None,
                    outputs: TaskOutputs::Schema(serde_json::json!({})),
                    retry_config: None,
                    timeout_ms: None,
                    priority: 5,
                },
            ],
            dependencies: HashMap::new(),
            parameters: HashMap::new(),
            tags: vec![],
            created_at: chrono::Utc::now(),
        };
        
        orchestrator.register_workflow(workflow);
        assert!(orchestrator.workflows.contains_key("wf_001"));
    }

    #[test]
    fn test_workflow_execution() {
        let mut orchestrator = AiOrchestrator::default_orchestrator();
        
        orchestrator.register_workflow(Workflow {
            id: "wf_002".to_string(),
            name: "Simple Workflow".to_string(),
            description: "Simple test".to_string(),
            version: "1.0.0".to_string(),
            tasks: vec![
                WorkflowTask {
                    id: "task_a".to_string(),
                    name: "Task A".to_string(),
                    task_type: TaskType::Preprocessing,
                    config: TaskConfig {
                        model: None,
                        parameters: HashMap::new(),
                        resource_requirements: ResourceRequirements::default(),
                        environment: ExecutionEnvironment {
                            env_type: EnvironmentType::Native,
                            docker_image: None,
                            python_version: None,
                            packages: vec![],
                            env_vars: HashMap::new(),
                        },
                    },
                    inputs: TaskInputs::None,
                    outputs: TaskOutputs::Schema(serde_json::json!({})),
                    retry_config: None,
                    timeout_ms: None,
                    priority: 5,
                },
            ],
            dependencies: HashMap::new(),
            parameters: HashMap::new(),
            tags: vec![],
            created_at: chrono::Utc::now(),
        });
        
        let execution_id = orchestrator.start_workflow("wf_002", HashMap::new()).unwrap();
        let execution = orchestrator.execute_workflow(&execution_id).unwrap();
        
        assert_eq!(execution.status, ExecutionStatus::Completed);
        assert_eq!(execution.progress, 1.0);
    }

    #[test]
    fn test_task_dependencies() {
        let mut orchestrator = AiOrchestrator::default_orchestrator();
        
        let mut dependencies = HashMap::new();
        dependencies.insert("task_b".to_string(), vec!["task_a".to_string()]);
        dependencies.insert("task_c".to_string(), vec!["task_b".to_string()]);
        
        orchestrator.register_workflow(Workflow {
            id: "wf_003".to_string(),
            name: "Dependent Workflow".to_string(),
            description: "Tasks with dependencies".to_string(),
            version: "1.0.0".to_string(),
            tasks: vec![
                WorkflowTask {
                    id: "task_a".to_string(),
                    name: "Task A".to_string(),
                    task_type: TaskType::Preprocessing,
                    config: TaskConfig {
                        model: None,
                        parameters: HashMap::new(),
                        resource_requirements: ResourceRequirements::default(),
                        environment: ExecutionEnvironment {
                            env_type: EnvironmentType::Native,
                            docker_image: None,
                            python_version: None,
                            packages: vec![],
                            env_vars: HashMap::new(),
                        },
                    },
                    inputs: TaskInputs::None,
                    outputs: TaskOutputs::Schema(serde_json::json!({})),
                    retry_config: None,
                    timeout_ms: None,
                    priority: 5,
                },
                WorkflowTask {
                    id: "task_b".to_string(),
                    name: "Task B".to_string(),
                    task_type: TaskType::Inference,
                    config: TaskConfig {
                        model: Some("model_v2".to_string()),
                        parameters: HashMap::new(),
                        resource_requirements: ResourceRequirements::default(),
                        environment: ExecutionEnvironment {
                            env_type: EnvironmentType::Native,
                            docker_image: None,
                            python_version: None,
                            packages: vec![],
                            env_vars: HashMap::new(),
                        },
                    },
                    inputs: TaskInputs::Reference("task_a".to_string()),
                    outputs: TaskOutputs::Schema(serde_json::json!({})),
                    retry_config: None,
                    timeout_ms: None,
                    priority: 5,
                },
                WorkflowTask {
                    id: "task_c".to_string(),
                    name: "Task C".to_string(),
                    task_type: TaskType::Postprocessing,
                    config: TaskConfig {
                        model: None,
                        parameters: HashMap::new(),
                        resource_requirements: ResourceRequirements::default(),
                        environment: ExecutionEnvironment {
                            env_type: EnvironmentType::Native,
                            docker_image: None,
                            python_version: None,
                            packages: vec![],
                            env_vars: HashMap::new(),
                        },
                    },
                    inputs: TaskInputs::Reference("task_b".to_string()),
                    outputs: TaskOutputs::Schema(serde_json::json!({})),
                    retry_config: None,
                    timeout_ms: None,
                    priority: 5,
                },
            ],
            dependencies,
            parameters: HashMap::new(),
            tags: vec![],
            created_at: chrono::Utc::now(),
        });
        
        let execution_id = orchestrator.start_workflow("wf_003", HashMap::new()).unwrap();
        let execution = orchestrator.execute_workflow(&execution_id).unwrap();
        
        assert_eq!(execution.status, ExecutionStatus::Completed);
        assert_eq!(execution.progress, 1.0);
        assert_eq!(execution.task_executions.len(), 3);
    }

    #[test]
    fn test_checkpoint_restore() {
        let mut orchestrator = AiOrchestrator::default_orchestrator();
        orchestrator.config.enable_checkpointing = true;
        
        orchestrator.register_workflow(Workflow {
            id: "wf_004".to_string(),
            name: "Checkpoint Workflow".to_string(),
            description: "Testing checkpoints".to_string(),
            version: "1.0.0".to_string(),
            tasks: vec![
                WorkflowTask {
                    id: "task_1".to_string(),
                    name: "Task 1".to_string(),
                    task_type: TaskType::Preprocessing,
                    config: TaskConfig {
                        model: None,
                        parameters: HashMap::new(),
                        resource_requirements: ResourceRequirements::default(),
                        environment: ExecutionEnvironment {
                            env_type: EnvironmentType::Native,
                            docker_image: None,
                            python_version: None,
                            packages: vec![],
                            env_vars: HashMap::new(),
                        },
                    },
                    inputs: TaskInputs::None,
                    outputs: TaskOutputs::Schema(serde_json::json!({})),
                    retry_config: None,
                    timeout_ms: None,
                    priority: 5,
                },
            ],
            dependencies: HashMap::new(),
            parameters: HashMap::new(),
            tags: vec![],
            created_at: chrono::Utc::now(),
        });
        
        let execution_id = orchestrator.start_workflow("wf_004", HashMap::new()).unwrap();
        orchestrator.execute_workflow(&execution_id).unwrap();
        
        // Checkpoint should be saved
        assert!(orchestrator.checkpoints.contains_key(&execution_id));
        
        // Restore from checkpoint
        let result = orchestrator.restore_from_checkpoint(&execution_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cancel_execution() {
        let mut orchestrator = AiOrchestrator::default_orchestrator();
        
        orchestrator.register_workflow(Workflow {
            id: "wf_005".to_string(),
            name: "Cancellable Workflow".to_string(),
            description: "Testing cancellation".to_string(),
            version: "1.0.0".to_string(),
            tasks: vec![
                WorkflowTask {
                    id: "task_x".to_string(),
                    name: "Task X".to_string(),
                    task_type: TaskType::Preprocessing,
                    config: TaskConfig {
                        model: None,
                        parameters: HashMap::new(),
                        resource_requirements: ResourceRequirements::default(),
                        environment: ExecutionEnvironment {
                            env_type: EnvironmentType::Native,
                            docker_image: None,
                            python_version: None,
                            packages: vec![],
                            env_vars: HashMap::new(),
                        },
                    },
                    inputs: TaskInputs::None,
                    outputs: TaskOutputs::Schema(serde_json::json!({})),
                    retry_config: None,
                    timeout_ms: None,
                    priority: 5,
                },
            ],
            dependencies: HashMap::new(),
            parameters: HashMap::new(),
            tags: vec![],
            created_at: chrono::Utc::now(),
        });
        
        let execution_id = orchestrator.start_workflow("wf_005", HashMap::new()).unwrap();
        orchestrator.cancel_execution(&execution_id).unwrap();
        
        let execution = orchestrator.get_execution(&execution_id).unwrap();
        assert_eq!(execution.status, ExecutionStatus::Cancelled);
    }
}
</content>