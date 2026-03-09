//! Installation Progress Module
//!
//! Provides progress tracking for the installer with:
//! - Step progress
//! - Overall progress
//! - Time estimation
//! - Status messages

use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Installation step
#[derive(Debug, Clone)]
pub struct InstallStep {
    /// Step ID
    pub id: u32,
    /// Step name
    pub name: String,
    /// Step description
    pub description: String,
    /// Step status
    pub status: StepStatus,
    /// Progress percentage (0-100)
    pub progress: u8,
    /// Duration in milliseconds
    pub duration: u64,
}

/// Step status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepStatus {
    /// Step is pending
    Pending,
    /// Step is in progress
    InProgress,
    /// Step completed successfully
    Completed,
    /// Step failed
    Failed,
    /// Step was skipped
    Skipped,
}

/// Installation phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallPhase {
    /// Preparing installation
    Prepare = 0,
    /// Partitioning disk
    Partition = 1,
    /// Formatting filesystems
    Format = 2,
    /// Copying system files
    CopyFiles = 3,
    /// Configuring bootloader
    Bootloader = 4,
    /// Configuring system
    Configure = 5,
    /// Creating user accounts
    Users = 6,
    /// Finalizing installation
    Finalize = 7,
    /// Installation complete
    Complete = 8,
}

/// Installation progress tracker
pub struct InstallerProgress {
    /// Current phase
    current_phase: AtomicU32,
    /// Current step
    current_step: AtomicU32,
    /// Overall progress (0-100)
    overall_progress: AtomicU32,
    /// Phase progress (0-100)
    phase_progress: AtomicU32,
    /// Start time (milliseconds since epoch)
    start_time: AtomicU64,
    /// Estimated remaining time (seconds)
    estimated_remaining: AtomicU32,
    /// Total steps
    total_steps: u32,
    /// Completed steps
    completed_steps: AtomicU32,
    /// Step list
    steps: Vec<InstallStep>,
}

impl InstallerProgress {
    /// Create a new progress tracker
    pub const fn new() -> Self {
        Self {
            current_phase: AtomicU32::new(InstallPhase::Prepare as u32),
            current_step: AtomicU32::new(0),
            overall_progress: AtomicU32::new(0),
            phase_progress: AtomicU32::new(0),
            start_time: AtomicU64::new(0),
            estimated_remaining: AtomicU32::new(0),
            total_steps: 0,
            completed_steps: AtomicU32::new(0),
            steps: Vec::new(),
        }
    }

    /// Reset progress tracker
    pub fn reset(&mut self) {
        self.current_phase.store(InstallPhase::Prepare as u32, Ordering::SeqCst);
        self.current_step.store(0, Ordering::SeqCst);
        self.overall_progress.store(0, Ordering::SeqCst);
        self.phase_progress.store(0, Ordering::SeqCst);
        self.start_time.store(0, Ordering::SeqCst);
        self.estimated_remaining.store(0, Ordering::SeqCst);
        self.total_steps = 0;
        self.completed_steps.store(0, Ordering::SeqCst);
        self.steps.clear();
    }

    /// Initialize progress tracker with steps
    pub fn init_steps(&mut self, steps: Vec<InstallStep>) {
        self.total_steps = steps.len() as u32;
        self.steps = steps;
    }

    /// Start installation
    pub fn start(&self) {
        self.start_time.store(
            chrono_timestamp_millis(),
            Ordering::SeqCst
        );
        self.set_phase(InstallPhase::Prepare);
    }

    /// Set current phase
    pub fn set_phase(&self, phase: InstallPhase) {
        self.current_phase.store(phase as u32, Ordering::SeqCst);
        self.phase_progress.store(0, Ordering::SeqCst);
        self.update_overall_progress();
    }

    /// Set phase progress
    pub fn set_phase_progress(&self, progress: u8) {
        self.phase_progress.store(progress.min(100) as u32, Ordering::SeqCst);
        self.update_overall_progress();
    }

    /// Set current step
    pub fn set_step(&self, step_index: u32) {
        self.current_step.store(step_index, Ordering::SeqCst);
    }

    /// Complete current step
    pub fn complete_step(&self) {
        let completed = self.completed_steps.fetch_add(1, Ordering::SeqCst) + 1;
        let total = self.total_steps;
        
        if total > 0 {
            let progress = ((completed as u32 * 100) / total) as u8;
            self.set_overall_progress(progress);
        }
    }

    /// Set overall progress
    pub fn set_overall_progress(&self, progress: u8) {
        self.overall_progress.store(progress.min(100) as u32, Ordering::SeqCst);
        self.update_estimated_remaining();
    }

