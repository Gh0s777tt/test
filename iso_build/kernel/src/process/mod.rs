//! Process management for VantisOS

use alloc::collections::BTreeMap;
use alloc::string::String;
use spin::Mutex;

extern crate alloc;

/// Process ID type
pub type Pid = u32;

/// Process states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Created,
    Running,
    Ready,
    Blocked,
    Zombie,
    Stopped,
}

/// Process structure
#[derive(Debug, Clone)]
pub struct Process {
    pub pid: Pid,
    pub state: ProcessState,
    pub name: String,
    pub priority: u8,
    pub uid: u32,
    pub gid: u32,
}

/// Process manager
pub struct ProcessManager {
    processes: BTreeMap<Pid, Process>,
    current_pid: Pid,
    next_pid: Pid,
}

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager {
            processes: BTreeMap::new(),
            current_pid: 0,
            next_pid: 1,
        }
    }
    
    pub fn create_process(&mut self, name: String) -> Pid {
        let pid = self.next_pid;
        self.next_pid += 1;
        
        let process = Process {
            pid,
            state: ProcessState::Created,
            name,
            priority: 0,
            uid: 0,
            gid: 0,
        };
        
        self.processes.insert(pid, process);
        pid
    }
    
    pub fn get_process(&self, pid: Pid) -> Option<&Process> {
        self.processes.get(&pid)
    }
    
    pub fn current(&self) -> Option<&Process> {
        self.processes.get(&self.current_pid)
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
    next_pid: 1,
});

/// Initialize process manager
pub fn init() {
    let mut manager = PROCESS_MANAGER.lock();
    manager.create_process(String::from("init"));
}