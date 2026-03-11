//! Kernel Debugger for VANTIS OS
//!
//! Provides an interactive kernel debugger with breakpoint management,
//! register inspection, memory examination, and call stack tracing.
//! Designed for safe kernel debugging with verified memory access bounds.

use core::fmt;

// ============================================================================
// Debugger Types
// ============================================================================

/// Breakpoint types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakpointType {
    /// Software breakpoint (instruction replacement)
    Software,
    /// Hardware breakpoint (debug register)
    Hardware,
    /// Watchpoint on memory read
    WatchRead,
    /// Watchpoint on memory write
    WatchWrite,
    /// Watchpoint on read or write
    WatchReadWrite,
}

/// Breakpoint state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakpointState {
    Enabled,
    Disabled,
    Hit,
    Expired,
}

/// A debugger breakpoint
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub id: u64,
    pub address: u64,
    pub bp_type: BreakpointType,
    pub state: BreakpointState,
    pub hit_count: u64,
    pub max_hits: Option<u64>,
    pub condition: Option<String>,
    pub label: String,
}

impl Breakpoint {
    pub fn new(id: u64, address: u64, bp_type: BreakpointType, label: &str) -> Self {
        Self {
            id,
            address,
            bp_type,
            state: BreakpointState::Enabled,
            hit_count: 0,
            max_hits: None,
            condition: None,
            label: label.to_string(),
        }
    }

    /// Record a hit and check if expired
    pub fn hit(&mut self) -> bool {
        self.hit_count += 1;
        self.state = BreakpointState::Hit;
        if let Some(max) = self.max_hits {
            if self.hit_count >= max {
                self.state = BreakpointState::Expired;
                return false; // expired
            }
        }
        true
    }

    /// Check if breakpoint is active
    pub fn is_active(&self) -> bool {
        self.state == BreakpointState::Enabled || self.state == BreakpointState::Hit
    }
}

// ============================================================================
// Register & Memory Types
// ============================================================================

/// CPU register set (x86_64-like)
#[derive(Debug, Clone, Default)]
pub struct RegisterSet {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

impl RegisterSet {
    /// Get register value by name
    pub fn get(&self, name: &str) -> Option<u64> {
        match name.to_lowercase().as_str() {
            "rax" => Some(self.rax),
            "rbx" => Some(self.rbx),
            "rcx" => Some(self.rcx),
            "rdx" => Some(self.rdx),
            "rsi" => Some(self.rsi),
            "rdi" => Some(self.rdi),
            "rbp" => Some(self.rbp),
            "rsp" => Some(self.rsp),
            "r8" => Some(self.r8),
            "r9" => Some(self.r9),
            "r10" => Some(self.r10),
            "r11" => Some(self.r11),
            "r12" => Some(self.r12),
            "r13" => Some(self.r13),
            "r14" => Some(self.r14),
            "r15" => Some(self.r15),
            "rip" | "pc" => Some(self.rip),
            "rflags" | "flags" => Some(self.rflags),
            _ => None,
        }
    }

    /// Set register value by name
    pub fn set(&mut self, name: &str, value: u64) -> bool {
        match name.to_lowercase().as_str() {
            "rax" => { self.rax = value; true }
            "rbx" => { self.rbx = value; true }
            "rcx" => { self.rcx = value; true }
            "rdx" => { self.rdx = value; true }
            "rsi" => { self.rsi = value; true }
            "rdi" => { self.rdi = value; true }
            "rbp" => { self.rbp = value; true }
            "rsp" => { self.rsp = value; true }
            "r8" => { self.r8 = value; true }
            "r9" => { self.r9 = value; true }
            "r10" => { self.r10 = value; true }
            "r11" => { self.r11 = value; true }
            "r12" => { self.r12 = value; true }
            "r13" => { self.r13 = value; true }
            "r14" => { self.r14 = value; true }
            "r15" => { self.r15 = value; true }
            "rip" | "pc" => { self.rip = value; true }
            "rflags" | "flags" => { self.rflags = value; true }
            _ => false,
        }
    }

