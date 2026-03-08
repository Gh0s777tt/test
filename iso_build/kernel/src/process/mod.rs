//! Process management for VantisOS
//! 
//! This module provides:
//! - Process creation and management
//! - Context switching
//! - Scheduler (Round-Robin with priorities)
//! - Process states and PCB

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;
use core::sync::atomic::{AtomicU32, Ordering};
use core::arch::asm;

use crate::serial_println;

extern crate alloc;

/// Process ID type
pub type Pid = u32;

/// Max processes
pub const MAX_PROCESSES: usize = 1024;

/// Kernel stack size (16 KB)
pub const KERNEL_STACK_SIZE: usize = 16 * 1024;

/// User stack size (256 KB)
pub const USER_STACK_SIZE: usize = 256 * 1024;

/// Maximum process name length
pub const MAX_NAME_LEN: usize = 64;

/// Next PID counter
static NEXT_PID: AtomicU32 = AtomicU32::new(1);

/// Process states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    /// Just created, not yet scheduled
    Created,
    /// Currently running
    Running,
    /// Ready to run
    Ready,
    /// Waiting for I/O or event
    Blocked,
    /// Terminated but not cleaned up
    Zombie,
    /// Stopped by signal
    Stopped,
}

impl Default for ProcessState {
    fn default() -> Self {
        ProcessState::Created
    }
}

/// Process priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Idle priority (lowest)
    Idle = 0,
    /// Low priority
    Low = 1,
    /// Normal priority (default)
    Normal = 2,
    /// High priority
    High = 3,
    /// Real-time priority (highest)
    RealTime = 4,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Normal
    }
}

/// CPU context saved during context switch
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CpuContext {
    /// General purpose registers
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    /// Instruction pointer
    pub rip: u64,
    /// Stack pointer
    pub rsp: u64,
    /// Flags register
    pub rflags: u64,
    /// CR3 (page table)
    pub cr3: u64,
}

/// Process Control Block
#[derive(Debug)]
pub struct Process {
    /// Process ID
    pub pid: Pid,
    /// Parent process ID
    pub ppid: Pid,
    /// Process name
    pub name: String,
    /// Process state
    pub state: ProcessState,
    /// Process priority
    pub priority: Priority,
    /// User ID
    pub uid: u32,
    /// Group ID
    pub gid: u32,
    /// CPU context
    pub context: CpuContext,
    /// Kernel stack pointer
    pub kernel_stack: u64,
    /// User stack pointer (if user mode)
    pub user_stack: Option<u64>,
    /// Exit code (for zombie processes)
    pub exit_code: i32,
    /// Total CPU time consumed (in ticks)
    pub cpu_time: u64,
    /// Number of times scheduled
    pub schedule_count: u64,
}

impl Process {
    /// Create a new process
    pub fn new(name: String) -> Self {
        let pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);
        
        Process {
            pid,
            ppid: 0,
            name,
            state: ProcessState::Created,
            priority: Priority::Normal,
            uid: 0,
            gid: 0,
            context: CpuContext::default(),
            kernel_stack: 0,
            user_stack: None,
            exit_code: 0,
            cpu_time: 0,
            schedule_count: 0,
        }
    }
    
    /// Set parent process
    pub fn set_parent(&mut self, ppid: Pid) {
        self.ppid = ppid;
    }
    
    /// Set priority
    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }
    
    /// Set user/group
    pub fn set_user(&mut self, uid: u32, gid: u32) {
        self.uid = uid;
        self.gid = gid;
    }
    
    /// Allocate kernel stack
    pub fn allocate_kernel_stack(&mut self) -> Result<(), &'static str> {
        // Allocate stack frames
        let stack_frames = (KERNEL_STACK_SIZE + 4095) / 4096;
        let mut frame_alloc = crate::memory::FRAME_ALLOCATOR.lock();
        let allocator = frame_alloc.as_mut().ok_or("No frame allocator")?;
        
        let mut stack_top: u64 = 0;
        for i in 0..stack_frames {
            let frame = allocator.allocate_frame().ok_or("Out of memory")?;
            let addr = frame.start_address().as_u64();
            if i == 0 {
                stack_top = addr + 4096;
            } else if addr + 4096 > stack_top {
                stack_top = addr + 4096;
            }
        }
        
        self.kernel_stack = stack_top;
        self.context.rsp = stack_top;
        
        Ok(())
    }
}

/// Scheduler run queue
struct RunQueue {
    /// Process queues by priority (5 levels)
    queues: [Vec<Pid>; 5],
    /// Bitmask of non-empty queues
    ready_mask: u8,
}

impl RunQueue {
    fn new() -> Self {
        RunQueue {
            queues: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            ready_mask: 0,
        }
    }
    
