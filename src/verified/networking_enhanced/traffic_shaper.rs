//! QoS Traffic Shaping for VANTIS OS
//!
//! Implements traffic classification and rate limiting using token bucket
//! algorithm. Supports multiple traffic classes from real-time to best-effort
//! with per-class and global statistics tracking.

use core::fmt;

// ============================================================================
// Traffic Classification
// ============================================================================

/// Traffic classes ordered by priority (highest first)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TrafficClass {
    /// Real-time traffic (VoIP, video conferencing)
    RealTime,
    /// Interactive traffic (SSH, gaming)
    Interactive,
    /// Business-critical applications
    BusinessCritical,
    /// Streaming media
    Streaming,
    /// Bulk transfer (backups, updates)
    BulkTransfer,
    /// Best-effort (default)
    BestEffort,
}

impl fmt::Display for TrafficClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrafficClass::RealTime => write!(f, "RealTime"),
            TrafficClass::Interactive => write!(f, "Interactive"),
            TrafficClass::BusinessCritical => write!(f, "BusinessCritical"),
            TrafficClass::Streaming => write!(f, "Streaming"),
            TrafficClass::BulkTransfer => write!(f, "BulkTransfer"),
            TrafficClass::BestEffort => write!(f, "BestEffort"),
        }
    }
}

// ============================================================================
// Token Bucket Rate Limiter
// ============================================================================

/// Token bucket algorithm for rate limiting
#[derive(Debug, Clone)]
pub struct TokenBucket {
    /// Maximum number of tokens (burst capacity)
    pub capacity: u64,
    /// Current number of available tokens
    pub tokens: u64,
    /// Token refill rate (tokens per second)
    pub rate: u64,
    /// Last refill timestamp (in microseconds)
    pub last_refill_us: u64,
}

impl TokenBucket {
    /// Create a new token bucket
    pub fn new(capacity: u64, rate: u64) -> Self {
        Self {
            capacity,
            tokens: capacity, // start full
            rate,
            last_refill_us: 0,
        }
    }

    /// Refill tokens based on elapsed time
    pub fn refill(&mut self, current_time_us: u64) {
        if current_time_us <= self.last_refill_us {
            return;
        }
        let elapsed_us = current_time_us - self.last_refill_us;
        let new_tokens = (self.rate as u128 * elapsed_us as u128 / 1_000_000) as u64;
        self.tokens = (self.tokens + new_tokens).min(self.capacity);
        self.last_refill_us = current_time_us;
    }

    /// Try to consume tokens for a packet
    pub fn try_consume(&mut self, tokens: u64, current_time_us: u64) -> bool {
        self.refill(current_time_us);
        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }

    /// Current fill ratio (0.0 to 1.0)
    pub fn fill_ratio(&self) -> f64 {
        if self.capacity == 0 {
            return 0.0;
        }
        self.tokens as f64 / self.capacity as f64
    }

    /// Check if bucket is empty
    pub fn is_empty(&self) -> bool {
        self.tokens == 0
    }
}

// ============================================================================
// QoS Policy
// ============================================================================

/// QoS policy for a traffic class
#[derive(Debug, Clone)]
pub struct QosPolicy {
    pub traffic_class: TrafficClass,
    /// Guaranteed minimum rate in bytes/sec
    pub min_rate_bps: u64,
    /// Maximum (burst) rate in bytes/sec
    pub max_rate_bps: u64,
    /// Burst size in bytes
    pub burst_size: u64,
    /// Maximum latency in microseconds (0 = no limit)
    pub max_latency_us: u64,
    /// Maximum jitter in microseconds (0 = no limit)
    pub max_jitter_us: u64,
    /// Drop probability when congested (0.0 to 1.0)
    pub drop_probability: f64,
    /// Whether this policy is active
    pub is_active: bool,
}

impl QosPolicy {
    pub fn new(traffic_class: TrafficClass, min_rate_bps: u64, max_rate_bps: u64) -> Self {
        Self {
            traffic_class,
            min_rate_bps,
            max_rate_bps,
            burst_size: max_rate_bps / 10, // 100ms burst
            max_latency_us: 0,
            max_jitter_us: 0,
            drop_probability: 0.0,
            is_active: true,
        }
    }

