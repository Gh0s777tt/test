//! Adaptive System Resource Allocation Module

use std::collections::HashMap;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationConfig {
    pub update_interval_ms: u64,
    pub max_cpu_per_process: f64,
    pub max_memory_per_process: usize,
    pub enable_predictive_allocation: bool,
    pub allocation_policy: AllocationPolicy,
}

impl Default for AllocationConfig {
    fn default() -> Self {
        Self {
            update_interval_ms: 100,
            max_cpu_per_process: 100.0,
            max_memory_per_process: 16 * 1024 * 1024 * 1024,
            enable_predictive_allocation: true,
            allocation_policy: AllocationPolicy::Balanced,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AllocationPolicy {
    Performance,
    Fairness,
    Balanced,
    EnergyEfficient,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResourceType {
    CPU,
    Memory,
    GPU,
    Network,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessPriority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

pub struct ResourceAllocator {
    config: AllocationConfig,
    allocations: Arc<RwLock<HashMap<String, ResourceAllocation>>>,
    pending_requests: Arc<RwLock<Vec<ResourceRequest>>>,
    system_resources: Arc<RwLock<SystemResources>>,
}

#[derive(Debug, Clone)]
pub struct ResourceRequest {
    pub process_id: String,
    pub resource_type: ResourceType,
    pub amount: f64,
    pub priority: ProcessPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub process_id: String,
    pub allocations: HashMap<ResourceType, f64>,
    pub allocation_time: Instant,
}

#[derive(Debug, Clone)]
struct SystemResources {
    total_cpu: f64,
    total_memory: usize,
    available_cpu: f64,
    available_memory: usize,
}

#[derive(Debug, Clone)]
pub enum AllocationError {
    InsufficientResources,
    NotAllocated,
    InvalidAmount,
    SystemBusy,
}

impl ResourceAllocator {
    pub fn new(config: AllocationConfig) -> Self {
        Self {
            config,
            allocations: Arc::new(RwLock::new(HashMap::new())),
            pending_requests: Arc::new(RwLock::new(Vec::new())),
            system_resources: Arc::new(RwLock::new(SystemResources {
                total_cpu: 100.0,
                total_memory: 64 * 1024 * 1024 * 1024,
                available_cpu: 100.0,
                available_memory: 64 * 1024 * 1024 * 1024,
            })),
        }
    }

    pub async fn request_resources(&self, request: ResourceRequest) -> Result<f64, AllocationError> {
        let mut pending = self.pending_requests.write().await;
        pending.push(request);
        
        self.process_pending().await?;
        
        let allocations = self.allocations.read().await;
        if let Some(allocation) = allocations.get(&request.process_id) {
            Ok(*allocation.allocations.get(&request.resource_type).unwrap_or(&0.0))
        } else {
            Err(AllocationError::NotAllocated)
        }
    }

    async fn process_pending(&self) -> Result<(), AllocationError> {
        let mut pending = self.pending_requests.write().await;
        if pending.is_empty() {
            return Ok(());
        }

        pending.sort_by(|a, b| b.priority.cmp(&a.priority));

        let mut to_process = Vec::new();
        std::mem::swap(&mut *pending, &mut to_process);
        drop(pending);

        let mut resources = self.system_resources.write().await;
        let mut allocations = self.allocations.write().await;

        for request in to_process {
            match self.allocate(&mut resources, &mut allocations, &request) {
                Ok(amount) => {
                    let process_alloc = allocations.entry(request.process_id.clone())
                        .or_insert_with(|| ResourceAllocation {
                            process_id: request.process_id.clone(),
                            allocations: HashMap::new(),
                            allocation_time: Instant::now(),
                        });
                    process_alloc.allocations.insert(request.resource_type, amount);
                }
                Err(e) => {
                    let mut pending = self.pending_requests.write().await;
                    pending.push(request);
                    drop(pending);
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    fn allocate(&self, resources: &mut SystemResources, allocations: &mut HashMap<String, ResourceAllocation>, request: &ResourceRequest) -> Result<f64, AllocationError> {
        let available = match request.resource_type {
            ResourceType::CPU => resources.available_cpu,
            ResourceType::Memory => resources.available_memory as f64,
            ResourceType::GPU => 100.0,
            ResourceType::Network => 1000.0,
        };

        if request.amount > available {
            return Err(AllocationError::InsufficientResources);
        }

        let max_amount = match request.resource_type {
            ResourceType::CPU => self.config.max_cpu_per_process,
            ResourceType::Memory => self.config.max_memory_per_process as f64,
            ResourceType::GPU => 100.0,
            ResourceType::Network => 1000.0,
        };

        let amount = request.amount.min(available).min(max_amount);
        
        if amount <= 0.0 {
            return Err(AllocationError::InvalidAmount);
        }

        match request.resource_type {
            ResourceType::CPU => resources.available_cpu -= amount,
            ResourceType::Memory => resources.available_memory -= amount as usize,
            ResourceType::GPU => {},
            ResourceType::Network => {},
        }

        Ok(amount)
    }

    pub async fn release_resources(&self, process_id: &str) {
        let mut allocations = self.allocations.write().await;
        if let Some(allocation) = allocations.remove(process_id) {
            let mut resources = self.system_resources.write().await;
            
            for (resource_type, amount) in allocation.allocations {
                match resource_type {
                    ResourceType::CPU => resources.available_cpu += amount,
                    ResourceType::Memory => resources.available_memory += amount as usize,
                    ResourceType::GPU => {},
                    ResourceType::Network => {},
                }
            }
        }
    }

    pub async fn get_allocation(&self, process_id: &str) -> Option<ResourceAllocation> {
        let allocations = self.allocations.read().await;
        allocations.get(process_id).cloned()
    }

    pub async fn get_system_resources(&self) -> SystemResources {
        self.system_resources.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_request_resources() {
        let allocator = ResourceAllocator::new(AllocationConfig::default());
        
        let request = ResourceRequest {
            process_id: "test".to_string(),
            resource_type: ResourceType::CPU,
            amount: 50.0,
            priority: ProcessPriority::Normal,
        };

        let result = allocator.request_resources(request).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 50.0);
    }

    #[tokio::test]
    async fn test_release_resources() {
        let allocator = ResourceAllocator::new(AllocationConfig::default());
        
        let request = ResourceRequest {
            process_id: "test".to_string(),
            resource_type: ResourceType::CPU,
            amount: 50.0,
            priority: ProcessPriority::Normal,
        };

        allocator.request_resources(request).await.unwrap();
        allocator.release_resources("test").await;
        
        let allocation = allocator.get_allocation("test").await;
        assert!(allocation.is_none());
    }
}
