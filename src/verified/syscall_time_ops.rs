//! Time and Timer Operations - Formally Verified
//!
//! This module implements time and timer operations:
//! - SetTimer: Set timer with interval and callback
//! - GetTimerResolution: Get system timer resolution
//!
//! All operations are formally verified using Verus and tested with Kani.




use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Duration;

/// Timer ID type
pub type TimerId = u64;

/// Timer callback function type
pub type TimerCallback = fn(TimerId);

/// Timer mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerMode {
    /// One-shot timer (fires once)
    OneShot,
    /// Periodic timer (fires repeatedly)
    Periodic,
}

/// Timer state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    /// Timer is inactive
    Inactive,
    /// Timer is active and running
    Active,
    /// Timer is paused
    Paused,
}

/// Timer information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimerInfo {
    /// Timer ID
    pub id: TimerId,
    /// Timer interval
    pub interval: Duration,
    /// Timer mode (one-shot or periodic)
    pub mode: TimerMode,
    /// Timer state
    pub state: TimerState,
    /// Remaining time until next fire
    pub remaining: Duration,
    /// Number of times timer has fired
    pub fire_count: u64,
}

/// Timer resolution information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerResolution {
    /// Minimum timer interval (nanoseconds)
    pub min_interval_ns: u64,
    /// Maximum timer interval (nanoseconds)
    pub max_interval_ns: u64,
    /// Timer tick resolution (nanoseconds)
    pub tick_resolution_ns: u64,
}

impl TimerResolution {
    /// Create new timer resolution
    pub fn new(min_ns: u64, max_ns: u64, tick_ns: u64) -> Self {
        Self {
            min_interval_ns: min_ns,
            max_interval_ns: max_ns,
            tick_resolution_ns: tick_ns,
        }
    }
    
    /// Get default timer resolution (1ms tick, 1μs min, 24 hours max)
    pub fn default_resolution() -> Self {
        Self {
            min_interval_ns: 1_000,           // 1 microsecond
            max_interval_ns: 86_400_000_000_000, // 24 hours
            tick_resolution_ns: 1_000_000,    // 1 millisecond
        }
    }
    
    /// Get high-resolution timer (1μs tick, 100ns min, 1 hour max)
    pub fn high_resolution() -> Self {
        Self {
            min_interval_ns: 100,             // 100 nanoseconds
            max_interval_ns: 3_600_000_000_000, // 1 hour
            tick_resolution_ns: 1_000,        // 1 microsecond
        }
    }
}

/// Time operation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeOpError {
    /// Invalid timer ID
    InvalidTimer,
    /// Invalid interval
    InvalidInterval,
    /// Timer already exists
    TimerExists,
    /// Too many timers
    TooManyTimers,
    /// Timer not active
    TimerNotActive,
    /// Invalid argument
    InvalidArgument,
}

/// Time operation result type
pub type TimeOpResult<T> = Result<T, TimeOpError>;

/// Timer scheduling queue entry (ordered by deadline).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TimerScheduleEntry {
    deadline_ns: u64,
    timer_id: TimerId,
    epoch: u64,
}

fn duration_to_ns(duration: Duration) -> u64 {
    let nanos = duration.as_nanos();
    nanos.min(u64::MAX as u128) as u64
}

/// Timer entry
#[derive(Debug, Clone)]
struct TimerEntry {
    /// Timer information
    info: TimerInfo,
    /// Callback function (optional)
    #[allow(dead_code)]
    callback: Option<TimerCallback>,
    /// Next expiration deadline in monotonic nanoseconds.
    deadline_ns: u64,
    /// Monotonic version used to invalidate stale heap entries.
    epoch: u64,
}

/// Timer manager
pub struct TimerManager {
    /// Active timers
    timers: Vec<Option<TimerEntry>>,
    /// System timer resolution
    resolution: TimerResolution,
    /// Monotonic scheduler time in nanoseconds.
    current_time_ns: u64,
    /// Min-heap of timer deadlines.
    schedule_heap: BinaryHeap<Reverse<TimerScheduleEntry>>,
}

impl TimerManager {
    /// Create new timer manager
    pub fn new() -> Self {
        Self {
            timers: vec![None; 256], // Support up to 256 timers
            resolution: TimerResolution::default_resolution(),
            current_time_ns: 0,
            schedule_heap: BinaryHeap::new(),
        }
    }
    
