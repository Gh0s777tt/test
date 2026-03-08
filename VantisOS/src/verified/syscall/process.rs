// Process System Calls
// fork, exec, exit, wait, getpid, getppid

use super::*;
// use crate::minimal_kernel::process::*;
// use crate::minimal_kernel::thread::*;
use std::vec::Vec;
use std::string::String;
use core::sync::atomic::{AtomicU64, Ordering};

// ============================================================================
// Process System Call Implementations
// ============================================================================

/// Exit system call - terminate current process
pub fn sys_exit_impl(args: &[u64]) -> Result<u64, &'static str> {
    if args.len() != 1 {
        return Err("exit requires 1 argument");
    }

    let exit_code = args[0] as i32;
    
    // In real implementation, this would:
    // 1. Close all file descriptors
    // 2. Free all memory
    // 3. Notify parent process
    // 4. Remove process from scheduler
    // 5. Set process state to Terminated
    
    // For now, just return exit code
    Ok(exit_code as u64)
}

/// Fork system call - create child process
pub fn sys_fork_impl(args: &[u64]) -> Result<u64, &'static str> {
    if args.len() != 0 {
        return Err("fork requires 0 arguments");
    }

    // In real implementation, this would:
    // 1. Create new process control block
    // 2. Copy parent's memory to child
    // 3. Copy parent's file descriptors to child
    // 4. Set child's PID
    // 5. Add child to scheduler
    // 6. Return child's PID to parent, 0 to child
    
    // For now, return a fake child PID
    let child_pid = 1000;
    Ok(child_pid)
}

/// Exec system call - execute new program
pub fn sys_exec_impl(args: &[u64]) -> Result<u64, &'static str> {
    if args.len() < 2 {
        return Err("exec requires at least 2 arguments");
    }

    let _path_ptr = args[0];
    let _argv_ptr = args[1];
    
    // In real implementation, this would:
    // 1. Load executable from path
    // 2. Parse executable format (ELF)
    // 3. Allocate new memory for process
    // 4. Load executable into memory
    // 5. Set up new stack
    // 6. Set up new heap
    // 7. Set entry point
    // 8. Start execution
    
    // For now, just return success
    Ok(0)
}

/// Wait system call - wait for child process
pub fn sys_wait_impl(args: &[u64]) -> Result<u64, &'static str> {
    if args.len() < 1 {
        return Err("wait requires at least 1 argument");
    }

    let _status_ptr = args[0];
    let _options = if args.len() > 1 { args[1] } else { 0 };
    
    // In real implementation, this would:
    // 1. Check if any child has exited
    // 2. If yes, return child's PID and status
    // 3. If no, block until child exits
    // 4. Handle WNOHANG option
    // 5. Handle WUNTRACED option
    
    // For now, return a fake child PID
    let child_pid = 1000;
    Ok(child_pid)
}

/// GetPid system call - get process ID
pub fn sys_getpid_impl(args: &[u64]) -> Result<u64, &'static str> {
    if args.len() != 0 {
        return Err("getpid requires 0 arguments");
    }

    // In real implementation, this would:
    // 1. Get current process from thread-local storage
    // 2. Return process's PID
    
    // For now, return init's PID
    Ok(1)
}

/// GetPpid system call - get parent process ID
pub fn sys_getppid_impl(args: &[u64]) -> Result<u64, &'static str> {
    if args.len() != 0 {
        return Err("getppid requires 0 arguments");
    }

    // In real implementation, this would:
    // 1. Get current process from thread-local storage
    // 2. Return process's parent PID
    
    // For now, return 0 (no parent for init)
    Ok(0)
}

// ============================================================================
// Process Control Block Extension
// ============================================================================

/// Extended process control block
#[derive(Debug, Clone)]
pub struct ProcessControlBlockExt {
    pub pid: u64,
    // pub base: ProcessControlBlock,
    pub parent_pid: u64,
    pub children: Vec<u64>,
    pub exit_code: Option<i32>,
    pub state: ProcessStateExt,
    pub file_descriptors: Vec<Option<FileDescriptor>>,
    pub working_directory: String,
    pub environment: Vec<(String, String)>,
}

