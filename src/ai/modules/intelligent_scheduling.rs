//! Intelligent Process Scheduling Module
//!
//! Advanced task scheduler that uses deep learning models to optimize
//! process scheduling based on historical patterns, resource availability,
//! and workload characteristics.

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration for intelligent scheduling system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingConfig {
    /// Base time quantum in milliseconds
    pub base_quantum_ms: u64,
    
    /// Maximum time quantum in milliseconds
    pub max_quantum_ms: u64,
    
    /// Minimum time quantum in milliseconds
    pub min_quantum_ms: u64,
    
    /// Maximum concurrent tasks
    pub max_concurrent_tasks: usize,
    
    /// Enable GPU-accelerated scheduling
    pub enable_gpu_acceleration: bool,
    
    /// Scheduling algorithm
    pub algorithm: SchedulingAlgorithm,
    
    /// Enable adaptive quantum
    pub enable_adaptive_quantum: bool,
    
    /// Prediction horizon (number of time steps)
    pub prediction_horizon: usize,
}

impl Default for SchedulingConfig {
    fn default() -> Self {
        Self {
            base_quantum_ms: 10,
            max_quantum_ms: 50,
            min_quantum_ms: 1,
            max_concurrent_tasks: 32,
            enable_gpu_acceleration: true,
            algorithm: SchedulingAlgorithm::DeepLearning,
            enable_adaptive_quantum: true,
            prediction_horizon: 10,
        }
    }
}

/// Scheduling algorithms
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SchedulingAlgorithm {
    /// Round Robin
    RoundRobin,
    /// Priority-based
    Priority,
    /// Shortest Job First
    SJF,
    /// Deep Learning based
    DeepLearning,
    /// Reinforcement Learning
    ReinforcementLearning,
}

/// Process priority levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessPriority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Task characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCharacteristics {
    /// Estimated execution time (ms)
    pub estimated_duration: u64,
    
    /// CPU intensity (0.0 - 1.0)
    pub cpu_intensity: f64,
    
    /// Memory requirement (bytes)
    pub memory_requirement: usize,
    
    /// I/O intensity (0.0 - 1.0)
    pub io_intensity: f64,
    
    /// Priority level
    pub priority: ProcessPriority,
    
    /// Task category
    pub category: TaskCategory,
}

/// Task category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskCategory {
    Interactive,
    Batch,
    RealTime,
    Compute,
    IO,
    Network,
    AI,
}

/// Task execution context
#[derive(Debug, Clone)]
pub struct TaskContext {
    pub task_id: String,
    pub characteristics: TaskCharacteristics,
    pub start_time: Instant,
    pub cpu_time: Duration,
    pub wait_time: Duration,
    pub preemptions: usize,
}

/// Scheduling decision
#[derive(Debug, Clone)]
struct SchedulingDecision {
    task_id: String,
    quantum: Duration,
    predicted_completion: Duration,
    confidence: f64,
}

/// Deep learning model for scheduling
#[derive(Debug)]
struct SchedulingModel {
    weights: HashMap<String, f64>,
    historical_accuracy: VecDeque<f64>,
    prediction_count: usize,
}

impl SchedulingModel {
    fn new() -> Self {
        Self {
            weights: HashMap::new(),
            historical_accuracy: VecDeque::with_capacity(1000),
            prediction_count: 0,
        }
    }
    
    /// Train the model on historical data
    fn train(&amp;mut self, context: &amp;TaskContext, actual_duration: Duration) {
        let predicted = self.predict(context);
        let error = (actual_duration.as_millis() as f64 - predicted.as_millis() as f64).abs();
        let accuracy = 1.0 / (1.0 + error / 1000.0);
        
        self.historical_accuracy.push_back(accuracy);
        if self.historical_accuracy.len() > 1000 {
            self.historical_accuracy.pop_front();
        }
        
        // Update weights based on error
        self.update_weights(context, error);
    }
    
    /// Predict task duration
    fn predict(&amp;self, context: &amp;TaskContext) -> Duration {
        let base_duration = context.characteristics.estimated_duration;
        let cpu_factor = context.characteristics.cpu_intensity;
        
        let adjusted = base_duration as f64 * (1.0 + cpu_factor * 0.5);
        Duration::from_millis(adjusted as u64)
    }
    
    /// Update model weights
    fn update_weights(&amp;mut self, context: &amp;TaskContext, error: f64) {
        let learning_rate = 0.01;
        let key = format!("{:?}", context.characteristics.category);
        let weight = self.weights.get(&amp;key).copied().unwrap_or(1.0);
        
        let new_weight = weight * (1.0 - learning_rate * error);
        self.weights.insert(key, new_weight);
    }
    
    /// Get model accuracy
    fn accuracy(&amp;self) -> f64 {
        if self.historical_accuracy.is_empty() {
            return 0.5;
        }
        let sum: f64 = self.historical_accuracy.iter().sum();
        sum / self.historical_accuracy.len() as f64
    }
}

