//! # Process State
//!
//! This module defines process states for the minimal kernel.

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    /// Process is ready to run
    Ready,
    /// Process is currently running
    Running,
    /// Process is blocked (waiting for I/O, etc.)
    Blocked,
    /// Process has terminated
    Terminated,
    /// Process is being created
    Creating,
    /// Process is being destroyed
    Destroying,
}

impl ProcessState {
    /// Check if process is runnable
    pub fn is_runnable(&self) -> bool {
        matches!(self, ProcessState::Ready | ProcessState::Running)
    }
    
    /// Check if process is alive
    pub fn is_alive(&self) -> bool {
        !matches!(self, ProcessState::Terminated | ProcessState::Destroying)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_state() {
        assert!(ProcessState::Ready.is_runnable());
        assert!(ProcessState::Running.is_runnable());
        assert!(!ProcessState::Blocked.is_runnable());
        assert!(!ProcessState::Terminated.is_runnable());
        
        assert!(ProcessState::Ready.is_alive());
        assert!(ProcessState::Running.is_alive());
        assert!(ProcessState::Blocked.is_alive());
        assert!(!ProcessState::Terminated.is_alive());
    }
}