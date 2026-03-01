// Process Management for VantisOS v0.5.0
// Provides process creation, termination, and management

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// Process ID counter
static NEXT_PID: AtomicU64 = AtomicU64::new(2);

// Process state
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ProcessState {
    Created,
    Ready,
    Running,
    Blocked,
    Terminated,
}

// Process priority
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ProcessPriority {
    Idle,
    Low,
    Normal,
    High,
    Realtime,
}

// Process control block
#[derive(Clone, Copy)]
pub struct ProcessControlBlock {
    pub pid: u64,
    pub ppid: u64,
    pub state: ProcessState,
    pub priority: ProcessPriority,
    pub stack_pointer: u64,
    pub instruction_pointer: u64,
    pub page_table: u64,
    pub exit_code: i32,
}

impl ProcessControlBlock {
    pub const fn new(pid: u64, ppid: u64) -> Self {
        ProcessControlBlock {
            pid,
            ppid,
            state: ProcessState::Created,
            priority: ProcessPriority::Normal,
            stack_pointer: 0,
            instruction_pointer: 0,
            page_table: 0,
            exit_code: 0,
        }
    }
}

// Process manager
pub struct ProcessManager {
    processes: [Option<ProcessControlBlock>; 1024],
    current_pid: u64,
}

impl ProcessManager {
    pub const fn new() -> Self {
        ProcessManager {
            processes: [None; 1024],
            current_pid: 0,
        }
    }
    
    // Create new process
    pub fn create_process(&mut self, ppid: u64) -> Option<u64> {
        let pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);
        
        if pid >= 1024 {
            return None;
        }
        
        let pcb = ProcessControlBlock::new(pid, ppid);
        self.processes[pid as usize] = Some(pcb);
        
        Some(pid)
    }
    
    // Terminate process
    pub fn terminate_process(&mut self, pid: u64, exit_code: i32) -> bool {
        if pid >= 1024 {
            return false;
        }
        
        if let Some(ref mut pcb) = self.processes[pid as usize] {
            pcb.state = ProcessState::Terminated;
            pcb.exit_code = exit_code;
            return true;
        }
        
        false
    }
    
    // Get process by PID
    pub fn get_process(&self, pid: u64) -> Option<&ProcessControlBlock> {
        if pid >= 1024 {
            return None;
        }
        
        self.processes[pid as usize].as_ref()
    }
    
    // Get mutable process by PID
    pub fn get_process_mut(&mut self, pid: u64) -> Option<&mut ProcessControlBlock> {
        if pid >= 1024 {
            return None;
        }
        
        self.processes[pid as usize].as_mut()
    }
    
    // Set process state
    pub fn set_process_state(&mut self, pid: u64, state: ProcessState) -> bool {
        if let Some(ref mut pcb) = self.get_process_mut(pid) {
            pcb.state = state;
            return true;
        }
        
        false
    }
    
    // Set process priority
    pub fn set_process_priority(&mut self, pid: u64, priority: ProcessPriority) -> bool {
        if let Some(ref mut pcb) = self.get_process_mut(pid) {
            pcb.priority = priority;
            return true;
        }
        
        false
    }
    
    // Get current process
    pub fn get_current_process(&self) -> Option<&ProcessControlBlock> {
        self.get_process(self.current_pid)
    }
    
    // Set current process
    pub fn set_current_process(&mut self, pid: u64) -> bool {
        if self.get_process(pid).is_some() {
            self.current_pid = pid;
            return true;
        }
        
        false
    }
    
    // Get process count
    pub fn get_process_count(&self) -> u64 {
        let mut count = 0;
        for pcb in &self.processes {
            if pcb.is_some() {
                count += 1;
            }
        }
        count
    }
    
    // Get process statistics
    pub fn get_process_stats(&self) -> ProcessStats {
        let mut stats = ProcessStats::new();
        
        for pcb in &self.processes {
            if let Some(ref p) = pcb {
                stats.total_processes += 1;
                
                match p.state {
                    ProcessState::Created => stats.created += 1,
                    ProcessState::Ready => stats.ready += 1,
                    ProcessState::Running => stats.running += 1,
                    ProcessState::Blocked => stats.blocked += 1,
                    ProcessState::Terminated => stats.terminated += 1,
                }
            }
        }
        
        stats
    }
}

// Process statistics
#[derive(Clone, Copy)]
pub struct ProcessStats {
    pub total_processes: u64,
    pub created: u64,
    pub ready: u64,
    pub running: u64,
    pub blocked: u64,
    pub terminated: u64,
}

impl ProcessStats {
    pub const fn new() -> Self {
        ProcessStats {
            total_processes: 0,
            created: 0,
            ready: 0,
            running: 0,
            blocked: 0,
            terminated: 0,
        }
    }
}

// Global process manager
static mut PROCESS_MANAGER: Option<ProcessManager> = None;

// Initialize process manager
pub fn process_init() {
    unsafe {
        PROCESS_MANAGER = Some(ProcessManager::new());
    }
}

// Get process manager
pub fn get_process_manager() -> &'static mut ProcessManager {
    unsafe {
        if PROCESS_MANAGER.is_none() {
            PROCESS_MANAGER = Some(ProcessManager::new());
        }
        match PROCESS_MANAGER {
            Some(ref mut pm) => pm,
            None => core::hint::unreachable_unchecked(),
        }
    }
}

// Create new process
pub fn create_process(ppid: u64) -> Option<u64> {
    get_process_manager().create_process(ppid)
}

// Terminate process
pub fn terminate_process(pid: u64, exit_code: i32) -> bool {
    get_process_manager().terminate_process(pid, exit_code)
}

// Get current PID
pub fn get_pid() -> u64 {
    get_process_manager().current_pid
}

// Get parent PID
pub fn get_ppid() -> u64 {
    if let Some(pcb) = get_process_manager().get_current_process() {
        return pcb.ppid;
    }
    0
}

// Exit current process
pub fn exit(exit_code: i32) -> ! {
    let pid = get_pid();
    terminate_process(pid, exit_code);
    
    // TODO: Switch to next process
    loop {}
}