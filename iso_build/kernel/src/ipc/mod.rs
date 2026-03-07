//! Inter-Process Communication for VantisOS
//! 
//! This module provides:
//! - Pipes
//! - Signals
//! - Shared memory
//! - Message queues

use alloc::collections::VecDeque;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::serial_println;

extern crate alloc;

/// Process ID type
pub type Pid = u32;

/// Signal numbers
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Signal {
    /// Hangup
    Hup = 1,
    /// Interrupt
    Int = 2,
    /// Quit
    Quit = 3,
    /// Illegal instruction
    Ill = 4,
    /// Trap
    Trap = 5,
    /// Abort
    Abrt = 6,
    /// Bus error
    Bus = 7,
    /// Floating point exception
    Fpe = 8,
    /// Kill
    Kill = 9,
    /// User defined 1
    Usr1 = 10,
    /// Segmentation fault
    Segv = 11,
    /// User defined 2
    Usr2 = 12,
    /// Pipe write
    Pipe = 13,
    /// Alarm clock
    Alrm = 14,
    /// Termination
    Term = 15,
    /// Stack fault
    StkFlt = 16,
    /// Child status changed
    Chld = 17,
    /// Continue
    Cont = 18,
    /// Stop
    Stop = 19,
    /// Terminal stop
    Tstp = 20,
    /// Terminal input
    Ttin = 21,
    /// Terminal output
    Ttou = 22,
    /// Urgent condition
    Urg = 23,
    /// CPU limit
    Xcpu = 24,
    /// File size limit
    Xfsz = 25,
    /// Virtual alarm
    VtAlrm = 26,
    /// Profiling alarm
    Prof = 27,
    /// Window change
    Winch = 28,
    /// I/O
    Io = 29,
    /// Power failure
    Pwr = 30,
    /// Bad system call
    Sys = 31,
}

impl Signal {
    /// Convert from number
    pub fn from_num(num: u8) -> Option<Self> {
        match num {
            1 => Some(Signal::Hup),
            2 => Some(Signal::Int),
            3 => Some(Signal::Quit),
            4 => Some(Signal::Ill),
            5 => Some(Signal::Trap),
            6 => Some(Signal::Abrt),
            7 => Some(Signal::Bus),
            8 => Some(Signal::Fpe),
            9 => Some(Signal::Kill),
            10 => Some(Signal::Usr1),
            11 => Some(Signal::Segv),
            12 => Some(Signal::Usr2),
            13 => Some(Signal::Pipe),
            14 => Some(Signal::Alrm),
            15 => Some(Signal::Term),
            16 => Some(Signal::StkFlt),
            17 => Some(Signal::Chld),
            18 => Some(Signal::Cont),
            19 => Some(Signal::Stop),
            20 => Some(Signal::Tstp),
            21 => Some(Signal::Ttin),
            22 => Some(Signal::Ttou),
            23 => Some(Signal::Urg),
            24 => Some(Signal::Xcpu),
            25 => Some(Signal::Xfsz),
            26 => Some(Signal::VtAlrm),
            27 => Some(Signal::Prof),
            28 => Some(Signal::Winch),
            29 => Some(Signal::Io),
            30 => Some(Signal::Pwr),
            31 => Some(Signal::Sys),
            _ => None,
        }
    }
    
    /// Is signal terminating?
    pub fn is_terminating(&self) -> bool {
        matches!(self, 
            Signal::Hup | Signal::Int | Signal::Quit | Signal::Ill | 
            Signal::Abrt | Signal::Fpe | Signal::Kill | Signal::Segv |
            Signal::Term | Signal::Xcpu | Signal::Xfsz | Signal::Sys)
    }
    
    /// Is signal stopping?
    pub fn is_stopping(&self) -> bool {
        matches!(self, Signal::Stop | Signal::Tstp | Signal::Ttin | Signal::Ttou)
    }
}

/// Signal action
#[derive(Debug, Clone, Copy)]
pub enum SigAction {
    /// Default action
    Default,
    /// Ignore signal
    Ignore,
    /// Custom handler (address)
    Handler(usize),
}

/// Pending signal
#[derive(Debug, Clone, Copy)]
pub struct PendingSignal {
    pub signal: Signal,
    pub sender: Pid,
    pub value: i32,
}

/// Signal queue for a process
pub struct SignalQueue {
    /// Pending signals
    pending: VecDeque<PendingSignal>,
    /// Blocked signals mask
    blocked: u32,
    /// Signal handlers
    handlers: [SigAction; 32],
}

impl SignalQueue {
    pub fn new() -> Self {
        let mut handlers = [SigAction::Default; 32];
        // Ignore SIGCHLD by default
        handlers[17] = SigAction::Ignore;
        // Ignore SIGWINCH by default
        handlers[28] = SigAction::Ignore;
        // Ignore SIGUSR1/USR2 by default
        handlers[10] = SigAction::Ignore;
        handlers[12] = SigAction::Ignore;
        
        SignalQueue {
            pending: VecDeque::new(),
            blocked: 0,
            handlers,
        }
    }
    
