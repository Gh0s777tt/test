//! Performance Profiler for VANTIS OS
//!
//! Provides hierarchical span-based profiling for kernel subsystems.
//! Tracks execution time, call counts, and generates flame-graph-compatible
//! output for performance analysis.

use core::fmt;

// ============================================================================
// Profiling Types
// ============================================================================

/// A profiling span representing a timed code region
#[derive(Debug, Clone)]
pub struct ProfileSpan {
    pub id: u64,
    pub name: String,
    pub parent_id: Option<u64>,
    pub start_ns: u64,
    pub end_ns: Option<u64>,
    pub call_count: u64,
    pub total_time_ns: u64,
    pub min_time_ns: u64,
    pub max_time_ns: u64,
    pub children: Vec<u64>,
}

impl ProfileSpan {
    pub fn new(id: u64, name: &str, parent_id: Option<u64>, start_ns: u64) -> Self {
        Self {
            id,
            name: name.to_string(),
            parent_id,
            start_ns,
            end_ns: None,
            call_count: 0,
            total_time_ns: 0,
            min_time_ns: u64::MAX,
            max_time_ns: 0,
            children: Vec::new(),
        }
    }

    /// Duration of the current span in nanoseconds
    pub fn duration_ns(&self) -> u64 {
        match self.end_ns {
            Some(end) => end.saturating_sub(self.start_ns),
            None => 0,
        }
    }

    /// Average time per call in nanoseconds
    pub fn avg_time_ns(&self) -> f64 {
        if self.call_count == 0 {
            return 0.0;
        }
        self.total_time_ns as f64 / self.call_count as f64
    }

    /// Duration in microseconds
    pub fn duration_us(&self) -> f64 {
        self.duration_ns() as f64 / 1000.0
    }

    /// Duration in milliseconds
    pub fn duration_ms(&self) -> f64 {
        self.duration_ns() as f64 / 1_000_000.0
    }

    /// Record a completed invocation
    pub fn record(&mut self, duration_ns: u64) {
        self.call_count += 1;
        self.total_time_ns += duration_ns;
        self.min_time_ns = self.min_time_ns.min(duration_ns);
        self.max_time_ns = self.max_time_ns.max(duration_ns);
    }
}

/// Profiler state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfilerState {
    Idle,
    Recording,
    Paused,
    Stopped,
}

// ============================================================================
// Profiler Report
// ============================================================================

/// A summary entry in the profiler report
#[derive(Debug, Clone)]
pub struct ProfileEntry {
    pub name: String,
    pub call_count: u64,
    pub total_time_ns: u64,
    pub avg_time_ns: f64,
    pub min_time_ns: u64,
    pub max_time_ns: u64,
    pub pct_of_total: f64,
}

/// Complete profiler report
#[derive(Debug, Clone)]
pub struct ProfileReport {
    pub entries: Vec<ProfileEntry>,
    pub total_time_ns: u64,
    pub total_spans: usize,
    pub recording_duration_ns: u64,
}

impl ProfileReport {
    /// Get the top N hottest spans by total time
    pub fn top_n(&self, n: usize) -> Vec<&ProfileEntry> {
        let mut sorted: Vec<&ProfileEntry> = self.entries.iter().collect();
        sorted.sort_by(|a, b| b.total_time_ns.cmp(&a.total_time_ns));
        sorted.truncate(n);
        sorted
    }

    /// Format as a simple text table
    pub fn to_table(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("{:<30} {:>8} {:>12} {:>12} {:>8}\n",
            "Span", "Calls", "Total(μs)", "Avg(μs)", "Pct%"));
        output.push_str(&"-".repeat(74));
        output.push('\n');

        let mut sorted = self.entries.clone();
        sorted.sort_by(|a, b| b.total_time_ns.cmp(&a.total_time_ns));

        for entry in &sorted {
            output.push_str(&format!("{:<30} {:>8} {:>12.1} {:>12.1} {:>7.1}%\n",
                entry.name,
                entry.call_count,
                entry.total_time_ns as f64 / 1000.0,
                entry.avg_time_ns / 1000.0,
                entry.pct_of_total,
            ));
        }
        output
    }
}

// ============================================================================
// Performance Profiler
// ============================================================================

/// Error types for the profiler
#[derive(Debug, Clone, PartialEq)]
pub enum ProfilerError {
    NotRecording,
    AlreadyRecording,
    SpanNotFound(u64),
    SpanAlreadyClosed(u64),
    InvalidState(ProfilerState),
}

impl fmt::Display for ProfilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfilerError::NotRecording => write!(f, "Profiler is not recording"),
            ProfilerError::AlreadyRecording => write!(f, "Profiler is already recording"),
            ProfilerError::SpanNotFound(id) => write!(f, "Span {} not found", id),
            ProfilerError::SpanAlreadyClosed(id) => write!(f, "Span {} already closed", id),
            ProfilerError::InvalidState(s) => write!(f, "Invalid profiler state: {:?}", s),
        }
    }
}