    /// Create a real-time policy with strict latency bounds
    pub fn realtime(min_rate_bps: u64, max_rate_bps: u64, max_latency_us: u64) -> Self {
        Self {
            traffic_class: TrafficClass::RealTime,
            min_rate_bps,
            max_rate_bps,
            burst_size: max_rate_bps / 20,
            max_latency_us,
            max_jitter_us: max_latency_us / 10,
            drop_probability: 0.0,
            is_active: true,
        }
    }

    /// Create a best-effort policy
    pub fn best_effort(max_rate_bps: u64) -> Self {
        Self {
            traffic_class: TrafficClass::BestEffort,
            min_rate_bps: 0,
            max_rate_bps,
            burst_size: max_rate_bps / 5,
            max_latency_us: 0,
            max_jitter_us: 0,
            drop_probability: 0.1,
            is_active: true,
        }
    }
}

// ============================================================================
// Shaping Decision
// ============================================================================

/// Decision made by the traffic shaper for a packet
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapingDecision {
    /// Allow the packet immediately
    Allow,
    /// Delay the packet (queue it)
    Delay { delay_us: u64 },
    /// Drop the packet
    Drop,
    /// Mark the packet for potential drop (ECN)
    Mark,
}

impl fmt::Display for ShapingDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShapingDecision::Allow => write!(f, "ALLOW"),
            ShapingDecision::Delay { delay_us } => write!(f, "DELAY({}μs)", delay_us),
            ShapingDecision::Drop => write!(f, "DROP"),
            ShapingDecision::Mark => write!(f, "MARK"),
        }
    }
}

// ============================================================================
// Traffic Statistics
// ============================================================================

/// Per-class traffic statistics
#[derive(Debug, Clone, Default)]
pub struct ClassStats {
    pub packets_allowed: u64,
    pub packets_delayed: u64,
    pub packets_dropped: u64,
    pub packets_marked: u64,
    pub bytes_allowed: u64,
    pub bytes_dropped: u64,
    pub total_delay_us: u64,
}

impl ClassStats {
    /// Total packets processed
    pub fn total_packets(&self) -> u64 {
        self.packets_allowed + self.packets_delayed + self.packets_dropped + self.packets_marked
    }

    /// Drop rate as a percentage
    pub fn drop_rate(&self) -> f64 {
        let total = self.total_packets();
        if total == 0 {
            return 0.0;
        }
        self.packets_dropped as f64 / total as f64 * 100.0
    }

    /// Average delay in microseconds
    pub fn avg_delay_us(&self) -> f64 {
        if self.packets_delayed == 0 {
            return 0.0;
        }
        self.total_delay_us as f64 / self.packets_delayed as f64
    }
}

// ============================================================================
// Traffic Shaper
// ============================================================================

/// Entry in the shaper combining policy and rate limiter
struct ShaperEntry {
    policy: QosPolicy,
    bucket: TokenBucket,
    stats: ClassStats,
}

/// The main traffic shaper engine
pub struct TrafficShaper {
    entries: Vec<ShaperEntry>,
    global_stats: ClassStats,
    global_rate_limit_bps: u64,
    global_bucket: TokenBucket,
}

impl TrafficShaper {
    /// Create a new traffic shaper with a global rate limit
    pub fn new(global_rate_limit_bps: u64) -> Self {
        let burst = global_rate_limit_bps / 10;
        Self {
            entries: Vec::new(),
            global_stats: ClassStats::default(),
            global_rate_limit_bps,
            global_bucket: TokenBucket::new(burst, global_rate_limit_bps),
        }
    }

    /// Add a QoS policy
    pub fn add_policy(&mut self, policy: QosPolicy) {
        let bucket = TokenBucket::new(policy.burst_size, policy.max_rate_bps);
        self.entries.push(ShaperEntry {
            policy,
            bucket,
            stats: ClassStats::default(),
        });
        // Sort by traffic class priority (highest first)
        self.entries.sort_by(|a, b| a.policy.traffic_class.cmp(&b.policy.traffic_class));
    }

    /// Remove a policy by traffic class
    pub fn remove_policy(&mut self, class: TrafficClass) -> bool {
        let initial = self.entries.len();
        self.entries.retain(|e| e.policy.traffic_class != class);
        self.entries.len() < initial
    }

