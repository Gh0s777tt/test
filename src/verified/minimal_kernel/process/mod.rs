//! # Process Management
//!
//! This module provides process management functionality for the minimal kernel.

pub mod pcb;
pub mod process;
pub mod state;

use pcb::ProcessControlBlock;
use process::ProcessManager;
use state::ProcessState;

/// Initialize process manager
pub fn init() {
    ProcessManager::init();
}

/// Process ID type
pub type Pid = u32;

/// Process error
#[derive(Debug, Clone, Copy)]
pub enum ProcessError {
    InvalidPid,
    ProcessNotFound,
    OutOfProcesses,
    InvalidBinary,
    PermissionDenied,
}

/// Process
pub struct Process {
    /// Process ID
    pub pid: Pid,
    /// Process state
    pub state: ProcessState,
    /// Process control block
    pub pcb: ProcessControlBlock,
    /// Parent process ID
    pub parent_pid: Option<Pid>,
    /// Child process IDs
    pub child_pids: Vec<Pid>,
}

impl Process {
    /// Create new process
    pub fn new(pid: Pid, parent_pid: Option<Pid>) -> Self {
        Self {
            pid,
            state: ProcessState::Ready,
            pcb: ProcessControlBlock::new(),
            parent_pid,
            child_pids: Vec::new(),
        }
    }
    
    /// Get process ID
    pub fn pid(&self) -> Pid {
        self.pid
    }
    
    /// Get process state
    pub fn state(&self) -> ProcessState {
        self.state
    }
    
    /// Set process state
    pub fn set_state(&mut self, state: ProcessState) {
        self.state = state;
    }
    
    /// Add child process
    pub fn add_child(&mut self, child_pid: Pid) {
        self.child_pids.push(child_pid);
    }
    
    /// Remove child process
    pub fn remove_child(&mut self, child_pid: Pid) {
        self.child_pids.retain(|&pid| pid != child_pid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_creation() {
        let process = Process::new(1, None);
        assert_eq!(process.pid(), 1);
        assert_eq!(process.state(), ProcessState::Ready);
    }
    
    #[test]
    fn test_child_processes() {
        let mut parent = Process::new(1, None);
        parent.add_child(2);
        parent.add_child(3);
        
        assert_eq!(parent.child_pids.len(), 2);
        
        parent.remove_child(2);
        assert_eq!(parent.child_pids.len(), 1);
    }
}