/// The main performance profiler
pub struct Profiler {
    spans: Vec<ProfileSpan>,
    active_span_stack: Vec<u64>,
    next_span_id: u64,
    state: ProfilerState,
    recording_start_ns: u64,
    recording_end_ns: u64,
}

impl Profiler {
    /// Create a new profiler
    pub fn new() -> Self {
        Self {
            spans: Vec::new(),
            active_span_stack: Vec::new(),
            next_span_id: 1,
            state: ProfilerState::Idle,
            recording_start_ns: 0,
            recording_end_ns: 0,
        }
    }

    /// Start recording
    pub fn start(&mut self, timestamp_ns: u64) -> Result<(), ProfilerError> {
        if self.state == ProfilerState::Recording {
            return Err(ProfilerError::AlreadyRecording);
        }
        self.state = ProfilerState::Recording;
        self.recording_start_ns = timestamp_ns;
        Ok(())
    }

    /// Stop recording
    pub fn stop(&mut self, timestamp_ns: u64) -> Result<(), ProfilerError> {
        if self.state != ProfilerState::Recording && self.state != ProfilerState::Paused {
            return Err(ProfilerError::NotRecording);
        }
        self.state = ProfilerState::Stopped;
        self.recording_end_ns = timestamp_ns;
        Ok(())
    }

    /// Pause recording
    pub fn pause(&mut self) -> Result<(), ProfilerError> {
        if self.state != ProfilerState::Recording {
            return Err(ProfilerError::NotRecording);
        }
        self.state = ProfilerState::Paused;
        Ok(())
    }

    /// Resume recording
    pub fn resume(&mut self) -> Result<(), ProfilerError> {
        if self.state != ProfilerState::Paused {
            return Err(ProfilerError::InvalidState(self.state));
        }
        self.state = ProfilerState::Recording;
        Ok(())
    }

    /// Begin a new profiling span
    pub fn begin_span(&mut self, name: &str, timestamp_ns: u64) -> Result<u64, ProfilerError> {
        if self.state != ProfilerState::Recording {
            return Err(ProfilerError::NotRecording);
        }

        let parent_id = self.active_span_stack.last().copied();
        let span_id = self.next_span_id;
        self.next_span_id += 1;

        let span = ProfileSpan::new(span_id, name, parent_id, timestamp_ns);
        self.spans.push(span);

        // Add as child of parent
        if let Some(pid) = parent_id {
            if let Some(parent) = self.spans.iter_mut().find(|s| s.id == pid) {
                parent.children.push(span_id);
            }
        }

        self.active_span_stack.push(span_id);
        Ok(span_id)
    }

    /// End a profiling span
    pub fn end_span(&mut self, span_id: u64, timestamp_ns: u64) -> Result<u64, ProfilerError> {
        let span = self.spans.iter_mut()
            .find(|s| s.id == span_id)
            .ok_or(ProfilerError::SpanNotFound(span_id))?;

        if span.end_ns.is_some() {
            return Err(ProfilerError::SpanAlreadyClosed(span_id));
        }

        span.end_ns = Some(timestamp_ns);
        let duration = timestamp_ns.saturating_sub(span.start_ns);
        span.record(duration);

        // Pop from active stack
        if let Some(pos) = self.active_span_stack.iter().rposition(|&id| id == span_id) {
            self.active_span_stack.remove(pos);
        }

        Ok(duration)
    }

    /// Record a pre-measured span (no begin/end needed)
    pub fn record_span(&mut self, name: &str, duration_ns: u64) -> Result<(), ProfilerError> {
        if self.state != ProfilerState::Recording {
            return Err(ProfilerError::NotRecording);
        }

        // Find existing span with same name or create new
        if let Some(span) = self.spans.iter_mut().find(|s| s.name == name) {
            span.record(duration_ns);
        } else {
            let span_id = self.next_span_id;
            self.next_span_id += 1;
            let mut span = ProfileSpan::new(span_id, name, None, 0);
            span.record(duration_ns);
            span.end_ns = Some(duration_ns);
            self.spans.push(span);
        }

        Ok(())
    }

    /// Generate a profiling report
    pub fn report(&self) -> ProfileReport {
        let total_time: u64 = self.spans.iter()
            .filter(|s| s.parent_id.is_none())
            .map(|s| s.total_time_ns)
            .sum();

        let entries: Vec<ProfileEntry> = self.spans.iter().map(|span| {
            let pct = if total_time > 0 {
                span.total_time_ns as f64 / total_time as f64 * 100.0
            } else {
                0.0
            };

            ProfileEntry {
                name: span.name.clone(),
                call_count: span.call_count,
                total_time_ns: span.total_time_ns,
                avg_time_ns: span.avg_time_ns(),
                min_time_ns: if span.min_time_ns == u64::MAX { 0 } else { span.min_time_ns },
                max_time_ns: span.max_time_ns,
                pct_of_total: pct,
            }
        }).collect();

        ProfileReport {
            entries,
            total_time_ns: total_time,
            total_spans: self.spans.len(),
            recording_duration_ns: self.recording_end_ns.saturating_sub(self.recording_start_ns),
        }
    }

