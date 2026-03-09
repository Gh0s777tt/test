<content>
//! System Coordinator Module
//! Provides cross-component coordination, resource arbitration,
//! and unified system-wide AI optimization for VantisOS.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// System Coordinator Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinatorConfig {
    /// Enable cross-component optimization
    pub enable_cross_optimization: bool,
    
    /// Enable resource arbitration
    pub enable_resource_arbitration: bool,
    
    /// Enable predictive scheduling
    pub enable_predictive_scheduling: bool,
    
    /// Enable conflict resolution
    pub enable_conflict_resolution: bool,
    
    /// Optimization interval in milliseconds
    pub optimization_interval_ms: u64,
    
    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,
    
    /// Resource allocation timeout in milliseconds
    pub allocation_timeout_ms: u64,
    
    /// Enable system-wide caching
    pub enable_system_caching: bool,
    
    /// Maximum system memory usage percentage
    pub max_memory_percent: u32,
    
    /// Enable health monitoring
    pub enable_health_monitoring: bool,
}

impl Default for CoordinatorConfig {
    fn default() -> Self {
        Self {
            enable_cross_optimization: true,
            enable_resource_arbitration: true,
            enable_predictive_scheduling: true,
            enable_conflict_resolution: true,
            optimization_interval_ms: 100,
            max_concurrent_operations: 100,
            allocation_timeout_ms: 5000,
            enable_system_caching: true,
            max_memory_percent: 80,
            enable_health_monitoring: true,
        }
    }
}

/// System-wide metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Total CPU utilization (0.0-1.0)
    pub cpu_utilization: f64,
    
    /// Total memory utilization (0.0-1.0)
    pub memory_utilization: f64,
    
    /// Total I/O operations per second
    pub io_ops_per_second: u64,
    
    /// Network throughput (MB/s)
    pub network_throughput: f64,
    
    /// Active processes
    pub active_processes: u32,
    
    /// System load average (1 min)
    pub load_average_1m: f64,
    
    /// System load average (5 min)
    pub load_average_5m: f64,
    
    /// System load average (15 min)
    pub load_average_15m: f64,
    
    /// Uptime in seconds
    pub uptime_seconds: u64,
    
    /// Number of active AI tasks
    pub active_ai_tasks: u32,
    
    /// Component health status
    pub component_health: HashMap<String, HealthStatus>,
    
    /// Last update timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            io_ops_per_second: 0,
            network_throughput: 0.0,
            active_processes: 0,
            load_average_1m: 0.0,
            load_average_5m: 0.0,
            load_average_15m: 0.0,
            uptime_seconds: 0,
            active_ai_tasks: 0,
            component_health: HashMap::new(),
            last_updated: chrono::Utc::now(),
        }
    }
}

/// Health status of a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Component name
    pub component: String,
    
    /// Health state
    pub state: HealthState,
    
    /// Last check timestamp
    pub last_check: chrono::DateTime<chrono::Utc>,
    
    /// Error count
    pub error_count: u64,
    
    /// Warning count
    pub warning_count: u64,
    
    /// Response time in ms
    pub response_time_ms: f64,
    
    /// Additional details
    pub details: HashMap<String, String>,
}

/// Health states
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthState {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Component identifier
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum ComponentId {
    FileSystem,
    Network,
    Database,
    Graphics,
    Memory,
    Cpu,
    AiEngine,
    Cache,
    Scheduler,
    Security,
}

impl ComponentId {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComponentId::FileSystem => "filesystem",
            ComponentId::Network => "network",
            ComponentId::Database => "database",
            ComponentId::Graphics => "graphics",
            ComponentId::Memory => "memory",
            ComponentId::Cpu => "cpu",
            ComponentId::AiEngine => "ai_engine",
            ComponentId::Cache => "cache",
            ComponentId::Scheduler => "scheduler",
            ComponentId::Security => "security",
        }
    }
}

/// Resource request from a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequest {
    /// Request ID
    pub request_id: String,
    
    /// Requesting component
    pub component: ComponentId,
    
    /// Resource type
    pub resource_type: ResourceType,
    
    /// Requested amount
    pub amount: u64,
    
    /// Priority (1-10, 10 highest)
    pub priority: u8,
    
    /// Whether request is time-critical
    pub time_critical: bool,
    
    /// Request timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Timeout in milliseconds
    pub timeout_ms: u64,
    
    /// Request state
    pub state: RequestState,
}