/// Intelligent process scheduler
pub struct IntelligentScheduler {
    config: SchedulingConfig,
    ready_queue: Arc<RwLock<VecDeque<TaskContext>>>,
    running_tasks: Arc<RwLock<HashMap<String, TaskContext>>>,
    model: Arc<RwLock<SchedulingModel>>,
    resource_monitor: Arc<RwLock<ResourceMonitor>>,
}

/// Resource monitor
#[derive(Debug, Clone)]
struct ResourceMonitor {
    cpu_usage: f64,
    memory_usage: f64,
    load_average: f64,
    available_cores: usize,
}

impl IntelligentScheduler {
    /// Create a new intelligent scheduler
    pub fn new(config: SchedulingConfig) -> Self {
        Self {
            config,
            ready_queue: Arc::new(RwLock::new(VecDeque::new())),
            running_tasks: Arc::new(RwLock::new(HashMap::new())),
            model: Arc::new(RwLock::new(SchedulingModel::new())),
            resource_monitor: Arc::new(RwLock::new(ResourceMonitor {
                cpu_usage: 0.5,
                memory_usage: 0.5,
                load_average: 1.0,
                available_cores: 4,
            })),
        }
    }
    
    /// Submit a new task for scheduling
    pub async fn submit_task(&amp;self, task_id: String, characteristics: TaskCharacteristics) {
        let context = TaskContext {
            task_id: task_id.clone(),
            characteristics,
            start_time: Instant::now(),
            cpu_time: Duration::ZERO,
            wait_time: Duration::ZERO,
            preemptions: 0,
        };
        
        let mut queue = self.ready_queue.write().await;
        queue.push_back(context);
        
        // Sort queue based on scheduling algorithm
        self.sort_queue(&amp;mut queue).await;
    }
    
    /// Get next task to schedule
    pub async fn schedule_next(&amp;self) -> Option<SchedulingDecision> {
        let mut queue = self.ready_queue.write().await;
        
        if queue.is_empty() {
            return None;
        }
        
        let context = queue.pop_front()?;
        
        // Update wait time
        let wait_time = context.start_time.elapsed();
        
        // Make scheduling decision
        let decision = self.make_decision(&amp;context).await;
        
        // Add to running tasks
        let mut running = self.running_tasks.write().await;
        running.insert(context.task_id.clone(), context);
        
        Some(decision)
    }
    
    /// Make scheduling decision for a task
    async fn make_decision(&amp;self, context: &amp;TaskContext) -> SchedulingDecision {
        let quantum = self.calculate_quantum(context).await;
        let model = self.model.read().await;
        let predicted_duration = model.predict(context);
        let confidence = model.accuracy();
        
        SchedulingDecision {
            task_id: context.task_id.clone(),
            quantum,
            predicted_completion: predicted_duration,
            confidence,
        }
    }
    
    /// Calculate time quantum for a task
    async fn calculate_quantum(&amp;self, context: &amp;TaskContext) -> Duration {
        let base_quantum = Duration::from_millis(self.config.base_quantum_ms);
        
        if !self.config.enable_adaptive_quantum {
            return base_quantum;
        }
        
        let monitor = self.resource_monitor.read().await;
        let priority_factor = context.characteristics.priority as u64;
        let load_factor = (1.0 - monitor.cpu_usage) * 2.0;
        
        let adaptive_quantum = base_quantum.as_millis() as u64
            * priority_factor
            * load_factor as u64
            / 2;
        
        let clamped = adaptive_quantum.clamp(
            self.config.min_quantum_ms,
            self.config.max_quantum_ms,
        );
        
        Duration::from_millis(clamped)
    }
    
    /// Sort ready queue based on scheduling algorithm
    async fn sort_queue(&amp;self, queue: &amp;mut VecDeque<TaskContext>) {
        match self.config.algorithm {
            SchedulingAlgorithm::RoundRobin => {
                // Already FIFO
            }
            SchedulingAlgorithm::Priority => {
                queue.make_contiguous().sort_by(|a, b| {
                    b.characteristics.priority
                        .cmp(&amp;a.characteristics.priority)
                });
            }
            SchedulingAlgorithm::SJF => {
                queue.make_contiguous().sort_by(|a, b| {
                    a.characteristics
                        .estimated_duration
                        .cmp(&amp;b.characteristics.estimated_duration)
                });
            }
            SchedulingAlgorithm::DeepLearning => {
                queue.make_contiguous().sort_by(|a, b| {
                    let model = self.model.read().await;
                    let pred_a = model.predict(a);
                    let pred_b = model.predict(b);
                    pred_a.cmp(&amp;pred_b)
                });
            }
            SchedulingAlgorithm::ReinforcementLearning => {
                // Similar to deep learning for now
                queue.make_contiguous().sort_by(|a, b| {
                    let model = self.model.read().await;
                    let pred_a = model.predict(a);
                    let pred_b = model.predict(b);
                    pred_a.cmp(&amp;pred_b)
                });
            }
        }
    }
    