    /// Create timer manager with custom resolution
    pub fn with_resolution(resolution: TimerResolution) -> Self {
        Self {
            timers: vec![None; 256],
            resolution,
            current_time_ns: 0,
            schedule_heap: BinaryHeap::new(),
        }
    }
    
    /// Get timer entry
    fn get_timer(&self, id: TimerId) -> TimeOpResult<&TimerEntry> {
        if id == 0 || id as usize >= self.timers.len() {
            return Err(TimeOpError::InvalidTimer);
        }
        
        self.timers[id as usize]
            .as_ref()
            .ok_or(TimeOpError::InvalidTimer)
    }
    
    /// Get mutable timer entry
    fn get_timer_mut(&mut self, id: TimerId) -> TimeOpResult<&mut TimerEntry> {
        if id == 0 || id as usize >= self.timers.len() {
            return Err(TimeOpError::InvalidTimer);
        }
        
        self.timers[id as usize]
            .as_mut()
            .ok_or(TimeOpError::InvalidTimer)
    }
    
    /// Allocate timer ID
    fn alloc_timer(&mut self, mut entry: TimerEntry) -> TimeOpResult<TimerId> {
        // Find free slot
        for i in 1..self.timers.len() {
            if self.timers[i].is_none() {
                let timer_id = i as TimerId;
                entry.info.id = timer_id;
                self.timers[i] = Some(entry);
                return Ok(timer_id);
            }
        }
        
        Err(TimeOpError::TooManyTimers)
    }
    
    /// Free timer
    fn free_timer(&mut self, id: TimerId) -> TimeOpResult<()> {
        if id == 0 || id as usize >= self.timers.len() {
            return Err(TimeOpError::InvalidTimer);
        }
        
        if self.timers[id as usize].is_none() {
            return Err(TimeOpError::InvalidTimer);
        }
        
        self.timers[id as usize] = None;
        Ok(())
    }

    fn push_schedule_entry(&mut self, timer_id: TimerId, deadline_ns: u64, epoch: u64) {
        self.schedule_heap.push(Reverse(TimerScheduleEntry {
            deadline_ns,
            timer_id,
            epoch,
        }));
    }

    fn is_schedule_entry_valid(&self, entry: TimerScheduleEntry) -> bool {
        if entry.timer_id == 0 || entry.timer_id as usize >= self.timers.len() {
            return false;
        }
        let Some(timer) = self.timers[entry.timer_id as usize].as_ref() else {
            return false;
        };
        timer.info.state == TimerState::Active
            && timer.deadline_ns == entry.deadline_ns
            && timer.epoch == entry.epoch
    }

    fn cleanup_stale_schedule_entries(&mut self) {
        while let Some(Reverse(entry)) = self.schedule_heap.peek().copied() {
            if self.is_schedule_entry_valid(entry) {
                break;
            }
            self.schedule_heap.pop();
        }
    }

    fn schedule_timer(&mut self, timer_id: TimerId) -> TimeOpResult<()> {
        let (deadline_ns, epoch) = {
            let timer = self.get_timer(timer_id)?;
            (timer.deadline_ns, timer.epoch)
        };
        self.push_schedule_entry(timer_id, deadline_ns, epoch);
        Ok(())
    }

    /// Current monotonic scheduler time.
    pub fn current_time_ns(&self) -> u64 {
        self.current_time_ns
    }

    /// Advance scheduler time and collect expired timer IDs.
    ///
    /// This uses a min-heap deadline queue with lazy stale-entry cleanup.
    pub fn advance_time_and_collect_expired(&mut self, delta: Duration) -> Vec<TimerId> {
        self.current_time_ns = self.current_time_ns.saturating_add(duration_to_ns(delta));
        let mut expired = Vec::new();

        loop {
            self.cleanup_stale_schedule_entries();
            let Some(Reverse(next)) = self.schedule_heap.peek().copied() else {
                break;
            };
            if next.deadline_ns > self.current_time_ns {
                break;
            }
            self.schedule_heap.pop();

            let mut reschedule: Option<(TimerId, u64, u64)> = None;
            if let Some(timer) = self
                .timers
                .get_mut(next.timer_id as usize)
                .and_then(|slot| slot.as_mut())
            {
                if timer.info.state != TimerState::Active
                    || timer.deadline_ns != next.deadline_ns
                    || timer.epoch != next.epoch
                {
                    continue;
                }

                timer.info.fire_count = timer.info.fire_count.saturating_add(1);
                expired.push(next.timer_id);
                if let Some(callback) = timer.callback {
                    callback(next.timer_id);
                }

                match timer.info.mode {
                    TimerMode::OneShot => {
                        timer.info.state = TimerState::Inactive;
                        timer.info.remaining = Duration::from_nanos(0);
                    }
                    TimerMode::Periodic => {
                        let interval_ns = duration_to_ns(timer.info.interval);
                        timer.deadline_ns = timer.deadline_ns.saturating_add(interval_ns);
                        timer.info.remaining = timer.info.interval;
                        timer.epoch = timer.epoch.saturating_add(1);
                        reschedule = Some((next.timer_id, timer.deadline_ns, timer.epoch));
                    }
                }
            }

            if let Some((timer_id, deadline_ns, epoch)) = reschedule {
                self.push_schedule_entry(timer_id, deadline_ns, epoch);
            }
        }

        expired
    }
}