/// Resource types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    CpuTime,
    Memory,
    DiskIO,
    NetworkBandwidth,
    GpuTime,
    GpuMemory,
    CacheSpace,
}

/// Request states
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RequestState {
    Pending,
    Approved,
    Denied,
    Completed,
    Timeout,
}

/// Resource allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    /// Allocation ID
    pub allocation_id: String,
    
    /// Component receiving allocation
    pub component: ComponentId,
    
    /// Resource type
    pub resource_type: ResourceType,
    
    /// Allocated amount
    pub allocated_amount: u64,
    
    /// Allocation timestamp
    pub allocated_at: chrono::DateTime<chrono::Utc>,
    
    /// Expiration timestamp
    pub expires_at: chrono::DateTime<chrono::Utc>,
    
    /// Current usage
    pub current_usage: u64,
}

/// Conflict between components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentConflict {
    /// Conflict ID
    pub conflict_id: String,
    
    /// Conflicting components
    pub components: Vec<ComponentId>,
    
    /// Conflict type
    pub conflict_type: ConflictType,
    
    /// Severity (1-10)
    pub severity: u8,
    
    /// Detection timestamp
    pub detected_at: chrono::DateTime<chrono::Utc>,
    
    /// Resolution status
    pub resolved: bool,
    
    /// Resolution strategy
    pub resolution: Option<ConflictResolution>,
}

/// Types of conflicts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    ResourceContention,
    PriorityInversion,
    Deadlock,
    Starvation,
    DependencyCycle,
    CapacityExceeded,
}

/// Conflict resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolution {
    /// Resolution strategy
    pub strategy: ResolutionStrategy,
    
    /// Resolution timestamp
    pub resolved_at: chrono::DateTime<chrono::Utc>,
    
    /// Actions taken
    pub actions: Vec<String>,
    
    /// Success status
    pub success: bool,
}

/// Resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    PriorityBased,
    RoundRobin,
    FirstComeFirstServed,
    ResourceReallocation,
    LoadBalancing,
    Throttling,
    Preemption,
}

/// Optimization action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationAction {
    /// Action ID
    pub action_id: String,
    
    /// Target component
    pub target_component: ComponentId,
    
    /// Action type
    pub action_type: OptimizationActionType,
    
    /// Parameters
    pub parameters: HashMap<String, f64>,
    
    /// Expected benefit
    pub expected_benefit: f64,
    
    /// Risk level (0.0-1.0)
    pub risk_level: f64,
    
    /// Execution timestamp
    pub executed_at: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Success status
    pub success: Option<bool>,
}

/// Types of optimization actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationActionType {
    ScaleUp,
    ScaleDown,
    Rebalance,
    Cache,
    Prefetch,
    Throttle,
    Prioritize,
    Defer,
}

/// Scheduled task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    /// Task ID
    pub task_id: String,
    
    /// Task name
    pub name: String,
    
    /// Target component
    pub component: ComponentId,
    
    /// Priority
    pub priority: u8,
    
    /// Estimated duration in ms
    pub estimated_duration_ms: u64,
    
    /// Dependencies
    pub dependencies: Vec<String>,
    
    /// Scheduled start time
    pub scheduled_start: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Actual start time
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Completion time
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Task state
    pub state: TaskState,
}

/// Task states
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskState {
    Queued,
    Scheduled,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// System event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    /// Event ID
    pub event_id: String,
    
    /// Event type
    pub event_type: SystemEventType,
    
    /// Source component
    pub source: ComponentId,
    
    /// Event data
    pub data: HashMap<String, String>,
    
    /// Event timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Severity
    pub severity: EventSeverity,
}

/// System event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEventType {
    ResourceAcquired,
    ResourceReleased,
    ConflictDetected,
    ConflictResolved,
    OptimizationPerformed,
    ThresholdExceeded,
    HealthChanged,
    TaskStarted,
    TaskCompleted,
    TaskFailed,
}

/// Event severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventSeverity {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

/// System Coordinator
pub struct SystemCoordinator {
    config: CoordinatorConfig,
    metrics: SystemMetrics,
    resource_requests: VecDeque<ResourceRequest>,
    allocations: HashMap<String, ResourceAllocation>,
    conflicts: Vec<ComponentConflict>,
    optimization_actions: Vec<OptimizationAction>,
    scheduled_tasks: Vec<ScheduledTask>,
    event_log: VecDeque<SystemEvent>,
    component_registry: HashMap<ComponentId, ComponentInfo>,
}

