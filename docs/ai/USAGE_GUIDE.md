# VantisOS AI Module Usage Guide

## Table of Contents

- [Getting Started](#getting-started)
- [Quick Start Examples](#quick-start-examples)
- [Advanced Usage](#advanced-usage)
- [Configuration](#configuration)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)
- [Common Use Cases](#common-use-cases)
- [Performance Tuning](#performance-tuning)

---

## Getting Started

### Prerequisites

- VantisOS v0.4.1 or later
- Rust 1.70+ for development
- Minimum 512MB RAM recommended for AI features
- Multi-core CPU (4+ cores recommended)

### Basic Setup

The AI module is initialized automatically when VantisOS boots. To verify it's running:

```rust
use vantisos::ai::{AIModule, AIConfig};

fn check_ai_status() {
    // Check if AI module is enabled
    let config = AIConfig::from_file("/etc/vantisos/ai_config.toml")?;
    
    if config.enabled {
        println!("AI module is enabled");
        println!("Scheduler: {}", config.scheduler.enabled);
        println!("Power Manager: {}", config.power.enabled);
        println!("Security: {}", config.security.enabled);
    }
}
```

---

## Quick Start Examples

### Example 1: Basic AI Initialization

```rust
use vantisos::ai::{AIModule, AIConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = AIConfig::default();
    
    // Initialize AI module
    let mut ai = AIModule::init(config)?;
    
    println!("AI Module initialized successfully");
    
    // Get resource usage
    let usage = ai.get_resource_usage();
    println!("Active models: {}", usage.model_count);
    println!("Total memory: {} MB", usage.total_memory_mb);
    
    // Cleanup
    ai.shutdown()?;
    
    Ok(())
}
```

### Example 2: Using the ML Scheduler

```rust
use vantisos::ai::Scheduler;

fn schedule_process_example() -> Result<(), Box<dyn std::error::Error>> {
    // Schedule a process with priority 50
    let pid = 1234u32;
    let priority = 50u8;
    
    let decision = Scheduler::schedule_process(pid, priority)?;
    
    println!("Process {} scheduled:", pid);
    println!("  Core: {}", decision.core);
    println!("  Priority: {}", decision.priority);
    println!("  Time slice: {} ms", decision.time_slice_ms);
    
    // Get scheduler statistics
    let stats = Scheduler::get_stats();
    println!("Total processes scheduled: {}", stats.total_scheduled);
    println!("Average latency: {:.2} ms", stats.avg_latency_ms);
    
    Ok(())
}
```

### Example 3: Adaptive Power Management

```rust
use vantisos::ai::{PowerManager, PowerState};

fn power_management_example() -> Result<(), Box<dyn std::error::Error>> {
    // Get current workload (0-100)
    let workload = 75u8;
    
    // Get power decision
    let decision = PowerManager::get_power_decision(workload)?;
    
    println!("Workload: {}%", workload);
    println!("Recommended state: {:?}", decision.state);
    println!("CPU frequency: {} MHz", decision.frequency_mhz);
    println!("Confidence: {:?}", decision.confidence);
    
    // Apply the decision
    let mut pm = PowerManager;
    pm.apply_decision(decision)?;
    
    println!("Power decision applied successfully");
    
    Ok(())
}
```

### Example 4: Threat Detection

```rust
use vantisos::ai::{Security, ThreatLevel};

fn security_example() -> Result<(), Box<dyn std::error::Error>> {
    // Analyze suspicious behavior
    let behavior = "process attempting to access kernel memory";
    
    let analysis = Security::analyze_behavior(behavior)?;
    
    println!("Behavior: {}", behavior);
    println!("Threat level: {:?}", analysis.level);
    println!("Confidence: {:?}", analysis.confidence);
    println!("Action: {:?}", analysis.action);
    println!("Description: {}", analysis.description);
    
    // Take action based on threat level
    match analysis.level {
        ThreatLevel::Critical => {
            println!("CRITICAL THREAT! Taking immediate action...");
            // Block and alert
        }
        ThreatLevel::High => {
            println!("High threat detected. Monitoring closely...");
            // Log and alert
        }
        ThreatLevel::Medium => {
            println!("Medium threat. Logging event...");
            // Log only
        }
        ThreatLevel::Low => {
            println!("Low threat. No action needed.");
        }
    }
    
    Ok(())
}
```

### Example 5: System Monitoring

```rust
use vantisos::ai::Monitoring;

fn monitoring_example() -> Result<(), Box<dyn std::error::Error>> {
    let monitoring = Monitoring;
    
    // Get current metrics
    let metrics = monitoring.get_metrics();
    
    println!("=== AI System Metrics ===");
    println!("CPU Usage: {:.1}%", metrics.cpu_usage_percent);
    println!("Memory Usage: {:.1}%", metrics.memory_usage_percent);
    println!("Disk I/O: {:.1}%", metrics.disk_io_percent);
    println!("Network I/O: {:.1}%", metrics.network_io_percent);
    println!("Inference Latency: {:.2} ms", metrics.inference_latency_ms);
    println!("Timestamp: {}", metrics.timestamp);
    
    // Check for model drift
    if let Some(alert) = monitoring.check_drift() {
        println!("\n⚠️  Model Drift Detected!");
        println!("Component: {}", alert.component);
        println!("Drift value: {:.2}", alert.drift_value);
        println!("Threshold: {:.2}", alert.threshold);
        println!("Recommendation: {}", alert.recommendation);
    } else {
        println!("\n✓ No model drift detected");
    }
    
    Ok(())
}
```

### Example 6: Natural Language Commands

```rust
use vantisos::ai::{NLP, CommandIntent};

fn nlp_example() -> Result<(), Box<dyn std::error::Error>> {
    // Parse natural language command
    let input = "show me the running processes";
    
    let cmd = NLP::parse_command(input)?;
    
    println!("Input: &quot;{}&quot;", cmd.raw_input);
    println!("Intent: {:?}", cmd.intent);
    println!("Confidence: {:?}", cmd.confidence);
    
    // Handle the command
    match cmd.intent {
        CommandIntent::ShowStatus => {
            println!("→ Showing system status...");
            // Execute status command
        }
        CommandIntent::ListProcesses => {
            println!("→ Listing processes...");
            // Execute process list command
        }
        CommandIntent::ShowMetrics => {
            println!("→ Showing metrics...");
            // Execute metrics command
        }
        _ => {
            println!("→ Unknown command");
        }
    }
    
    Ok(())
}
```

---

## Advanced Usage

### Custom Configuration

```rust
use vantisos::ai::{AIModule, AIConfig, SchedulerConfig, PowerConfig};

fn custom_config_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create custom configuration
    let config = AIConfig {
        enabled: true,
        scheduler: SchedulerConfig {
            enabled: true,
            max_latency_ms: 5,        // Lower latency
            min_time_slice_ms: 10,
            max_time_slice_ms: 50,
        },
        power: PowerConfig {
            enabled: true,
            power_mode: vantisos::ai::PowerState::Performance,
            min_cpu_freq_mhz: 1000,
            max_cpu_freq_mhz: 4000,
        },
        ..Default::default()
    };
    
    // Validate configuration
    config.validate()?;
    
    // Initialize with custom config
    let mut ai = AIModule::init(config)?;
    
    // Use AI module...
    
    ai.shutdown()?;
    Ok(())
}
```

### Data Collection and Processing

```rust
use vantisos::ai::{DataCollector, DataProcessor, CollectorConfig, ProcessorConfig};

fn data_pipeline_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create data collector
    let collector_config = CollectorConfig {
        buffer_size: 1024,
        max_value: 100.0,
        min_value: 0.0,
    };
    let mut collector = DataCollector::new(collector_config);
    
    // Collect some data points
    for i in 0..100 {
        let value = (i as f64 * 3.14) % 100.0;
        collector.collect(value)?;
    }
    
    // Get collected data
    let data = collector.get_data();
    println!("Collected {} data points", data.len());
    
    // Process data
    let processor_config = ProcessorConfig {
        window_size: 10,
        normalization_method: vantisos::ai::NormalizationMethod::MinMax,
    };
    let processor = DataProcessor::new(processor_config);
    
    let features = processor.process(data);
    
    println!("=== Data Features ===");
    println!("Mean: {:.2}", features.mean);
    println!("Std Dev: {:.2}", features.std);
    println!("Min: {:.2}", features.min);
    println!("Max: {:.2}", features.max);
    println!("50th percentile: {:.2}", features.percentiles[1]);
    
    Ok(())
}
```

### Model Training with Privacy

```rust
use vantisos::ai::{Trainer, TrainerConfig};

fn training_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create trainer with differential privacy
    let config = TrainerConfig {
        privacy_epsilon: 1.0,      // Privacy budget
        learning_rate: 0.01,
        epochs: 100,
        batch_size: 32,
    };
    
    let mut trainer = Trainer::new(config);
    
    // Generate training data
    let training_data: Vec<f64> = (0..1000)
        .map(|i| (i as f64 / 1000.0) * 100.0)
        .collect();
    
    println!("Training model with {} samples...", training_data.len());
    
    // Train the model
    let model = trainer.train(&training_data)?;
    
    println!("Model trained successfully!");
    println!("Model ID: {}", model.id);
    println!("Version: {}", model.version);
    println!("Accuracy: {:.2}%", model.accuracy * 100.0);
    
    // Validate with test data
    let test_data: Vec<f64> = (0..100)
        .map(|i| (i as f64 / 100.0) * 100.0)
        .collect();
    
    let validation_score = trainer.validate(&model, &test_data);
    println!("Validation score: {:.2}%", validation_score * 100.0);
    
    Ok(())
}
```

### Predictive Maintenance

```rust
use vantisos::ai::{Maintenance, SystemMetrics, HealthTrend};

fn predictive_maintenance_example() -> Result<(), Box<dyn std::error::Error>> {
    let maintenance = Maintenance;
    
    // Current system metrics
    let metrics = SystemMetrics {
        cpu_temp: 65.0,
        cpu_usage: 45.0,
        memory_usage: 60.0,
        disk_health: 95.0,
        uptime_hours: 8760.0,  // 1 year
    };
    
    // Predict health
    let prediction = maintenance.predict_health(&metrics)?;
    
    println!("=== Health Prediction ===");
    println!("Overall health score: {:.1}%", prediction.health_score * 100.0);
    
    if let Some(ttf) = prediction.time_to_failure_hours {
        println!("Estimated time to failure: {:.1} hours", ttf);
    } else {
        println!("No imminent failure predicted");
    }
    
    println!("\nComponent Health:");
    for component in &prediction.components {
        println!("  {}: {:.1}% ({:?})", 
                 component.name, 
                 component.health * 100.0,
                 component.trend);
    }
    
    println!("\nRecommendations:");
    for rec in &prediction.recommendations {
        println!("  • {}", rec);
    }
    
    // Get maintenance schedule
    let schedule = maintenance.get_schedule();
    println!("\nNext scheduled maintenance: {}", schedule.next_maintenance);
    
    Ok(())
}
```

---

## Configuration

### Configuration File Format

The AI module can be configured via `/etc/vantisos/ai_config.toml`:

```toml
[general]
enabled = true
log_level = "info"

[scheduler]
enabled = true
max_latency_ms = 10
min_time_slice_ms = 5
max_time_slice_ms = 100

[power]
enabled = true
power_mode = "Balanced"
min_cpu_freq_mhz = 800
max_cpu_freq_mhz = 4000

[security]
enabled = true
response_mode = "LogAndAlert"
alert_threshold = 70

[monitoring]
enabled = true
interval_ms = 1000
drift_detection = true
drift_threshold = 10.0

[privacy]
differential_privacy = true
privacy_epsilon = 1.0
```

### Runtime Configuration Changes

```rust
use vantisos::ai::{AIModule, AIConfig};

fn update_configuration() -> Result<(), Box<dyn std::error::Error>> {
    // Load current config
    let mut config = AIConfig::from_file("/etc/vantisos/ai_config.toml")?;
    
    // Modify settings
    config.scheduler.max_latency_ms = 5;
    config.power.power_mode = vantisos::ai::PowerState::Performance;
    
    // Validate and save
    config.validate()?;
    config.save_to_file("/etc/vantisos/ai_config.toml")?;
    
    println!("Configuration updated. Restart AI module to apply changes.");
    
    Ok(())
}
```

---

## Best Practices

### 1. Resource Management

```rust
// Good: Proper cleanup
fn process_with_ai() -> Result<(), Box<dyn std::error::Error>> {
    let mut ai = AIModule::init(AIConfig::default())?;
    
    // Use AI module
    let decision = Scheduler::schedule_process(1234, 50)?;
    
    // Always shutdown
    ai.shutdown()?;
    Ok(())
}

// Better: Use RAII pattern
use std::sync::Arc;

struct AIGuard {
    ai: Option<AIModule>,
}

impl AIGuard {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(AIGuard {
            ai: Some(AIModule::init(AIConfig::default())?),
        })
    }
}

impl Drop for AIGuard {
    fn drop(&mut self) {
        if let Some(ref mut ai) = self.ai {
            let _ = ai.shutdown();
        }
    }
}
```

### 2. Error Handling

```rust
use vantisos::ai::AIError;

fn robust_ai_operation() {
    let result = Scheduler::schedule_process(1234, 50);
    
    match result {
        Ok(decision) => {
            println!("Scheduled successfully");
            // Use decision
        }
        Err(AIError::ModelNotFound(id)) => {
            eprintln!("Model {} not found. Loading default...", id);
            // Load default model
        }
        Err(AIError::ResourceExhausted) => {
            eprintln!("AI resources exhausted. Throttling...");
            // Throttle operations
        }
        Err(e) => {
            eprintln!("AI error: {}", e);
            // Log and continue
        }
    }
}
```

### 3. Performance Monitoring

```rust
use std::time::Instant;

fn monitored_ai_operation() {
    let start = Instant::now();
    
    let decision = Scheduler::schedule_process(1234, 50).unwrap();
    
    let duration = start.elapsed();
    
    if duration.as_millis() > 10 {
        eprintln!("Warning: AI decision took {}ms (budget: 10ms)", 
                  duration.as_millis());
    }
    
    // Use decision
}
```

### 4. Confidence-Based Decision Making

```rust
use vantisos::ai::Confidence;

fn confidence_based_action(decision: &Decision) {
    match decision.confidence {
        Confidence::High => {
            // Trust the decision completely
            execute_decision(decision);
        }
        Confidence::Medium => {
            // Use decision but with safeguards
            execute_decision_with_validation(decision);
        }
        Confidence::Low => {
            // Don't trust the decision
            fall_back_to_default();
        }
    }
}
```

---

## Troubleshooting

### Issue 1: AI Module Not Initializing

**Symptoms:**
- Error: `AIError::InitializationFailed`
- Module not responding

**Solutions:**
```rust
// Check configuration
let config = AIConfig::from_file("/etc/vantisos/ai_config.toml")?;
if let Err(e) = config.validate() {
    eprintln!("Invalid configuration: {}", e);
    return Err(e.into());
}

// Check system resources
let available_memory = get_available_memory_mb();
if available_memory < 512 {
    eprintln!("Insufficient memory: {} MB (required: 512 MB)", 
              available_memory);
    return Err("Insufficient memory".into());
}
```

### Issue 2: High Latency in AI Decisions

**Symptoms:**
- Decisions taking > 10ms
- System sluggish

**Solutions:**
```rust
// Reduce model complexity
let config = AIConfig {
    scheduler: SchedulerConfig {
        max_latency_ms: 5,  // Stricter budget
        ..Default::default()
    },
    ..Default::default()
};

// Monitor and throttle
let metrics = Monitoring::get_metrics();
if metrics.inference_latency_ms > 10.0 {
    // Reduce AI activity
    ai.pause()?;
}
```

### Issue 3: Model Drift Detected

**Symptoms:**
- Drift alerts in logs
- Decreasing accuracy

**Solutions:**
```rust
let monitoring = Monitoring;
if let Some(alert) = monitoring.check_drift() {
    println!("Drift detected in {}", alert.component);
    
    // Retrain model
    let mut trainer = Trainer::new(TrainerConfig::default());
    let model = trainer.train(&training_data)?;
    
    // Update model
    ai.update_model(model)?;
}
```

### Issue 4: Memory Exhaustion

**Symptoms:**
- AI module crashes
- Out of memory errors

**Solutions:**
```rust
// Monitor memory usage
let usage = ai.get_resource_usage();
if usage.total_memory_mb > 400 {
    // Approaching limit, cleanup
    ai.unload_unused_models()?;
    
    // Clear caches
    ai.clear_data_cache()?;
}
```

---

## Common Use Cases

### Use Case 1: Real-Time Process Scheduling

```rust
use vantisos::ai::Scheduler;

fn schedule_realtime_process(pid: u32) -> Result<(), Box<dyn std::error::Error>> {
    // High priority for real-time processes
    let decision = Scheduler::schedule_process(pid, 90)?;
    
    // Ensure low latency
    assert!(decision.time_slice_ms < 20, 
            "Real-time process needs low latency");
    
    // Execute on specified core
    assign_to_core(pid, decision.core)?;
    
    Ok(())
}
```

### Use Case 2: Power-Aware Batch Processing

```rust
use vantisos::ai::{PowerManager, PowerState};

fn run_batch_job() -> Result<(), Box<dyn std::error::Error>> {
    // Switch to power save mode
    let mut pm = PowerManager;
    let decision = PowerManager::get_power_decision(30)?; // Low workload
    
    pm.apply_decision(decision)?;
    
    // Run batch job
    execute_batch_job()?;
    
    // Return to balanced mode
    let decision = PowerManager::get_power_decision(50)?;
    pm.apply_decision(decision)?;
    
    Ok(())
}
```

### Use Case 3: Security Monitoring Dashboard

```rust
use vantisos::ai::{Security, ThreatLevel};

fn security_monitor() -> Result<(), Box<dyn std::error::Error>> {
    let behaviors = vec![
        "process reading /etc/passwd",
        "network connection to 192.168.1.100:22",
        "process modifying kernel memory",
    ];
    
    for behavior in behaviors {
        let analysis = Security::analyze_behavior(behavior)?;
        
        if analysis.level >= ThreatLevel::High {
            alert_admin(behavior, &analysis)?;
        }
    }
    
    Ok(())
}
```

### Use Case 4: Automated System Health Check

```rust
use vantisos::ai::{Monitoring, Maintenance};

fn system_health_check() -> Result<(), Box<dyn std::error::Error>> {
    // Get current metrics
    let metrics = Monitoring::get_metrics();
    
    // Check thresholds
    if metrics.cpu_usage_percent > 80.0 {
        println!("WARNING: High CPU usage");
    }
    
    if metrics.memory_usage_percent > 85.0 {
        println!("WARNING: High memory usage");
    }
    
    // Predict health
    let prediction = Maintenance::predict_health(&to_system_metrics(&metrics))?;
    
    if prediction.health_score < 0.7 {
        println!("WARNING: Poor health score: {:.1}%", 
                 prediction.health_score * 100.0);
    }
    
    Ok(())
}
```

---

## Performance Tuning

### Optimization 1: Reduce Inference Latency

```rust
// Use quantized models
let config = AIConfig {
    models: ModelsConfig {
        quantization: QuantizationLevel::Int8,
        ..Default::default()
    },
    ..Default::default()
};
```

### Optimization 2: Batch Processing

```rust
// Batch multiple requests
let pids = vec![1234, 1235, 1236, 1237, 1238];
let decisions: Vec<_> = pids.into_iter()
    .map(|pid| Scheduler::schedule_process(pid, 50))
    .collect::<Result<Vec<_>, _>>()?;
```

### Optimization 3: Model Caching

```rust
// Cache frequently used models
ai.preload_model("scheduler_model_v1")?;
ai.preload_model("security_model_v1")?;
```

### Optimization 4: Asynchronous Operations

```rust
use std::thread;

// Run AI operations in background
thread::spawn(move || {
    let decision = Scheduler::schedule_process(1234, 50).unwrap();
    channel.send(decision).unwrap();
});
```

---

*Last updated: March 4, 2026*
*Version: 1.3.0*