/// Extended process state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessStateExt {
    Created,
    Ready,
    Running,
    Blocked,
    Terminated,
    Zombie,
}

impl ProcessControlBlockExt {
    pub fn new(pid: u64, parent_pid: u64) -> Self {
        Self {
            pid,
            // base: ProcessControlBlock::new(pid),
            parent_pid,
            children: Vec::new(),
            exit_code: None,
            state: ProcessStateExt::Created,
            file_descriptors: Vec::new(),
            working_directory: String::from("/"),
            environment: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child_pid: u64) {
        self.children.push(child_pid);
    }

    pub fn remove_child(&mut self, child_pid: u64) {
        self.children.retain(|&pid| pid != child_pid);
    }

    pub fn set_exit_code(&mut self, code: i32) {
        self.exit_code = Some(code);
        self.state = ProcessStateExt::Zombie;
    }

    pub fn is_zombie(&self) -> bool {
        self.state == ProcessStateExt::Zombie
    }
}

// ============================================================================
// File Descriptor
// ============================================================================

/// File descriptor
#[derive(Debug, Clone)]
pub struct FileDescriptor {
    pub fd: usize,
    pub flags: u32,
    pub offset: u64,
    pub path: String,
}

impl FileDescriptor {
    pub fn new(fd: usize, path: String) -> Self {
        Self {
            fd,
            flags: 0,
            offset: 0,
            path,
        }
    }
}

// ============================================================================
// Process Manager
// ============================================================================

/// Process manager
pub struct ProcessManager {
    processes: Vec<Option<ProcessControlBlockExt>>,
    next_pid: AtomicU64,
    current_pid: AtomicU64,
}

impl ProcessManager {
    pub fn new() -> Self {
        let mut processes = Vec::with_capacity(1024);
        processes.resize(1024, None);
        
        // Create init process (PID 1)
        let init = ProcessControlBlockExt::new(1, 0);
        processes[1] = Some(init);

        Self {
            processes,
            next_pid: AtomicU64::new(2),
            current_pid: AtomicU64::new(1),
        }
    }

    /// Allocate new PID
    pub fn allocate_pid(&self) -> u64 {
        self.next_pid.fetch_add(1, Ordering::SeqCst)
    }

    /// Get current process
    pub fn get_current(&self) -> Option<&ProcessControlBlockExt> {
        let pid = self.current_pid.load(Ordering::SeqCst);
        self.processes.get(pid as usize)?.as_ref()
    }

    /// Get current process (mutable)
    pub fn get_current_mut(&mut self) -> Option<&mut ProcessControlBlockExt> {
        let pid = self.current_pid.load(Ordering::SeqCst);
        self.processes.get_mut(pid as usize)?.as_mut()
    }

    /// Get process by PID
    pub fn get_process(&self, pid: u64) -> Option<&ProcessControlBlockExt> {
        self.processes.get(pid as usize)?.as_ref()
    }

    /// Get process by PID (mutable)
    pub fn get_process_mut(&mut self, pid: u64) -> Option<&mut ProcessControlBlockExt> {
        self.processes.get_mut(pid as usize)?.as_mut()
    }

    /// Create new process
    pub fn create_process(&mut self, parent_pid: u64) -> Result<u64, &'static str> {
        let pid = self.allocate_pid();
        
        if pid >= 1024 {
            return Err("Maximum process limit reached");
        }

        let process = ProcessControlBlockExt::new(pid, parent_pid);
        self.processes[pid as usize] = Some(process);

        // Add to parent's children
        if let Some(parent) = self.get_process_mut(parent_pid) {
            parent.add_child(pid);
        }

        Ok(pid)
    }

    /// Fork current process
    pub fn fork(&mut self) -> Result<u64, &'static str> {
        let current = self.get_current().ok_or("No current process")?;
        let parent_pid = current.pid;
        
        let child_pid = self.create_process(parent_pid)?;
        
        // Copy parent's state to child
        let parent_data = self.get_process(parent_pid).map(|p| {
            (p.file_descriptors.clone(), p.working_directory.clone(), p.environment.clone())
        });
        
