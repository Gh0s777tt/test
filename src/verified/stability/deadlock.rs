//! Deadlock Prevention Module
//! 
//! This module provides deadlock detection and prevention mechanisms
//! including lock ordering enforcement and deadlock detection algorithms.

use alloc::collections::BTreeSet;
use alloc::sync::Arc;
use spin::Mutex;

/// Deadlock type
#[derive(Debug, Clone, Copy)]
pub enum DeadlockType {
    LockCycle,
    ResourceWaitCycle,
    PotentialDeadlock,
}

/// Deadlock information
#[derive(Debug, Clone)]
pub struct DeadlockInfo {
    pub deadlock_type: DeadlockType,
    pub involved_threads: Vec<usize>,
    pub involved_locks: Vec<usize>,
    pub cycle_detected: bool,
    pub timestamp: u64,
}

/// Lock ordering node
#[derive(Debug, Clone)]
struct LockNode {
    lock_address: usize,
    outgoing: BTreeSet<usize>,
    incoming: BTreeSet<usize>,
}

/// Deadlock detector
pub struct DeadlockDetector {
    lock_graph: Arc<Mutex<BTreeMap<usize, LockNode>>>,
    detected_deadlocks: Arc<Mutex<Vec<DeadlockInfo>>>,
    enabled: Arc<Mutex<bool>>,
}

impl DeadlockDetector {
    /// Create a new deadlock detector
    pub fn new() -> Self {
        Self {
            lock_graph: Arc::new(Mutex::new(BTreeMap::new())),
            detected_deadlocks: Arc::new(Mutex::new(Vec::new())),
            enabled: Arc::new(Mutex::new(true)),
        }
    }

    /// Enable or disable deadlock detection
    pub fn set_enabled(&self, enabled: bool) {
        *self.enabled.lock() = enabled;
    }

    /// Track a lock acquisition
    pub fn track_lock_acquire(&self, lock_address: usize, thread_id: usize, held_locks: &[usize]) {
        if !*self.enabled.lock() {
            return;
        }

        // Create lock dependencies
        for held_lock in held_locks {
            let mut graph = self.lock_graph.lock();
            
            // Create nodes if they don't exist
            graph.entry(*held_lock).or_insert_with(|| LockNode {
                lock_address: *held_lock,
                outgoing: BTreeSet::new(),
                incoming: BTreeSet::new(),
            });
            
            graph.entry(lock_address).or_insert_with(|| LockNode {
                lock_address,
                outgoing: BTreeSet::new(),
                incoming: BTreeSet::new(),
            });
            
            // Create dependency: held_lock -> lock_address
            graph.get_mut(&held_lock).unwrap().outgoing.insert(lock_address);
            graph.get_mut(&lock_address).unwrap().incoming.insert(*held_lock);
            
            // Check for cycles
            if self.detect_cycle(&graph, *held_lock, lock_address) {
                let cycle = self.extract_cycle(&graph, *held_lock, lock_address);
                self.record_deadlock(DeadlockInfo {
                    deadlock_type: DeadlockType::LockCycle,
                    involved_threads: vec![thread_id],
                    involved_locks: cycle,
                    cycle_detected: true,
                    timestamp: self.current_timestamp(),
                });
            }
        }
    }

    /// Track a lock release
    pub fn track_lock_release(&self, lock_address: usize) {
        if !*self.enabled.lock() {
            return;
        }

        let mut graph = self.lock_graph.lock();
        
        if let Some(node) = graph.remove(&lock_address) {
            // Remove all dependencies
            for outgoing in node.outgoing {
                if let Some(target) = graph.get_mut(&outgoing) {
                    target.incoming.remove(&lock_address);
                }
            }
            
            for incoming in node.incoming {
                if let Some(source) = graph.get_mut(&incoming) {
                    source.outgoing.remove(&lock_address);
                }
            }
        }
    }

    /// Detect cycles in the lock graph
    fn detect_cycle(&self, graph: &BTreeMap<usize, LockNode>, start: usize, target: usize) -> bool {
        let mut visited = BTreeSet::new();
        self.detect_cycle_recursive(graph, start, target, &mut visited)
    }

    fn detect_cycle_recursive(
        &self,
        graph: &BTreeMap<usize, LockNode>,
        current: usize,
        target: usize,
        visited: &mut BTreeSet<usize>,
    ) -> bool {
        if current == target {
            return true;
        }
        
        if visited.contains(&current) {
            return false;
        }
        
        visited.insert(current);
        
        if let Some(node) = graph.get(&current) {
            for &next in node.outgoing.iter() {
                if self.detect_cycle_recursive(graph, next, target, visited) {
                    return true;
                }
            }
        }
        
        false
    }

