// Thread Synchronization Primitives
//
// This module provides thread synchronization primitives for VantisOS, including:
// - Mutex (Mutual Exclusion)
// - Semaphore
// - Condition Variable
// - Read-Write Lock

#![no_std]

use crate::verified::minimal_kernel::thread::{ThreadControlBlock, ThreadState};
use crate::verified::minimal_kernel::thread_manager::{ThreadManager, ThreadId};
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

/// Mutex (Mutual Exclusion)
pub struct Mutex {
    /// Locked flag
    locked: AtomicBool,
    /// Owner thread ID
    owner: AtomicU32,
    /// Lock count (for recursive locks)
    lock_count: AtomicU32,
    /// Wait queue
    wait_queue: [Option<ThreadId>; 64],
    /// Wait queue count
    wait_count: AtomicU32,
}

impl Mutex {
    /// Create a new mutex
    pub const fn new() -> Self {
        Mutex {
            locked: AtomicBool::new(false),
            owner: AtomicU32::new(0),
            lock_count: AtomicU32::new(0),
            wait_queue: [None; 64],
            wait_count: AtomicU32::new(0),
        }
    }

    /// Lock the mutex
    pub fn lock(&mut self, thread_manager: &mut ThreadManager) -> bool {
        let tid = thread_manager.get_current_thread()
            .map(|t| t.get_tid())
            .unwrap_or(0);

        // Check if already owned by current thread (recursive lock)
        if self.owner.load(Ordering::SeqCst) == tid {
            self.lock_count.fetch_add(1, Ordering::SeqCst);
            return true;
        }

        // Try to acquire lock
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // Add to wait queue
            if self.wait_count.load(Ordering::SeqCst) < 64 {
                for i in 0..64 {
                    if self.wait_queue[i].is_none() {
                        self.wait_queue[i] = Some(tid);
                        self.wait_count.fetch_add(1, Ordering::SeqCst);
                        break;
                    }
                }
            }

            // Set thread state to blocked
            if let Some(thread) = thread_manager.get_thread_mut(tid) {
                thread.set_state(ThreadState::Blocked);
            }

            // Yield to scheduler
            // TODO: Call scheduler to switch threads
        }

        // Acquired lock
        self.owner.store(tid, Ordering::SeqCst);
        self.lock_count.store(1, Ordering::SeqCst);
        true
    }

    /// Try to lock the mutex (non-blocking)
    pub fn try_lock(&mut self, thread_manager: &ThreadManager) -> bool {
        let tid = thread_manager.get_current_thread()
            .map(|t| t.get_tid())
            .unwrap_or(0);

        // Check if already owned by current thread (recursive lock)
        if self.owner.load(Ordering::SeqCst) == tid {
            self.lock_count.fetch_add(1, Ordering::SeqCst);
            return true;
        }

        // Try to acquire lock
        if self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            self.owner.store(tid, Ordering::SeqCst);
            self.lock_count.store(1, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    /// Unlock the mutex
    pub fn unlock(&mut self, thread_manager: &mut ThreadManager) -> bool {
        let tid = thread_manager.get_current_thread()
            .map(|t| t.get_tid())
            .unwrap_or(0);

        // Check if owned by current thread
        if self.owner.load(Ordering::SeqCst) != tid {
            return false;
        }

        // Decrement lock count
        let count = self.lock_count.fetch_sub(1, Ordering::SeqCst) - 1;

        if count == 0 {
            // Release lock
            self.locked.store(false, Ordering::Release);
            self.owner.store(0, Ordering::SeqCst);

            // Wake up next thread in wait queue
            if self.wait_count.load(Ordering::SeqCst) > 0 {
                for i in 0..64 {
                    if let Some(wait_tid) = self.wait_queue[i] {
                        self.wait_queue[i] = None;
                        self.wait_count.fetch_sub(1, Ordering::SeqCst);

                        // Set thread state to ready
                        if let Some(thread) = thread_manager.get_thread_mut(wait_tid) {
                            thread.set_state(ThreadState::Ready);
                        }
                        break;
                    }
                }
            }
        }

        true
    }

    /// Check if mutex is locked
    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::SeqCst)
    }

    /// Get owner thread ID
    pub fn get_owner(&self) -> ThreadId {
        self.owner.load(Ordering::SeqCst)
    }
}