/// Component registration info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInfo {
    /// Component ID
    pub id: ComponentId,
    
    /// Component name
    pub name: String,
    
    /// Component version
    pub version: String,
    
    /// Priority level
    pub priority: u8,
    
    /// Resource quotas
    pub quotas: HashMap<String, u64>,
    
    /// Current usage
    pub current_usage: HashMap<String, u64>,
    
    /// Registration timestamp
    pub registered_at: chrono::DateTime<chrono::Utc>,
    
    /// Last heartbeat
    pub last_heartbeat: chrono::DateTime<chrono::Utc>,
}

impl SystemCoordinator {
    /// Create a new system coordinator
    pub fn new(config: CoordinatorConfig) -> Self {
        Self {
            config,
            metrics: SystemMetrics::default(),
            resource_requests: VecDeque::new(),
            allocations: HashMap::new(),
            conflicts: Vec::new(),
            optimization_actions: Vec::new(),
            scheduled_tasks: Vec::new(),
            event_log: VecDeque::new(),
            component_registry: HashMap::new(),
        }
    }
    
    /// Create with default configuration
    pub fn default_coordinator() -> Self {
        Self::new(CoordinatorConfig::default())
    }
    
    /// Register a component
    pub fn register_component(&mut self, info: ComponentInfo) {
        let id = info.id.clone();
        self.component_registry.insert(id.clone(), info);
        
        // Initialize health status
        self.metrics.component_health.insert(
            id.as_str().to_string(),
            HealthStatus {
                component: id.as_str().to_string(),
                state: HealthState::Healthy,
                last_check: chrono::Utc::now(),
                error_count: 0,
                warning_count: 0,
                response_time_ms: 0.0,
                details: HashMap::new(),
            },
        );
    }
    
    /// Unregister a component
    pub fn unregister_component(&mut self, id: &ComponentId) {
        self.component_registry.remove(id);
        self.metrics.component_health.remove(id.as_str());
    }
    
    /// Request resources
    pub fn request_resources(&mut self, request: ResourceRequest) -> Result<String, CoordinatorError> {
        if !self.config.enable_resource_arbitration {
            return Err(CoordinatorError::ArbitrationDisabled);
        }
        
        let request_id = request.request_id.clone();
        
        // Check for potential conflicts
        if self.config.enable_conflict_resolution {
            if let Some(conflict) = self.detect_conflict(&request) {
                self.resolve_conflict(&conflict)?;
            }
        }
        
        // Process request based on priority and availability
        let approved = self.evaluate_request(&request);
        
        if approved {
            let allocation = self.create_allocation(&request);
            self.allocations.insert(allocation.allocation_id.clone(), allocation);
            self.log_event(SystemEvent {
                event_id: uuid::Uuid::new_v4().to_string(),
                event_type: SystemEventType::ResourceAcquired,
                source: request.component.clone(),
                data: HashMap::new(),
                timestamp: chrono::Utc::now(),
                severity: EventSeverity::Info,
            });
            Ok(request_id)
        } else {
            self.resource_requests.push_back(request);
            Err(CoordinatorError::ResourceUnavailable)
        }
    }
    
    /// Release resources
    pub fn release_resources(&mut self, allocation_id: &str) -> Result<(), CoordinatorError> {
        if let Some(allocation) = self.allocations.remove(allocation_id) {
            self.log_event(SystemEvent {
                event_id: uuid::Uuid::new_v4().to_string(),
                event_type: SystemEventType::ResourceReleased,
                source: allocation.component,
                data: HashMap::new(),
                timestamp: chrono::Utc::now(),
                severity: EventSeverity::Info,
            });
            Ok(())
        } else {
            Err(CoordinatorError::AllocationNotFound(allocation_id.to_string()))
        }
    }
    
    /// Detect potential conflicts
    fn detect_conflict(&self, request: &ResourceRequest) -> Option<ComponentConflict> {
        // Check for resource contention with existing allocations
        for allocation in self.allocations.values() {
            if self.requests_conflict(request, allocation) {
                return Some(ComponentConflict {
                    conflict_id: uuid::Uuid::new_v4().to_string(),
                    components: vec![request.component.clone(), allocation.component.clone()],
                    conflict_type: ConflictType::ResourceContention,
                    severity: 5,
                    detected_at: chrono::Utc::now(),
                    resolved: false,
                    resolution: None,
                });
            }
        }
        None
    }
    
