//! Scheduler Optimization Module
//! 
//! This module provides scheduler optimization capabilities including
//! CPU affinity, priority tuning, and load balancing.

use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use spin::Mutex;

/// Scheduling priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority(u8);

impl Priority {
    /// Create a new priority (0-255, higher is more important)
    pub const fn new(value: u8) -> Self {
        Self(value)
    }

    /// Real-time priority
    pub const fn realtime(value: u8) -> Self {
        Self(value)
    }

    /// High priority
    pub const fn high() -> Self {
        Self(200)
    }

    /// Normal priority
    pub const fn normal() -> Self {
        Self(128)
    }

    /// Low priority
    pub const fn low() -> Self {
        Self(50)
    }

    /// Idle priority
    pub const fn idle() -> Self {
        Self(0)
    }
}

/// Scheduling policy
#[derive(Debug, Clone, Copy)]
pub enum SchedulingPolicy {
    RoundRobin,
    FIFO,
    Other,
    Batch,
    Idle,
    Deadline,
}

/// CPU affinity
#[derive(Debug, Clone)]
pub struct CpuAffinity {
    pub cpu_mask: u64,
}

impl CpuAffinity {
    /// Create a new CPU affinity
    pub fn new(cpu_mask: u64) -> Self {
        Self { cpu_mask }
    }

    /// Set affinity for a specific CPU
    pub fn set_cpu(&mut self, cpu: usize) {
        self.cpu_mask = 1 << cpu;
    }

    /// Add a CPU to the affinity
    pub fn add_cpu(&mut self, cpu: usize) {
        self.cpu_mask |= 1 << cpu;
    }

    /// Remove a CPU from the affinity
    pub fn remove_cpu(&mut self, cpu: usize) {
        self.cpu_mask &= !(1 << cpu);
    }

    /// Check if a CPU is in the affinity
    pub fn has_cpu(&self, cpu: usize) -> bool {
        (self.cpu_mask & (1 << cpu)) != 0
    }

    /// Get all CPUs in the affinity
    pub fn cpus(&self) -> Vec<usize> {
        let mut cpus = Vec::new();
        for cpu in 0..64 {
            if self.has_cpu(cpu) {
                cpus.push(cpu);
            }
        }
        cpus
    }
}

impl Default for CpuAffinity {
    fn default() -> Self {
        Self { cpu_mask: u64::MAX }
    }
}

/// Task scheduling information
#[derive(Debug, Clone)]
pub struct TaskSchedInfo {
    pub task_id: usize,
    pub priority: Priority,
    pub policy: SchedulingPolicy,
    pub cpu_affinity: CpuAffinity,
    pub cpu: Option<usize>,
    pub runtime_ns: u64,
    pub vruntime_ns: u64,
}

/// Scheduler optimizer
pub struct SchedulerOptimizer {
    tasks: Arc<Mutex<BTreeMap<usize, TaskSchedInfo>>>,
    load_balancer_enabled: bool,
    auto_tuning_enabled: bool,
}