    /// List all register names
    pub fn register_names() -> &'static [&'static str] {
        &["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "rsp",
          "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15",
          "rip", "rflags"]
    }
}

/// A stack frame in the call trace
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub frame_number: u32,
    pub return_address: u64,
    pub function_name: String,
    pub module_name: String,
    pub offset: u64,
}

impl fmt::Display for StackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{} 0x{:016x} {}!{}+0x{:x}",
            self.frame_number, self.return_address,
            self.module_name, self.function_name, self.offset)
    }
}

/// Memory region descriptor
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub base: u64,
    pub size: usize,
    pub data: Vec<u8>,
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
    pub label: String,
}

impl MemoryRegion {
    pub fn new(base: u64, data: Vec<u8>, label: &str) -> Self {
        let size = data.len();
        Self {
            base,
            size,
            data,
            readable: true,
            writable: true,
            executable: false,
            label: label.to_string(),
        }
    }

    /// Check if an address falls within this region
    pub fn contains(&self, addr: u64) -> bool {
        addr >= self.base && addr < self.base + self.size as u64
    }

    /// Read bytes from the region
    pub fn read(&self, addr: u64, len: usize) -> Option<&[u8]> {
        if !self.readable || !self.contains(addr) {
            return None;
        }
        let offset = (addr - self.base) as usize;
        if offset + len > self.data.len() {
            return None;
        }
        Some(&self.data[offset..offset + len])
    }

    /// Write bytes to the region
    pub fn write(&mut self, addr: u64, data: &[u8]) -> bool {
        if !self.writable || !self.contains(addr) {
            return false;
        }
        let offset = (addr - self.base) as usize;
        if offset + data.len() > self.data.len() {
            return false;
        }
        self.data[offset..offset + data.len()].copy_from_slice(data);
        true
    }
}

// ============================================================================
// Debugger State
// ============================================================================

/// Debugger execution state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebuggerState {
    /// Not attached to any target
    Detached,
    /// Attached and target is running
    Running,
    /// Target is stopped at a breakpoint
    Stopped,
    /// Single-stepping
    Stepping,
}

// ============================================================================
// Kernel Debugger
// ============================================================================

/// Error types for the debugger
#[derive(Debug, Clone, PartialEq)]
pub enum DebuggerError {
    NotAttached,
    AlreadyAttached,
    BreakpointNotFound(u64),
    BreakpointLimitReached(usize),
    InvalidAddress(u64),
    MemoryAccessDenied(u64),
    RegisterNotFound(String),
    InvalidState(DebuggerState),
}

impl fmt::Display for DebuggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DebuggerError::NotAttached => write!(f, "Debugger not attached"),
            DebuggerError::AlreadyAttached => write!(f, "Debugger already attached"),
            DebuggerError::BreakpointNotFound(id) => write!(f, "Breakpoint {} not found", id),
            DebuggerError::BreakpointLimitReached(max) => write!(f, "Breakpoint limit {} reached", max),
            DebuggerError::InvalidAddress(addr) => write!(f, "Invalid address: 0x{:x}", addr),
            DebuggerError::MemoryAccessDenied(addr) => write!(f, "Memory access denied: 0x{:x}", addr),
            DebuggerError::RegisterNotFound(name) => write!(f, "Register not found: {}", name),
            DebuggerError::InvalidState(s) => write!(f, "Invalid debugger state: {:?}", s),
        }
    }
}

/// Maximum hardware breakpoints
const MAX_BREAKPOINTS: usize = 256;

/// The main kernel debugger
pub struct KernelDebugger {
    state: DebuggerState,
    breakpoints: Vec<Breakpoint>,
    next_bp_id: u64,
    registers: RegisterSet,
    memory_regions: Vec<MemoryRegion>,
    call_stack: Vec<StackFrame>,
    command_history: Vec<String>,
    step_count: u64,
}

impl KernelDebugger {
    /// Create a new kernel debugger
    pub fn new() -> Self {
        Self {
            state: DebuggerState::Detached,
            breakpoints: Vec::new(),
            next_bp_id: 1,
            registers: RegisterSet::default(),
            memory_regions: Vec::new(),
            call_stack: Vec::new(),
            command_history: Vec::new(),
            step_count: 0,
        }
    }

