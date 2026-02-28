// Thread Scheduler
//
// This module provides thread scheduling for VantisOS, including:
// - Round-robin scheduling
// - Priority-based scheduling
// - Thread state transitions
// - Context switching

#![no_std]

use crate::verified::minimal_kernel::thread::{ThreadControlBlock, ThreadState, ThreadPriority};
use crate::verified::minimal_kernel::thread_manager::{ThreadManager, ThreadId};
use crate::verified::minimal_kernel::timer::get_ticks;
use core::sync::atomic::{AtomicU64, Ordering};

/// Scheduling algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadSchedulingAlgorithm {
    /// Round-robin scheduling
    RoundRobin,
    /// Priority-based scheduling
    Priority,
    /// Multilevel feedback queue
    MultilevelFeedbackQueue,
}

/// Thread scheduler
pub struct ThreadScheduler {
    /// Thread manager
    thread_manager: ThreadManager,
    /// Current thread
    current_thread: AtomicU64,
    /// Scheduling algorithm
    algorithm: ThreadSchedulingAlgorithm,
    /// Time quantum (in ticks)
    time_quantum: u64,
    /// Last schedule time
    last_schedule_time: AtomicU64,
    /// Schedule count
    schedule_count: AtomicU64,
}

impl ThreadScheduler {
    /// Create a new thread scheduler
    pub fn new(thread_manager: ThreadManager) -> Self {
        ThreadScheduler {
            thread_manager,
            current_thread: AtomicU64::new(0),
            algorithm: ThreadSchedulingAlgorithm::RoundRobin,
            time_quantum: 5, // 5 ticks = 5 ms at 1000 Hz
            last_schedule_time: AtomicU64::new(0),
            schedule_count: AtomicU64::new(0),
        }
    }

    /// Schedule next thread
    pub fn schedule(&mut self) -> Option<ThreadId> {
        let current_time = get_ticks();
        self.last_schedule_time.store(current_time, Ordering::SeqCst);
        self.schedule_count.fetch_add(1, Ordering::SeqCst);

        match self.algorithm {
            ThreadSchedulingAlgorithm::RoundRobin => self.schedule_round_robin(),
            ThreadSchedulingAlgorithm::Priority => self.schedule_priority(),
            ThreadSchedulingAlgorithm::MultilevelFeedbackQueue => self.schedule_mlfq(),
        }
    }

    /// Round-robin scheduling
    fn schedule_round_robin(&mut self) -> Option<ThreadId> {
        let current_tid = self.current_thread.load(Ordering::SeqCst) as ThreadId;
        let threads = self.thread_manager.get_threads();
        let mut next_tid = None;
        let mut found_current = false;

        // Find next ready thread
        for i in 0..threads.len() {
            if let Some(thread) = &threads[i] {
                let tid = thread.get_tid();
                
                // Skip current thread if it's still running
                if tid == current_tid && thread.get_state() == ThreadState::Running {
                    found_current = true;
                    continue;
                }
                
                // Find next ready thread
                if thread.get_state() == ThreadState::Ready {
                    if found_current || current_tid == 0 {
                        next_tid = Some(tid);
                        break;
                    }
                }
                
                if tid == current_tid {
                    found_current = true;
                }
            }
        }

        // If no ready thread found, wrap around
        if next_tid.is_none() {
            for i in 0..threads.len() {
                if let Some(thread) = &threads[i] {
                    if thread.get_state() == ThreadState::Ready {
                        next_tid = Some(thread.get_tid());
                        break;
                    }
                }
            }
        }

        // Update current thread
        if let Some(tid) = next_tid {
            self.current_thread.store(tid as u64, Ordering::SeqCst);
            self.thread_manager.set_current_thread(tid);
            
            // Set thread state to running
            if let Some(thread) = self.thread_manager.get_thread_mut(tid) {
                thread.set_state(ThreadState::Running);
            }
        }

        next_tid
    }

    /// Priority-based scheduling
    fn schedule_priority(&mut self) -> Option<ThreadId> {
        let threads = self.thread_manager.get_threads();
        let mut highest_priority = ThreadPriority::Idle;
        let mut next_tid = None;

        // Find highest priority ready thread
        for i in 0..threads.len() {
            if let Some(thread) = &threads[i] {
                if thread.get_state() == ThreadState::Ready {
                    let priority = thread.get_priority();
                    if priority > highest_priority {
                        highest_priority = priority;
                        next_tid = Some(thread.get_tid());
                    }
                }
            }
        }

        // Update current thread
        if let Some(tid) = next_tid {
            self.current_thread.store(tid as u64, Ordering::SeqCst);
            self.thread_manager.set_current_thread(tid);
            
            // Set thread state to running
            if let Some(thread) = self.thread_manager.get_thread_mut(tid) {
                thread.set_state(ThreadState::Running);
            }
        }

        next_tid
    }

    /// Multilevel feedback queue scheduling
    fn schedule_mlfq(&mut self) -> Option<ThreadId> {
        // TODO: Implement MLFQ scheduling
        // For now, fall back to priority scheduling
        self.schedule_priority()
    }