    /// Check if request conflicts with allocation
    fn requests_conflict(&self, request: &ResourceRequest, allocation: &ResourceAllocation) -> bool {
        // Simplified conflict detection
        match (&request.resource_type, &allocation.resource_type) {
            (ResourceType::Memory, ResourceType::Memory) => {
                let total_allocated: u64 = self.allocations.values()
                    .filter(|a| matches!(a.resource_type, ResourceType::Memory))
                    .map(|a| a.allocated_amount)
                    .sum();
                total_allocated + request.amount > self.get_memory_limit()
            }
            (ResourceType::CpuTime, ResourceType::CpuTime) => {
                request.priority < 5 && allocation.allocated_amount > 50
            }
            _ => false,
        }
    }
    
    /// Get memory limit
    fn get_memory_limit(&self) -> u64 {
        // Simulated: 8GB * max_memory_percent
        8192 * self.config.max_memory_percent as u64 / 100
    }
    
    /// Resolve conflict
    fn resolve_conflict(&mut self, conflict: &ComponentConflict) -> Result<(), CoordinatorError> {
        let resolution = match conflict.conflict_type {
            ConflictType::ResourceContention => {
                self.resolve_contention(conflict)
            }
            ConflictType::PriorityInversion => {
                self.resolve_priority_inversion(conflict)
            }
            ConflictType::Deadlock => {
                self.resolve_deadlock(conflict)
            }
            _ => ConflictResolution {
                strategy: ResolutionStrategy::PriorityBased,
                resolved_at: chrono::Utc::now(),
                actions: vec!["No action needed".to_string()],
                success: true,
            },
        };
        
        let mut resolved_conflict = conflict.clone();
        resolved_conflict.resolved = resolution.success;
        resolved_conflict.resolution = Some(resolution);
        self.conflicts.push(resolved_conflict);
        
        Ok(())
    }
    
    /// Resolve resource contention
    fn resolve_contention(&self, conflict: &ComponentConflict) -> ConflictResolution {
        ConflictResolution {
            strategy: ResolutionStrategy::PriorityBased,
            resolved_at: chrono::Utc::now(),
            actions: vec!["Higher priority request granted".to_string()],
            success: true,
        }
    }
    
    /// Resolve priority inversion
    fn resolve_priority_inversion(&self, conflict: &ComponentConflict) -> ConflictResolution {
        ConflictResolution {
            strategy: ResolutionStrategy::Preemption,
            resolved_at: chrono::Utc::now(),
            actions: vec!["Lower priority task preempted".to_string()],
            success: true,
        }
    }
    
    /// Resolve deadlock
    fn resolve_deadlock(&self, conflict: &ComponentConflict) -> ConflictResolution {
        ConflictResolution {
            strategy: ResolutionStrategy::ResourceReallocation,
            resolved_at: chrono::Utc::now(),
            actions: vec!["Resources reallocated to break deadlock".to_string()],
            success: true,
        }
    }
    
    /// Evaluate resource request
    fn evaluate_request(&self, request: &ResourceRequest) -> bool {
        // Check component priority
        let component_priority = self.component_registry
            .get(&request.component)
            .map(|c| c.priority)
            .unwrap_or(5);
        
        // Check resource availability
        let available = match request.resource_type {
            ResourceType::Memory => self.get_available_memory(),
            ResourceType::CpuTime => 100, // Percentage available
            ResourceType::CacheSpace => self.get_available_cache(),
            _ => 100,
        };
        
        request.amount <= available && (request.priority >= 5 || component_priority >= 5)
    }
    
    /// Get available memory
    fn get_available_memory(&self) -> u64 {
        let used: u64 = self.allocations.values()
            .filter(|a| matches!(a.resource_type, ResourceType::Memory))
            .map(|a| a.allocated_amount)
            .sum();
        self.get_memory_limit().saturating_sub(used)
    }
    
    /// Get available cache space
    fn get_available_cache(&self) -> u64 {
        let used: u64 = self.allocations.values()
            .filter(|a| matches!(a.resource_type, ResourceType::CacheSpace))
            .map(|a| a.allocated_amount)
            .sum();
        1024 - used // Simulated 1GB cache
    }
    