impl Default for Mutex {
    fn default() -> Self {
        Self::new()
    }
}

/// Semaphore
pub struct Semaphore {
    /// Count
    count: AtomicU32,
    /// Maximum count
    max_count: u32,
    /// Wait queue
    wait_queue: [Option<ThreadId>; 64],
    /// Wait queue count
    wait_count: AtomicU32,
}

impl Semaphore {
    /// Create a new semaphore
    pub const fn new(initial_count: u32, max_count: u32) -> Self {
        Semaphore {
            count: AtomicU32::new(initial_count),
            max_count,
            wait_queue: [None; 64],
            wait_count: AtomicU32::new(0),
        }
    }

    /// Wait (acquire) semaphore
    pub fn wait(&mut self, thread_manager: &mut ThreadManager) -> bool {
        let tid = thread_manager.get_current_thread()
            .map(|t| t.get_tid())
            .unwrap_or(0);

        // Try to decrement count
        while self.count.fetch_sub(1, Ordering::Acquire) == 0 {
            // Add to wait queue
            if self.wait_count.load(Ordering::SeqCst) < 64 {
                for i in 0..64 {
                    if self.wait_queue[i].is_none() {
                        self.wait_queue[i] = Some(tid);
                        self.wait_count.fetch_add(1, Ordering::SeqCst);
                        break;
                    }
                }
            }

            // Set thread state to blocked
            if let Some(thread) = thread_manager.get_thread_mut(tid) {
                thread.set_state(ThreadState::Blocked);
            }

            // Yield to scheduler
            // TODO: Call scheduler to switch threads
        }

        true
    }

    /// Signal (release) semaphore
    pub fn signal(&mut self, thread_manager: &mut ThreadManager) -> bool {
        // Increment count
        let old_count = self.count.fetch_add(1, Ordering::Release);

        // Check if count exceeds maximum
        if old_count >= self.max_count {
            self.count.fetch_sub(1, Ordering::Release);
            return false;
        }

        // Wake up next thread in wait queue
        if self.wait_count.load(Ordering::SeqCst) > 0 {
            for i in 0..64 {
                if let Some(wait_tid) = self.wait_queue[i] {
                    self.wait_queue[i] = None;
                    self.wait_count.fetch_sub(1, Ordering::SeqCst);

                    // Set thread state to ready
                    if let Some(thread) = thread_manager.get_thread_mut(wait_tid) {
                        thread.set_state(ThreadState::Ready);
                    }
                    break;
                }
            }
        }

        true
    }

    /// Try to wait (non-blocking)
    pub fn try_wait(&mut self) -> bool {
        self.count.fetch_update(Ordering::Acquire, Ordering::Relaxed, |c| {
            if c > 0 {
                Some(c - 1)
            } else {
                None
            }
        }).is_ok()
    }

    /// Get current count
    pub fn get_count(&self) -> u32 {
        self.count.load(Ordering::SeqCst)
    }
}

/// Condition Variable
pub struct ConditionVariable {
    /// Wait queue
    wait_queue: [Option<ThreadId>; 64],
    /// Wait queue count
    wait_count: AtomicU32,
}

impl ConditionVariable {
    /// Create a new condition variable
    pub const fn new() -> Self {
        ConditionVariable {
            wait_queue: [None; 64],
            wait_count: AtomicU32::new(0),
        }
    }

    /// Wait on condition variable
    pub fn wait(&mut self, mutex: &mut Mutex, thread_manager: &mut ThreadManager) -> bool {
        let tid = thread_manager.get_current_thread()
            .map(|t| t.get_tid())
            .unwrap_or(0);

        // Add to wait queue
        if self.wait_count.load(Ordering::SeqCst) < 64 {
            for i in 0..64 {
                if self.wait_queue[i].is_none() {
                    self.wait_queue[i] = Some(tid);
                    self.wait_count.fetch_add(1, Ordering::SeqCst);
                    break;
                }
            }
        }

        // Release mutex
        mutex.unlock(thread_manager);

        // Set thread state to blocked
        if let Some(thread) = thread_manager.get_thread_mut(tid) {
            thread.set_state(ThreadState::Blocked);
        }

        // Yield to scheduler
        // TODO: Call scheduler to switch threads

        // Reacquire mutex when woken
        mutex.lock(thread_manager)
    }

