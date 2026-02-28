//! # Synchronization Primitives
//!
//! This module provides synchronization primitives for the minimal kernel.

use super::Tid;

/// Mutex
pub struct Mutex {
    /// Locked flag
    locked: bool,
    /// Owner thread ID
    owner: Option<Tid>,
}

impl Mutex {
    /// Create new mutex
    pub fn new() -> Self {
        Self {
            locked: false,
            owner: None,
        }
    }
    
    /// Lock mutex
    pub fn lock(&mut self, tid: Tid) -> Result<(), ()> {
        if self.locked {
            Err(())
        } else {
            self.locked = true;
            self.owner = Some(tid);
            Ok(())
        }
    }
    
    /// Unlock mutex
    pub fn unlock(&mut self, tid: Tid) -> Result<(), ()> {
        if !self.locked {
            Err(())
        } else if self.owner != Some(tid) {
            Err(())
        } else {
            self.locked = false;
            self.owner = None;
            Ok(())
        }
    }
    
    /// Check if mutex is locked
    pub fn is_locked(&self) -> bool {
        self.locked
    }
}

/// Semaphore
pub struct Semaphore {
    /// Count
    count: isize,
    /// Maximum count
    max_count: isize,
}

impl Semaphore {
    /// Create new semaphore
    pub fn new(initial_count: isize, max_count: isize) -> Self {
        Self {
            count: initial_count,
            max_count,
        }
    }
    
    /// Wait (acquire) semaphore
    pub fn wait(&mut self) -> Result<(), ()> {
        if self.count > 0 {
            self.count -= 1;
            Ok(())
        } else {
            Err(())
        }
    }
    
    /// Signal (release) semaphore
    pub fn signal(&mut self) -> Result<(), ()> {
        if self.count < self.max_count {
            self.count += 1;
            Ok(())
        } else {
            Err(())
        }
    }
    
    /// Get current count
    pub fn count(&self) -> isize {
        self.count
    }
}

/// Condition variable
pub struct CondVar {
    /// Waiting threads
    waiting_threads: Vec<Tid>,
}

impl CondVar {
    /// Create new condition variable
    pub fn new() -> Self {
        Self {
            waiting_threads: Vec::new(),
        }
    }
    
    /// Wait on condition variable
    pub fn wait(&mut self, tid: Tid, mutex: &mut Mutex) {
        // Release mutex
        let _ = mutex.unlock(tid);
        
        // Add to waiting list
        self.waiting_threads.push(tid);
    }
    
    /// Signal one waiting thread
    pub fn signal(&mut self) -> Option<Tid> {
        self.waiting_threads.pop()
    }
    
    /// Signal all waiting threads
    pub fn broadcast(&mut self) -> Vec<Tid> {
        let mut threads = Vec::new();
        while let Some(tid) = self.waiting_threads.pop() {
            threads.push(tid);
        }
        threads
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mutex() {
        let mut mutex = Mutex::new();
        
        // Lock mutex
        assert!(mutex.lock(1).is_ok());
        assert!(mutex.is_locked());
        
        // Try to lock again - should fail
        assert!(mutex.lock(2).is_err());
        
        // Unlock mutex
        assert!(mutex.unlock(1).is_ok());
        assert!(!mutex.is_locked());
    }
    
    #[test]
    fn test_semaphore() {
        let mut semaphore = Semaphore::new(2, 2);
        
        // Wait twice
        assert!(semaphore.wait().is_ok());
        assert!(semaphore.wait().is_ok());
        assert_eq!(semaphore.count(), 0);
        
        // Try to wait again - should fail
        assert!(semaphore.wait().is_err());
        
        // Signal
        assert!(semaphore.signal().is_ok());
        assert_eq!(semaphore.count(), 1);
    }
    
    #[test]
    fn test_condvar() {
        let mut condvar = CondVar::new();
        let mut mutex = Mutex::new();
        
        // Wait
        condvar.wait(1, &mut mutex);
        
        // Signal
        let tid = condvar.signal().unwrap();
        assert_eq!(tid, 1);
    }
}