    /// Extract cycle from the lock graph
    fn extract_cycle(&self, graph: &BTreeMap<usize, LockNode>, start: usize, target: usize) -> Vec<usize> {
        let mut cycle = Vec::new();
        let mut current = start;
        
        cycle.push(current);
        
        while current != target {
            if let Some(node) = graph.get(&current) {
                if let Some(&next) = node.outgoing.iter().next() {
                    cycle.push(next);
                    current = next;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        cycle
    }

    /// Record a deadlock
    fn record_deadlock(&self, deadlock: DeadlockInfo) {
        let mut deadlocks = self.detected_deadlocks.lock();
        
        // Avoid duplicates
        for existing in deadlocks.iter() {
            if existing.deadlock_type == deadlock.deadlock_type
                && existing.involved_locks == deadlock.involved_locks
            {
                return;
            }
        }
        
        deadlocks.push(deadlock);
    }

    /// Get detected deadlocks
    pub fn detected_deadlocks(&self) -> Vec<DeadlockInfo> {
        self.detected_deadlocks.lock().clone()
    }

    /// Clear detected deadlocks
    pub fn clear(&self) {
        self.detected_deadlocks.lock().clear();
        self.lock_graph.lock().clear();
    }

    /// Get statistics
    pub fn statistics(&self) -> DeadlockDetectionStatistics {
        let deadlocks = self.detected_deadlocks.lock();
        
        DeadlockDetectionStatistics {
            total_deadlocks: deadlocks.len(),
            lock_cycles: deadlocks.iter()
                .filter(|d| d.deadlock_type == DeadlockType::LockCycle)
                .count(),
        }
    }

    /// Get current timestamp (placeholder)
    fn current_timestamp(&self) -> u64 {
        // In a real implementation, this would return actual time
        0
    }
}

impl Default for DeadlockDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Deadlock detection statistics
#[derive(Debug, Clone, Copy)]
pub struct DeadlockDetectionStatistics {
    pub total_deadlocks: usize,
    pub lock_cycles: usize,
}

/// Deadlock prevention strategy
#[derive(Debug, Clone, Copy)]
pub enum DeadlockPreventionStrategy {
    LockOrdering,
    LockTimeout,
    LockHierarchy,
    NoPrevention,
}

/// Deadlock prevention configuration
#[derive(Debug, Clone)]
pub struct DeadlockPreventionConfig {
    pub strategy: DeadlockPreventionStrategy,
    pub lock_timeout_seconds: Option<u64>,
    pub detection_interval_ms: u64,
}

impl Default for DeadlockPreventionConfig {
    fn default() -> Self {
        Self {
            strategy: DeadlockPreventionStrategy::LockOrdering,
            lock_timeout_seconds: None,
            detection_interval_ms: 100,
        }
    }
}

/// Deadlock prevention manager
pub struct DeadlockPreventionManager {
    detector: DeadlockDetector,
    config: DeadlockPreventionConfig,
}

impl DeadlockPreventionManager {
    /// Create a new deadlock prevention manager
    pub fn new(config: DeadlockPreventionConfig) -> Self {
        Self {
            detector: DeadlockDetector::new(),
            config,
        }
    }

    /// Get the detector
    pub fn detector(&self) -> &DeadlockDetector {
        &self.detector
    }

    /// Prevent deadlock using configured strategy
    pub fn prevent_deadlock(&self, lock_address: usize, thread_id: usize, held_locks: &[usize]) -> bool {
        match self.config.strategy {
            DeadlockPreventionStrategy::LockOrdering => {
                self.detector.track_lock_acquire(lock_address, thread_id, held_locks);
                true
            }
            DeadlockPreventionStrategy::LockTimeout => {
                // In a real implementation, this would use timeout-based locking
                true
            }
            DeadlockPreventionStrategy::LockHierarchy => {
                // In a real implementation, this would enforce a lock hierarchy
                true
            }
            DeadlockPreventionStrategy::NoPrevention => true,
        }
    }

    /// Get detected deadlocks
    pub fn detected_deadlocks(&self) -> Vec<DeadlockInfo> {
        self.detector.detected_deadlocks()
    }

    /// Get statistics
    pub fn statistics(&self) -> DeadlockDetectionStatistics {
        self.detector.statistics()
    }
}

impl Default for DeadlockPreventionManager {
    fn default() -> Self {
        Self::new(DeadlockPreventionConfig::default())
    }
}

/// Global deadlock detector
static DEADLOCK_DETECTOR: spin::Once<DeadlockDetector> = spin::Once::new();

/// Get the global deadlock detector
pub fn deadlock_detector() -> &'static DeadlockDetector {
    DEADLOCK_DETECTOR.call_once(|| DeadlockDetector::new())
}

/// Track a lock acquisition
pub fn track_lock_acquire(lock_address: usize, thread_id: usize, held_locks: &[usize]) {
    deadlock_detector().track_lock_acquire(lock_address, thread_id, held_locks);
}

/// Track a lock release
pub fn track_lock_release(lock_address: usize) {
    deadlock_detector().track_lock_release(lock_address);
}

/// Get detected deadlocks
pub fn detected_deadlocks() -> Vec<DeadlockInfo> {
    deadlock_detector().detected_deadlocks()
}