    /// Create resource allocation
    fn create_allocation(&self, request: &ResourceRequest) -> ResourceAllocation {
        ResourceAllocation {
            allocation_id: uuid::Uuid::new_v4().to_string(),
            component: request.component.clone(),
            resource_type: request.resource_type.clone(),
            allocated_amount: request.amount,
            allocated_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(300),
            current_usage: 0,
        }
    }
    
    /// Perform system optimization
    pub fn optimize_system(&mut self) -> Vec<OptimizationAction> {
        if !self.config.enable_cross_optimization {
            return Vec::new();
        }
        
        let mut actions = Vec::new();
        
        // Analyze system metrics
        if self.metrics.cpu_utilization > 0.8 {
            actions.push(OptimizationAction {
                action_id: uuid::Uuid::new_v4().to_string(),
                target_component: ComponentId::Cpu,
                action_type: OptimizationActionType::Throttle,
                parameters: [("factor".to_string(), 0.8)].into_iter().collect(),
                expected_benefit: 0.2,
                risk_level: 0.1,
                executed_at: None,
                success: None,
            });
        }
        
        if self.metrics.memory_utilization > 0.85 {
            actions.push(OptimizationAction {
                action_id: uuid::Uuid::new_v4().to_string(),
                target_component: ComponentId::Memory,
                action_type: OptimizationActionType::Cache,
                parameters: [("factor".to_string(), 0.7)].into_iter().collect(),
                expected_benefit: 0.15,
                risk_level: 0.2,
                executed_at: None,
                success: None,
            });
        }
        
        // Store and return actions
        for action in &actions {
            self.optimization_actions.push(action.clone());
        }
        
        actions
    }
    
    /// Schedule a task
    pub fn schedule_task(&mut self, task: ScheduledTask) -> Result<(), CoordinatorError> {
        if !self.config.enable_predictive_scheduling {
            return Err(CoordinatorError::SchedulingDisabled);
        }
        
        // Check dependencies
        for dep_id in &task.dependencies {
            if !self.scheduled_tasks.iter().any(|t| &t.task_id == dep_id && t.state == TaskState::Completed) {
                return Err(CoordinatorError::DependencyNotMet(dep_id.clone()));
            }
        }
        
        // Find optimal slot
        let scheduled_start = self.find_optimal_slot(&task);
        let mut task = task;
        task.scheduled_start = Some(scheduled_start);
        task.state = TaskState::Scheduled;
        
        self.scheduled_tasks.push(task);
        Ok(())
    }
    
    /// Find optimal time slot for task
    fn find_optimal_slot(&self, task: &ScheduledTask) -> chrono::DateTime<chrono::Utc> {
        // Simplified: schedule for immediate execution
        // In production, would analyze current load and predict optimal time
        chrono::Utc::now() + chrono::Duration::milliseconds(100)
    }
    
    /// Execute scheduled tasks
    pub fn execute_pending_tasks(&mut self) -> Vec<String> {
        let mut completed = Vec::new();
        let now = chrono::Utc::now();
        
        for task in &mut self.scheduled_tasks {
            if task.state == TaskState::Scheduled {
                if let Some(scheduled_start) = task.scheduled_start {
                    if now >= scheduled_start {
                        task.state = TaskState::Running;
                        task.started_at = Some(now);
                        
                        // Simulate task execution
                        task.state = TaskState::Completed;
                        task.completed_at = Some(now);
                        completed.push(task.task_id.clone());
                    }
                }
            }
        }
        
        completed
    }
    
    /// Update system metrics
    pub fn update_metrics(&mut self, metrics: SystemMetrics) {
        self.metrics = metrics;
        
        // Check for threshold violations
        if self.metrics.cpu_utilization > 0.9 {
            self.log_event(SystemEvent {
                event_id: uuid::Uuid::new_v4().to_string(),
                event_type: SystemEventType::ThresholdExceeded,
                source: ComponentId::Cpu,
                data: [("threshold".to_string(), "0.9".to_string())].into_iter().collect(),
                timestamp: chrono::Utc::now(),
                severity: EventSeverity::Warning,
            });
        }
    }
    
    /// Get component health
    pub fn get_component_health(&self, component: &ComponentId) -> Option<&HealthStatus> {
        self.metrics.component_health.get(component.as_str())
    }
    
