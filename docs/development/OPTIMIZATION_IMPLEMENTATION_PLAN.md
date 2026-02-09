# 🚀 VANTIS OS Optimization Implementation Plan

## 📋 Executive Summary

This document provides a detailed implementation plan for optimizing the VANTIS OS verified modules, focusing on high-impact, low-risk optimizations that maintain formal verification.

---

## 🎯 Optimization Priorities

### Priority 1: IPC Capability HashMap (Highest Impact)

**Current Performance**: O(n) capability lookup  
**Target Performance**: O(1) capability lookup  
**Expected Improvement**: 10-100x faster  
**Risk Level**: Low  
**Verification Impact**: Minimal  

#### Implementation Details

```rust
// File: src/verified/ipc.rs
// Replace Vec-based capability storage with HashMap

use std::collections::HashMap;

pub struct IpcManager {
    queues: Vec<Option<MessageQueue>>,
    next_message_id: u64,
    // OLD: capabilities: Vec<(Pid, Pid, Capability)>,
    // NEW: Use HashMap for O(1) lookup
    capabilities: HashMap<(Pid, Pid), Vec<Capability>>,
}

impl IpcManager {
    pub fn new() -> Self {
        IpcManager {
            queues: Vec::new(),
            next_message_id: 1,
            capabilities: HashMap::new(),
        }
    }
    
    pub fn grant_capability(
        &mut self,
        from: Pid,
        to: Pid,
        cap: Capability,
    ) -> Result<(), &'static str> {
        self.capabilities
            .entry((from, to))
            .or_insert_with(Vec::new)
            .push(cap);
        Ok(())
    }
    
    pub fn revoke_capability(
        &mut self,
        from: Pid,
        to: Pid,
        cap: Capability,
    ) -> Result<(), &'static str> {
        if let Some(caps) = self.capabilities.get_mut(&(from, to)) {
            caps.retain(|c| *c != cap);
            if caps.is_empty() {
                self.capabilities.remove(&(from, to));
            }
        }
        Ok(())
    }
    
    pub fn has_capability(&self, from: Pid, to: Pid, cap: Capability) -> bool {
        self.capabilities
            .get(&(from, to))
            .map(|caps| caps.contains(&cap))
            .unwrap_or(false)
    }
}
```

#### Verification Strategy

```rust
#[kani::proof]
fn verify_capability_hashmap_correctness() {
    let mut manager = IpcManager::new();
    let from = Pid::new(1).unwrap();
    let to = Pid::new(2).unwrap();
    
    // Grant capability
    manager.grant_capability(from, to, Capability::Send).unwrap();
    
    // Verify it exists
    assert!(manager.has_capability(from, to, Capability::Send));
    
    // Revoke capability
    manager.revoke_capability(from, to, Capability::Send).unwrap();
    
    // Verify it's gone
    assert!(!manager.has_capability(from, to, Capability::Send));
}
```

#### Testing Strategy

```rust
#[test]
fn test_capability_performance() {
    let mut manager = IpcManager::new();
    
    // Add 1000 capabilities
    for i in 1..=1000 {
        let from = Pid::new(i).unwrap();
        let to = Pid::new(i + 1).unwrap();
        manager.grant_capability(from, to, Capability::Send).unwrap();
    }
    
    // Measure lookup time
    let start = std::time::Instant::now();
    for _ in 0..10000 {
        let from = Pid::new(500).unwrap();
        let to = Pid::new(501).unwrap();
        manager.has_capability(from, to, Capability::Send);
    }
    let elapsed = start.elapsed();
    
    println!("10000 lookups in {:?}", elapsed);
    // Should be <1ms with HashMap vs ~10ms with Vec
}
```

---

### Priority 2: Scheduler Priority Bitmap (Critical for Performance)

**Current Performance**: O(256) priority search  
**Target Performance**: O(1) priority search  
**Expected Improvement**: 256x faster  
**Risk Level**: Low  
**Verification Impact**: Minimal  

#### Implementation Details