    /// Attach the debugger
    pub fn attach(&mut self) -> Result<(), DebuggerError> {
        if self.state != DebuggerState::Detached {
            return Err(DebuggerError::AlreadyAttached);
        }
        self.state = DebuggerState::Stopped;
        Ok(())
    }

    /// Detach the debugger
    pub fn detach(&mut self) -> Result<(), DebuggerError> {
        if self.state == DebuggerState::Detached {
            return Err(DebuggerError::NotAttached);
        }
        self.state = DebuggerState::Detached;
        self.breakpoints.clear();
        Ok(())
    }

    /// Set a breakpoint
    pub fn set_breakpoint(
        &mut self,
        address: u64,
        bp_type: BreakpointType,
        label: &str,
    ) -> Result<u64, DebuggerError> {
        self.require_attached()?;

        if self.breakpoints.len() >= MAX_BREAKPOINTS {
            return Err(DebuggerError::BreakpointLimitReached(MAX_BREAKPOINTS));
        }

        let bp_id = self.next_bp_id;
        self.next_bp_id += 1;
        self.breakpoints.push(Breakpoint::new(bp_id, address, bp_type, label));
        Ok(bp_id)
    }

    /// Remove a breakpoint
    pub fn remove_breakpoint(&mut self, bp_id: u64) -> Result<(), DebuggerError> {
        let idx = self.breakpoints.iter().position(|b| b.id == bp_id)
            .ok_or(DebuggerError::BreakpointNotFound(bp_id))?;
        self.breakpoints.remove(idx);
        Ok(())
    }

    /// Enable a breakpoint
    pub fn enable_breakpoint(&mut self, bp_id: u64) -> Result<(), DebuggerError> {
        let bp = self.breakpoints.iter_mut().find(|b| b.id == bp_id)
            .ok_or(DebuggerError::BreakpointNotFound(bp_id))?;
        bp.state = BreakpointState::Enabled;
        Ok(())
    }

    /// Disable a breakpoint
    pub fn disable_breakpoint(&mut self, bp_id: u64) -> Result<(), DebuggerError> {
        let bp = self.breakpoints.iter_mut().find(|b| b.id == bp_id)
            .ok_or(DebuggerError::BreakpointNotFound(bp_id))?;
        bp.state = BreakpointState::Disabled;
        Ok(())
    }

    /// Continue execution
    pub fn continue_execution(&mut self) -> Result<(), DebuggerError> {
        self.require_attached()?;
        self.state = DebuggerState::Running;
        Ok(())
    }

    /// Single step
    pub fn step(&mut self) -> Result<(), DebuggerError> {
        self.require_attached()?;
        self.state = DebuggerState::Stepping;
        self.step_count += 1;
        // Simulate stepping: advance RIP
        self.registers.rip += 1;
        self.state = DebuggerState::Stopped;
        Ok(())
    }

    /// Simulate hitting a breakpoint at an address
    pub fn simulate_hit(&mut self, address: u64) -> Option<u64> {
        let bp = self.breakpoints.iter_mut()
            .find(|b| b.address == address && b.is_active())?;
        bp.hit();
        self.state = DebuggerState::Stopped;
        self.registers.rip = address;
        Some(bp.id)
    }

    /// Read a register
    pub fn read_register(&self, name: &str) -> Result<u64, DebuggerError> {
        self.require_attached()?;
        self.registers.get(name)
            .ok_or_else(|| DebuggerError::RegisterNotFound(name.to_string()))
    }

    /// Write a register
    pub fn write_register(&mut self, name: &str, value: u64) -> Result<(), DebuggerError> {
        self.require_attached()?;
        if self.registers.set(name, value) {
            Ok(())
        } else {
            Err(DebuggerError::RegisterNotFound(name.to_string()))
        }
    }

    /// Add a memory region for examination
    pub fn add_memory_region(&mut self, region: MemoryRegion) {
        self.memory_regions.push(region);
    }