    fn add(&mut self, pid: Pid, priority: Priority) {
        let idx = priority as usize;
        self.queues[idx].push(pid);
        self.ready_mask |= 1 << idx;
    }
    
    fn remove(&mut self, pid: Pid, priority: Priority) {
        let idx = priority as usize;
        self.queues[idx].retain(|&p| p != pid);
        if self.queues[idx].is_empty() {
            self.ready_mask &= !(1 << idx);
        }
    }
    
    fn next(&mut self) -> Option<Pid> {
        // Find highest priority non-empty queue
        for i in (0..5).rev() {
            if self.ready_mask & (1 << i) != 0 && !self.queues[i].is_empty() {
                // Round-robin: take from front, put at back
                let pid = self.queues[i].remove(0);
                if !self.queues[i].is_empty() {
                    self.queues[i].push(pid);
                } else {
                    self.ready_mask &= !(1 << i);
                }
                return Some(pid);
            }
        }
        None
    }
    
    fn is_empty(&self) -> bool {
        self.ready_mask == 0
    }
}

/// Process manager
pub struct ProcessManager {
    /// All processes
    processes: BTreeMap<Pid, Process>,
    /// Currently running process
    current_pid: Pid,
    /// Idle process PID
    idle_pid: Pid,
    /// Run queue
    run_queue: RunQueue,
    /// Timer ticks since boot
    ticks: u64,
    /// Time slice per process (in ticks)
    time_slice: u64,
}

impl ProcessManager {
    /// Create a new process manager
    pub fn new() -> Self {
        let mut manager = ProcessManager {
            processes: BTreeMap::new(),
            current_pid: 0,
            idle_pid: 0,
            run_queue: RunQueue::new(),
            ticks: 0,
            time_slice: 10, // 10 ticks per time slice
        };
        
        // Create idle process
        let idle = Process::new(String::from("idle"));
        manager.idle_pid = idle.pid;
        manager.processes.insert(idle.pid, idle);
        
        manager
    }
    
    /// Create a new process
    pub fn create_process(&mut self, name: String) -> Pid {
        let mut process = Process::new(name);
        
        // Set parent
        if self.current_pid != 0 {
            process.ppid = self.current_pid;
        }
        
        // Allocate stack
        if process.allocate_kernel_stack().is_err() {
            serial_println!("[PROC] Failed to allocate stack for process");
        }
        
        let pid = process.pid;
        self.processes.insert(pid, process);
        
        serial_println!("[PROC] Created process {} (pid={})", 
            self.processes.get(&pid).unwrap().name, pid);
        
        pid
    }
    
    /// Create kernel thread
    pub fn create_kernel_thread(&mut self, name: String, entry_point: fn() -> !) -> Pid {
        let pid = self.create_process(name);
        
        if let Some(process) = self.processes.get_mut(&pid) {
            process.context.rip = entry_point as u64;
            process.state = ProcessState::Ready;
            
            // Add to run queue
            self.run_queue.add(pid, process.priority);
        }
        
        pid
    }
    
    /// Get process by PID
    pub fn get_process(&self, pid: Pid) -> Option<&Process> {
        self.processes.get(&pid)
    }
    
    /// Get mutable process by PID
    pub fn get_process_mut(&mut self, pid: Pid) -> Option<&mut Process> {
        self.processes.get_mut(&pid)
    }
    
    /// Get current process
    pub fn current(&self) -> Option<&Process> {
        self.processes.get(&self.current_pid)
    }
    
    /// Get current process mutable
    pub fn current_mut(&mut self) -> Option<&mut Process> {
        self.processes.get_mut(&self.current_pid)
    }
    
    /// Get current PID
    pub fn current_pid(&self) -> Pid {
        self.current_pid
    }
    
    /// Set process state
    pub fn set_state(&mut self, pid: Pid, state: ProcessState) {
        if let Some(process) = self.processes.get_mut(&pid) {
            let old_state = process.state;
            process.state = state;
            
            serial_println!("[PROC] Process {} state: {:?} -> {:?}", 
                pid, old_state, state);
            
            // Update run queue
            match state {
                ProcessState::Ready => {
                    self.run_queue.add(pid, process.priority);
                }
                ProcessState::Blocked | ProcessState::Stopped => {
                    self.run_queue.remove(pid, process.priority);
                }
                _ => {}
            }
        }
    }
    
    /// Block current process
    pub fn block_current(&mut self) {
        if self.current_pid != 0 {
            self.set_state(self.current_pid, ProcessState::Blocked);
        }
    }
    
    /// Wake up process
    pub fn wakeup(&mut self, pid: Pid) {
        self.set_state(pid, ProcessState::Ready);
    }
    
