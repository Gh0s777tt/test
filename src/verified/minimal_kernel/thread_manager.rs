// Thread Manager
//
// This module provides thread management for VantisOS, including:
// - Thread creation and termination
// - Thread scheduling
// - Thread state management
// - Thread synchronization

#![no_std]

use crate::verified::minimal_kernel::thread::{ThreadControlBlock, ThreadState, ThreadPriority};
use crate::verified::minimal_kernel::process::process_manager::ProcessManager;
use crate::verified::minimal_kernel::memory::page_alloc::PageAllocator;
use core::sync::atomic::{AtomicU32, Ordering};

/// Thread ID type
pub type ThreadId = u32;

/// Invalid thread ID
pub const INVALID_TID: ThreadId = 0;

/// Maximum number of threads
pub const MAX_THREADS: usize = 4096;

/// Thread manager
pub struct ThreadManager {
    /// Thread table
    threads: [Option<ThreadControlBlock>; MAX_THREADS],
    /// Next thread ID
    next_tid: AtomicU32,
    /// Current thread
    current_thread: AtomicU32,
    /// Thread count
    thread_count: AtomicU32,
}

impl ThreadManager {
    /// Create a new thread manager
    pub fn new() -> Self {
        ThreadManager {
            threads: [None; MAX_THREADS],
            next_tid: AtomicU32::new(1),
            current_thread: AtomicU32::new(0),
            thread_count: AtomicU32::new(0),
        }
    }

    /// Create a new thread
    pub fn create_thread(
        &mut self,
        pid: u32,
        entry_point: u64,
        stack_pointer: u64,
        priority: ThreadPriority,
        page_allocator: &mut PageAllocator,
    ) -> Option<ThreadId> {
        // Allocate thread ID
        let tid = self.next_tid.fetch_add(1, Ordering::SeqCst);
        if tid == INVALID_TID {
            return None;
        }

        // Find free slot in thread table
        let slot = self.find_free_slot()?;
        if slot >= MAX_THREADS {
            return None;
        }

        // Allocate stack for thread
        let stack_size = 64 * 1024; // 64 KB default stack
        let stack_base = page_allocator.allocate_pages(stack_size / 4096)?;

        // Create thread control block
        let tcb = ThreadControlBlock::new(
            tid,
            pid,
            entry_point,
            stack_base,
            stack_size,
            stack_pointer,
            priority,
        );

        // Add to thread table
        self.threads[slot] = Some(tcb);
        self.thread_count.fetch_add(1, Ordering::SeqCst);

        Some(tid)
    }

    /// Terminate a thread
    pub fn terminate_thread(&mut self, tid: ThreadId, exit_code: i32) -> bool {
        // Find thread
        let slot = self.find_thread_slot(tid)?;
        let thread = self.threads[slot].as_mut()?;

        // Set exit code
        thread.set_exit_code(exit_code);

        // Set state to terminated
        thread.set_state(ThreadState::Terminated);

        // Free thread stack
        // TODO: Free stack pages

        // Remove from thread table
        self.threads[slot] = None;
        self.thread_count.fetch_sub(1, Ordering::SeqCst);

        true
    }

    /// Get thread by ID
    pub fn get_thread(&self, tid: ThreadId) -> Option<&ThreadControlBlock> {
        let slot = self.find_thread_slot(tid)?;
        self.threads[slot].as_ref()
    }

    /// Get thread by ID (mutable)
    pub fn get_thread_mut(&mut self, tid: ThreadId) -> Option<&mut ThreadControlBlock> {
        let slot = self.find_thread_slot(tid)?;
        self.threads[slot].as_mut()
    }

    /// Get current thread
    pub fn get_current_thread(&self) -> Option<&ThreadControlBlock> {
        let tid = self.current_thread.load(Ordering::SeqCst);
        self.get_thread(tid)
    }

    /// Get current thread (mutable)
    pub fn get_current_thread_mut(&mut self) -> Option<&mut ThreadControlBlock> {
        let tid = self.current_thread.load(Ordering::SeqCst);
        self.get_thread_mut(tid)
    }

    /// Set current thread
    pub fn set_current_thread(&self, tid: ThreadId) {
        self.current_thread.store(tid, Ordering::SeqCst);
    }

    /// Get thread count
    pub fn get_thread_count(&self) -> usize {
        self.thread_count.load(Ordering::SeqCst)
    }

    /// Get all threads
    pub fn get_threads(&self) -> &[Option<ThreadControlBlock>; MAX_THREADS] {
        &self.threads
    }