    /// Send a signal
    pub fn send(&mut self, signal: Signal, sender: Pid, value: i32) -> bool {
        let sig_num = signal as u8 as usize;
        if sig_num == 0 || sig_num > 31 {
            return false;
        }
        
        // Check if ignored
        if matches!(self.handlers[sig_num], SigAction::Ignore) {
            return true;
        }
        
        self.pending.push_back(PendingSignal {
            signal,
            sender,
            value,
        });
        
        true
    }
    
    /// Check for pending signal
    pub fn has_pending(&self) -> bool {
        self.pending.iter().any(|s| {
            let sig_bit = 1u32 << (s.signal as u8);
            (self.blocked & sig_bit) == 0
        })
    }
    
    /// Get next pending signal
    pub fn pop_pending(&mut self) -> Option<PendingSignal> {
        let blocked = self.blocked;
        self.pending.iter().position(|s| {
            let sig_bit = 1u32 << (s.signal as u8);
            (blocked & sig_bit) == 0
        }).map(|idx| self.pending.remove(idx).unwrap())
    }
    
    /// Block/unblock signals
    pub fn set_blocked(&mut self, mask: u32) {
        self.blocked = mask;
    }
    
    /// Get blocked mask
    pub fn get_blocked(&self) -> u32 {
        self.blocked
    }
    
    /// Set handler
    pub fn set_handler(&mut self, signal: Signal, action: SigAction) {
        let sig_num = signal as u8 as usize;
        if sig_num > 0 && sig_num <= 31 {
            self.handlers[sig_num] = action;
        }
    }
    
    /// Get handler
    pub fn get_handler(&self, signal: Signal) -> &SigAction {
        &self.handlers[signal as u8 as usize]
    }
}

impl Default for SignalQueue {
    fn default() -> Self {
        Self::new()
    }
}

// ========== Pipes ==========

/// Default pipe buffer size
pub const PIPE_BUF_SIZE: usize = 4096;

/// Next pipe ID
static NEXT_PIPE_ID: AtomicU32 = AtomicU32::new(1);

/// Pipe buffer
pub struct Pipe {
    /// Pipe ID
    pub id: u32,
    /// Buffer
    buffer: VecDeque<u8>,
    /// Read end open
    read_open: bool,
    /// Write end open
    write_open: bool,
    /// Reader process
    reader_pid: Option<Pid>,
    /// Writer process
    writer_pid: Option<Pid>,
}

impl Pipe {
    /// Create new pipe
    pub fn new() -> Self {
        Pipe {
            id: NEXT_PIPE_ID.fetch_add(1, Ordering::SeqCst),
            buffer: VecDeque::with_capacity(PIPE_BUF_SIZE),
            read_open: true,
            write_open: true,
            reader_pid: None,
            writer_pid: None,
        }
    }
    
    /// Read from pipe
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, PipeError> {
        if !self.read_open {
            return Err(PipeError::Closed);
        }
        
        if self.buffer.is_empty() {
            if !self.write_open {
                return Ok(0); // EOF
            }
            return Err(PipeError::WouldBlock);
        }
        
        let count = core::cmp::min(buf.len(), self.buffer.len());
        for i in 0..count {
            buf[i] = self.buffer.pop_front().unwrap();
        }
        
        Ok(count)
    }
    
    /// Write to pipe
    pub fn write(&mut self, buf: &[u8]) -> Result<usize, PipeError> {
        if !self.write_open {
            return Err(PipeError::Closed);
        }
        
        let available = PIPE_BUF_SIZE - self.buffer.len();
        if available == 0 {
            return Err(PipeError::WouldBlock);
        }
        
        let count = core::cmp::min(buf.len(), available);
        for &byte in &buf[..count] {
            self.buffer.push_back(byte);
        }
        
        Ok(count)
    }
    
    /// Close read end
    pub fn close_read(&mut self) {
        self.read_open = false;
    }
    
    /// Close write end
    pub fn close_write(&mut self) {
        self.write_open = false;
        self.buffer.clear();
    }
    
    /// Get buffer usage
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

impl Default for Pipe {
    fn default() -> Self {
        Self::new()
    }
}

/// Pipe error
#[derive(Debug, Clone, Copy)]
pub enum PipeError {
    Closed,
    WouldBlock,
    BrokenPipe,
}

// ========== Message Queues ==========

/// Message queue ID
pub type MqId = u32;

/// Message queue message
#[derive(Debug, Clone)]
pub struct Message {
    /// Message type
    pub mtype: i64,
    /// Message data
    pub data: alloc::vec::Vec<u8>,
}

/// Message queue
pub struct MessageQueue {
    /// Queue ID
    pub id: MqId,
    /// Messages
    messages: VecDeque<Message>,
    /// Maximum messages
    max_msgs: usize,
    /// Maximum message size
    max_msgsize: usize,
    /// Creator PID
    creator: Pid,
}

impl MessageQueue {
    pub fn new(id: MqId, max_msgs: usize, max_msgsize: usize, creator: Pid) -> Self {
        MessageQueue {
            id,
            messages: VecDeque::new(),
            max_msgs,
            max_msgsize,
            creator,
        }
    }
    
