# POSIX Timer Migration Guide - VantisOS v0.5.0

## Overview

Starting with VantisOS v0.5.0, several timer-related syscalls have been **deprecated** in favor of a more modern, object-oriented `UserSpaceTimer` API. This guide will help you migrate your code to use the new API.

## Deprecated Functions

The following functions have been deprecated and will be removed in a future release:

1. `sys_pause_timer()` → Use `UserSpaceTimer::pause()`
2. `sys_resume_timer()` → Use `UserSpaceTimer::resume()`
3. `sys_get_timer_info()` → Use `UserSpaceTimer::get_info()`
4. `sys_get_timer_resolution()` → Use `TIMER_RESOLUTION_NS` constant

## Why These Changes?

The old syscalls had several limitations:
- **No encapsulation**: Timer state was managed through raw IDs
- **Error-prone**: Easy to pass wrong timer IDs or mix up operations
- **Poor type safety**: No guarantee that operations are on valid timers
- **Inconsistent API**: Some functions took `&mut TimerManager`, others `&TimerManager`

The new `UserSpaceTimer` API provides:
- **Object-oriented**: Timer state is encapsulated in a struct
- **Type-safe**: Timer ID is bound to the timer instance
- **Modern API**: Consistent borrowing patterns
- **Better documentation**: Comprehensive doc comments with examples
- **Backward compatible**: Old functions still work with deprecation warnings

## Migration Examples

### Example 1: Creating and Managing a Timer

**Old API (Deprecated):**
```rust
use crate::syscall_time_ops::{
    sys_set_timer, sys_pause_timer, sys_resume_timer, 
    sys_get_timer_info, TimerMode
};
use std::time::Duration;

let mut manager = TimerManager::new();

// Create timer
let timer_id = sys_set_timer(
    &mut manager,
    Duration::from_millis(100),
    TimerMode::Periodic,
    None
).unwrap();

// Pause timer
sys_pause_timer(&mut manager, timer_id).unwrap();

// Resume timer
sys_resume_timer(&mut manager, timer_id).unwrap();

// Get timer info
let info = sys_get_timer_info(&manager, timer_id).unwrap();
```

**New API (Recommended):**
```rust
use crate::syscall_time_ops::{
    UserSpaceTimer, TimerMode, TimerManager
};
use std::time::Duration;

let mut manager = TimerManager::new();

// Create timer
let mut timer = UserSpaceTimer::new(
    &mut manager,
    Duration::from_millis(100),
    TimerMode::Periodic,
    None
).unwrap();

// Pause timer
timer.pause(&mut manager).unwrap();

// Resume timer
timer.resume(&mut manager).unwrap();

// Get timer info
let info = timer.get_info(&manager);
```

### Example 2: Getting Timer Resolution

**Old API (Deprecated):**
```rust
use crate::syscall_time_ops::sys_get_timer_resolution;

let manager = TimerManager::new();
let resolution = sys_get_timer_resolution(&manager);
let min_ns = resolution.min_interval_ns;
```

**New API (Recommended):**
```rust
use crate::syscall_time_ops::TIMER_RESOLUTION_NS;

// Use the constant directly
let min_ns = TIMER_RESOLUTION_NS;
```

### Example 3: Error Handling

**Old API (Deprecated):**
```rust
use crate::syscall_time_ops::{
    sys_pause_timer, TimeOpError
};

let mut manager = TimerManager::new();
let timer_id = 999; // Invalid ID

match sys_pause_timer(&mut manager, timer_id) {
    Ok(()) => println!("Timer paused"),
    Err(TimeOpError::InvalidTimer) => {
        eprintln!("Invalid timer ID");
    }
    Err(TimeOpError::TimerNotActive) => {
        eprintln!("Timer is not active");
    }
    Err(e) => {
        eprintln!("Other error: {:?}", e);
    }
}
```

**New API (Recommended):**
```rust
use crate::syscall_time_ops::{
    UserSpaceTimer, TimerManager, TimeOpError
};

let mut manager = TimerManager::new();
let timer_id = 999;

let timer = UserSpaceTimer { id: timer_id };

match timer.pause(&mut manager) {
    Ok(()) => println!("Timer paused"),
    Err(TimeOpError::InvalidTimer) => {
        eprintln!("Invalid timer ID");
    }
    Err(TimeOpError::TimerNotActive) => {
        eprintln!("Timer is not active");
    }
    Err(e) => {
        eprintln!("Other error: {:?}", e);
    }
}
```

### Example 4: Cancelling a Timer