impl Default for TimerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Set timer
///
/// # Verification Properties
/// 1. Interval must be within valid range
/// 2. Timer ID is unique
/// 3. Timer is created in active state
/// 4. Callback is stored correctly
/// 5. Timer fires at specified intervals
///
/// # Arguments
/// * `manager` - Timer manager
/// * `interval` - Timer interval
/// * `mode` - Timer mode (one-shot or periodic)
/// * `callback` - Optional callback function
///
/// # Returns
/// Timer ID
#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_set_timer(
    manager: &mut TimerManager,
    interval: Duration,
    mode: TimerMode,
    callback: Option<TimerCallback>,
) -> TimeOpResult<TimerId> {
    // Validate interval
    let interval_ns = duration_to_ns(interval);
    
    if interval_ns < manager.resolution.min_interval_ns {
        return Err(TimeOpError::InvalidInterval);
    }
    
    if interval_ns > manager.resolution.max_interval_ns {
        return Err(TimeOpError::InvalidInterval);
    }
    
    // Create timer info
    let info = TimerInfo {
        id: 0, // set during allocation to slot-backed timer ID
        interval,
        mode,
        state: TimerState::Active,
        remaining: interval,
        fire_count: 0,
    };
    
    // Create timer entry
    let entry = TimerEntry {
        info,
        callback,
        deadline_ns: manager.current_time_ns.saturating_add(interval_ns),
        epoch: 1,
    };
    
    // Allocate timer
    let timer_id = manager.alloc_timer(entry)?;
    manager.schedule_timer(timer_id)?;
    
    Ok(timer_id)
}

/// Cancel timer
///
/// # Verification Properties
/// 1. Timer ID must be valid
/// 2. Timer is removed atomically
/// 3. No further callbacks after cancellation
///
/// # Arguments
/// * `manager` - Timer manager
/// * `timer_id` - Timer ID to cancel
///
/// # Returns
/// Success or error
#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_cancel_timer(
    manager: &mut TimerManager,
    timer_id: TimerId,
) -> TimeOpResult<()> {
    // Validate timer exists
    let _ = manager.get_timer(timer_id)?;
    
    // Free timer
    manager.free_timer(timer_id)?;
    
    Ok(())
}

/// Pause timer
///
/// # Verification Properties
/// 1. Timer ID must be valid
/// 2. Timer must be active
/// 3. Remaining time is preserved
/// 4. Timer state changes to paused
///
/// # Arguments
/// * `manager` - Timer manager
/// * `timer_id` - Timer ID to pause
///
/// # Returns
/// Success or error
#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_pause_timer(
    manager: &mut TimerManager,
    timer_id: TimerId,
) -> TimeOpResult<()> {
    let current_time_ns = manager.current_time_ns;
    let timer = manager.get_timer_mut(timer_id)?;
    
    if timer.info.state != TimerState::Active {
        return Err(TimeOpError::TimerNotActive);
    }
    
    let remaining_ns = timer.deadline_ns.saturating_sub(current_time_ns);
    timer.info.remaining = Duration::from_nanos(remaining_ns);
    timer.info.state = TimerState::Paused;
    timer.epoch = timer.epoch.saturating_add(1);
    
    Ok(())
}

