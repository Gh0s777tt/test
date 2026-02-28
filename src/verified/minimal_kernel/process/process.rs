//! # Process Operations
//!
//! This module provides process operations for the minimal kernel.

use super::{Process, ProcessError, Pid};
use super::pcb::ProcessControlBlock;

/// Process manager
pub struct ProcessManager {
    /// Process table
    processes: Vec<Option<Process>>,
    /// Next PID to allocate
    next_pid: Pid,
}

impl ProcessManager {
    /// Initialize process manager
    pub fn init() {
        // This is a placeholder - actual implementation would:
        // 1. Initialize process table
        // 2. Create init process
        // 3. Set up process management
    }
    
    /// Create new process
    pub fn create_process(&mut self, binary: &[u8]) -> Result<Pid, ProcessError> {
        // Allocate PID
        let pid = self.next_pid;
        self.next_pid += 1;
        
        // Create process
        let process = Process::new(pid, None);
        
        // Add to process table
        if pid < self.processes.len() as Pid {
            self.processes[pid as usize] = Some(process);
        } else {
            self.processes.push(Some(process));
        }
        
        Ok(pid)
    }
    
    /// Terminate process
    pub fn terminate_process(&mut self, pid: Pid) -> Result<(), ProcessError> {
        // Find process
        if pid >= self.processes.len() as Pid {
            return Err(ProcessError::InvalidPid);
        }
        
        if self.processes[pid as usize].is_none() {
            return Err(ProcessError::ProcessNotFound);
        }
        
        // Terminate process
        self.processes[pid as usize] = None;
        
        Ok(())
    }
    
    /// Get process
    pub fn get_process(&self, pid: Pid) -> Option<&Process> {
        if pid < self.processes.len() as Pid {
            self.processes[pid as usize].as_ref()
        } else {
            None
        }
    }
    
    /// Get process mutably
    pub fn get_process_mut(&mut self, pid: Pid) -> Option<&mut Process> {
        if pid < self.processes.len() as Pid {
            self.processes[pid as usize].as_mut()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_terminate_process() {
        let mut manager = ProcessManager {
            processes: Vec::new(),
            next_pid: 1,
        };
        
        // Create process
        let pid = manager.create_process(&[]).unwrap();
        assert_eq!(pid, 1);
        
        // Get process
        let process = manager.get_process(pid).unwrap();
        assert_eq!(process.pid(), 1);
        
        // Terminate process
        manager.terminate_process(pid).unwrap();
        
        // Process should be gone
        assert!(manager.get_process(pid).is_none());
    }
}