    /// Get the number of active policies
    pub fn policy_count(&self) -> usize {
        self.entries.len()
    }

    /// Process a packet and return the shaping decision
    pub fn shape_packet(
        &mut self,
        class: TrafficClass,
        packet_size: u64,
        current_time_us: u64,
    ) -> ShapingDecision {
        // Check global rate limit first
        if !self.global_bucket.try_consume(packet_size, current_time_us) {
            self.global_stats.packets_dropped += 1;
            self.global_stats.bytes_dropped += packet_size;
            return ShapingDecision::Drop;
        }

        // Find the matching policy
        let entry = match self.entries.iter_mut().find(|e| e.policy.traffic_class == class) {
            Some(e) => e,
            None => {
                // No policy: allow by default
                self.global_stats.packets_allowed += 1;
                self.global_stats.bytes_allowed += packet_size;
                return ShapingDecision::Allow;
            }
        };

        if !entry.policy.is_active {
            entry.stats.packets_allowed += 1;
            entry.stats.bytes_allowed += packet_size;
            self.global_stats.packets_allowed += 1;
            self.global_stats.bytes_allowed += packet_size;
            return ShapingDecision::Allow;
        }

        // Try to consume tokens from the class bucket
        if entry.bucket.try_consume(packet_size, current_time_us) {
            entry.stats.packets_allowed += 1;
            entry.stats.bytes_allowed += packet_size;
            self.global_stats.packets_allowed += 1;
            self.global_stats.bytes_allowed += packet_size;
            ShapingDecision::Allow
        } else {
            // Bucket empty: decide based on class priority
            match class {
                TrafficClass::RealTime | TrafficClass::Interactive => {
                    // High priority: delay rather than drop
                    let delay = Self::estimate_delay(&entry.bucket, packet_size);
                    entry.stats.packets_delayed += 1;
                    entry.stats.total_delay_us += delay;
                    self.global_stats.packets_delayed += 1;
                    self.global_stats.total_delay_us += delay;
                    ShapingDecision::Delay { delay_us: delay }
                }
                TrafficClass::BusinessCritical | TrafficClass::Streaming => {
                    // Medium priority: mark for potential drop
                    entry.stats.packets_marked += 1;
                    self.global_stats.packets_marked += 1;
                    ShapingDecision::Mark
                }
                TrafficClass::BulkTransfer | TrafficClass::BestEffort => {
                    // Low priority: drop
                    entry.stats.packets_dropped += 1;
                    entry.stats.bytes_dropped += packet_size;
                    self.global_stats.packets_dropped += 1;
                    self.global_stats.bytes_dropped += packet_size;
                    ShapingDecision::Drop
                }
            }
        }
    }

    /// Estimate delay until enough tokens are available
    fn estimate_delay(bucket: &TokenBucket, needed: u64) -> u64 {
        if bucket.rate == 0 {
            return 1_000_000; // 1 second max
        }
        let deficit = needed.saturating_sub(bucket.tokens);
        (deficit as u128 * 1_000_000 / bucket.rate as u128) as u64
    }

    /// Get statistics for a traffic class
    pub fn class_stats(&self, class: TrafficClass) -> Option<&ClassStats> {
        self.entries.iter()
            .find(|e| e.policy.traffic_class == class)
            .map(|e| &e.stats)
    }

    /// Get global statistics
    pub fn global_stats(&self) -> &ClassStats {
        &self.global_stats
    }

    /// Reset all statistics
    pub fn reset_stats(&mut self) {
        self.global_stats = ClassStats::default();
        for entry in &mut self.entries {
            entry.stats = ClassStats::default();
        }
    }