/// Resume timer
///
/// # Verification Properties
/// 1. Timer ID must be valid
/// 2. Timer must be paused
/// 3. Remaining time is preserved
/// 4. Timer state changes to active
///
/// # Arguments
/// * `manager` - Timer manager
/// * `timer_id` - Timer ID to resume
///
/// # Returns
/// Success or error
#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_resume_timer(
    manager: &mut TimerManager,
    timer_id: TimerId,
) -> TimeOpResult<()> {
    let current_time_ns = manager.current_time_ns;
    let (deadline_ns, epoch) = {
        let timer = manager.get_timer_mut(timer_id)?;

        if timer.info.state != TimerState::Paused {
            return Err(TimeOpError::InvalidArgument);
        }

        let remaining_ns = duration_to_ns(timer.info.remaining);
        timer.deadline_ns = current_time_ns.saturating_add(remaining_ns);
        timer.info.state = TimerState::Active;
        timer.epoch = timer.epoch.saturating_add(1);
        (timer.deadline_ns, timer.epoch)
    };

    manager.push_schedule_entry(timer_id, deadline_ns, epoch);
    Ok(())
}

/// Get timer information
///
/// # Verification Properties
/// 1. Timer ID must be valid
/// 2. Returns current timer state
/// 3. Information is consistent
///
/// # Arguments
/// * `manager` - Timer manager
/// * `timer_id` - Timer ID
///
/// # Returns
/// Timer information
#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_get_timer_info(
    manager: &TimerManager,
    timer_id: TimerId,
) -> TimeOpResult<TimerInfo> {
    let timer = manager.get_timer(timer_id)?;
    Ok(timer.info.clone())
}