    /// Read memory at an address
    pub fn read_memory(&self, address: u64, length: usize) -> Result<Vec<u8>, DebuggerError> {
        self.require_attached()?;

        for region in &self.memory_regions {
            if let Some(data) = region.read(address, length) {
                return Ok(data.to_vec());
            }
        }

        Err(DebuggerError::InvalidAddress(address))
    }

    /// Write memory at an address
    pub fn write_memory(&mut self, address: u64, data: &[u8]) -> Result<(), DebuggerError> {
        self.require_attached()?;

        for region in &mut self.memory_regions {
            if region.contains(address) {
                if !region.writable {
                    return Err(DebuggerError::MemoryAccessDenied(address));
                }
                if region.write(address, data) {
                    return Ok(());
                }
            }
        }

        Err(DebuggerError::InvalidAddress(address))
    }

    /// Set the call stack (for display)
    pub fn set_call_stack(&mut self, frames: Vec<StackFrame>) {
        self.call_stack = frames;
    }

    /// Get the current call stack
    pub fn call_stack(&self) -> &[StackFrame] {
        &self.call_stack
    }

    /// Format call stack as string
    pub fn backtrace(&self) -> String {
        let mut output = String::new();
        for frame in &self.call_stack {
            output.push_str(&format!("{}\n", frame));
        }
        output
    }

    /// Execute a debugger command (simplified command parser)
    pub fn execute_command(&mut self, cmd: &str) -> Result<String, DebuggerError> {
        self.command_history.push(cmd.to_string());

        let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Ok(String::new());
        }

        match parts[0] {
            "regs" | "registers" => {
                let mut output = String::new();
                for name in RegisterSet::register_names() {
                    if let Some(val) = self.registers.get(name) {
                        output.push_str(&format!("{:>6} = 0x{:016x}\n", name, val));
                    }
                }
                Ok(output)
            }
            "bt" | "backtrace" => {
                Ok(self.backtrace())
            }
            "bp" | "breakpoints" => {
                let mut output = String::new();
                for bp in &self.breakpoints {
                    output.push_str(&format!("#{} 0x{:016x} {:?} {:?} hits={} {}\n",
                        bp.id, bp.address, bp.bp_type, bp.state, bp.hit_count, bp.label));
                }
                Ok(output)
            }
            "step" | "s" => {
                self.step()?;
                Ok(format!("Stepped to 0x{:016x}", self.registers.rip))
            }
            "continue" | "c" => {
                self.continue_execution()?;
                Ok("Continuing...".to_string())
            }
            _ => Ok(format!("Unknown command: {}", parts[0])),
        }
    }

    /// Get debugger state
    pub fn state(&self) -> DebuggerState {
        self.state
    }

    /// Get breakpoint count
    pub fn breakpoint_count(&self) -> usize {
        self.breakpoints.len()
    }

    /// Get step count
    pub fn step_count(&self) -> u64 {
        self.step_count
    }

    /// Get command history
    pub fn command_history(&self) -> &[String] {
        &self.command_history
    }

    fn require_attached(&self) -> Result<(), DebuggerError> {
        if self.state == DebuggerState::Detached {
            return Err(DebuggerError::NotAttached);
        }
        Ok(())
    }
}

impl Default for KernelDebugger {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attach_detach() {
        let mut dbg = KernelDebugger::new();
        assert_eq!(dbg.state(), DebuggerState::Detached);
        dbg.attach().unwrap();
        assert_eq!(dbg.state(), DebuggerState::Stopped);
        dbg.detach().unwrap();
        assert_eq!(dbg.state(), DebuggerState::Detached);
    }

    #[test]
    fn test_set_breakpoint() {
        let mut dbg = KernelDebugger::new();
        dbg.attach().unwrap();
        let bp_id = dbg.set_breakpoint(0x1000, BreakpointType::Software, "main").unwrap();
        assert_eq!(bp_id, 1);
        assert_eq!(dbg.breakpoint_count(), 1);
    }