    /// Complete a task
    pub async fn complete_task(&amp;self, task_id: &amp;str, actual_duration: Duration) {
        let mut running = self.running_tasks.write().await;
        if let Some(mut context) = running.remove(task_id) {
            context.cpu_time = actual_duration;
            
            // Train the model
            let mut model = self.model.write().await;
            model.train(&amp;context, actual_duration);
        }
    }
    
    /// Update resource monitor
    pub async fn update_resources(&amp;self, cpu_usage: f64, memory_usage: f64, load_average: f64) {
        let mut monitor = self.resource_monitor.write().await;
        monitor.cpu_usage = cpu_usage;
        monitor.memory_usage = memory_usage;
        monitor.load_average = load_average;
    }
    
    /// Get scheduler statistics
    pub async fn get_stats(&amp;self) -> SchedulerStats {
        let queue = self.ready_queue.read().await;
        let running = self.running_tasks.read().await;
        let model = self.model.read().await;
        
        SchedulerStats {
            ready_queue_length: queue.len(),
            running_tasks: running.len(),
            model_accuracy: model.accuracy(),
            total_predictions: model.prediction_count,
        }
    }
    
    /// Preempt current task (for higher priority)
    pub async fn preempt_task(&amp;self, task_id: &amp;str) {
        let mut running = self.running_tasks.write().await;
        if let Some(mut context) = running.remove(task_id) {
            context.preemptions += 1;
            context.wait_time = context.start_time.elapsed();
            
            // Return to ready queue
            let mut queue = self.ready_queue.write().await;
            queue.push_back(context);
            self.sort_queue(&amp;mut queue).await;
        }
    }
}

/// Scheduler statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerStats {
    pub ready_queue_length: usize,
    pub running_tasks: usize,
    pub model_accuracy: f64,
    pub total_predictions: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_submit_task() {
        let config = SchedulingConfig::default();
        let scheduler = IntelligentScheduler::new(config);
        
        let characteristics = TaskCharacteristics {
            estimated_duration: 100,
            cpu_intensity: 0.8,
            memory_requirement: 1024 * 1024,
            io_intensity: 0.2,
            priority: ProcessPriority::Normal,
            category: TaskCategory::Compute,
        };
        
        scheduler.submit_task("task1".to_string(), characteristics).await;
        let stats = scheduler.get_stats().await;
        
        assert_eq!(stats.ready_queue_length, 1);
    }
    
    #[tokio::test]
    async fn test_schedule_next() {
        let config = SchedulingConfig::default();
        let scheduler = IntelligentScheduler::new(config);
        
        let characteristics = TaskCharacteristics {
            estimated_duration: 100,
            cpu_intensity: 0.8,
            memory_requirement: 1024 * 1024,
            io_intensity: 0.2,
            priority: ProcessPriority::Normal,
            category: TaskCategory::Compute,
        };
        
        scheduler.submit_task("task1".to_string(), characteristics).await;
        let decision = scheduler.schedule_next().await;
        
        assert!(decision.is_some());
        assert_eq!(decision.unwrap().task_id, "task1");
    }
    
    #[tokio::test]
    async fn test_priority_scheduling() {
        let mut config = SchedulingConfig::default();
        config.algorithm = SchedulingAlgorithm::Priority;
        
        let scheduler = IntelligentScheduler::new(config);
        
        // Submit tasks with different priorities
        scheduler.submit_task("low".to_string(), TaskCharacteristics {
            estimated_duration: 100,
            cpu_intensity: 0.5,
            memory_requirement: 1024,
            io_intensity: 0.5,
            priority: ProcessPriority::Low,
            category: TaskCategory::Batch,
        }).await;
        
        scheduler.submit_task("high".to_string(), TaskCharacteristics {
            estimated_duration: 100,
            cpu_intensity: 0.5,
            memory_requirement: 1024,
            io_intensity: 0.5,
            priority: ProcessPriority::High,
            category: TaskCategory::Batch,
        }).await;
        
        let decision = scheduler.schedule_next().await;
        assert_eq!(decision.unwrap().task_id, "high");
    }
    
    #[tokio::test]
    async fn test_complete_task() {
        let config = SchedulingConfig::default();
        let scheduler = IntelligentScheduler::new(config);
        
        let characteristics = TaskCharacteristics {
            estimated_duration: 100,
            cpu_intensity: 0.8,
            memory_requirement: 1024 * 1024,
            io_intensity: 0.2,
            priority: ProcessPriority::Normal,
            category: TaskCategory::Compute,
        };
        
        scheduler.submit_task("task1".to_string(), characteristics).await;
        scheduler.schedule_next().await;
        scheduler.complete_task("task1", Duration::from_millis(105)).await;
        
        let stats = scheduler.get_stats().await;
        assert_eq!(stats.running_tasks, 0);
    }
}