**Old API (Deprecated):**
```rust
use crate::syscall_time_ops::{
    sys_set_timer, sys_cancel_timer
};

let mut manager = TimerManager::new();
let timer_id = sys_set_timer(
    &mut manager,
    Duration::from_millis(100),
    TimerMode::OneShot,
    None
).unwrap();

// Cancel timer
sys_cancel_timer(&mut manager, timer_id).unwrap();
```

**New API (Recommended):**
```rust
use crate::syscall_time_ops::UserSpaceTimer;

let mut manager = TimerManager::new();
let mut timer = UserSpaceTimer::new(
    &mut manager,
    Duration::from_millis(100),
    TimerMode::OneShot,
    None
).unwrap();

// Cancel timer (consumes the timer)
timer.cancel(&mut manager).unwrap();
```

## API Comparison Table

| Old API | New API | Notes |
|---------|---------|-------|
| `sys_pause_timer(manager, id)` | `timer.pause(manager)` | Timer ID is encapsulated |
| `sys_resume_timer(manager, id)` | `timer.resume(manager)` | Timer ID is encapsulated |
| `sys_get_timer_info(manager, id)` | `timer.get_info(manager)` | Returns `TimerInfo` directly |
| `sys_get_timer_resolution(manager)` | `TIMER_RESOLUTION_NS` | Constant instead of function call |
| `sys_set_timer(manager, ...)` | `UserSpaceTimer::new(manager, ...)` | Creates `UserSpaceTimer` instance |
| `sys_cancel_timer(manager, id)` | `timer.cancel(manager)` | Consumes the timer |

## Benefits of New API

### 1. Better Error Messages
```rust
// Old: You might forget to check if timer exists
sys_pause_timer(&mut manager, timer_id); // Might panic!

// New: The timer struct ensures you're working with a valid timer
timer.pause(&mut manager)?; // Proper error handling
```

### 2. Prevents Accidental Misuse
```rust
// Old: Easy to accidentally use wrong timer ID
let timer1 = sys_set_timer(&mut manager, interval1, ...)?;
let timer2 = sys_set_timer(&mut manager, interval2, ...)?;
sys_pause_timer(&mut manager, timer1)?; // OK
sys_pause_timer(&mut manager, timer2)?; // OK
sys_pause_timer(&mut manager, timer1)?; // What if we meant timer2?

// New: Timer is bound to the struct
let timer1 = UserSpaceTimer::new(&mut manager, interval1, ...)?;
let timer2 = UserSpaceTimer::new(&mut manager, interval2, ...)?;
timer1.pause(&mut manager)?; // Clear which timer we're pausing
timer2.pause(&mut manager)?; // Clear which timer we're pausing
```

### 3. More Intuitive Ownership
```rust
// Old: Timer ID is just a number, easy to duplicate
let timer_id = sys_set_timer(&mut manager, ...)?;
sys_cancel_timer(&mut manager, timer_id)?;
sys_cancel_timer(&mut manager, timer_id)?; // Oops! Cancelling twice!

// New: Timer is owned, can only be cancelled once
let timer = UserSpaceTimer::new(&mut manager, ...)?;
timer.cancel(&mut manager)?; // OK
// timer.cancel(&mut manager)?; // Won't compile - timer was consumed
```

## Migration Checklist

- [ ] Identify all uses of deprecated syscalls
- [ ] Replace `sys_pause_timer` with `UserSpaceTimer::pause()`
- [ ] Replace `sys_resume_timer` with `UserSpaceTimer::resume()`
- [ ] Replace `sys_get_timer_info` with `UserSpaceTimer::get_info()`
- [ ] Replace `sys_get_timer_resolution` with `TIMER_RESOLUTION_NS`
- [ ] Update error handling if needed
- [ ] Test all modified code
- [ ] Remove `#[allow(deprecated)]` attributes if any

## Timeline

- **v0.5.0**: Functions deprecated, warnings issued
- **v0.6.0**: Functions still work but warnings are more prominent
- **v0.7.0**: Functions removed (tentative)

## Need Help?

If you encounter any issues during migration:
1. Check the examples in `syscall_time_ops.rs`
2. Review the comprehensive documentation on `UserSpaceTimer`
3. Open an issue on GitHub with the tag `migration-help`

## Summary

The new `UserSpaceTimer` API is more modern, safer, and easier to use. While the old syscalls still work, we strongly recommend migrating to the new API to benefit from improved type safety and better error handling.

For more information, see the source code documentation in `src/verified/syscall_time_ops.rs`.