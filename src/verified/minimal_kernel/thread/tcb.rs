//! # Thread Control Block
//!
//! This module defines the thread control block (TCB).

/// Thread control block
pub struct ThreadControlBlock {
    /// Thread ID
    pub tid: super::Tid,
    /// Thread state
    pub state: super::ThreadState,
    /// Thread priority
    pub priority: super::Priority,
    /// Program counter
    pub pc: usize,
    /// Stack pointer
    pub sp: usize,
    /// Base pointer
    pub bp: usize,
    /// Registers
    pub registers: [usize; 16],
    /// Stack base
    pub stack_base: usize,
    /// Stack size
    pub stack_size: usize,
}

impl ThreadControlBlock {
    /// Create new thread control block
    pub fn new() -> Self {
        Self {
            tid: 0,
            state: super::ThreadState::Ready,
            priority: super::Priority::Normal,
            pc: 0,
            sp: 0,
            bp: 0,
            registers: [0; 16],
            stack_base: 0,
            stack_size: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tcb_creation() {
        let tcb = ThreadControlBlock::new();
        assert_eq!(tcb.tid, 0);
        assert_eq!(tcb.state, super::ThreadState::Ready);
    }
}