// Process Scheduler
//
// This module provides process scheduling for VantisOS, including:
// - Round-robin scheduling
// - Priority-based scheduling
// - Process state transitions
// - Context switching

#![no_std]

use crate::verified::minimal_kernel::process::{ProcessControlBlock, ProcessState, ProcessPriority};
use crate::verified::minimal_kernel::process_manager::{ProcessManager, ProcessId};
use crate::verified::minimal_kernel::thread::{ThreadControlBlock, ThreadState};
use crate::verified::minimal_kernel::timer::get_ticks;
use core::sync::atomic::{AtomicU64, Ordering};

/// Scheduling algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingAlgorithm {
    /// Round-robin scheduling
    RoundRobin,
    /// Priority-based scheduling
    Priority,
    /// Multilevel feedback queue
    MultilevelFeedbackQueue,
}

/// Scheduler
pub struct ProcessScheduler {
    /// Process manager
    process_manager: ProcessManager,
    /// Current process
    current_process: AtomicU64,
    /// Scheduling algorithm
    algorithm: SchedulingAlgorithm,
    /// Time quantum (in ticks)
    time_quantum: u64,
    /// Last schedule time
    last_schedule_time: AtomicU64,
    /// Schedule count
    schedule_count: AtomicU64,
}

impl ProcessScheduler {
    /// Create a new scheduler
    pub fn new(process_manager: ProcessManager) -> Self {
        ProcessScheduler {
            process_manager,
            current_process: AtomicU64::new(0),
            algorithm: SchedulingAlgorithm::RoundRobin,
            time_quantum: 10, // 10 ticks = 10 ms at 1000 Hz
            last_schedule_time: AtomicU64::new(0),
            schedule_count: AtomicU64::new(0),
        }
    }

    /// Schedule next process
    pub fn schedule(&mut self) -> Option<ProcessId> {
        let current_time = get_ticks();
        self.last_schedule_time.store(current_time, Ordering::SeqCst);
        self.schedule_count.fetch_add(1, Ordering::SeqCst);

        match self.algorithm {
            SchedulingAlgorithm::RoundRobin => self.schedule_round_robin(),
            SchedulingAlgorithm::Priority => self.schedule_priority(),
            SchedulingAlgorithm::MultilevelFeedbackQueue => self.schedule_mlfq(),
        }
    }

    /// Round-robin scheduling
    fn schedule_round_robin(&mut self) -> Option<ProcessId> {
        let current_pid = self.current_process.load(Ordering::SeqCst) as ProcessId;
        let processes = self.process_manager.get_processes();
        let mut next_pid = None;
        let mut found_current = false;

        // Find next ready process
        for i in 0..processes.len() {
            if let Some(process) = &processes[i] {
                let pid = process.get_pid();
                
                // Skip current process if it's still running
                if pid == current_pid && process.get_state() == ProcessState::Running {
                    found_current = true;
                    continue;
                }
                
                // Find next ready process
                if process.get_state() == ProcessState::Ready {
                    if found_current || current_pid == 0 {
                        next_pid = Some(pid);
                        break;
                    }
                }
                
                if pid == current_pid {
                    found_current = true;
                }
            }
        }

        // If no ready process found, wrap around
        if next_pid.is_none() {
            for i in 0..processes.len() {
                if let Some(process) = &processes[i] {
                    if process.get_state() == ProcessState::Ready {
                        next_pid = Some(process.get_pid());
                        break;
                    }
                }
            }
        }

        // Update current process
        if let Some(pid) = next_pid {
            self.current_process.store(pid as u64, Ordering::SeqCst);
            self.process_manager.set_current_process(pid);
            
            // Set process state to running
            if let Some(process) = self.process_manager.get_process_mut(pid) {
                process.set_state(ProcessState::Running);
            }
        }

        next_pid
    }

    /// Priority-based scheduling
    fn schedule_priority(&mut self) -> Option<ProcessId> {
        let processes = self.process_manager.get_processes();
        let mut highest_priority = ProcessPriority::Idle;
        let mut next_pid = None;

        // Find highest priority ready process
        for i in 0..processes.len() {
            if let Some(process) = &processes[i] {
                if process.get_state() == ProcessState::Ready {
                    let priority = process.get_priority();
                    if priority > highest_priority {
                        highest_priority = priority;
                        next_pid = Some(process.get_pid());
                    }
                }
            }
        }

        // Update current process
        if let Some(pid) = next_pid {
            self.current_process.store(pid as u64, Ordering::SeqCst);
            self.process_manager.set_current_process(pid);
            
            // Set process state to running
            if let Some(process) = self.process_manager.get_process_mut(pid) {
                process.set_state(ProcessState::Running);
            }
        }

        next_pid
    }

    /// Multilevel feedback queue scheduling
    fn schedule_mlfq(&mut self) -> Option<ProcessId> {
        // TODO: Implement MLFQ scheduling
        // For now, fall back to priority scheduling
        self.schedule_priority()
    }

    /// Check if current process should be preempted
    pub fn should_preempt(&self) -> bool {
        let current_time = get_ticks();
        let last_schedule = self.last_schedule_time.load(Ordering::SeqCst);
        
        (current_time - last_schedule) >= self.time_quantum
    }

