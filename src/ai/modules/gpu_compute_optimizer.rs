//! GPU Compute Optimizer for AI Workloads
//! 
//! This module implements intelligent GPU resource management and optimization
//! specifically designed for AI/ML workloads, including dynamic scheduling,
//! memory management, and performance tuning.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// GPU optimization errors
#[derive(Error, Debug)]
pub enum GpuOptimizerError {
    #[error("No available GPU resources")]
    NoResourcesAvailable,
    
    #[error("Invalid GPU ID: {0}")]
    InvalidGpuId(u32),
    
    #[error("Memory allocation failed: required {required} MB, available {available} MB")]
    InsufficientMemory { required: u64, available: u64 },
    
    #[error("Workload execution failed: {0}")]
    ExecutionFailed(String),
}

/// Configuration for GPU compute optimizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuOptimizerConfig {
    /// Maximum GPU memory usage percentage (0-100)
    pub max_memory_usage_percent: u8,
    
    /// Number of GPU devices to manage
    pub num_gpus: u32,
    
    /// Enable dynamic batch sizing
    pub enable_dynamic_batching: bool,
    
    /// Enable memory compression
    pub enable_compression: bool,
    
    /// Enable workload prioritization
    pub enable_prioritization: bool,
    
    /// Monitoring interval in milliseconds
    pub monitoring_interval_ms: u64,
    
    /// Threshold for switching between GPUs
    pub load_balancing_threshold: f64,
}

/// GPU device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDeviceInfo {
    /// GPU device ID
    pub device_id: u32,
    
    /// Total GPU memory in MB
    pub total_memory_mb: u64,
    
    /// Available GPU memory in MB
    pub available_memory_mb: u64,
    
    /// Current utilization percentage (0-100)
    pub utilization: f64,
    
    /// Temperature in Celsius
    pub temperature_celsius: f64,
    
    /// Power consumption in watts
    pub power_watts: f64,
    
    /// Number of active workloads
    pub active_workloads: u32,
    
    /// Compute capability
    pub compute_capability: String,
}

/// AI workload characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadCharacteristics {
    /// Unique identifier for the workload
    pub workload_id: String,
    
    /// Type of AI workload
    pub workload_type: WorkloadType,
    
    /// Required GPU memory in MB
    pub required_memory_mb: u64,
    
    /// Expected execution time in seconds
    pub expected_duration: Duration,
    
    /// Priority level (0-100, higher is more important)
    pub priority: u8,
    
    /// Batch size (if applicable)
    pub batch_size: Option<usize>,
    
    /// Number of model parameters
    pub num_parameters: u64,
    
    /// Estimated FLOPs
    pub estimated_flops: u64,
}

/// Types of AI workloads
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WorkloadType {
    /// Training workload
    Training,
    
    /// Inference workload
    Inference,
    
    /// Fine-tuning workload
    FineTuning,
    
    /// Data preprocessing
    Preprocessing,
    
    /// Model serving
    Serving,
}

/// Workload execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkloadStatus {
    /// Waiting for resources
    Pending,
    
    /// Currently executing
    Running {
        gpu_id: u32,
        start_time: Instant,
        progress: f64,
    },
    
    /// Completed successfully
    Completed {
        gpu_id: u32,
        duration: Duration,
    },
    
    /// Failed with error
    Failed {
        error: String,
    },
}

/// GPU workload assignment
#[derive(Debug, Clone)]
pub struct WorkloadAssignment {
    pub workload_id: String,
    pub gpu_id: u32,
    pub allocated_memory_mb: u64,
    pub start_time: Instant,
    pub priority: u8,
}

/// Performance metrics for GPU
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuPerformanceMetrics {
    /// Total workloads processed
    pub total_workloads: u64,
    
    /// Successful workloads
    pub successful_workloads: u64,
    
    /// Failed workloads
    pub failed_workloads: u64,
    
    /// Average execution time
    pub avg_execution_time_ms: u64,
    
    /// Average GPU utilization
    pub avg_utilization: f64,
    
    /// Memory efficiency (used/allocated)
    pub memory_efficiency: f64,
    
    /// Energy per FLOP (J/TFLOP)
    pub energy_per_tflop: f64,
}