    /// Update component health
    pub fn update_component_health(&mut self, component: ComponentId, status: HealthStatus) {
        let old_state = self.metrics.component_health.get(component.as_str())
            .map(|s| s.state.clone());
        
        self.metrics.component_health.insert(component.as_str().to_string(), status.clone());
        
        // Log health change
        if old_state.as_ref() != Some(&status.state) {
            self.log_event(SystemEvent {
                event_id: uuid::Uuid::new_v4().to_string(),
                event_type: SystemEventType::HealthChanged,
                source: component,
                data: HashMap::new(),
                timestamp: chrono::Utc::now(),
                severity: match status.state {
                    HealthState::Healthy => EventSeverity::Info,
                    HealthState::Degraded => EventSeverity::Warning,
                    HealthState::Unhealthy => EventSeverity::Error,
                    HealthState::Unknown => EventSeverity::Warning,
                },
            });
        }
    }
    
    /// Log system event
    fn log_event(&mut self, event: SystemEvent) {
        self.event_log.push_back(event);
        
        // Keep limited history
        while self.event_log.len() > 10000 {
            self.event_log.pop_front();
        }
    }
    
    /// Get recent events
    pub fn get_recent_events(&self, count: usize) -> Vec<&SystemEvent> {
        self.event_log.iter().rev().take(count).collect()
    }
    
    /// Get system metrics
    pub fn get_metrics(&self) -> SystemMetrics {
        self.metrics.clone()
    }
    
    /// Get active allocations
    pub fn get_active_allocations(&self) -> Vec<&ResourceAllocation> {
        self.allocations.values().collect()
    }
    
    /// Get pending requests
    pub fn get_pending_requests(&self) -> Vec<&ResourceRequest> {
        self.resource_requests.iter().collect()
    }
    
    /// Check system health
    pub fn check_system_health(&self) -> SystemHealthReport {
        let mut unhealthy_components = Vec::new();
        let mut degraded_components = Vec::new();
        
        for (name, status) in &self.metrics.component_health {
            match status.state {
                HealthState::Unhealthy => unhealthy_components.push(name.clone()),
                HealthState::Degraded => degraded_components.push(name.clone()),
                _ => {}
            }
        }
        
        SystemHealthReport {
            overall_health: if unhealthy_components.is_empty() {
                if degraded_components.is_empty() {
                    HealthState::Healthy
                } else {
                    HealthState::Degraded
                }
            } else {
                HealthState::Unhealthy
            },
            unhealthy_components,
            degraded_components,
            total_allocations: self.allocations.len(),
            pending_requests: self.resource_requests.len(),
            active_tasks: self.scheduled_tasks.iter().filter(|t| t.state == TaskState::Running).count(),
        }
    }
}

/// System health report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthReport {
    pub overall_health: HealthState,
    pub unhealthy_components: Vec<String>,
    pub degraded_components: Vec<String>,
    pub total_allocations: usize,
    pub pending_requests: usize,
    pub active_tasks: usize,
}

/// Coordinator error types
#[derive(Debug, thiserror::Error)]
pub enum CoordinatorError {
    #[error("Resource arbitration is disabled")]
    ArbitrationDisabled,
    
    #[error("Resource unavailable")]
    ResourceUnavailable,
    
    #[error("Allocation not found: {0}")]
    AllocationNotFound(String),
    
    #[error("Scheduling is disabled")]
    SchedulingDisabled,
    
    #[error("Dependency not met: {0}")]
    DependencyNotMet(String),
    
    #[error("Conflict resolution failed: {0}")]
    ConflictResolutionFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinator_creation() {
        let config = CoordinatorConfig::default();
        let coordinator = SystemCoordinator::new(config);
        
        assert_eq!(coordinator.allocations.len(), 0);
        assert_eq!(coordinator.resource_requests.len(), 0);
    }

    #[test]
    fn test_component_registration() {
        let mut coordinator = SystemCoordinator::default_coordinator();
        
        let info = ComponentInfo {
            id: ComponentId::FileSystem,
            name: "FileSystem".to_string(),
            version: "1.0.0".to_string(),
            priority: 5,
            quotas: HashMap::new(),
            current_usage: HashMap::new(),
            registered_at: chrono::Utc::now(),
            last_heartbeat: chrono::Utc::now(),
        };
        
        coordinator.register_component(info);
        
        assert!(coordinator.component_registry.contains_key(&ComponentId::FileSystem));
        assert!(coordinator.metrics.component_health.contains_key("filesystem"));
    }