    /// Reset the profiler
    pub fn reset(&mut self) {
        self.spans.clear();
        self.active_span_stack.clear();
        self.next_span_id = 1;
        self.state = ProfilerState::Idle;
        self.recording_start_ns = 0;
        self.recording_end_ns = 0;
    }

    /// Get profiler state
    pub fn state(&self) -> ProfilerState {
        self.state
    }

    /// Get number of recorded spans
    pub fn span_count(&self) -> usize {
        self.spans.len()
    }

    /// Get a span by ID
    pub fn get_span(&self, id: u64) -> Option<&ProfileSpan> {
        self.spans.iter().find(|s| s.id == id)
    }
}

impl Default for Profiler {
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
    fn test_profiler_lifecycle() {
        let mut profiler = Profiler::new();
        assert_eq!(profiler.state(), ProfilerState::Idle);

        profiler.start(0).unwrap();
        assert_eq!(profiler.state(), ProfilerState::Recording);

        profiler.stop(1000).unwrap();
        assert_eq!(profiler.state(), ProfilerState::Stopped);
    }

    #[test]
    fn test_begin_end_span() {
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();

        let span_id = profiler.begin_span("test_fn", 100).unwrap();
        let duration = profiler.end_span(span_id, 500).unwrap();

        assert_eq!(duration, 400);
        assert_eq!(profiler.span_count(), 1);

        let span = profiler.get_span(span_id).unwrap();
        assert_eq!(span.call_count, 1);
        assert_eq!(span.total_time_ns, 400);
    }

    #[test]
    fn test_nested_spans() {
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();

        let outer = profiler.begin_span("outer", 100).unwrap();
        let inner = profiler.begin_span("inner", 200).unwrap();
        profiler.end_span(inner, 400).unwrap();
        profiler.end_span(outer, 500).unwrap();

        let outer_span = profiler.get_span(outer).unwrap();
        assert!(outer_span.children.contains(&inner));

        let inner_span = profiler.get_span(inner).unwrap();
        assert_eq!(inner_span.parent_id, Some(outer));
    }

    #[test]
    fn test_record_span() {
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();

        profiler.record_span("fast_op", 100).unwrap();
        profiler.record_span("fast_op", 200).unwrap();
        profiler.record_span("fast_op", 150).unwrap();

        let span = profiler.spans.iter().find(|s| s.name == "fast_op").unwrap();
        assert_eq!(span.call_count, 3);
        assert_eq!(span.total_time_ns, 450);
        assert_eq!(span.min_time_ns, 100);
        assert_eq!(span.max_time_ns, 200);
    }

    #[test]
    fn test_report_generation() {
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();

        profiler.record_span("alloc", 1000).unwrap();
        profiler.record_span("compute", 5000).unwrap();
        profiler.record_span("io", 3000).unwrap();

        profiler.stop(10000).unwrap();

        let report = profiler.report();
        assert_eq!(report.total_spans, 3);
        assert!(report.total_time_ns > 0);

        let top = report.top_n(1);
        assert_eq!(top[0].name, "compute");
    }

    #[test]
    fn test_report_table() {
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();
        profiler.record_span("test", 1000).unwrap();
        profiler.stop(2000).unwrap();

        let report = profiler.report();
        let table = report.to_table();
        assert!(table.contains("test"));
    }

    #[test]
    fn test_pause_resume() {
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();
        profiler.pause().unwrap();
        assert_eq!(profiler.state(), ProfilerState::Paused);
        profiler.resume().unwrap();
        assert_eq!(profiler.state(), ProfilerState::Recording);
    }

    #[test]
    fn test_not_recording_error() {
        let mut profiler = Profiler::new();
        let result = profiler.begin_span("test", 0);
        assert!(matches!(result, Err(ProfilerError::NotRecording)));
    }

    #[test]
    fn test_span_already_closed() {
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();
        let id = profiler.begin_span("test", 0).unwrap();
        profiler.end_span(id, 100).unwrap();
        let result = profiler.end_span(id, 200);
        assert!(matches!(result, Err(ProfilerError::SpanAlreadyClosed(_))));
    }

    #[test]
    fn test_reset() {
        let mut profiler = Profiler::new();
        profiler.start(0).unwrap();
        profiler.record_span("test", 100).unwrap();
        profiler.reset();
        assert_eq!(profiler.state(), ProfilerState::Idle);
        assert_eq!(profiler.span_count(), 0);
    }

    #[test]
    fn test_span_duration_helpers() {
        let mut span = ProfileSpan::new(1, "test", None, 1000);
        span.end_ns = Some(2500);
        assert_eq!(span.duration_ns(), 1500);
        assert!((span.duration_us() - 1.5).abs() < 1e-5);
        assert!((span.duration_ms() - 0.0015).abs() < 1e-7);
    }
}