    /// Find threads by process ID
    pub fn find_threads_by_pid(&self, pid: u32) -> Vec<ThreadId> {
        let mut threads = Vec::new();
        for i in 0..MAX_THREADS {
            if let Some(thread) = &self.threads[i] {
                if thread.get_pid() == pid {
                    threads.push(thread.get_tid());
                }
            }
        }
        threads
    }

    /// Get thread state
    pub fn get_thread_state(&self, tid: ThreadId) -> Option<ThreadState> {
        self.get_thread(tid).map(|t| t.get_state())
    }

    /// Set thread state
    pub fn set_thread_state(&mut self, tid: ThreadId, state: ThreadState) -> bool {
        if let Some(thread) = self.get_thread_mut(tid) {
            thread.set_state(state);
            true
        } else {
            false
        }
    }

    /// Get thread priority
    pub fn get_thread_priority(&self, tid: ThreadId) -> Option<ThreadPriority> {
        self.get_thread(tid).map(|t| t.get_priority())
    }

    /// Set thread priority
    pub fn set_thread_priority(&mut self, tid: ThreadId, priority: ThreadPriority) -> bool {
        if let Some(thread) = self.get_thread_mut(tid) {
            thread.set_priority(priority);
            true
        } else {
            false
        }
    }

    /// Find free slot in thread table
    fn find_free_slot(&self) -> Option<usize> {
        for i in 0..MAX_THREADS {
            if self.threads[i].is_none() {
                return Some(i);
            }
        }
        None
    }

    /// Find thread slot by TID
    fn find_thread_slot(&self, tid: ThreadId) -> Option<usize> {
        for i in 0..MAX_THREADS {
            if let Some(thread) = &self.threads[i] {
                if thread.get_tid() == tid {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Print thread list
    pub fn print_thread_list(&self) {
        // TODO: Implement printing when console is available
        // For now, this is a placeholder
    }

    /// Get thread statistics
    pub fn get_thread_statistics(&self) -> ThreadStatistics {
        let mut stats = ThreadStatistics::new();
        
        for i in 0..MAX_THREADS {
            if let Some(thread) = &self.threads[i] {
                stats.total_threads += 1;
                
                match thread.get_state() {
                    ThreadState::Running => stats.running_threads += 1,
                    ThreadState::Ready => stats.ready_threads += 1,
                    ThreadState::Blocked => stats.blocked_threads += 1,
                    ThreadState::Terminated => stats.terminated_threads += 1,
                }
            }
        }
        
        stats
    }
}

impl Default for ThreadManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread statistics
#[derive(Debug, Clone, Copy)]
pub struct ThreadStatistics {
    /// Total threads
    pub total_threads: usize,
    /// Running threads
    pub running_threads: usize,
    /// Ready threads
    pub ready_threads: usize,
    /// Blocked threads
    pub blocked_threads: usize,
    /// Terminated threads
    pub terminated_threads: usize,
}

impl ThreadStatistics {
    pub fn new() -> Self {
        ThreadStatistics {
            total_threads: 0,
            running_threads: 0,
            ready_threads: 0,
            blocked_threads: 0,
            terminated_threads: 0,
        }
    }
}

impl Default for ThreadStatistics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_manager_creation() {
        let manager = ThreadManager::new();
        assert_eq!(manager.get_thread_count(), 0);
        assert_eq!(manager.get_current_thread().is_none(), true);
    }

    #[test]
    fn test_thread_creation() {
        let mut manager = ThreadManager::new();
        let mut page_allocator = PageAllocator::new(1024 * 1024 * 1024);
        
        let tid = manager.create_thread(
            1,
            0x1000,
            0x2000,
            ThreadPriority::Normal,
            &mut page_allocator,
        );
        
        assert!(tid.is_some());
        assert_eq!(manager.get_thread_count(), 1);
    }

    #[test]
    fn test_thread_termination() {
        let mut manager = ThreadManager::new();
        let mut page_allocator = PageAllocator::new(1024 * 1024 * 1024);
        
        let tid = manager.create_thread(
            1,
            0x1000,
            0x2000,
            ThreadPriority::Normal,
            &mut page_allocator,
        ).unwrap();
        
        assert!(manager.terminate_thread(tid, 0));
        assert_eq!(manager.get_thread_count(), 0);
    }

    #[test]
    fn test_thread_statistics() {
        let mut manager = ThreadManager::new();
        let mut page_allocator = PageAllocator::new(1024 * 1024 * 1024);
        
        let _tid = manager.create_thread(
            1,
            0x1000,
            0x2000,
            ThreadPriority::Normal,
            &mut page_allocator,
        );
        
        let stats = manager.get_thread_statistics();
        assert_eq!(stats.total_threads, 1);
    }
}