    #[test]
    fn test_resource_request() {
        let mut coordinator = SystemCoordinator::default_coordinator();
        
        // Register component first
        coordinator.register_component(ComponentInfo {
            id: ComponentId::FileSystem,
            name: "FileSystem".to_string(),
            version: "1.0.0".to_string(),
            priority: 7,
            quotas: HashMap::new(),
            current_usage: HashMap::new(),
            registered_at: chrono::Utc::now(),
            last_heartbeat: chrono::Utc::now(),
        });
        
        let request = ResourceRequest {
            request_id: "req_001".to_string(),
            component: ComponentId::FileSystem,
            resource_type: ResourceType::Memory,
            amount: 1024,
            priority: 7,
            time_critical: false,
            timestamp: chrono::Utc::now(),
            timeout_ms: 5000,
            state: RequestState::Pending,
        };
        
        let result = coordinator.request_resources(request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_resource_release() {
        let mut coordinator = SystemCoordinator::default_coordinator();
        
        coordinator.register_component(ComponentInfo {
            id: ComponentId::Database,
            name: "Database".to_string(),
            version: "1.0.0".to_string(),
            priority: 6,
            quotas: HashMap::new(),
            current_usage: HashMap::new(),
            registered_at: chrono::Utc::now(),
            last_heartbeat: chrono::Utc::now(),
        });
        
        let request = ResourceRequest {
            request_id: "req_002".to_string(),
            component: ComponentId::Database,
            resource_type: ResourceType::CacheSpace,
            amount: 256,
            priority: 6,
            time_critical: false,
            timestamp: chrono::Utc::now(),
            timeout_ms: 5000,
            state: RequestState::Pending,
        };
        
        let result = coordinator.request_resources(request);
        assert!(result.is_ok());
        
        let allocations = coordinator.get_active_allocations();
        assert_eq!(allocations.len(), 1);
        
        let allocation_id = allocations[0].allocation_id.clone();
        let release_result = coordinator.release_resources(&allocation_id);
        assert!(release_result.is_ok());
        
        assert_eq!(coordinator.get_active_allocations().len(), 0);
    }

    #[test]
    fn test_system_optimization() {
        let mut coordinator = SystemCoordinator::default_coordinator();
        
        // Set high CPU utilization
        let mut metrics = SystemMetrics::default();
        metrics.cpu_utilization = 0.9;
        coordinator.update_metrics(metrics);
        
        let actions = coordinator.optimize_system();
        assert!(!actions.is_empty());
    }

    #[test]
    fn test_task_scheduling() {
        let mut coordinator = SystemCoordinator::default_coordinator();
        
        let task = ScheduledTask {
            task_id: "task_001".to_string(),
            name: "Test Task".to_string(),
            component: ComponentId::AiEngine,
            priority: 5,
            estimated_duration_ms: 100,
            dependencies: Vec::new(),
            scheduled_start: None,
            started_at: None,
            completed_at: None,
            state: TaskState::Queued,
        };
        
        let result = coordinator.schedule_task(task);
        assert!(result.is_ok());
        
        let completed = coordinator.execute_pending_tasks();
        assert!(completed.contains(&"task_001".to_string()));
    }

    #[test]
    fn test_health_monitoring() {
        let mut coordinator = SystemCoordinator::default_coordinator();
        
        coordinator.update_component_health(ComponentId::Network, HealthStatus {
            component: "network".to_string(),
            state: HealthState::Degraded,
            last_check: chrono::Utc::now(),
            error_count: 2,
            warning_count: 5,
            response_time_ms: 150.0,
            details: HashMap::new(),
        });
        
        let health = coordinator.get_component_health(&ComponentId::Network);
        assert!(health.is_some());
        assert_eq!(health.unwrap().state, HealthState::Degraded);
        
        let report = coordinator.check_system_health();
        assert_eq!(report.overall_health, HealthState::Degraded);
        assert!(report.degraded_components.contains(&"network".to_string()));
    }

    #[test]
    fn test_event_logging() {
        let mut coordinator = SystemCoordinator::default_coordinator();
        
        coordinator.log_event(SystemEvent {
            event_id: "evt_001".to_string(),
            event_type: SystemEventType::OptimizationPerformed,
            source: ComponentId::AiEngine,
            data: HashMap::new(),
            timestamp: chrono::Utc::now(),
            severity: EventSeverity::Info,
        });
        
        let events = coordinator.get_recent_events(10);
        assert_eq!(events.len(), 1);
    }
}
</content>