    #[test]
    fn test_remove_breakpoint() {
        let mut dbg = KernelDebugger::new();
        dbg.attach().unwrap();
        let bp_id = dbg.set_breakpoint(0x1000, BreakpointType::Software, "main").unwrap();
        dbg.remove_breakpoint(bp_id).unwrap();
        assert_eq!(dbg.breakpoint_count(), 0);
    }

    #[test]
    fn test_enable_disable_breakpoint() {
        let mut dbg = KernelDebugger::new();
        dbg.attach().unwrap();
        let bp_id = dbg.set_breakpoint(0x1000, BreakpointType::Software, "test").unwrap();
        dbg.disable_breakpoint(bp_id).unwrap();
        dbg.enable_breakpoint(bp_id).unwrap();
    }

    #[test]
    fn test_simulate_hit() {
        let mut dbg = KernelDebugger::new();
        dbg.attach().unwrap();
        dbg.set_breakpoint(0x1000, BreakpointType::Software, "main").unwrap();
        let hit = dbg.simulate_hit(0x1000);
        assert!(hit.is_some());
        assert_eq!(dbg.state(), DebuggerState::Stopped);
    }

    #[test]
    fn test_registers() {
        let mut dbg = KernelDebugger::new();
        dbg.attach().unwrap();
        dbg.write_register("rax", 42).unwrap();
        assert_eq!(dbg.read_register("rax").unwrap(), 42);
    }

    #[test]
    fn test_invalid_register() {
        let mut dbg = KernelDebugger::new();
        dbg.attach().unwrap();
        let result = dbg.read_register("xyz");
        assert!(matches!(result, Err(DebuggerError::RegisterNotFound(_))));
    }

    #[test]
    fn test_memory_read_write() {
        let mut dbg = KernelDebugger::new();
        dbg.attach().unwrap();
        dbg.add_memory_region(MemoryRegion::new(0x1000, vec![0; 256], "stack"));

        dbg.write_memory(0x1000, &[0xDE, 0xAD, 0xBE, 0xEF]).unwrap();
        let data = dbg.read_memory(0x1000, 4).unwrap();
        assert_eq!(data, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn test_memory_invalid_address() {
        let mut dbg = KernelDebugger::new();
        dbg.attach().unwrap();
        let result = dbg.read_memory(0xDEAD, 4);
        assert!(matches!(result, Err(DebuggerError::InvalidAddress(_))));
    }

    #[test]
    fn test_step() {
        let mut dbg = KernelDebugger::new();
        dbg.attach().unwrap();
        dbg.registers.rip = 0x1000;
        dbg.step().unwrap();
        assert_eq!(dbg.registers.rip, 0x1001);
        assert_eq!(dbg.step_count(), 1);
    }

    #[test]
    fn test_execute_command() {
        let mut dbg = KernelDebugger::new();
        dbg.attach().unwrap();
        let output = dbg.execute_command("regs").unwrap();
        assert!(output.contains("rax"));
        assert!(output.contains("rip"));
    }

    #[test]
    fn test_backtrace() {
        let mut dbg = KernelDebugger::new();
        dbg.attach().unwrap();
        dbg.set_call_stack(vec![
            StackFrame { frame_number: 0, return_address: 0x1000, function_name: "main".into(), module_name: "kernel".into(), offset: 0 },
            StackFrame { frame_number: 1, return_address: 0x2000, function_name: "init".into(), module_name: "kernel".into(), offset: 0x10 },
        ]);
        let bt = dbg.backtrace();
        assert!(bt.contains("main"));
        assert!(bt.contains("init"));
    }

    #[test]
    fn test_not_attached_error() {
        let dbg = KernelDebugger::new();
        let result = dbg.read_register("rax");
        assert!(matches!(result, Err(DebuggerError::NotAttached)));
    }

    #[test]
    fn test_breakpoint_max_hits() {
        let mut bp = Breakpoint::new(1, 0x1000, BreakpointType::Software, "test");
        bp.max_hits = Some(2);
        assert!(bp.hit()); // hit 1
        assert!(!bp.hit()); // hit 2 - expired
        assert_eq!(bp.state, BreakpointState::Expired);
    }
}