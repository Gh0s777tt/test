//! # Thread Scheduler
//!
//! This module provides thread scheduling for the minimal kernel.

use super::{Tid, ThreadState, Priority};

/// Scheduler trait
pub trait Scheduler {
    /// Schedule next thread
    fn schedule(&mut self) -> Option<Tid>;
    
    /// Add thread to scheduler
    fn add_thread(&mut self, tid: Tid, priority: Priority);
    
    /// Remove thread from scheduler
    fn remove_thread(&mut self, tid: Tid);
    
    /// Update thread state
    fn update_thread_state(&mut self, tid: Tid, state: ThreadState);
}

/// Round-robin scheduler
pub struct RoundRobinScheduler {
    /// Ready queue
    ready_queue: Vec<Tid>,
    /// Thread priorities
    priorities: Vec<Priority>,
}

impl RoundRobinScheduler {
    /// Initialize scheduler
    pub fn init() {
        // This is a placeholder - actual implementation would:
        // 1. Initialize ready queue
        // 2. Set up scheduling parameters
        // 3. Configure time slices
    }
    
    /// Create new round-robin scheduler
    pub fn new() -> Self {
        Self {
            ready_queue: Vec::new(),
            priorities: Vec::new(),
        }
    }
}

impl Scheduler for RoundRobinScheduler {
    fn schedule(&mut self) -> Option<Tid> {
        // Get next ready thread
        if let Some(tid) = self.ready_queue.pop_front() {
            // Re-add to end of queue
            self.ready_queue.push_back(tid);
            Some(tid)
        } else {
            None
        }
    }
    
    fn add_thread(&mut self, tid: Tid, priority: Priority) {
        // Add to ready queue
        self.ready_queue.push_back(tid);
        self.priorities.push(priority);
    }
    
    fn remove_thread(&mut self, tid: Tid) {
        // Remove from ready queue
        self.ready_queue.retain(|&t| t != tid);
    }
    
    fn update_thread_state(&mut self, tid: Tid, state: ThreadState) {
        // Update thread state
        if state == ThreadState::Ready {
            if !self.ready_queue.contains(&tid) {
                self.ready_queue.push_back(tid);
            }
        } else if state != ThreadState::Running {
            self.remove_thread(tid);
        }
    }
}

/// Priority scheduler
pub struct PriorityScheduler {
    /// Ready queues for each priority level
    ready_queues: [Vec<Tid>; 5],
}

impl PriorityScheduler {
    /// Create new priority scheduler
    pub fn new() -> Self {
        Self {
            ready_queues: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        }
    }
}

impl Scheduler for PriorityScheduler {
    fn schedule(&mut self) -> Option<Tid> {
        // Schedule highest priority ready thread
        for queue in &mut self.ready_queues {
            if let Some(tid) = queue.pop_front() {
                // Re-add to end of queue
                queue.push_back(tid);
                return Some(tid);
            }
        }
        None
    }
    
    fn add_thread(&mut self, tid: Tid, priority: Priority) {
        // Add to appropriate priority queue
        let priority_idx = priority as usize;
        self.ready_queues[priority_idx].push_back(tid);
    }
    
    fn remove_thread(&mut self, tid: Tid) {
        // Remove from all queues
        for queue in &mut self.ready_queues {
            queue.retain(|&t| t != tid);
        }
    }
    
    fn update_thread_state(&mut self, tid: Tid, state: ThreadState) {
        // Update thread state
        if state == ThreadState::Ready {
            // Find thread and add to appropriate queue
            for queue in &mut self.ready_queues {
                if queue.contains(&tid) {
                    return;
                }
            }
        } else if state != ThreadState::Running {
            self.remove_thread(tid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_round_robin_scheduler() {
        let mut scheduler = RoundRobinScheduler::new();
        
        // Add threads
        scheduler.add_thread(1, Priority::Normal);
        scheduler.add_thread(2, Priority::Normal);
        scheduler.add_thread(3, Priority::Normal);
        
        // Schedule threads
        let tid1 = scheduler.schedule().unwrap();
        let tid2 = scheduler.schedule().unwrap();
        let tid3 = scheduler.schedule().unwrap();
        
        assert_eq!(tid1, 1);
        assert_eq!(tid2, 2);
        assert_eq!(tid3, 3);
    }
    
    #[test]
    fn test_priority_scheduler() {
        let mut scheduler = PriorityScheduler::new();
        
        // Add threads with different priorities
        scheduler.add_thread(1, Priority::Low);
        scheduler.add_thread(2, Priority::High);
        scheduler.add_thread(3, Priority::Normal);
        
        // Schedule - should get high priority thread first
        let tid = scheduler.schedule().unwrap();
        assert_eq!(tid, 2);
    }
}