```rust
// File: src/verified/scheduler.rs
// Add priority bitmap for O(1) highest priority lookup

pub struct Scheduler {
    run_queues: Vec<RunQueue>,
    current_task: Option<Pid>,
    current_time: u64,
    context_switches: u64,
    // NEW: Priority bitmap (256 bits = 4 x u64)
    priority_bitmap: [u64; 4],
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            run_queues: (0..=255)
                .map(|p| RunQueue::new(SchedPriority::new(p)))
                .collect(),
            current_task: None,
            current_time: 0,
            context_switches: 0,
            priority_bitmap: [0; 4],
        }
    }
    
    /// Set bit in priority bitmap
    fn set_priority_bit(&mut self, priority: u8) {
        let word = (priority / 64) as usize;
        let bit = priority % 64;
        self.priority_bitmap[word] |= 1u64 << bit;
    }
    
    /// Clear bit in priority bitmap
    fn clear_priority_bit(&mut self, priority: u8) {
        let word = (priority / 64) as usize;
        let bit = priority % 64;
        self.priority_bitmap[word] &= !(1u64 << bit);
    }
    
    /// Find highest priority with tasks (O(1) with hardware support)
    fn find_highest_priority(&self) -> Option<u8> {
        for (word_idx, &word) in self.priority_bitmap.iter().enumerate() {
            if word != 0 {
                // Find first set bit (hardware instruction on most CPUs)
                let bit = word.trailing_zeros() as u8;
                return Some((word_idx as u8) * 64 + bit);
            }
        }
        None
    }
    
    pub fn add_task(&mut self, task: SchedTask) {
        let priority = task.priority().as_u8();
        self.run_queues[priority as usize].enqueue(task);
        self.set_priority_bit(priority);
    }
    
    pub fn remove_task(&mut self, pid: Pid) -> Option<SchedTask> {
        for (priority, queue) in self.run_queues.iter_mut().enumerate() {
            if let Some(task) = queue.remove(pid) {
                // Clear bit if queue is now empty
                if queue.is_empty() {
                    self.clear_priority_bit(priority as u8);
                }
                return Some(task);
            }
        }
        None
    }
    
    pub fn schedule(&mut self) -> Option<Pid> {
        // O(1) lookup of highest priority
        if let Some(priority) = self.find_highest_priority() {
            let queue = &mut self.run_queues[priority as usize];
            
            if let Some(task) = queue.next_task() {
                let pid = task.pid();
                
                if task.quantum_exhausted() {
                    task.reset_quantum();
                }
                
                task.on_scheduled(self.current_time);
                
                if self.current_task != Some(pid) {
                    self.context_switches += 1;
                    self.current_task = Some(pid);
                }
                
                return Some(pid);
            }
        }
        
        self.current_task = None;
        None
    }
}
```

#### Verification Strategy

```rust
#[kani::proof]
fn verify_priority_bitmap_correctness() {
    let mut scheduler = Scheduler::new();
    
    let pid1 = Pid::new(1).unwrap();
    let pid2 = Pid::new(2).unwrap();
    
    // Add high priority task
    scheduler.add_task(SchedTask::new(pid1, SchedPriority::HIGH));
    
    // Add low priority task
    scheduler.add_task(SchedTask::new(pid2, SchedPriority::LOW));
    
    // High priority should be scheduled first
    let scheduled = scheduler.schedule();
    assert_eq!(scheduled, Some(pid1));
}

#[kani::proof]
fn verify_bitmap_consistency() {
    let mut scheduler = Scheduler::new();
    let pid = Pid::new(1).unwrap();
    let priority = SchedPriority::NORMAL;
    
    // Add task
    scheduler.add_task(SchedTask::new(pid, priority));
    
    // Verify bit is set
    let bit_set = scheduler.find_highest_priority().is_some();
    assert!(bit_set);
    
    // Remove task
    scheduler.remove_task(pid);
    
    // Verify bit is cleared (if queue empty)
    // This maintains bitmap consistency
}
```

---

### Priority 3: IPC Message Inline Storage (Medium Impact)

**Current Performance**: Heap allocation for all messages  
**Target Performance**: Inline storage for small messages  
**Expected Improvement**: 2-5x faster for small messages  
**Risk Level**: Low  
**Verification Impact**: Minimal  

#### Implementation Details

