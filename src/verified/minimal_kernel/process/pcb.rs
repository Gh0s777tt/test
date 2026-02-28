//! # Process Control Block
//!
//! This module defines the process control block (PCB).

/// Process control block
pub struct ProcessControlBlock {
    /// Process ID
    pub pid: super::Pid,
    /// Parent process ID
    pub parent_pid: Option<super::Pid>,
    /// Process state
    pub state: super::ProcessState,
    /// Program counter
    pub pc: usize,
    /// Stack pointer
    pub sp: usize,
    /// Base pointer
    pub bp: usize,
    /// Registers
    pub registers: [usize; 16],
    /// Memory map
    pub memory_map: Vec<MemoryRegion>,
}

/// Memory region
pub struct MemoryRegion {
    /// Start address
    pub start: usize,
    /// End address
    pub end: usize,
    /// Protection flags
    pub protection: super::super::memory::ProtectionFlags,
}

impl ProcessControlBlock {
    /// Create new process control block
    pub fn new() -> Self {
        Self {
            pid: 0,
            parent_pid: None,
            state: super::ProcessState::Ready,
            pc: 0,
            sp: 0,
            bp: 0,
            registers: [0; 16],
            memory_map: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pcb_creation() {
        let pcb = ProcessControlBlock::new();
        assert_eq!(pcb.pid, 0);
        assert_eq!(pcb.state, super::ProcessState::Ready);
    }
}