    /// Get current phase
    pub fn current_phase(&self) -> InstallPhase {
        match self.current_phase.load(Ordering::SeqCst) {
            0 => InstallPhase::Prepare,
            1 => InstallPhase::Partition,
            2 => InstallPhase::Format,
            3 => InstallPhase::CopyFiles,
            4 => InstallPhase::Bootloader,
            5 => InstallPhase::Configure,
            6 => InstallPhase::Users,
            7 => InstallPhase::Finalize,
            _ => InstallPhase::Complete,
        }
    }

    /// Get overall progress
    pub fn overall_progress(&self) -> u8 {
        self.overall_progress.load(Ordering::SeqCst) as u8
    }

    /// Get phase progress
    pub fn phase_progress(&self) -> u8 {
        self.phase_progress.load(Ordering::SeqCst) as u8
    }

    /// Get elapsed time in milliseconds
    pub fn elapsed_time(&self) -> u64 {
        let start = self.start_time.load(Ordering::SeqCst);
        if start == 0 {
            return 0;
        }
        chrono_timestamp_millis().saturating_sub(start)
    }

    /// Get estimated remaining time in seconds
    pub fn estimated_remaining(&self) -> u32 {
        self.estimated_remaining.load(Ordering::SeqCst)
    }

    /// Update overall progress based on phase
    fn update_overall_progress(&self) {
        let phase = self.current_phase();
        let phase_progress = self.phase_progress.load(Ordering::SeqCst);
        
        // Calculate overall progress based on phase weight
        let phase_weight = match phase {
            InstallPhase::Prepare => 0.0,
            InstallPhase::Partition => 0.05,
            InstallPhase::Format => 0.10,
            InstallPhase::CopyFiles => 0.80,
            InstallPhase::Bootloader => 0.85,
            InstallPhase::Configure => 0.90,
            InstallPhase::Users => 0.95,
            InstallPhase::Finalize => 0.99,
            InstallPhase::Complete => 1.0,
        };
        
        let prev_phase_weight = phase_weight;
        let next_phase_weight = match phase {
            InstallPhase::Prepare => 0.05,
            InstallPhase::Partition => 0.10,
            InstallPhase::Format => 0.80,
            InstallPhase::CopyFiles => 0.85,
            InstallPhase::Bootloader => 0.90,
            InstallPhase::Configure => 0.95,
            InstallPhase::Users => 0.99,
            InstallPhase::Finalize => 1.0,
            InstallPhase::Complete => 1.0,
        };
        
        let phase_range = next_phase_weight - prev_phase_weight;
        let overall = prev_phase_weight + (phase_range * (phase_progress as f64 / 100.0));
        
        self.overall_progress.store((overall * 100.0) as u32, Ordering::SeqCst);
    }

    /// Update estimated remaining time
    fn update_estimated_remaining(&self) {
        let elapsed = self.elapsed_time();
        let progress = self.overall_progress.load(Ordering::SeqCst);
        
        if progress == 0 || progress >= 100 {
            self.estimated_remaining.store(0, Ordering::SeqCst);
            return;
        }
        
        // Estimate based on elapsed time and progress
        // total_time = elapsed * 100 / progress
        // remaining = total_time - elapsed
        let total_estimated_ms = (elapsed as f64 * 100.0 / progress as f64) as u64;
        let remaining_ms = total_estimated_ms.saturating_sub(elapsed);
        let remaining_seconds = (remaining_ms / 1000) as u32;
        
        self.estimated_remaining.store(remaining_seconds, Ordering::SeqCst);
    }

    /// Get status message for current phase
    pub fn status_message(&self) -> String {
        match self.current_phase() {
            InstallPhase::Prepare => String::from("Preparing installation..."),
            InstallPhase::Partition => String::from("Partitioning disk..."),
            InstallPhase::Format => String::from("Formatting filesystems..."),
            InstallPhase::CopyFiles => String::from("Copying system files..."),
            InstallPhase::Bootloader => String::from("Installing bootloader..."),
            InstallPhase::Configure => String::from("Configuring system..."),
            InstallPhase::Users => String::from("Creating user accounts..."),
            InstallPhase::Finalize => String::from("Finalizing installation..."),
            InstallPhase::Complete => String::from("Installation complete!"),
        }
    }
}

impl Default for InstallerProgress {
    fn default() -> Self {
        Self::new()
    }
}

/// Get current timestamp in milliseconds (placeholder)
fn chrono_timestamp_millis() -> u64 {
    // Placeholder: In real implementation, use system time
    0
}