/// GPU Compute Optimizer
pub struct GpuComputeOptimizer {
    config: GpuOptimizerConfig,
    
    /// GPU device information
    gpu_devices: Arc<RwLock<HashMap<u32, GpuDeviceInfo>>>,
    
    /// Active workload assignments
    active_workloads: Arc<RwLock<HashMap<String, WorkloadAssignment>>>,
    
    /// Pending workloads queue
    pending_workloads: Arc<RwLock<VecDeque<WorkloadCharacteristics>>>,
    
    /// Workload status tracking
    workload_status: Arc<RwLock<HashMap<String, WorkloadStatus>>>,
    
    /// GPU resource semaphores
    gpu_semaphores: Vec<Arc<Semaphore>>,
    
    /// Performance metrics
    metrics: Arc<RwLock<GpuPerformanceMetrics>>,
    
    /// Execution history
    execution_history: Arc<RwLock<VecDeque<ExecutionRecord>>>,
}

/// Record of workload execution
#[derive(Debug, Clone)]
struct ExecutionRecord {
    workload_id: String,
    gpu_id: u32,
    start_time: Instant,
    end_time: Instant,
    success: bool,
    memory_used_mb: u64,
}

impl GpuComputeOptimizer {
    pub fn new(config: GpuOptimizerConfig) -> Self {
        let gpu_semaphores = (0..config.num_gpus)
            .map(|_| Arc::new(Semaphore::new(1)))
            .collect();

        Self {
            config,
            gpu_devices: Arc::new(RwLock::new(HashMap::new())),
            active_workloads: Arc::new(RwLock::new(HashMap::new())),
            pending_workloads: Arc::new(RwLock::new(VecDeque::new())),
            workload_status: Arc::new(RwLock::new(HashMap::new())),
            gpu_semaphores,
            metrics: Arc::new(RwLock::new(GpuPerformanceMetrics {
                total_workloads: 0,
                successful_workloads: 0,
                failed_workloads: 0,
                avg_execution_time_ms: 0,
                avg_utilization: 0.0,
                memory_efficiency: 0.0,
                energy_per_tflop: 0.0,
            })),
            execution_history: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
        }
    }

    /// Initialize GPU devices with default information
    pub async fn initialize_devices(&self) -> Result<(), GpuOptimizerError> {
        let mut devices = self.gpu_devices.write().await;
        
        for device_id in 0..self.config.num_gpus {
            devices.insert(device_id, GpuDeviceInfo {
                device_id,
                total_memory_mb: 24000, // Default 24GB
                available_memory_mb: 24000,
                utilization: 0.0,
                temperature_celsius: 30.0,
                power_watts: 0.0,
                active_workloads: 0,
                compute_capability: "8.6".to_string(),
            });
        }

        Ok(())
    }

    /// Submit a new workload for execution
    pub async fn submit_workload(&self, workload: WorkloadCharacteristics) -> Result<(), GpuOptimizerError> {
        // Validate workload
        if workload.required_memory_mb == 0 {
            return Err(GpuOptimizerError::ExecutionFailed(
                "Invalid memory requirement".to_string()
            ));
        }

        // Update status to pending
        {
            let mut status = self.workload_status.write().await;
            status.insert(workload.workload_id.clone(), WorkloadStatus::Pending);
        }

        // Add to pending queue
        {
            let mut queue = self.pending_workloads.write().await;
            
            if self.config.enable_prioritization {
                // Insert in priority order
                let mut idx = 0;
                for existing in queue.iter() {
                    if existing.priority < workload.priority {
                        break;
                    }
                    idx += 1;
                }
                queue.insert(idx, workload);
            } else {
                queue.push_back(workload);
            }
        }

        // Try to schedule immediately
        self.schedule_workloads().await?;

        Ok(())
    }