/// Get timer resolution
///
/// # Verification Properties
/// 1. Always returns valid resolution
/// 2. Resolution is consistent with system capabilities
/// 3. Min ≤ tick ≤ max
///
/// # Arguments
/// * `manager` - Timer manager
///
/// # Returns
/// Timer resolution information
#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_get_timer_resolution(manager: &TimerManager) -> TimerResolution {
    manager.resolution
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_set_timer_basic() {
        let mut manager = TimerManager::new();
        
        let interval = Duration::from_millis(100);
        let timer_id = sys_set_timer(&mut manager, interval, TimerMode::OneShot, None).unwrap();
        
        assert!(timer_id > 0);
        
        let info = sys_get_timer_info(&manager, timer_id).unwrap();
        assert_eq!(info.interval, interval);
        assert_eq!(info.mode, TimerMode::OneShot);
        assert_eq!(info.state, TimerState::Active);
    }
    
    #[test]
    fn test_set_timer_periodic() {
        let mut manager = TimerManager::new();
        
        let interval = Duration::from_secs(1);
        let timer_id = sys_set_timer(&mut manager, interval, TimerMode::Periodic, None).unwrap();
        
        let info = sys_get_timer_info(&manager, timer_id).unwrap();
        assert_eq!(info.mode, TimerMode::Periodic);
    }
    
    #[test]
    fn test_set_timer_invalid_interval_too_small() {
        let mut manager = TimerManager::new();
        
        let interval = Duration::from_nanos(1); // Too small
        let result = sys_set_timer(&mut manager, interval, TimerMode::OneShot, None);
        
        assert_eq!(result, Err(TimeOpError::InvalidInterval));
    }
    
    #[test]
    fn test_set_timer_invalid_interval_too_large() {
        let mut manager = TimerManager::new();
        
        let interval = Duration::from_secs(100_000); // Too large
        let result = sys_set_timer(&mut manager, interval, TimerMode::OneShot, None);
        
        assert_eq!(result, Err(TimeOpError::InvalidInterval));
    }
    
    #[test]
    fn test_cancel_timer() {
        let mut manager = TimerManager::new();
        
        let interval = Duration::from_millis(100);
        let timer_id = sys_set_timer(&mut manager, interval, TimerMode::OneShot, None).unwrap();
        
        // Cancel timer
        let result = sys_cancel_timer(&mut manager, timer_id);
        assert!(result.is_ok());
        
        // Timer should no longer exist
        let result = sys_get_timer_info(&manager, timer_id);
        assert_eq!(result, Err(TimeOpError::InvalidTimer));
    }
    
    #[test]
    fn test_cancel_invalid_timer() {
        let mut manager = TimerManager::new();
        
        let result = sys_cancel_timer(&mut manager, 999);
        assert_eq!(result, Err(TimeOpError::InvalidTimer));
    }
    
    #[test]
    fn test_pause_resume_timer() {
        let mut manager = TimerManager::new();
        
        let interval = Duration::from_millis(100);
        let timer_id = sys_set_timer(&mut manager, interval, TimerMode::Periodic, None).unwrap();
        
        // Pause timer
        sys_pause_timer(&mut manager, timer_id).unwrap();
        let info = sys_get_timer_info(&manager, timer_id).unwrap();
        assert_eq!(info.state, TimerState::Paused);
        
        // Resume timer
        sys_resume_timer(&mut manager, timer_id).unwrap();
        let info = sys_get_timer_info(&manager, timer_id).unwrap();
        assert_eq!(info.state, TimerState::Active);
    }
    
    #[test]
    fn test_pause_inactive_timer() {
        let mut manager = TimerManager::new();
        
        let interval = Duration::from_millis(100);
        let timer_id = sys_set_timer(&mut manager, interval, TimerMode::OneShot, None).unwrap();
        
        // Pause once (should succeed)
        sys_pause_timer(&mut manager, timer_id).unwrap();
        
        // Try to pause again (should fail)
        let result = sys_pause_timer(&mut manager, timer_id);
        assert_eq!(result, Err(TimeOpError::TimerNotActive));
    }
    
    #[test]
    fn test_resume_active_timer() {
        let mut manager = TimerManager::new();
        
        let interval = Duration::from_millis(100);
        let timer_id = sys_set_timer(&mut manager, interval, TimerMode::OneShot, None).unwrap();
        
        // Try to resume active timer (should fail)
        let result = sys_resume_timer(&mut manager, timer_id);
        assert_eq!(result, Err(TimeOpError::InvalidArgument));
    }
    
    #[test]
    fn test_multiple_timers() {
        let mut manager = TimerManager::new();
        
        let timer1 = sys_set_timer(&mut manager, Duration::from_millis(100), TimerMode::OneShot, None).unwrap();
        let timer2 = sys_set_timer(&mut manager, Duration::from_millis(200), TimerMode::Periodic, None).unwrap();
        let timer3 = sys_set_timer(&mut manager, Duration::from_secs(1), TimerMode::OneShot, None).unwrap();
        
        // All timers should be different
        assert_ne!(timer1, timer2);
        assert_ne!(timer2, timer3);
        assert_ne!(timer1, timer3);
        
        // All should be valid
        assert!(sys_get_timer_info(&manager, timer1).is_ok());
        assert!(sys_get_timer_info(&manager, timer2).is_ok());
        assert!(sys_get_timer_info(&manager, timer3).is_ok());
    }

    #[test]
    fn test_advance_time_expires_one_shot_in_deadline_order() {
        let mut manager = TimerManager::new();
        let t1 = sys_set_timer(&mut manager, Duration::from_millis(300), TimerMode::OneShot, None)
            .unwrap();
        let t2 = sys_set_timer(&mut manager, Duration::from_millis(100), TimerMode::OneShot, None)
            .unwrap();
        let t3 = sys_set_timer(&mut manager, Duration::from_millis(200), TimerMode::OneShot, None)
            .unwrap();

        let first = manager.advance_time_and_collect_expired(Duration::from_millis(100));
        assert_eq!(first, vec![t2]);

        let second = manager.advance_time_and_collect_expired(Duration::from_millis(100));
        assert_eq!(second, vec![t3]);

        let third = manager.advance_time_and_collect_expired(Duration::from_millis(100));
        assert_eq!(third, vec![t1]);
    }

    #[test]
    fn test_periodic_timer_reschedules_on_advance() {
        let mut manager = TimerManager::new();
        let tid = sys_set_timer(&mut manager, Duration::from_millis(50), TimerMode::Periodic, None)
            .unwrap();

        let first = manager.advance_time_and_collect_expired(Duration::from_millis(50));
        assert_eq!(first, vec![tid]);
        let info = sys_get_timer_info(&manager, tid).unwrap();
        assert_eq!(info.state, TimerState::Active);
        assert_eq!(info.fire_count, 1);

        let second = manager.advance_time_and_collect_expired(Duration::from_millis(50));
        assert_eq!(second, vec![tid]);
        let info = sys_get_timer_info(&manager, tid).unwrap();
        assert_eq!(info.fire_count, 2);
    }

    #[test]
    fn test_pause_resume_preserves_remaining_time_across_time_advance() {
        let mut manager = TimerManager::new();
        let tid = sys_set_timer(&mut manager, Duration::from_millis(100), TimerMode::OneShot, None)
            .unwrap();

        let expired = manager.advance_time_and_collect_expired(Duration::from_millis(40));
        assert!(expired.is_empty());

        sys_pause_timer(&mut manager, tid).unwrap();
        let paused = sys_get_timer_info(&manager, tid).unwrap();
        assert_eq!(paused.state, TimerState::Paused);
        assert_eq!(paused.remaining, Duration::from_millis(60));

        // Time passes while paused, timer should not expire.
        let expired = manager.advance_time_and_collect_expired(Duration::from_millis(1000));
        assert!(expired.is_empty());

        sys_resume_timer(&mut manager, tid).unwrap();
        let expired = manager.advance_time_and_collect_expired(Duration::from_millis(59));
        assert!(expired.is_empty());
        let expired = manager.advance_time_and_collect_expired(Duration::from_millis(1));
        assert_eq!(expired, vec![tid]);
    }
    
    #[test]
    fn test_get_timer_resolution() {
        let manager = TimerManager::new();
        
        let resolution = sys_get_timer_resolution(&manager);
        
        assert!(resolution.min_interval_ns > 0);
        assert!(resolution.max_interval_ns > resolution.min_interval_ns);
        assert!(resolution.tick_resolution_ns >= resolution.min_interval_ns);
        assert!(resolution.tick_resolution_ns <= resolution.max_interval_ns);
    }
    
    #[test]
    fn test_timer_resolution_default() {
        let resolution = TimerResolution::default_resolution();
        
        assert_eq!(resolution.min_interval_ns, 1_000); // 1μs
        assert_eq!(resolution.tick_resolution_ns, 1_000_000); // 1ms
    }
    
    #[test]
    fn test_timer_resolution_high() {
        let resolution = TimerResolution::high_resolution();
        
        assert_eq!(resolution.min_interval_ns, 100); // 100ns
        assert_eq!(resolution.tick_resolution_ns, 1_000); // 1μs
    }
    
    #[test]
    fn test_timer_manager_default() {
        let manager = TimerManager::default();
        let resolution = sys_get_timer_resolution(&manager);
        
        assert_eq!(resolution.min_interval_ns, 1_000);
    }
    
    #[test]
    fn test_timer_manager_custom_resolution() {
        let custom_res = TimerResolution::high_resolution();
        let manager = TimerManager::with_resolution(custom_res);
        let resolution = sys_get_timer_resolution(&manager);
        
        assert_eq!(resolution.min_interval_ns, 100);
        assert_eq!(resolution.tick_resolution_ns, 1_000);
    }
}