    /// Signal one waiting thread
    pub fn signal(&mut self, thread_manager: &mut ThreadManager) -> bool {
        if self.wait_count.load(Ordering::SeqCst) > 0 {
            for i in 0..64 {
                if let Some(wait_tid) = self.wait_queue[i] {
                    self.wait_queue[i] = None;
                    self.wait_count.fetch_sub(1, Ordering::SeqCst);

                    // Set thread state to ready
                    if let Some(thread) = thread_manager.get_thread_mut(wait_tid) {
                        thread.set_state(ThreadState::Ready);
                    }
                    return true;
                }
            }
        }
        false
    }

    /// Signal all waiting threads
    pub fn broadcast(&mut self, thread_manager: &mut ThreadManager) -> usize {
        let mut count = 0;

        while self.wait_count.load(Ordering::SeqCst) > 0 {
            for i in 0..64 {
                if let Some(wait_tid) = self.wait_queue[i] {
                    self.wait_queue[i] = None;
                    self.wait_count.fetch_sub(1, Ordering::SeqCst);

                    // Set thread state to ready
                    if let Some(thread) = thread_manager.get_thread_mut(wait_tid) {
                        thread.set_state(ThreadState::Ready);
                    }
                    count += 1;
                    break;
                }
            }
        }

        count
    }
}

impl Default for ConditionVariable {
    fn default() -> Self {
        Self::new()
    }
}

/// Read-Write Lock
pub struct RwLock {
    /// Read count
    read_count: AtomicU32,
    /// Write flag
    write_flag: AtomicBool,
    /// Write owner
    write_owner: AtomicU32,
    /// Read wait queue
    read_wait_queue: [Option<ThreadId>; 64],
    /// Write wait queue
    write_wait_queue: [Option<ThreadId>; 64],
    /// Read wait count
    read_wait_count: AtomicU32,
    /// Write wait count
    write_wait_count: AtomicU32,
}

impl RwLock {
    /// Create a new read-write lock
    pub const fn new() -> Self {
        RwLock {
            read_count: AtomicU32::new(0),
            write_flag: AtomicBool::new(false),
            write_owner: AtomicU32::new(0),
            read_wait_queue: [None; 64],
            write_wait_queue: [None; 64],
            read_wait_count: AtomicU32::new(0),
            write_wait_count: AtomicU32::new(0),
        }
    }

    /// Acquire read lock
    pub fn read_lock(&mut self, thread_manager: &mut ThreadManager) -> bool {
        let tid = thread_manager.get_current_thread()
            .map(|t| t.get_tid())
            .unwrap_or(0);

        // Wait if write lock is held or write waiters exist
        while self.write_flag.load(Ordering::SeqCst) || self.write_wait_count.load(Ordering::SeqCst) > 0 {
            // Add to read wait queue
            if self.read_wait_count.load(Ordering::SeqCst) < 64 {
                for i in 0..64 {
                    if self.read_wait_queue[i].is_none() {
                        self.read_wait_queue[i] = Some(tid);
                        self.read_wait_count.fetch_add(1, Ordering::SeqCst);
                        break;
                    }
                }
            }

            // Set thread state to blocked
            if let Some(thread) = thread_manager.get_thread_mut(tid) {
                thread.set_state(ThreadState::Blocked);
            }

            // Yield to scheduler
            // TODO: Call scheduler to switch threads
        }

        // Acquire read lock
        self.read_count.fetch_add(1, Ordering::Acquire);
        true
    }