    /// Schedule pending workloads to available GPUs
    async fn schedule_workloads(&self) -> Result<(), GpuOptimizerError> {
        let mut scheduled = Vec::new();

        // Process pending workloads
        loop {
            let workload_opt = {
                let mut queue = self.pending_workloads.write().await;
                queue.pop_front()
            };

            let workload = match workload_opt {
                Some(w) => w,
                None => break,
            };

            // Find best GPU for this workload
            if let Some(gpu_id) = self.find_best_gpu(&workload).await? {
                // Allocate and execute
                if let Err(e) = self.execute_workload(gpu_id, workload).await {
                    log::error!("Failed to execute workload {}: {}", workload.workload_id, e);
                    
                    // Update status to failed
                    {
                        let mut status = self.workload_status.write().await;
                        status.insert(workload.workload_id.clone(), WorkloadStatus::Failed {
                            error: e.to_string(),
                        });
                    }

                    // Update metrics
                    {
                        let mut metrics = self.metrics.write().await;
                        metrics.total_workloads += 1;
                        metrics.failed_workloads += 1;
                    }
                } else {
                    scheduled.push(workload.workload_id.clone());
                }
            } else {
                // No available GPU, put back in queue
                {
                    let mut queue = self.pending_workloads.write().await;
                    queue.push_front(workload);
                }
                break;
            }
        }

        Ok(())
    }

    /// Find the best GPU for a given workload
    async fn find_best_gpu(&self, workload: &WorkloadCharacteristics) -> Result<Option<u32>, GpuOptimizerError> {
        let devices = self.gpu_devices.read().await;
        let active = self.active_workloads.read().await;

        let mut best_gpu: Option<u32> = None;
        let mut best_score = f64::MIN;

        for (&gpu_id, device) in devices.iter() {
            // Check if GPU has enough memory
            let max_memory = (device.total_memory_mb as f64 
                * self.config.max_memory_usage_percent as f64 / 100.0) as u64;
            
            if device.available_memory_mb < workload.required_memory_mb {
                continue;
            }

            // Check if GPU semaphore is available
            let semaphore = &self.gpu_semaphores[gpu_id as usize];
            if semaphore.available_permits() == 0 {
                continue;
            }

            // Calculate suitability score
            let score = self.calculate_gpu_score(gpu_id, device, workload, &active).await;

            if score > best_score {
                best_score = score;
                best_gpu = Some(gpu_id);
            }
        }

        Ok(best_gpu)
    }

    /// Calculate suitability score for a GPU
    async fn calculate_gpu_score(
        &self,
        gpu_id: u32,
        device: &GpuDeviceInfo,
        workload: &WorkloadCharacteristics,
        active: &HashMap<String, WorkloadAssignment>,
    ) -> f64 {
        let mut score = 0.0;

        // Memory availability score (prefer GPUs with more free memory)
        let memory_score = device.available_memory_mb as f64 / device.total_memory_mb as f64;
        score += memory_score * 40.0;

        // Utilization score (prefer less utilized GPUs)
        let utilization_score = (100.0 - device.utilization) / 100.0;
        score += utilization_score * 30.0;

        // Temperature score (prefer cooler GPUs)
        let temp_score = (100.0 - device.temperature_celsius) / 70.0;
        score += temp_score.max(0.0) * 15.0;

        // Workload type compatibility
        let compatibility_score = match workload.workload_type {
            WorkloadType::Training => 20.0,
            WorkloadType::Inference => 25.0,
            WorkloadType::FineTuning => 18.0,
            WorkloadType::Preprocessing => 15.0,
            WorkloadType::Serving => 22.0,
        };
        score += compatibility_score;

        // Load balancing
        let gpu_workloads = active.values()
            .filter(|w| w.gpu_id == gpu_id)
            .count() as f64;
        
        let load_balance_score = 1.0 - (gpu_workloads / self.config.num_gpus as f64);
        score += load_balance_score * 10.0;

        score
    }