        if let Some(child) = self.get_process_mut(child_pid) {
            if let Some((fds, wd, env)) = parent_data {
                child.file_descriptors = fds;
                child.working_directory = wd;
                child.environment = env;
            }
        }

        Ok(child_pid)
    }

    /// Exit current process
    pub fn exit(&mut self, exit_code: i32) -> Result<(), &'static str> {
        let current_pid = self.current_pid.load(Ordering::SeqCst);
        
        if let Some(process) = self.get_process_mut(current_pid) {
            process.set_exit_code(exit_code);
        }

        Ok(())
    }

    /// Wait for child process
    pub fn wait(&self, pid: Option<u64>) -> Result<(u64, i32), &'static str> {
        let current = self.get_current().ok_or("No current process")?;
        
        // Find zombie child
        for &child_pid in &current.children {
            if let Some(child) = self.get_process(child_pid) {
                if child.is_zombie() {
                    if pid.is_none() || pid == Some(child_pid) {
                        if let Some(code) = child.exit_code {
                            return Ok((child_pid, code));
                        }
                    }
                }
            }
        }

        Err("No child process available")
    }

    /// Get current PID
    pub fn get_pid(&self) -> u64 {
        self.current_pid.load(Ordering::SeqCst)
    }

    /// Get parent PID
    pub fn get_ppid(&self) -> Result<u64, &'static str> {
        let current = self.get_current().ok_or("No current process")?;
        Ok(current.parent_pid)
    }

    /// Set current process
    pub fn set_current(&mut self, pid: u64) {
        self.current_pid.store(pid, Ordering::SeqCst);
    }

    /// Get process statistics
    pub fn get_stats(&self) -> ProcessStats {
        let mut stats = ProcessStats {
            total_processes: 0,
            running_processes: 0,
            zombie_processes: 0,
            current_pid: self.current_pid.load(Ordering::SeqCst),
        };

        for process in &self.processes {
            if let Some(p) = process {
                stats.total_processes += 1;
                if p.state == ProcessStateExt::Running {
                    stats.running_processes += 1;
                }
                if p.is_zombie() {
                    stats.zombie_processes += 1;
                }
            }
        }

        stats
    }
}

/// Process statistics
#[derive(Debug, Clone)]
pub struct ProcessStats {
    pub total_processes: usize,
    pub running_processes: usize,
    pub zombie_processes: usize,
    pub current_pid: u64,
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_manager_create() {
        let manager = ProcessManager::new();
        assert_eq!(manager.get_pid(), 1);
        assert_eq!(manager.get_ppid().unwrap(), 0);
    }

    #[test]
    fn test_process_manager_fork() {
        let mut manager = ProcessManager::new();
        let child_pid = manager.fork().unwrap();
        assert!(child_pid > 1);
    }

    #[test]
    fn test_process_manager_exit() {
        let mut manager = ProcessManager::new();
        manager.exit(0).unwrap();
        
        let current = manager.get_current().unwrap();
        assert!(current.is_zombie());
    }

    #[test]
    fn test_process_manager_wait() {
        let mut manager = ProcessManager::new();
        let child_pid = manager.fork().unwrap();
        
        // Simulate child exit
        if let Some(child) = manager.get_process_mut(child_pid) {
            child.set_exit_code(0);
        }
        
        let (pid, code) = manager.wait(None).unwrap();
        assert_eq!(pid, child_pid);
        assert_eq!(code, 0);
    }

    #[test]
    fn test_process_stats() {
        let manager = ProcessManager::new();
        let stats = manager.get_stats();
        
        assert_eq!(stats.total_processes, 1);
        assert_eq!(stats.current_pid, 1);
    }

    #[test]
    fn test_sys_exit() {
        let result = sys_exit_impl(&[0]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_sys_fork() {
        let result = sys_fork_impl(&[]);
        assert!(result.is_ok());
        assert!(result.unwrap() > 0);
    }

    #[test]
    fn test_sys_exec() {
        let result = sys_exec_impl(&[0, 0]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sys_wait() {
        let result = sys_wait_impl(&[0]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sys_getpid() {
        let result = sys_getpid_impl(&[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_sys_getppid() {
        let result = sys_getppid_impl(&[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }
}