    /// Release read lock
    pub fn read_unlock(&mut self, thread_manager: &mut ThreadManager) -> bool {
        let old_count = self.read_count.fetch_sub(1, Ordering::Release);

        if old_count == 0 {
            return false;
        }

        // Wake up write waiter if no more readers
        if old_count == 1 && self.write_wait_count.load(Ordering::SeqCst) > 0 {
            for i in 0..64 {
                if let Some(wait_tid) = self.write_wait_queue[i] {
                    self.write_wait_queue[i] = None;
                    self.write_wait_count.fetch_sub(1, Ordering::SeqCst);

                    // Set thread state to ready
                    if let Some(thread) = thread_manager.get_thread_mut(wait_tid) {
                        thread.set_state(ThreadState::Ready);
                    }
                    break;
                }
            }
        }

        true
    }

    /// Acquire write lock
    pub fn write_lock(&mut self, thread_manager: &mut ThreadManager) -> bool {
        let tid = thread_manager.get_current_thread()
            .map(|t| t.get_tid())
            .unwrap_or(0);

        // Wait if write lock is held or readers exist
        while self.write_flag.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err()
            || self.read_count.load(Ordering::SeqCst) > 0
        {
            // Add to write wait queue
            if self.write_wait_count.load(Ordering::SeqCst) < 64 {
                for i in 0..64 {
                    if self.write_wait_queue[i].is_none() {
                        self.write_wait_queue[i] = Some(tid);
                        self.write_wait_count.fetch_add(1, Ordering::SeqCst);
                        break;
                    }
                }
            }

            // Set thread state to blocked
            if let Some(thread) = thread_manager.get_thread_mut(tid) {
                thread.set_state(ThreadState::Blocked);
            }

            // Yield to scheduler
            // TODO: Call scheduler to switch threads
        }

        // Acquire write lock
        self.write_owner.store(tid, Ordering::SeqCst);
        true
    }

    /// Release write lock
    pub fn write_unlock(&mut self, thread_manager: &mut ThreadManager) -> bool {
        let tid = thread_manager.get_current_thread()
            .map(|t| t.get_tid())
            .unwrap_or(0);

        // Check if owned by current thread
        if self.write_owner.load(Ordering::SeqCst) != tid {
            return false;
        }

        // Release write lock
        self.write_flag.store(false, Ordering::Release);
        self.write_owner.store(0, Ordering::SeqCst);

        // Wake up all read waiters
        while self.read_wait_count.load(Ordering::SeqCst) > 0 {
            for i in 0..64 {
                if let Some(wait_tid) = self.read_wait_queue[i] {
                    self.read_wait_queue[i] = None;
                    self.read_wait_count.fetch_sub(1, Ordering::SeqCst);

                    // Set thread state to ready
                    if let Some(thread) = thread_manager.get_thread_mut(wait_tid) {
                        thread.set_state(ThreadState::Ready);
                    }
                    break;
                }
            }
        }

        // Wake up one write waiter
        if self.write_wait_count.load(Ordering::SeqCst) > 0 {
            for i in 0..64 {
                if let Some(wait_tid) = self.write_wait_queue[i] {
                    self.write_wait_queue[i] = None;
                    self.write_wait_count.fetch_sub(1, Ordering::SeqCst);

                    // Set thread state to ready
                    if let Some(thread) = thread_manager.get_thread_mut(wait_tid) {
                        thread.set_state(ThreadState::Ready);
                    }
                    break;
                }
            }
        }

        true
    }

    /// Get read count
    pub fn get_read_count(&self) -> u32 {
        self.read_count.load(Ordering::SeqCst)
    }

    /// Check if write locked
    pub fn is_write_locked(&self) -> bool {
        self.write_flag.load(Ordering::SeqCst)
    }
}

impl Default for RwLock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutex_creation() {
        let mutex = Mutex::new();
        assert!(!mutex.is_locked());
        assert_eq!(mutex.get_owner(), 0);
    }

    #[test]
    fn test_semaphore_creation() {
        let semaphore = Semaphore::new(5, 10);
        assert_eq!(semaphore.get_count(), 5);
    }

    #[test]
    fn test_condition_variable_creation() {
        let cv = ConditionVariable::new();
        // Condition variable created successfully
    }

    #[test]
    fn test_rwlock_creation() {
        let rwlock = RwLock::new();
        assert_eq!(rwlock.get_read_count(), 0);
        assert!(!rwlock.is_write_locked());
    }
}