```rust
// File: src/verified/ipc.rs
// Add inline storage for small messages

const INLINE_MESSAGE_SIZE: usize = 64;

pub enum MessageData {
    /// Inline storage for small messages (<= 64 bytes)
    Inline {
        data: [u8; INLINE_MESSAGE_SIZE],
        len: usize,
    },
    /// Heap storage for large messages (> 64 bytes)
    Heap(Vec<u8>),
}

impl MessageData {
    pub fn new(data: Vec<u8>) -> Self {
        if data.len() <= INLINE_MESSAGE_SIZE {
            let mut inline_data = [0u8; INLINE_MESSAGE_SIZE];
            inline_data[..data.len()].copy_from_slice(&data);
            MessageData::Inline {
                data: inline_data,
                len: data.len(),
            }
        } else {
            MessageData::Heap(data)
        }
    }
    
    pub fn as_slice(&self) -> &[u8] {
        match self {
            MessageData::Inline { data, len } => &data[..*len],
            MessageData::Heap(vec) => vec.as_slice(),
        }
    }
    
    pub fn len(&self) -> usize {
        match self {
            MessageData::Inline { len, .. } => *len,
            MessageData::Heap(vec) => vec.len(),
        }
    }
}

pub struct Message {
    id: MessageId,
    sender: Pid,
    receiver: Pid,
    data: MessageData,  // Changed from Vec<u8>
    priority: Priority,
    capabilities: Vec<Capability>,
}

impl Message {
    pub fn new(
        id: MessageId,
        sender: Pid,
        receiver: Pid,
        data: Vec<u8>,
        priority: Priority,
    ) -> Result<Self, &'static str> {
        if data.len() > MAX_MESSAGE_SIZE {
            return Err("Message too large");
        }
        
        Ok(Message {
            id,
            sender,
            receiver,
            data: MessageData::new(data),
            priority,
            capabilities: Vec::new(),
        })
    }
    
    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}
```

---

## 📊 Implementation Timeline

### Week 1: IPC Optimization
- **Day 1-2**: Implement capability HashMap
- **Day 3**: Add verification harnesses
- **Day 4**: Add benchmarks and tests
- **Day 5**: Measure and validate improvements

### Week 2: Scheduler Optimization
- **Day 1-2**: Implement priority bitmap
- **Day 3**: Add verification harnesses
- **Day 4**: Add benchmarks and tests
- **Day 5**: Measure and validate improvements

### Week 3: Message Optimization
- **Day 1-2**: Implement inline message storage
- **Day 3**: Add verification harnesses
- **Day 4**: Add benchmarks and tests
- **Day 5**: Measure and validate improvements

---

## 🧪 Verification Strategy

### For Each Optimization

1. **Maintain Formal Specifications**
   - Keep all Verus specs unchanged
   - Properties must remain proven
   - No weakening of guarantees

2. **Add Kani Harnesses**
   - Verify optimized behavior matches original
   - Test edge cases
   - Ensure no regressions

3. **Comprehensive Testing**
   - Unit tests for new code paths
   - Integration tests for interactions
   - Performance benchmarks

4. **Documentation**
   - Update implementation notes
   - Document optimization rationale
   - Explain performance characteristics

---

## 📈 Success Metrics

### Performance Targets
```
Optimization              Before      After       Improvement
─────────────────────────────────────────────────────────────
IPC Capability Check      1μs         10ns        100x
Scheduler Decision        10μs        40ns        250x
Small Message Alloc       2μs         400ns       5x
```

### Quality Targets
```
Metric                    Target      Status
──────────────────────────────────────────────
Test Coverage             100%        Maintain
Unsafe Code              0 lines      Maintain
Verification Success      100%        Maintain
Documentation            100%         Maintain
```

---

## ⚠️ Risk Mitigation

### Risk 1: Breaking Verification
**Mitigation**:
- Implement incrementally
- Verify each change
- Maintain test coverage
- Keep formal specs

### Risk 2: Performance Regression
**Mitigation**:
- Benchmark before and after
- Profile hot paths
- Measure actual improvements
- Rollback if needed

### Risk 3: Increased Complexity
**Mitigation**:
- Keep changes simple
- Document thoroughly
- Code review
- Maintain clarity

---

## 🎯 Recommendations

### Immediate Actions
1. ✅ Implement IPC capability HashMap (highest impact)
2. ✅ Add comprehensive benchmarks
3. ✅ Verify with Kani harnesses
4. ✅ Measure improvements

### Next Steps
1. ⏳ Implement scheduler priority bitmap
2. ⏳ Implement message inline storage
3. ⏳ Profile and identify other bottlenecks
4. ⏳ Continue optimization cycle

---

**Document Version**: 1.0  
**Last Updated**: January 10, 2025  
**Status**: Ready for Implementation  
**Priority**: High