    /// Send message
    pub fn send(&mut self, msg: Message) -> Result<(), IpcError> {
        if self.messages.len() >= self.max_msgs {
            return Err(IpcError::QueueFull);
        }
        
        if msg.data.len() > self.max_msgsize {
            return Err(IpcError::MsgTooBig);
        }
        
        self.messages.push_back(msg);
        Ok(())
    }
    
    /// Receive message
    pub fn recv(&mut self, mtype: i64) -> Option<Message> {
        if mtype == 0 {
            self.messages.pop_front()
        } else if mtype > 0 {
            // Find first message with exact type
            self.messages.iter().position(|m| m.mtype == mtype)
                .map(|idx| self.messages.remove(idx).unwrap())
        } else {
            // Find first message with type <= |mtype|
            let target = -mtype;
            self.messages.iter().position(|m| m.mtype <= target)
                .map(|idx| self.messages.remove(idx).unwrap())
        }
    }
}

/// IPC error
#[derive(Debug, Clone, Copy)]
pub enum IpcError {
    QueueFull,
    MsgTooBig,
    NotFound,
    Permission,
}

// ========== Shared Memory ==========

/// Shared memory ID
pub type ShmId = u32;

/// Shared memory segment
pub struct SharedMemory {
    /// Segment ID
    pub id: ShmId,
    /// Size in bytes
    pub size: usize,
    /// Physical address
    pub phys_addr: u64,
    /// Attached processes
    pub attached: Vec<Pid>,
    /// Creator
    pub creator: Pid,
}

impl SharedMemory {
    pub fn new(id: ShmId, size: usize, creator: Pid) -> Self {
        SharedMemory {
            id,
            size,
            phys_addr: 0, // Would be allocated
            attached: Vec::new(),
            creator,
        }
    }
    
    /// Attach process
    pub fn attach(&mut self, pid: Pid) {
        if !self.attached.contains(&pid) {
            self.attached.push(pid);
        }
    }
    
    /// Detach process
    pub fn detach(&mut self, pid: Pid) {
        self.attached.retain(|&p| p != pid);
    }
}

// ========== Global IPC State ==========

/// IPC state
pub struct IpcState {
    /// Named pipes (by ID)
    pub pipes: alloc::collections::BTreeMap<u32, Arc<Mutex<Pipe>>>,
    /// Message queues
    pub msg_queues: alloc::collections::BTreeMap<MqId, MessageQueue>,
    /// Shared memory segments
    pub shm_segments: alloc::collections::BTreeMap<ShmId, SharedMemory>,
    /// Next IDs
    next_mq_id: AtomicU32,
    next_shm_id: AtomicU32,
}

impl IpcState {
    pub fn new() -> Self {
        IpcState {
            pipes: alloc::collections::BTreeMap::new(),
            msg_queues: alloc::collections::BTreeMap::new(),
            shm_segments: alloc::collections::BTreeMap::new(),
            next_mq_id: AtomicU32::new(1),
            next_shm_id: AtomicU32::new(1),
        }
    }
    
    /// Create pipe
    pub fn create_pipe(&mut self) -> u32 {
        let pipe = Arc::new(Mutex::new(Pipe::new()));
        let id = pipe.lock().id;
        self.pipes.insert(id, pipe);
        id
    }
    
    /// Create message queue
    pub fn create_mq(&mut self, max_msgs: usize, max_msgsize: usize, creator: Pid) -> MqId {
        let id = self.next_mq_id.fetch_add(1, Ordering::SeqCst);
        let mq = MessageQueue::new(id, max_msgs, max_msgsize, creator);
        self.msg_queues.insert(id, mq);
        id
    }
    
    /// Create shared memory
    pub fn create_shm(&mut self, size: usize, creator: Pid) -> ShmId {
        let id = self.next_shm_id.fetch_add(1, Ordering::SeqCst);
        let shm = SharedMemory::new(id, size, creator);
        self.shm_segments.insert(id, shm);
        id
    }
}

impl Default for IpcState {
    fn default() -> Self {
        Self::new()
    }
}

/// Global IPC state
pub static IPC_STATE: Mutex<IpcState> = Mutex::new(IpcState {
    pipes: alloc::collections::BTreeMap::new(),
    msg_queues: alloc::collections::BTreeMap::new(),
    shm_segments: alloc::collections::BTreeMap::new(),
    next_mq_id: AtomicU32::new(1),
    next_shm_id: AtomicU32::new(1),
});

/// Initialize IPC
pub fn init() {
    serial_println!("[OK] IPC initialized");
}