    /// Terminate current process
    pub fn exit_current(&mut self, exit_code: i32) {
        if let Some(process) = self.processes.get_mut(&self.current_pid) {
            process.exit_code = exit_code;
            process.state = ProcessState::Zombie;
            self.run_queue.remove(process.pid, process.priority);
            
            serial_println!("[PROC] Process {} exited with code {}", 
                process.pid, exit_code);
        }
    }
    
    /// Schedule next process
    pub fn schedule(&mut self) -> Option<Pid> {
        // Get next process from run queue
        self.run_queue.next().or(Some(self.idle_pid))
    }
    
    /// Timer tick - called from timer interrupt
    pub fn tick(&mut self) {
        self.ticks += 1;
        
        // Update CPU time for current process
        if let Some(process) = self.current_mut() {
            process.cpu_time += 1;
        }
        
        // Check for time slice expiration
        if self.ticks % self.time_slice == 0 {
            // Time to reschedule
            // In a real implementation, this would trigger a context switch
        }
    }
    
    /// Get process count
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }
    
    /// Get runnable process count
    pub fn runnable_count(&self) -> usize {
        self.run_queue.queues.iter().map(|q| q.len()).sum()
    }
    
    /// List all processes (for debugging)
    pub fn list_processes(&self) {
        serial_println!("PID   PPID  STATE     PRIO  NAME");
        serial_println!("----  ----  --------  ----  ----");
        
        for (_, process) in &self.processes {
            let state = match process.state {
                ProcessState::Created => "CREATED",
                ProcessState::Running => "RUNNING",
                ProcessState::Ready => "READY",
                ProcessState::Blocked => "BLOCKED",
                ProcessState::Zombie => "ZOMBIE",
                ProcessState::Stopped => "STOPPED",
            };
            
            let prio = match process.priority {
                Priority::Idle => "IDLE",
                Priority::Low => "LOW",
                Priority::Normal => "NORM",
                Priority::High => "HIGH",
                Priority::RealTime => "RT",
            };
            
            serial_println!("{:4}  {:4}  {:8}  {:4}  {}", 
                process.pid, process.ppid, state, prio, process.name);
        }
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global process manager
pub static PROCESS_MANAGER: Mutex<ProcessManager> = Mutex::new(ProcessManager {
    processes: BTreeMap::new(),
    current_pid: 0,
    idle_pid: 0,
    run_queue: RunQueue {
        queues: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        ready_mask: 0,
    },
    ticks: 0,
    time_slice: 10,
});

/// Initialize process manager
pub fn init() {
    let mut manager = PROCESS_MANAGER.lock();
    
    // Create init process
    let init_pid = manager.create_process(String::from("init"));
    manager.set_state(init_pid, ProcessState::Ready);
    manager.current_pid = init_pid;
    
    serial_println!("[OK] Process manager initialized");
}

/// Perform context switch
/// 
/// # Safety
/// This function is unsafe because it directly manipulates CPU state
#[unsafe(naked)]
pub unsafe extern "C" fn context_switch(
    _old_context: *mut CpuContext,
    _new_context: *const CpuContext,
) {
    // Save old context
    core::arch::naked_asm!(
        // Save general purpose registers
        "push rax",
        "push rbx",
        "push rcx",
        "push rdx",
        "push rsi",
        "push rdi",
        "push rbp",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        
        // Save old stack pointer
        "mov [rdi + 0x78], rsp",  // rsp offset in CpuContext
        
        // Load new context
        "mov rsp, [rsi + 0x78]",  // new stack pointer
        
        // Restore general purpose registers
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rbp",
        "pop rdi",
        "pop rsi",
        "pop rdx",
        "pop rcx",
        "pop rbx",
        "pop rax",
        
        // Return to new process
        "ret"
    );
}

/// Schedule and switch to next process
pub fn do_schedule() {
    let next_pid = {
        let mut manager = PROCESS_MANAGER.lock();
        manager.schedule()
    };
    
    if let Some(pid) = next_pid {
        let current_pid = PROCESS_MANAGER.lock().current_pid;
        
        if pid != current_pid {
            // Perform context switch
            serial_println!("[SCHED] Switching from {} to {}", current_pid, pid);
            
            // Update states
            {
                let mut manager = PROCESS_MANAGER.lock();
                
                if let Some(old) = manager.get_process_mut(current_pid) {
                    if old.state == ProcessState::Running {
                        old.state = ProcessState::Ready;
                    }
                }
                
                if let Some(new) = manager.get_process_mut(pid) {
                    new.state = ProcessState::Running;
                    new.schedule_count += 1;
                }
                
                manager.current_pid = pid;
            }
            
            // Actual context switch would happen here
        }
    }
}

/// Timer tick handler for scheduling
pub fn timer_tick() {
    let mut manager = PROCESS_MANAGER.lock();
    manager.tick();
}