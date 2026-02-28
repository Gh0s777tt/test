// Process Manager
//
// This module provides process management for VantisOS, including:
// - Process creation and termination
// - Process scheduling
// - Process state management
// - Process resource management

#![no_std]

use crate::verified::minimal_kernel::process::{ProcessControlBlock, ProcessState, ProcessPriority};
use crate::verified::minimal_kernel::thread::{ThreadControlBlock, ThreadState};
use crate::verified::minimal_kernel::memory::page_alloc::PageAllocator;
use crate::verified::minimal_kernel::memory::memory_region::{MemoryRegion, MemoryRegionType, MemoryRegionManager};
use core::sync::atomic::{AtomicU32, Ordering};

/// Process ID type
pub type ProcessId = u32;

/// Invalid process ID
pub const INVALID_PID: ProcessId = 0;

/// Kernel process ID
pub const KERNEL_PID: ProcessId = 1;

/// Maximum number of processes
pub const MAX_PROCESSES: usize = 1024;

/// Process manager
pub struct ProcessManager {
    /// Process table
    processes: [Option<ProcessControlBlock>; MAX_PROCESSES],
    /// Next process ID
    next_pid: AtomicU32,
    /// Current process
    current_process: AtomicU32,
    /// Process count
    process_count: AtomicU32,
}

impl ProcessManager {
    /// Create a new process manager
    pub fn new() -> Self {
        ProcessManager {
            processes: [None; MAX_PROCESSES],
            next_pid: AtomicU32::new(2), // Start from 2 (0 = invalid, 1 = kernel)
            current_process: AtomicU32::new(KERNEL_PID),
            process_count: AtomicU32::new(0),
        }
    }

    /// Create a new process
    pub fn create_process(
        &mut self,
        name: &'static str,
        entry_point: u64,
        priority: ProcessPriority,
        page_allocator: &mut PageAllocator,
        region_manager: &mut MemoryRegionManager,
    ) -> Option<ProcessId> {
        // Allocate process ID
        let pid = self.next_pid.fetch_add(1, Ordering::SeqCst);
        if pid == INVALID_PID {
            return None;
        }

        // Find free slot in process table
        let slot = self.find_free_slot()?;
        if slot >= MAX_PROCESSES {
            return None;
        }

        // Allocate memory for process
        let memory_size = 16 * 1024 * 1024; // 16 MB default
        let memory_start = region_manager.allocate_region(
            memory_size,
            MemoryRegionType::UserCode,
            name,
        )?;

        // Create process control block
        let pcb = ProcessControlBlock::new(
            pid,
            name,
            entry_point,
            memory_start,
            memory_size,
            priority,
        );

        // Add to process table
        self.processes[slot] = Some(pcb);
        self.process_count.fetch_add(1, Ordering::SeqCst);

        Some(pid)
    }

    /// Terminate a process
    pub fn terminate_process(&mut self, pid: ProcessId, exit_code: i32) -> bool {
        // Find process
        let slot = self.find_process_slot(pid)?;
        let process = self.processes[slot].as_mut()?;

        // Set exit code
        process.set_exit_code(exit_code);

        // Set state to terminated
        process.set_state(ProcessState::Terminated);

        // Free process memory
        // TODO: Free memory regions

        // Remove from process table
        self.processes[slot] = None;
        self.process_count.fetch_sub(1, Ordering::SeqCst);

        true
    }

    /// Get process by ID
    pub fn get_process(&self, pid: ProcessId) -> Option<&ProcessControlBlock> {
        let slot = self.find_process_slot(pid)?;
        self.processes[slot].as_ref()
    }

    /// Get process by ID (mutable)
    pub fn get_process_mut(&mut self, pid: ProcessId) -> Option<&mut ProcessControlBlock> {
        let slot = self.find_process_slot(pid)?;
        self.processes[slot].as_mut()
    }

    /// Get current process
    pub fn get_current_process(&self) -> Option<&ProcessControlBlock> {
        let pid = self.current_process.load(Ordering::SeqCst);
        self.get_process(pid)
    }

    /// Get current process (mutable)
    pub fn get_current_process_mut(&mut self) -> Option<&mut ProcessControlBlock> {
        let pid = self.current_process.load(Ordering::SeqCst);
        self.get_process_mut(pid)
    }

    /// Set current process
    pub fn set_current_process(&self, pid: ProcessId) {
        self.current_process.store(pid, Ordering::SeqCst);
    }

    /// Get process count
    pub fn get_process_count(&self) -> usize {
        self.process_count.load(Ordering::SeqCst)
    }

    /// Get all processes
    pub fn get_processes(&self) -> &[Option<ProcessControlBlock>; MAX_PROCESSES] {
        &self.processes
    }

    /// Find process by name
    pub fn find_process_by_name(&self, name: &str) -> Option<ProcessId> {
        for i in 0..MAX_PROCESSES {
            if let Some(process) = &self.processes[i] {
                if process.get_name() == name {
                    return Some(process.get_pid());
                }
            }
        }
        None
    }

    /// Get process state
    pub fn get_process_state(&self, pid: ProcessId) -> Option<ProcessState> {
        self.get_process(pid).map(|p| p.get_state())
    }