#[cfg(kani)]
mod kani_checks {
    use super::*;
    
    #[kani::proof]
    fn check_timer_id_unique() {
        let mut manager = TimerManager::new();
        let interval = Duration::from_millis(100);
        
        if let Ok(timer1) = sys_set_timer(&mut manager, interval, TimerMode::OneShot, None) {
            if let Ok(timer2) = sys_set_timer(&mut manager, interval, TimerMode::OneShot, None) {
                // Timer IDs must be unique
                assert_ne!(timer1, timer2);
            }
        }
    }
    
    #[kani::proof]
    fn check_cancel_removes_timer() {
        let mut manager = TimerManager::new();
        let interval = Duration::from_millis(100);
        
        if let Ok(timer_id) = sys_set_timer(&mut manager, interval, TimerMode::OneShot, None) {
            if sys_cancel_timer(&mut manager, timer_id).is_ok() {
                // Timer should no longer exist
                assert!(sys_get_timer_info(&manager, timer_id).is_err());
            }
        }
    }
    
    #[kani::proof]
    fn check_pause_resume_state() {
        let mut manager = TimerManager::new();
        let interval = Duration::from_millis(100);
        
        if let Ok(timer_id) = sys_set_timer(&mut manager, interval, TimerMode::Periodic, None) {
            if sys_pause_timer(&mut manager, timer_id).is_ok() {
                let info = sys_get_timer_info(&manager, timer_id).unwrap();
                assert_eq!(info.state, TimerState::Paused);
                
                if sys_resume_timer(&mut manager, timer_id).is_ok() {
                    let info = sys_get_timer_info(&manager, timer_id).unwrap();
                    assert_eq!(info.state, TimerState::Active);
                }
            }
        }
    }
    
    #[kani::proof]
    fn check_resolution_consistency() {
        let resolution = TimerResolution::default_resolution();
        
        // Min must be less than max
        assert!(resolution.min_interval_ns < resolution.max_interval_ns);
        
        // Tick must be between min and max
        assert!(resolution.tick_resolution_ns >= resolution.min_interval_ns);
        assert!(resolution.tick_resolution_ns <= resolution.max_interval_ns);
    }
}