    /// Execute a workload on a specific GPU
    async fn execute_workload(&self, gpu_id: u32, workload: WorkloadCharacteristics) -> Result<(), GpuOptimizerError> {
        let semaphore = &self.gpu_semaphores[gpu_id as usize];
        let _permit = semaphore.acquire().await.map_err(|_| {
            GpuOptimizerError::NoResourcesAvailable
        })?;

        // Update GPU device information
        {
            let mut devices = self.gpu_devices.write().await;
            if let Some(device) = devices.get_mut(&gpu_id) {
                device.available_memory_mb -= workload.required_memory_mb;
                device.active_workloads += 1;
            }
        }

        // Create assignment
        let assignment = WorkloadAssignment {
            workload_id: workload.workload_id.clone(),
            gpu_id,
            allocated_memory_mb: workload.required_memory_mb,
            start_time: Instant::now(),
            priority: workload.priority,
        };

        {
            let mut active = self.active_workloads.write().await;
            active.insert(workload.workload_id.clone(), assignment.clone());
        }

        // Update status to running
        {
            let mut status = self.workload_status.write().await;
            status.insert(workload.workload_id.clone(), WorkloadStatus::Running {
                gpu_id,
                start_time: Instant::now(),
                progress: 0.0,
            });
        }

        // Simulate workload execution
        let execution_start = Instant::now();
        let success = tokio::time::timeout(
            workload.expected_duration,
            self.simulate_workload_execution(&workload, gpu_id)
        ).await.is_ok();
        let duration = execution_start.elapsed();

        // Update GPU device information
        {
            let mut devices = self.gpu_devices.write().await;
            if let Some(device) = devices.get_mut(&gpu_id) {
                device.available_memory_mb += workload.required_memory_mb;
                device.active_workloads -= 1;
                device.utilization = (device.utilization * 0.7 + 80.0 * 0.3).min(100.0);
            }
        }

        // Remove from active workloads
        {
            let mut active = self.active_workloads.write().await;
            active.remove(&workload.workload_id);
        }

        // Record execution
        {
            let mut history = self.execution_history.write().await;
            history.push_back(ExecutionRecord {
                workload_id: workload.workload_id.clone(),
                gpu_id,
                start_time: assignment.start_time,
                end_time: Instant::now(),
                success,
                memory_used_mb: workload.required_memory_mb,
            });

            if history.len() > 1000 {
                history.pop_front();
            }
        }

        // Update status
        {
            let mut status = self.workload_status.write().await;
            if success {
                status.insert(workload.workload_id.clone(), WorkloadStatus::Completed {
                    gpu_id,
                    duration,
                });

                // Update metrics
                let mut metrics = self.metrics.write().await;
                metrics.total_workloads += 1;
                metrics.successful_workloads += 1;
                metrics.avg_execution_time_ms = (
                    metrics.avg_execution_time_ms * (metrics.total_workloads - 1) as u64
                    + duration.as_millis() as u64
                ) / metrics.total_workloads;
            } else {
                status.insert(workload.workload_id.clone(), WorkloadStatus::Failed {
                    error: "Execution timeout".to_string(),
                });

                let mut metrics = self.metrics.write().await;
                metrics.total_workloads += 1;
                metrics.failed_workloads += 1;
            }
        }

        Ok(())
    }

    /// Simulate workload execution (placeholder for actual GPU execution)
    async fn simulate_workload_execution(&self, workload: &WorkloadCharacteristics, gpu_id: u32) -> Result<(), GpuOptimizerError> {
        log::info!(
            "Executing workload {} on GPU {} (type: {:?}, memory: {} MB)",
            workload.workload_id,
            gpu_id,
            workload.workload_type,
            workload.required_memory_mb
        );

        // Simulate progress updates
        for progress in (0..=10).map(|i| i as f64 * 10.0) {
            tokio::time::sleep(workload.expected_duration / 10).await;
            
            let mut status = self.workload_status.write().await;
            if let Some(WorkloadStatus::Running { progress: p, .. }) = status.get_mut(&workload.workload_id) {
                *p = progress;
            }
        }

        Ok(())
    }

    /// Get current GPU status
    pub async fn get_gpu_status(&self, gpu_id: u32) -> Result<GpuDeviceInfo, GpuOptimizerError> {
        let devices = self.gpu_devices.read().await;
        devices.get(&gpu_id)
            .cloned()
            .ok_or(GpuOptimizerError::InvalidGpuId(gpu_id))
    }

    /// Get workload status
    pub async fn get_workload_status(&self, workload_id: &str) -> Option<WorkloadStatus> {
        let status = self.workload_status.read().await;
        status.get(workload_id).cloned()
    }

    /// Get performance metrics
    pub async fn get_metrics(&self) -> GpuPerformanceMetrics {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }

    /// Cancel a running workload
    pub async fn cancel_workload(&self, workload_id: &str) -> Result<(), GpuOptimizerError> {
        // Check if workload is running
        {
            let status = self.workload_status.read().await;
            match status.get(workload_id) {
                Some(WorkloadStatus::Running { .. }) => {},
                Some(_) => return Ok(()), // Not running, nothing to cancel
                None => return Err(GpuOptimizerError::ExecutionFailed(
                    "Workload not found".to_string()
                )),
            }
        }

        // Remove from active workloads
        let gpu_id = {
            let mut active = self.active_workloads.write().await;
            active.remove(workload_id)
                .map(|a| a.gpu_id)
        };

        if let Some(gpu_id) = gpu_id {
            // Release GPU resources
            let mut devices = self.gpu_devices.write().await;
            if let Some(device) = devices.get_mut(&gpu_id) {
                device.active_workloads = device.active_workloads.saturating_sub(1);
            }

            // Update status
            let mut status = self.workload_status.write().await;
            status.insert(workload_id.to_string(), WorkloadStatus::Failed {
                error: "Cancelled by user".to_string(),
            });
        }

        Ok(())
    }

    /// Run the optimizer monitoring loop
    pub async fn run(&self) {
        let mut interval = tokio::time::interval(Duration::from_millis(self.config.monitoring_interval_ms));
        
        loop {
            interval.tick().await;

            // Update GPU utilization based on active workloads
            self.update_gpu_utilization().await;
            
            // Try to schedule pending workloads
            if let Err(e) = self.schedule_workloads().await {
                log::error!("Failed to schedule workloads: {}", e);
            }
        }
    }

    /// Update GPU utilization based on active workloads
    async fn update_gpu_utilization(&self) {
        let active = self.active_workloads.read().await;
        let mut devices = self.gpu_devices.write().await;

        for (&gpu_id, device) in devices.iter_mut() {
            let active_count = active.values()
                .filter(|w| w.gpu_id == gpu_id)
                .count() as f64;
            
            // Estimate utilization based on active workloads
            let estimated_utilization = (active_count * 25.0).min(100.0);
            device.utilization = (device.utilization * 0.8 + estimated_utilization * 0.2).min(100.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_optimizer_creation() {
        let config = GpuOptimizerConfig {
            max_memory_usage_percent: 90,
            num_gpus: 2,
            enable_dynamic_batching: true,
            enable_compression: false,
            enable_prioritization: true,
            monitoring_interval_ms: 1000,
            load_balancing_threshold: 0.7,
        };

        let optimizer = GpuComputeOptimizer::new(config);
        optimizer.initialize_devices().await.unwrap();

        let gpu0 = optimizer.get_gpu_status(0).await.unwrap();
        assert_eq!(gpu0.device_id, 0);
        assert_eq!(gpu0.total_memory_mb, 24000);
    }

    #[tokio::test]
    async fn test_workload_submission() {
        let config = GpuOptimizerConfig {
            max_memory_usage_percent: 90,
            num_gpus: 2,
            enable_dynamic_batching: true,
            enable_compression: false,
            enable_prioritization: true,
            monitoring_interval_ms: 1000,
            load_balancing_threshold: 0.7,
        };

        let optimizer = GpuComputeOptimizer::new(config);
        optimizer.initialize_devices().await.unwrap();

        let workload = WorkloadCharacteristics {
            workload_id: "test-1".to_string(),
            workload_type: WorkloadType::Inference,
            required_memory_mb: 1000,
            expected_duration: Duration::from_millis(100),
            priority: 50,
            batch_size: Some(32),
            num_parameters: 1000000,
            estimated_flops: 1000000000,
        };

        optimizer.submit_workload(workload).await.unwrap();
        
        // Check status
        let status = optimizer.get_workload_status("test-1").await;
        assert!(status.is_some());
    }

    #[test]
    fn test_workload_characteristics() {
        let workload = WorkloadCharacteristics {
            workload_id: "test".to_string(),
            workload_type: WorkloadType::Training,
            required_memory_mb: 8000,
            expected_duration: Duration::from_secs(300),
            priority: 80,
            batch_size: Some(64),
            num_parameters: 10000000,
            estimated_flops: 10000000000,
        };

        assert_eq!(workload.workload_id, "test");
        assert_eq!(workload.workload_type, WorkloadType::Training);
        assert_eq!(workload.required_memory_mb, 8000);
    }
}