    /// Set process state
    pub fn set_process_state(&mut self, pid: ProcessId, state: ProcessState) -> bool {
        if let Some(process) = self.get_process_mut(pid) {
            process.set_state(state);
            true
        } else {
            false
        }
    }

    /// Get process priority
    pub fn get_process_priority(&self, pid: ProcessId) -> Option<ProcessPriority> {
        self.get_process(pid).map(|p| p.get_priority())
    }

    /// Set process priority
    pub fn set_process_priority(&mut self, pid: ProcessId, priority: ProcessPriority) -> bool {
        if let Some(process) = self.get_process_mut(pid) {
            process.set_priority(priority);
            true
        } else {
            false
        }
    }

    /// Add thread to process
    pub fn add_thread(&mut self, pid: ProcessId, thread: ThreadControlBlock) -> bool {
        if let Some(process) = self.get_process_mut(pid) {
            process.add_thread(thread);
            true
        } else {
            false
        }
    }

    /// Remove thread from process
    pub fn remove_thread(&mut self, pid: ProcessId, tid: u32) -> bool {
        if let Some(process) = self.get_process_mut(pid) {
            process.remove_thread(tid);
            true
        } else {
            false
        }
    }

    /// Get process threads
    pub fn get_process_threads(&self, pid: ProcessId) -> Option<&[Option<ThreadControlBlock>; 256]> {
        self.get_process(pid).map(|p| p.get_threads())
    }

    /// Find free slot in process table
    fn find_free_slot(&self) -> Option<usize> {
        for i in 0..MAX_PROCESSES {
            if self.processes[i].is_none() {
                return Some(i);
            }
        }
        None
    }

    /// Find process slot by PID
    fn find_process_slot(&self, pid: ProcessId) -> Option<usize> {
        for i in 0..MAX_PROCESSES {
            if let Some(process) = &self.processes[i] {
                if process.get_pid() == pid {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Print process list
    pub fn print_process_list(&self) {
        // TODO: Implement printing when console is available
        // For now, this is a placeholder
    }

    /// Get process statistics
    pub fn get_process_statistics(&self) -> ProcessStatistics {
        let mut stats = ProcessStatistics::new();
        
        for i in 0..MAX_PROCESSES {
            if let Some(process) = &self.processes[i] {
                stats.total_processes += 1;
                
                match process.get_state() {
                    ProcessState::Running => stats.running_processes += 1,
                    ProcessState::Ready => stats.ready_processes += 1,
                    ProcessState::Blocked => stats.blocked_processes += 1,
                    ProcessState::Terminated => stats.terminated_processes += 1,
                }
                
                stats.total_threads += process.get_thread_count();
            }
        }
        
        stats
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Process statistics
#[derive(Debug, Clone, Copy)]
pub struct ProcessStatistics {
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
}

impl ProcessStatistics {
    pub fn new() -> Self {
        ProcessStatistics {
            total_processes: 0,
            running_processes: 0,
            ready_processes: 0,
            blocked_processes: 0,
            terminated_processes: 0,
            total_threads: 0,
        }
    }
}

impl Default for ProcessStatistics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_manager_creation() {
        let manager = ProcessManager::new();
        assert_eq!(manager.get_process_count(), 0);
        assert_eq!(manager.get_current_process().is_none(), true);
    }

    #[test]
    fn test_process_creation() {
        let mut manager = ProcessManager::new();
        let mut page_allocator = PageAllocator::new(1024 * 1024 * 1024);
        let mut region_manager = MemoryRegionManager::new();
        
        // Add available memory region
        let region = MemoryRegion::new(
            0x10000000,
            0x20000000,
            MemoryRegionType::Available,
            "Available",
        );
        region_manager.add_region(region);
        
        let pid = manager.create_process(
            "test",
            0x1000,
            ProcessPriority::Normal,
            &mut page_allocator,
            &mut region_manager,
        );
        
        assert!(pid.is_some());
        assert_eq!(manager.get_process_count(), 1);
    }

    #[test]
    fn test_process_termination() {
        let mut manager = ProcessManager::new();
        let mut page_allocator = PageAllocator::new(1024 * 1024 * 1024);
        let mut region_manager = MemoryRegionManager::new();
        
        // Add available memory region
        let region = MemoryRegion::new(
            0x10000000,
            0x20000000,
            MemoryRegionType::Available,
            "Available",
        );
        region_manager.add_region(region);
        
        let pid = manager.create_process(
            "test",
            0x1000,
            ProcessPriority::Normal,
            &mut page_allocator,
            &mut region_manager,
        ).unwrap();
        
        assert!(manager.terminate_process(pid, 0));
        assert_eq!(manager.get_process_count(), 0);
    }

    #[test]
    fn test_process_statistics() {
        let mut manager = ProcessManager::new();
        let mut page_allocator = PageAllocator::new(1024 * 1024 * 1024);
        let mut region_manager = MemoryRegionManager::new();
        
        // Add available memory region
        let region = MemoryRegion::new(
            0x10000000,
            0x20000000,
            MemoryRegionType::Available,
            "Available",
        );
        region_manager.add_region(region);
        
        let _pid = manager.create_process(
            "test",
            0x1000,
            ProcessPriority::Normal,
            &mut page_allocator,
            &mut region_manager,
        );
        
        let stats = manager.get_process_statistics();
        assert_eq!(stats.total_processes, 1);
    }
}