    /// Check if current thread should be preempted
    pub fn should_preempt(&self) -> bool {
        let current_time = get_ticks();
        let last_schedule = self.last_schedule_time.load(Ordering::SeqCst);
        
        (current_time - last_schedule) >= self.time_quantum
    }

    /// Preempt current thread
    pub fn preempt(&mut self) {
        let current_tid = self.current_thread.load(Ordering::SeqCst) as ThreadId;
        
        // Set current thread to ready
        if let Some(thread) = self.thread_manager.get_thread_mut(current_tid) {
            if thread.get_state() == ThreadState::Running {
                thread.set_state(ThreadState::Ready);
            }
        }
    }

    /// Yield current thread
    pub fn yield_thread(&mut self) {
        self.preempt();
        self.schedule();
    }

    /// Get current thread
    pub fn get_current_thread(&self) -> Option<ThreadId> {
        let tid = self.current_thread.load(Ordering::SeqCst) as ThreadId;
        if tid == 0 {
            None
        } else {
            Some(tid)
        }
    }

    /// Set scheduling algorithm
    pub fn set_algorithm(&mut self, algorithm: ThreadSchedulingAlgorithm) {
        self.algorithm = algorithm;
    }

    /// Get scheduling algorithm
    pub fn get_algorithm(&self) -> ThreadSchedulingAlgorithm {
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

    /// Get thread manager
    pub fn get_thread_manager(&self) -> &ThreadManager {
        &self.thread_manager
    }

    /// Get thread manager (mutable)
    pub fn get_thread_manager_mut(&mut self) -> &mut ThreadManager {
        &mut self.thread_manager
    }

    /// Get scheduler statistics
    pub fn get_statistics(&self) -> ThreadSchedulerStatistics {
        let stats = self.thread_manager.get_thread_statistics();
        
        ThreadSchedulerStatistics {
            total_threads: stats.total_threads,
            running_threads: stats.running_threads,
            ready_threads: stats.ready_threads,
            blocked_threads: stats.blocked_threads,
            terminated_threads: stats.terminated_threads,
            schedule_count: self.get_schedule_count(),
            algorithm: self.algorithm,
            time_quantum: self.time_quantum,
        }
    }
}

impl Default for ThreadScheduler {
    fn default() -> Self {
        Self::new(ThreadManager::new())
    }
}

/// Thread scheduler statistics
#[derive(Debug, Clone, Copy)]
pub struct ThreadSchedulerStatistics {
    /// Total threads
    pub total_threads: usize,
    /// Running threads
    pub running_threads: usize,
    /// Ready threads
    pub ready_threads: usize,
    /// Blocked threads
    pub blocked_threads: usize,
    /// Terminated threads
    pub terminated_threads: usize,
    /// Schedule count
    pub schedule_count: u64,
    /// Scheduling algorithm
    pub algorithm: ThreadSchedulingAlgorithm,
    /// Time quantum
    pub time_quantum: u64,
}

/// Context switch
pub struct ThreadContextSwitch {
    /// Old thread ID
    old_tid: ThreadId,
    /// New thread ID
    new_tid: ThreadId,
    /// Switch time (ticks)
    switch_time: u64,
}

impl ThreadContextSwitch {
    /// Create a new context switch
    pub fn new(old_tid: ThreadId, new_tid: ThreadId) -> Self {
        ThreadContextSwitch {
            old_tid,
            new_tid,
            switch_time: get_ticks(),
        }
    }

    /// Get old thread ID
    pub fn get_old_tid(&self) -> ThreadId {
        self.old_tid
    }

    /// Get new thread ID
    pub fn get_new_tid(&self) -> ThreadId {
        self.new_tid
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
    fn test_thread_scheduler_creation() {
        let thread_manager = ThreadManager::new();
        let scheduler = ThreadScheduler::new(thread_manager);
        
        assert_eq!(scheduler.get_algorithm(), ThreadSchedulingAlgorithm::RoundRobin);
        assert_eq!(scheduler.get_time_quantum(), 5);
        assert_eq!(scheduler.get_schedule_count(), 0);
    }

    #[test]
    fn test_thread_scheduler_algorithm() {
        let thread_manager = ThreadManager::new();
        let mut scheduler = ThreadScheduler::new(thread_manager);
        
        scheduler.set_algorithm(ThreadSchedulingAlgorithm::Priority);
        assert_eq!(scheduler.get_algorithm(), ThreadSchedulingAlgorithm::Priority);
        
        scheduler.set_time_quantum(10);
        assert_eq!(scheduler.get_time_quantum(), 10);
    }

    #[test]
    fn test_thread_context_switch() {
        let context_switch = ThreadContextSwitch::new(1, 2);
        
        assert_eq!(context_switch.get_old_tid(), 1);
        assert_eq!(context_switch.get_new_tid(), 2);
    }

    #[test]
    fn test_thread_preemption() {
        let thread_manager = ThreadManager::new();
        let scheduler = ThreadScheduler::new(thread_manager);
        
        // Initially should not preempt
        assert!(!scheduler.should_preempt());
    }
}