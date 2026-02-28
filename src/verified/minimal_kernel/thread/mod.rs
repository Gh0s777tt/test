//! # Thread Management
//!
//! This module provides thread management functionality for the minimal kernel.

pub mod tcb;
pub mod thread;
pub mod scheduler;
pub mod thread_manager;
pub mod thread_scheduler;
pub mod sync;

use tcb::ThreadControlBlock;
use thread::ThreadManager;
use scheduler::Scheduler;

/// Initialize thread scheduler
pub fn init() {
    ThreadManager::init();
    Scheduler::init();
}

/// Start idle thread
pub fn start_idle() {
    ThreadManager::start_idle();
}

/// Schedule next thread
pub fn schedule() -> ! {
    loop {
        // Get next thread to run
        if let Some(tid) = Scheduler::schedule() {
            // Switch to thread
            ThreadManager::switch_to(tid);
        } else {
            // No threads to run - halt
            unsafe {
                arch::halt();
            }
        }
    }
}

/// Thread ID type
pub type Tid = u32;

/// Thread state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThreadState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

/// Thread priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Realtime = 4,
}

/// Thread
pub struct Thread {
    /// Thread ID
    pub tid: Tid,
    /// Thread state
    pub state: ThreadState,
    /// Thread priority
    pub priority: Priority,
    /// Thread control block
    pub tcb: ThreadControlBlock,
    /// Process ID
    pub pid: Option<super::process::Pid>,
}

impl Thread {
    /// Create new thread
    pub fn new(tid: Tid, pid: Option<super::process::Pid>) -> Self {
        Self {
            tid,
            state: ThreadState::Ready,
            priority: Priority::Normal,
            tcb: ThreadControlBlock::new(),
            pid,
        }
    }
    
    /// Get thread ID
    pub fn tid(&self) -> Tid {
        self.tid
    }
    
    /// Get thread state
    pub fn state(&self) -> ThreadState {
        self.state
    }
    
    /// Set thread state
    pub fn set_state(&mut self, state: ThreadState) {
        self.state = state;
    }
    
    /// Get thread priority
    pub fn priority(&self) -> Priority {
        self.priority
    }
    
    /// Set thread priority
    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }
}

/// Architecture-specific functions
mod arch {
    /// Halt the CPU
    pub unsafe fn halt() {
        asm!("hlt");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_thread_creation() {
        let thread = Thread::new(1, Some(1));
        assert_eq!(thread.tid(), 1);
        assert_eq!(thread.state(), ThreadState::Ready);
        assert_eq!(thread.priority(), Priority::Normal);
    }
    
    #[test]
    fn test_thread_state() {
        let mut thread = Thread::new(1, Some(1));
        thread.set_state(ThreadState::Running);
        assert_eq!(thread.state(), ThreadState::Running);
    }
    
    #[test]
    fn test_thread_priority() {
        let mut thread = Thread::new(1, Some(1));
        thread.set_priority(Priority::High);
        assert_eq!(thread.priority(), Priority::High);
    }
}