    /// Get the global rate limit
    pub fn global_rate_limit(&self) -> u64 {
        self.global_rate_limit_bps
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_shaper() -> TrafficShaper {
        let mut shaper = TrafficShaper::new(1_000_000_000); // 1 Gbps global
        shaper.add_policy(QosPolicy::realtime(100_000_000, 200_000_000, 1000));
        shaper.add_policy(QosPolicy::new(TrafficClass::Interactive, 50_000_000, 100_000_000));
        shaper.add_policy(QosPolicy::new(TrafficClass::Streaming, 200_000_000, 500_000_000));
        shaper.add_policy(QosPolicy::best_effort(100_000_000));
        shaper
    }

    #[test]
    fn test_token_bucket_basic() {
        let mut bucket = TokenBucket::new(1000, 100);
        assert_eq!(bucket.tokens, 1000);
        assert!(bucket.try_consume(500, 0));
        assert_eq!(bucket.tokens, 500);
        assert!(bucket.try_consume(500, 0));
        assert!(!bucket.try_consume(1, 0)); // empty
    }

    #[test]
    fn test_token_bucket_refill() {
        let mut bucket = TokenBucket::new(1000, 100);
        bucket.tokens = 0;
        bucket.last_refill_us = 0;
        bucket.refill(1_000_000); // 1 second later
        assert_eq!(bucket.tokens, 100); // 100 tokens/sec * 1 sec
    }

    #[test]
    fn test_token_bucket_fill_ratio() {
        let bucket = TokenBucket::new(1000, 100);
        assert!((bucket.fill_ratio() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_shaper_creation() {
        let shaper = setup_shaper();
        assert_eq!(shaper.policy_count(), 4);
    }

    #[test]
    fn test_shaper_allow() {
        let mut shaper = setup_shaper();
        let decision = shaper.shape_packet(TrafficClass::RealTime, 1500, 0);
        assert_eq!(decision, ShapingDecision::Allow);
    }

    #[test]
    fn test_shaper_drop_best_effort() {
        let mut shaper = TrafficShaper::new(1_000_000_000);
        // Very small bucket for best effort
        shaper.add_policy(QosPolicy {
            traffic_class: TrafficClass::BestEffort,
            min_rate_bps: 0,
            max_rate_bps: 1,
            burst_size: 1,
            max_latency_us: 0,
            max_jitter_us: 0,
            drop_probability: 0.5,
            is_active: true,
        });

        // First packet might succeed, subsequent should drop
        let _ = shaper.shape_packet(TrafficClass::BestEffort, 100, 0);
        let decision = shaper.shape_packet(TrafficClass::BestEffort, 100, 0);
        assert_eq!(decision, ShapingDecision::Drop);
    }

    #[test]
    fn test_shaper_delay_realtime() {
        let mut shaper = TrafficShaper::new(1_000_000_000);
        shaper.add_policy(QosPolicy {
            traffic_class: TrafficClass::RealTime,
            min_rate_bps: 100,
            max_rate_bps: 100,
            burst_size: 10,
            max_latency_us: 1000,
            max_jitter_us: 100,
            drop_probability: 0.0,
            is_active: true,
        });

        // Exhaust the bucket
        let _ = shaper.shape_packet(TrafficClass::RealTime, 10, 0);
        let decision = shaper.shape_packet(TrafficClass::RealTime, 100, 0);
        assert!(matches!(decision, ShapingDecision::Delay { .. }));
    }

    #[test]
    fn test_shaper_stats() {
        let mut shaper = setup_shaper();
        shaper.shape_packet(TrafficClass::RealTime, 1500, 0);
        shaper.shape_packet(TrafficClass::RealTime, 1500, 1000);

        let stats = shaper.class_stats(TrafficClass::RealTime).unwrap();
        assert!(stats.total_packets() >= 2);

        let global = shaper.global_stats();
        assert!(global.total_packets() >= 2);
    }

    #[test]
    fn test_remove_policy() {
        let mut shaper = setup_shaper();
        assert_eq!(shaper.policy_count(), 4);
        assert!(shaper.remove_policy(TrafficClass::BestEffort));
        assert_eq!(shaper.policy_count(), 3);
        assert!(!shaper.remove_policy(TrafficClass::BestEffort)); // already removed
    }

    #[test]
    fn test_reset_stats() {
        let mut shaper = setup_shaper();
        shaper.shape_packet(TrafficClass::RealTime, 1500, 0);
        shaper.reset_stats();
        assert_eq!(shaper.global_stats().total_packets(), 0);
    }

    #[test]
    fn test_class_stats_drop_rate() {
        let mut stats = ClassStats::default();
        stats.packets_allowed = 90;
        stats.packets_dropped = 10;
        assert!((stats.drop_rate() - 10.0).abs() < 1e-5);
    }
}