impl SchedulerOptimizer {
    /// Create a new scheduler optimizer
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(BTreeMap::new())),
            load_balancer_enabled: true,
            auto_tuning_enabled: true,
        }
    }

    /// Add a task to the scheduler
    pub fn add_task(&self, task: TaskSchedInfo) {
        let mut tasks = self.tasks.lock();
        tasks.insert(task.task_id, task);
    }

    /// Remove a task from the scheduler
    pub fn remove_task(&self, task_id: usize) {
        let mut tasks = self.tasks.lock();
        tasks.remove(&task_id);
    }

    /// Set task priority
    pub fn set_priority(&self, task_id: usize, priority: Priority) {
        let mut tasks = self.tasks.lock();
        if let Some(task) = tasks.get_mut(&task_id) {
            task.priority = priority;
        }
    }

    /// Set task CPU affinity
    pub fn set_affinity(&self, task_id: usize, affinity: CpuAffinity) {
        let mut tasks = self.tasks.lock();
        if let Some(task) = tasks.get_mut(&task_id) {
            task.cpu_affinity = affinity;
        }
    }

    /// Set task scheduling policy
    pub fn set_policy(&self, task_id: usize, policy: SchedulingPolicy) {
        let mut tasks = self.tasks.lock();
        if let Some(task) = tasks.get_mut(&task_id) {
            task.policy = policy;
        }
    }

    /// Get task information
    pub fn get_task(&self, task_id: usize) -> Option<TaskSchedInfo> {
        self.tasks.lock().get(&task_id).cloned()
    }

    /// Get all tasks
    pub fn all_tasks(&self) -> Vec<TaskSchedInfo> {
        self.tasks.lock().values().cloned().collect()
    }

    /// Load balance tasks across CPUs
    pub fn load_balance(&self, num_cpus: usize) {
        if !self.load_balancer_enabled {
            return;
        }

        let mut tasks = self.tasks.lock();
        let mut cpu_loads: Vec<Vec<usize>> = vec![Vec::new(); num_cpus];

        // Distribute tasks based on priority
        let mut task_list: Vec<_> = tasks.values().collect();
        task_list.sort_by(|a, b| b.priority.cmp(&a.priority));

        for task in task_list {
            // Find least loaded CPU
            let mut min_cpu = 0;
            let mut min_load = usize::MAX;
            
            for cpu in 0..num_cpus {
                if cpu_loads[cpu].len() < min_load {
                    min_load = cpu_loads[cpu].len();
                    min_cpu = cpu;
                }
            }
            
            cpu_loads[min_cpu].push(task.task_id);
            
            // Update task affinity
            if let Some(t) = tasks.get_mut(&task.task_id) {
                t.cpu = Some(min_cpu);
                t.cpu_affinity = CpuAffinity::new(1 << min_cpu);
            }
        }
    }

    /// Auto-tune task parameters
    pub fn auto_tune(&self) {
        if !self.auto_tuning_enabled {
            return;
        }

        let mut tasks = self.tasks.lock();
        
        for task in tasks.values_mut() {
            // Adjust priority based on runtime
            if task.runtime_ns > 1_000_000_000 { // 1 second
                // Demote long-running tasks
                if task.priority.0 > 10 {
                    task.priority.0 -= 10;
                }
            }
        }
    }

    /// Get statistics
    pub fn statistics(&self) -> SchedulerStatistics {
        let tasks = self.tasks.lock();
        
        let total_tasks = tasks.len();
        let high_priority = tasks.values().filter(|t| t.priority >= Priority::high()).count();
        let normal_priority = tasks.values().filter(|t| t.priority >= Priority::normal() && t.priority < Priority::high()).count();
        let low_priority = tasks.values().filter(|t| t.priority < Priority::normal()).count();
        
        SchedulerStatistics {
            total_tasks,
            high_priority_tasks: high_priority,
            normal_priority_tasks: normal_priority,
            low_priority_tasks: low_priority,
        }
    }

    /// Enable or disable load balancing
    pub fn set_load_balancer_enabled(&mut self, enabled: bool) {
        self.load_balancer_enabled = enabled;
    }

    /// Enable or disable auto tuning
    pub fn set_auto_tuning_enabled(&mut self, enabled: bool) {
        self.auto_tuning_enabled = enabled;
    }
}

impl Default for SchedulerOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Scheduler statistics
#[derive(Debug, Clone, Copy)]
pub struct SchedulerStatistics {
    pub total_tasks: usize,
    pub high_priority_tasks: usize,
    pub normal_priority_tasks: usize,
    pub low_priority_tasks: usize,
}

/// Global scheduler optimizer
static SCHEDULER_OPTIMIZER: spin::Once<SchedulerOptimizer> = spin::Once::new();

/// Get the global scheduler optimizer
pub fn scheduler_optimizer() -> &'static SchedulerOptimizer {
    SCHEDULER_OPTIMIZER.call_once(|| SchedulerOptimizer::default())
}

/// Add a task to the scheduler
pub fn add_task(task: TaskSchedInfo) {
    scheduler_optimizer().add_task(task);
}

/// Set task priority
pub fn set_task_priority(task_id: usize, priority: Priority) {
    scheduler_optimizer().set_priority(task_id, priority);
}

/// Set task CPU affinity
pub fn set_task_affinity(task_id: usize, affinity: CpuAffinity) {
    scheduler_optimizer().set_affinity(task_id, affinity);
}

/// Load balance tasks
pub fn load_balance(num_cpus: usize) {
    scheduler_optimizer().load_balance(num_cpus);
}

/// Auto-tune tasks
pub fn auto_tune() {
    scheduler_optimizer().auto_tune();
}

/// Get scheduler statistics
pub fn scheduler_statistics() -> SchedulerStatistics {
    scheduler_optimizer().statistics()
}