    /// Preempt current process
    pub fn preempt(&mut self) {
        let current_pid = self.current_process.load(Ordering::SeqCst) as ProcessId;
        
        // Set current process to ready
        if let Some(process) = self.process_manager.get_process_mut(current_pid) {
            if process.get_state() == ProcessState::Running {
                process.set_state(ProcessState::Ready);
            }
        }
    }

    /// Yield current process
    pub fn yield_process(&mut self) {
        self.preempt();
        self.schedule();
    }

    /// Get current process
    pub fn get_current_process(&self) -> Option<ProcessId> {
        let pid = self.current_process.load(Ordering::SeqCst) as ProcessId;
        if pid == 0 {
            None
        } else {
            Some(pid)
        }
    }

    /// Set scheduling algorithm
    pub fn set_algorithm(&mut self, algorithm: SchedulingAlgorithm) {
        self.algorithm = algorithm;
    }

    /// Get scheduling algorithm
    pub fn get_algorithm(&self) -> SchedulingAlgorithm {
        self.algorithm
    }

    /// Set time quantum
    pub fn set_time_quantum(&mut self, quantum: u64) {
        self.time_quantum = quantum;
    }

    /// Get time quantum
    pub fn get_time_quantum(&self) -> u64 {
        self.time_quantum
    }

    /// Get schedule count
    pub fn get_schedule_count(&self) -> u64 {
        self.schedule_count.load(Ordering::SeqCst)
    }

    /// Get process manager
    pub fn get_process_manager(&self) -> &ProcessManager {
        &self.process_manager
    }

    /// Get process manager (mutable)
    pub fn get_process_manager_mut(&mut self) -> &mut ProcessManager {
        &mut self.process_manager
    }

    /// Get scheduler statistics
    pub fn get_statistics(&self) -> SchedulerStatistics {
        let stats = self.process_manager.get_process_statistics();
        
        SchedulerStatistics {
            total_processes: stats.total_processes,
            running_processes: stats.running_processes,
            ready_processes: stats.ready_processes,
            blocked_processes: stats.blocked_processes,
            terminated_processes: stats.terminated_processes,
            total_threads: stats.total_threads,
            schedule_count: self.get_schedule_count(),
            algorithm: self.algorithm,
            time_quantum: self.time_quantum,
        }
    }
}

impl Default for ProcessScheduler {
    fn default() -> Self {
        Self::new(ProcessManager::new())
    }
}

/// Scheduler statistics
#[derive(Debug, Clone, Copy)]
pub struct SchedulerStatistics {
    /// Total processes
    pub total_processes: usize,
    /// Running processes
    pub running_processes: usize,
    /// Ready processes
    pub ready_processes: usize,
    /// Blocked processes
    pub blocked_processes: usize,
    /// Terminated processes
    pub terminated_processes: usize,
    /// Total threads
    pub total_threads: usize,
    /// Schedule count
    pub schedule_count: u64,
    /// Scheduling algorithm
    pub algorithm: SchedulingAlgorithm,
    /// Time quantum
    pub time_quantum: u64,
}

/// Context switch
pub struct ContextSwitch {
    /// Old process ID
    old_pid: ProcessId,
    /// New process ID
    new_pid: ProcessId,
    /// Switch time (ticks)
    switch_time: u64,
}

impl ContextSwitch {
    /// Create a new context switch
    pub fn new(old_pid: ProcessId, new_pid: ProcessId) -> Self {
        ContextSwitch {
            old_pid,
            new_pid,
            switch_time: get_ticks(),
        }
    }

    /// Get old process ID
    pub fn get_old_pid(&self) -> ProcessId {
        self.old_pid
    }

    /// Get new process ID
    pub fn get_new_pid(&self) -> ProcessId {
        self.new_pid
    }

    /// Get switch time
    pub fn get_switch_time(&self) -> u64 {
        self.switch_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        let process_manager = ProcessManager::new();
        let scheduler = ProcessScheduler::new(process_manager);
        
        assert_eq!(scheduler.get_algorithm(), SchedulingAlgorithm::RoundRobin);
        assert_eq!(scheduler.get_time_quantum(), 10);
        assert_eq!(scheduler.get_schedule_count(), 0);
    }

    #[test]
    fn test_scheduler_algorithm() {
        let process_manager = ProcessManager::new();
        let mut scheduler = ProcessScheduler::new(process_manager);
        
        scheduler.set_algorithm(SchedulingAlgorithm::Priority);
        assert_eq!(scheduler.get_algorithm(), SchedulingAlgorithm::Priority);
        
        scheduler.set_time_quantum(20);
        assert_eq!(scheduler.get_time_quantum(), 20);
    }

    #[test]
    fn test_context_switch() {
        let context_switch = ContextSwitch::new(1, 2);
        
        assert_eq!(context_switch.get_old_pid(), 1);
        assert_eq!(context_switch.get_new_pid(), 2);
    }

    #[test]
    fn test_preemption() {
        let process_manager = ProcessManager::new();
        let scheduler = ProcessScheduler::new(process_manager);
        
        // Initially should not preempt
        assert!(!scheduler.should_preempt());
    }
}