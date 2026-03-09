<content>
//! Graphics Integration Module
//! Provides AI-powered GPU resource management, graphics optimization,
//! and rendering pipeline enhancements for VantisOS.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Configuration for graphics AI integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsConfig {
    /// Enable GPU optimization
    pub enable_gpu_optimization: bool,
    
    /// Enable adaptive rendering
    pub enable_adaptive_rendering: bool,
    
    /// Enable resource prediction
    pub enable_resource_prediction: bool,
    
    /// Enable frame rate optimization
    pub enable_fps_optimization: bool,
    
    /// Target frame rate
    pub target_fps: u32,
    
    /// Maximum GPU memory usage (percentage 0-100)
    pub max_gpu_memory_percent: u32,
    
    /// Enable power management
    pub enable_power_management: bool,
    
    /// Power saving mode threshold (fps)
    pub power_saving_threshold: u32,
    
    /// Quality preset (1-5, 5 highest)
    pub quality_preset: u8,
    
    /// Enable VSync prediction
    pub enable_vsync_prediction: bool,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self {
            enable_gpu_optimization: true,
            enable_adaptive_rendering: true,
            enable_resource_prediction: true,
            enable_fps_optimization: true,
            target_fps: 60,
            max_gpu_memory_percent: 80,
            enable_power_management: true,
            power_saving_threshold: 30,
            quality_preset: 3,
            enable_vsync_prediction: true,
        }
    }
}

/// Graphics performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsMetrics {
    /// Current frame rate
    pub current_fps: f64,
    
    /// Average frame rate
    pub avg_fps: f64,
    
    /// Frame time variance
    pub frame_time_variance: f64,
    
    /// GPU utilization (0.0-1.0)
    pub gpu_utilization: f64,
    
    /// GPU memory used (MB)
    pub gpu_memory_used_mb: u64,
    
    /// GPU memory total (MB)
    pub gpu_memory_total_mb: u64,
    
    /// Render queue depth
    pub render_queue_depth: u32,
    
    /// Draw calls per frame
    pub draw_calls_per_frame: u32,
    
    /// Triangles rendered per frame
    pub triangles_per_frame: u64,
    
    /// Texture memory used (MB)
    pub texture_memory_mb: u64,
    
    /// Number of active shaders
    pub active_shaders: u32,
    
    /// GPU temperature (Celsius)
    pub gpu_temperature: f32,
    
    /// Power consumption (Watts)
    pub power_consumption: f32,
    
    /// Last update timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for GraphicsMetrics {
    fn default() -> Self {
        Self {
            current_fps: 0.0,
            avg_fps: 0.0,
            frame_time_variance: 0.0,
            gpu_utilization: 0.0,
            gpu_memory_used_mb: 0,
            gpu_memory_total_mb: 8192,
            render_queue_depth: 0,
            draw_calls_per_frame: 0,
            triangles_per_frame: 0,
            texture_memory_mb: 0,
            active_shaders: 0,
            gpu_temperature: 0.0,
            power_consumption: 0.0,
            last_updated: chrono::Utc::now(),
        }
    }
}

/// GPU Device Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDeviceInfo {
    /// Device name
    pub name: String,
    
    /// Vendor (NVIDIA, AMD, Intel)
    pub vendor: GpuVendor,
    
    /// Driver version
    pub driver_version: String,
    
    /// Total memory in MB
    pub total_memory_mb: u64,
    
    /// CUDA/OpenCL compute units
    pub compute_units: u32,
    
    /// Maximum clock frequency (MHz)
    pub max_clock_mhz: u32,
    
    /// Supported features
    pub features: Vec<GpuFeature>,
    
    /// Performance tier (1-5)
    pub performance_tier: u8,
}

/// GPU Vendor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Apple,
    Qualcomm,
    Unknown,
}

/// GPU Features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GpuFeature {
    RayTracing,
    Dlss,
    Fsr,
    VariableRateShading,
    MeshShaders,
    SamplerFeedback,
    DirectStorage,
    TensorCores,
}

/// Rendering profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingProfile {
    /// Profile name
    pub name: String,
    
    /// Quality level (1-5)
    pub quality_level: u8,
    
    /// Anti-aliasing mode
    pub anti_aliasing: AntiAliasingMode,
    
    /// Shadow quality (1-5)
    pub shadow_quality: u8,
    
    /// Texture resolution scale
    pub texture_scale: f32,
    
    /// Draw distance
    pub draw_distance: DrawDistance,
    
    /// Post-processing enabled
    pub post_processing: bool,
    
    /// Motion blur enabled
    pub motion_blur: bool,
    
    /// Ambient occlusion quality (0-3)
    pub ao_quality: u8,
    
    /// Reflection quality (0-3)
    pub reflection_quality: u8,
}

impl Default for RenderingProfile {
    fn default() -> Self {
        Self {
            name: "Balanced".to_string(),
            quality_level: 3,
            anti_aliasing: AntiAliasingMode::Taa,
            shadow_quality: 3,
            texture_scale: 1.0,
            draw_distance: DrawDistance::Medium,
            post_processing: true,
            motion_blur: false,
            ao_quality: 2,
            reflection_quality: 2,
        }
    }
}

/// Anti-aliasing modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AntiAliasingMode {
    None,
    Fxaa,
    Taa,
    Msaa2x,
    Msaa4x,
    Msaa8x,
    Dlss,
    Fsr2,
}

/// Draw distance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DrawDistance {
    Near,
    Medium,
    Far,
    Ultra,
    Infinite,
}

/// GPU Resource allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuResourceAllocation {
    /// Resource ID
    pub resource_id: String,
    
    /// Resource type
    pub resource_type: GpuResourceType,
    
    /// Memory allocated (MB)
    pub memory_mb: u64,
    
    /// Priority (1-10, 10 highest)
    pub priority: u8,
    
    /// Last access time
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    
    /// Access frequency
    pub access_frequency: u64,
}

/// GPU Resource types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GpuResourceType {
    Texture,
    Buffer,
    Shader,
    Pipeline,
    Framebuffer,
    Sampler,
}

/// Frame timing analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameTimingAnalysis {
    /// Frame number
    pub frame_number: u64,
    
    /// Total frame time (ms)
    pub frame_time_ms: f64,
    
    /// CPU time (ms)
    pub cpu_time_ms: f64,
    
    /// GPU time (ms)
    pub gpu_time_ms: f64,
    
    /// Wait time for GPU (ms)
    pub gpu_wait_ms: f64,
    
    /// Pipeline stages timing
    pub stage_timings: HashMap<String, f64>,
    
    /// Frame dropped
    pub dropped: bool,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Adaptive quality adjustment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveQualityAdjustment {
    /// Adjustment type
    pub adjustment_type: QualityAdjustmentType,
    
    /// Parameter to adjust
    pub parameter: String,
    
    /// Current value
    pub current_value: f64,
    
    /// Adjusted value
    pub adjusted_value: f64,
    
    /// Reason for adjustment
    pub reason: String,
    
    /// Expected improvement
    pub expected_improvement: f64,
}

/// Types of quality adjustments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityAdjustmentType {
    Increase,
    Decrease,
    Maintain,
}

/// Rendering prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingPrediction {
    /// Predicted frame time
    pub predicted_frame_time_ms: f64,
    
    /// Confidence score
    pub confidence: f64,
    
    /// Recommended quality adjustments
    pub recommended_adjustments: Vec<AdaptiveQualityAdjustment>,
    
    /// Predicted GPU utilization
    pub predicted_gpu_utilization: f64,
    
    /// Predicted memory usage
    pub predicted_memory_mb: u64,
    
    /// Estimated power consumption
    pub estimated_power_w: f32,
}

/// Graphics Integration Manager
pub struct GraphicsIntegration {
    config: GraphicsConfig,
    metrics: GraphicsMetrics,
    gpu_device: Option<GpuDeviceInfo>,
    rendering_profiles: HashMap<String, RenderingProfile>,
    current_profile: String,
    resource_allocations: HashMap<String, GpuResourceAllocation>,
    frame_history: Vec<FrameTimingAnalysis>,
    adaptive_quality_history: Vec<AdaptiveQualityAdjustment>,
}

impl GraphicsIntegration {
    /// Create a new graphics integration manager
    pub fn new(config: GraphicsConfig) -> Self {
        let mut profiles = HashMap::new();
        
        profiles.insert("Low".to_string(), RenderingProfile {
            name: "Low".to_string(),
            quality_level: 1,
            anti_aliasing: AntiAliasingMode::Fxaa,
            shadow_quality: 1,
            texture_scale: 0.5,
            draw_distance: DrawDistance::Near,
            post_processing: false,
            motion_blur: false,
            ao_quality: 0,
            reflection_quality: 0,
        });
        
        profiles.insert("Balanced".to_string(), RenderingProfile::default());
        
        profiles.insert("High".to_string(), RenderingProfile {
            name: "High".to_string(),
            quality_level: 4,
            anti_aliasing: AntiAliasingMode::Taa,
            shadow_quality: 4,
            texture_scale: 1.0,
            draw_distance: DrawDistance::Far,
            post_processing: true,
            motion_blur: true,
            ao_quality: 2,
            reflection_quality: 3,
        });
        
        profiles.insert("Ultra".to_string(), RenderingProfile {
            name: "Ultra".to_string(),
            quality_level: 5,
            anti_aliasing: AntiAliasingMode::Dlss,
            shadow_quality: 5,
            texture_scale: 1.0,
            draw_distance: DrawDistance::Ultra,
            post_processing: true,
            motion_blur: true,
            ao_quality: 3,
            reflection_quality: 3,
        });
        
        Self {
            config,
            metrics: GraphicsMetrics::default(),
            gpu_device: None,
            rendering_profiles: profiles,
            current_profile: "Balanced".to_string(),
            resource_allocations: HashMap::new(),
            frame_history: Vec::new(),
            adaptive_quality_history: Vec::new(),
        }
    }
    
    /// Create with default configuration
    pub fn default_manager() -> Self {
        Self::new(GraphicsConfig::default())
    }
    
    /// Initialize GPU device
    pub fn initialize_gpu(&mut self, device_info: GpuDeviceInfo) {
        self.gpu_device = Some(device_info.clone());
        self.metrics.gpu_memory_total_mb = device_info.total_memory_mb;
    }
    
    /// Begin frame processing
    pub fn begin_frame(&mut self) -> FrameContext {
        FrameContext {
            frame_number: self.frame_history.len() as u64 + 1,
            start_time: Instant::now(),
            stage_timings: HashMap::new(),
        }
    }
    
    /// End frame processing
    pub fn end_frame(&mut self, mut context: FrameContext) {
        let frame_time = context.start_time.elapsed().as_millis() as f64;
        let frame_number = context.frame_number;
        
        let analysis = FrameTimingAnalysis {
            frame_number,
            frame_time_ms: frame_time,
            cpu_time_ms: frame_time * 0.3, // Simulated
            gpu_time_ms: frame_time * 0.7, // Simulated
            gpu_wait_ms: frame_time * 0.1,
            stage_timings: context.stage_timings,
            dropped: frame_time > (1000.0 / self.config.target_fps as f64) * 1.5,
            timestamp: chrono::Utc::now(),
        };
        
        // Update metrics
        self.update_metrics(&analysis);
        
        // Store frame history
        self.frame_history.push(analysis);
        
        // Keep limited history
        if self.frame_history.len() > 1000 {
            self.frame_history.remove(0);
        }
        
        // Adaptive quality adjustment
        if self.config.enable_adaptive_rendering {
            self.adjust_quality_automatically();
        }
    }
    
    /// Update metrics from frame analysis
    fn update_metrics(&mut self, analysis: &FrameTimingAnalysis) {
        self.metrics.current_fps = 1000.0 / analysis.frame_time_ms;
        
        // Update average FPS
        let frame_count = self.frame_history.len() + 1;
        self.metrics.avg_fps = (self.metrics.avg_fps * (frame_count - 1) as f64 
            + self.metrics.current_fps) / frame_count as f64;
        
        // Update frame time variance
        if self.frame_history.len() > 1 {
            let avg_frame_time = 1000.0 / self.metrics.avg_fps;
            let diff = analysis.frame_time_ms - avg_frame_time;
            self.metrics.frame_time_variance = 
                (self.metrics.frame_time_variance * (frame_count - 2) as f64 + diff * diff) 
                / (frame_count - 1) as f64;
        }
        
        self.metrics.last_updated = chrono::Utc::now();
    }
    
    /// Automatically adjust quality based on performance
    fn adjust_quality_automatically(&mut self) {
        if self.frame_history.len() < 10 {
            return;
        }
        
        // Get recent frame performance
        let recent_frames: Vec<_> = self.frame_history.iter().rev().take(10).collect();
        let avg_recent_fps: f64 = recent_frames.iter().map(|f| 1000.0 / f.frame_time_ms).sum::<f64>() / 10.0;
        
        // Check if adjustment needed
        let target = self.config.target_fps as f64;
        let performance_ratio = avg_recent_fps / target;
        
        if performance_ratio < 0.8 {
            // FPS too low, reduce quality
            self.decrease_quality();
        } else if performance_ratio > 1.2 && self.current_profile != "Ultra" {
            // FPS good, can increase quality
            self.increase_quality();
        }
    }
    
    /// Decrease rendering quality
    fn decrease_quality(&mut self) {
        let profiles = vec!["Ultra", "High", "Balanced", "Low"];
        if let Some(current_idx) = profiles.iter().position(|p| *p == self.current_profile) {
            if current_idx < profiles.len() - 1 {
                let new_profile = profiles[current_idx + 1].to_string();
                self.set_rendering_profile(&new_profile);
                
                self.adaptive_quality_history.push(AdaptiveQualityAdjustment {
                    adjustment_type: QualityAdjustmentType::Decrease,
                    parameter: "profile".to_string(),
                    current_value: 1.0,
                    adjusted_value: 0.8,
                    reason: "Low FPS detected".to_string(),
                    expected_improvement: 0.2,
                });
            }
        }
    }
    
    /// Increase rendering quality
    fn increase_quality(&mut self) {
        let profiles = vec!["Low", "Balanced", "High", "Ultra"];
        if let Some(current_idx) = profiles.iter().position(|p| *p == self.current_profile) {
            if current_idx < profiles.len() - 1 {
                let new_profile = profiles[current_idx + 1].to_string();
                self.set_rendering_profile(&new_profile);
                
                self.adaptive_quality_history.push(AdaptiveQualityAdjustment {
                    adjustment_type: QualityAdjustmentType::Increase,
                    parameter: "profile".to_string(),
                    current_value: 1.0,
                    adjusted_value: 1.2,
                    reason: "High FPS available".to_string(),
                    expected_improvement: 0.3,
                });
            }
        }
    }
    
    /// Set rendering profile
    pub fn set_rendering_profile(&mut self, profile_name: &str) {
        if self.rendering_profiles.contains_key(profile_name) {
            self.current_profile = profile_name.to_string();
        }
    }
    
    /// Get current rendering profile
    pub fn get_current_profile(&self) -> Option<&RenderingProfile> {
        self.rendering_profiles.get(&self.current_profile)
    }
    
    /// Predict rendering performance
    pub fn predict_rendering(&self, scene_complexity: f64) -> RenderingPrediction {
        let base_frame_time = 1000.0 / self.config.target_fps as f64;
        
        // Factor in scene complexity
        let predicted_frame_time = base_frame_time * (1.0 + scene_complexity * 0.5);
        
        // Generate recommendations
        let mut recommendations = Vec::new();
        
        if predicted_frame_time > base_frame_time * 1.5 {
            recommendations.push(AdaptiveQualityAdjustment {
                adjustment_type: QualityAdjustmentType::Decrease,
                parameter: "texture_scale".to_string(),
                current_value: 1.0,
                adjusted_value: 0.75,
                reason: "High predicted frame time".to_string(),
                expected_improvement: 0.25,
            });
        }
        
        let profile = self.get_current_profile();
        let quality_level = profile.map(|p| p.quality_level).unwrap_or(3) as f64;
        
        RenderingPrediction {
            predicted_frame_time_ms: predicted_frame_time,
            confidence: 0.8,
            recommended_adjustments: recommendations,
            predicted_gpu_utilization: (scene_complexity * 0.5 + quality_level * 0.1).min(1.0),
            predicted_memory_mb: self.metrics.gpu_memory_used_mb + (scene_complexity * 100.0) as u64,
            estimated_power_w: (50.0 + scene_complexity * 100.0) as f32,
        }
    }
    
    /// Allocate GPU resource
    pub fn allocate_resource(&mut self, allocation: GpuResourceAllocation) -> Result<(), GraphicsError> {
        // Check memory availability
        let current_usage = self.resource_allocations.values().map(|a| a.memory_mb).sum::<u64>();
        let max_memory = (self.metrics.gpu_memory_total_mb as f64 
            * self.config.max_gpu_memory_percent as f64 / 100.0) as u64;
        
        if current_usage + allocation.memory_mb > max_memory {
            return Err(GraphicsError::OutOfMemory(format!(
                "Requested {} MB, available {} MB",
                allocation.memory_mb,
                max_memory - current_usage
            )));
        }
        
        self.resource_allocations.insert(allocation.resource_id.clone(), allocation);
        Ok(())
    }
    
    /// Free GPU resource
    pub fn free_resource(&mut self, resource_id: &str) {
        self.resource_allocations.remove(resource_id);
    }
    
    /// Get memory usage statistics
    pub fn get_memory_stats(&self) -> MemoryStatistics {
        let total_allocated: u64 = self.resource_allocations.values().map(|a| a.memory_mb).sum();
        let texture_memory: u64 = self.resource_allocations.values()
            .filter(|a| matches!(a.resource_type, GpuResourceType::Texture))
            .map(|a| a.memory_mb)
            .sum();
        let buffer_memory: u64 = self.resource_allocations.values()
            .filter(|a| matches!(a.resource_type, GpuResourceType::Buffer))
            .map(|a| a.memory_mb)
            .sum();
        
        MemoryStatistics {
            total_allocated_mb: total_allocated,
            texture_memory_mb: texture_memory,
            buffer_memory_mb: buffer_memory,
            available_mb: self.metrics.gpu_memory_total_mb - total_allocated,
            utilization: total_allocated as f64 / self.metrics.gpu_memory_total_mb as f64,
        }
    }
    
    /// Get graphics metrics
    pub fn get_metrics(&self) -> GraphicsMetrics {
        self.metrics.clone()
    }
    
    /// Get frame history
    pub fn get_frame_history(&self, count: usize) -> Vec<&FrameTimingAnalysis> {
        self.frame_history.iter().rev().take(count).collect()
    }
    
    /// Optimize for power saving
    pub fn enable_power_saving(&mut self) {
        self.config.target_fps = self.config.power_saving_threshold;
        self.set_rendering_profile("Low");
    }
    
    /// Disable power saving
    pub fn disable_power_saving(&mut self) {
        self.config.target_fps = 60;
        self.set_rendering_profile("Balanced");
    }
}

/// Frame context for tracking frame execution
pub struct FrameContext {
    frame_number: u64,
    start_time: Instant,
    stage_timings: HashMap<String, f64>,
}

impl FrameContext {
    /// Record timing for a rendering stage
    pub fn record_stage(&mut self, stage_name: &str, duration_ms: f64) {
        self.stage_timings.insert(stage_name.to_string(), duration_ms);
    }
}

/// Memory statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStatistics {
    pub total_allocated_mb: u64,
    pub texture_memory_mb: u64,
    pub buffer_memory_mb: u64,
    pub available_mb: u64,
    pub utilization: f64,
}

/// Graphics error types
#[derive(Debug, thiserror::Error)]
pub enum GraphicsError {
    #[error("Out of GPU memory: {0}")]
    OutOfMemory(String),
    
    #[error("Device initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Shader compilation failed: {0}")]
    ShaderCompilationFailed(String),
    
    #[error("Pipeline creation failed: {0}")]
    PipelineCreationFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphics_integration_creation() {
        let config = GraphicsConfig::default();
        let graphics = GraphicsIntegration::new(config);
        
        assert_eq!(graphics.current_profile, "Balanced");
        assert!(graphics.rendering_profiles.len() >= 4);
    }

    #[test]
    fn test_rendering_profile_selection() {
        let mut graphics = GraphicsIntegration::default_manager();
        
        graphics.set_rendering_profile("Ultra");
        assert_eq!(graphics.current_profile, "Ultra");
        
        let profile = graphics.get_current_profile().unwrap();
        assert_eq!(profile.quality_level, 5);
    }

    #[test]
    fn test_frame_processing() {
        let mut graphics = GraphicsIntegration::default_manager();
        
        let context = graphics.begin_frame();
        let frame_num = context.frame_number;
        
        graphics.end_frame(context);
        
        assert_eq!(graphics.frame_history.len(), 1);
        assert_eq!(graphics.frame_history[0].frame_number, frame_num);
    }

    #[test]
    fn test_rendering_prediction() {
        let graphics = GraphicsIntegration::default_manager();
        
        let prediction = graphics.predict_rendering(0.5);
        
        assert!(prediction.predicted_frame_time_ms > 0.0);
        assert!(prediction.confidence > 0.0);
    }

    #[test]
    fn test_resource_allocation() {
        let mut graphics = GraphicsIntegration::default_manager();
        graphics.metrics.gpu_memory_total_mb = 1024;
        
        let allocation = GpuResourceAllocation {
            resource_id: "tex_001".to_string(),
            resource_type: GpuResourceType::Texture,
            memory_mb: 256,
            priority: 5,
            last_accessed: chrono::Utc::now(),
            access_frequency: 100,
        };
        
        let result = graphics.allocate_resource(allocation);
        assert!(result.is_ok());
        
        let stats = graphics.get_memory_stats();
        assert_eq!(stats.total_allocated_mb, 256);
    }

    #[test]
    fn test_memory_limit() {
        let mut graphics = GraphicsIntegration::default_manager();
        graphics.metrics.gpu_memory_total_mb = 100;
        
        let allocation = GpuResourceAllocation {
            resource_id: "tex_001".to_string(),
            resource_type: GpuResourceType::Texture,
            memory_mb: 150,
            priority: 5,
            last_accessed: chrono::Utc::now(),
            access_frequency: 100,
        };
        
        let result = graphics.allocate_resource(allocation);
        assert!(result.is_err());
    }

    #[test]
    fn test_power_saving() {
        let mut graphics = GraphicsIntegration::default_manager();
        
        graphics.enable_power_saving();
        assert_eq!(graphics.config.target_fps, graphics.config.power_saving_threshold);
        assert_eq!(graphics.current_profile, "Low");
        
        graphics.disable_power_saving();
        assert_eq!(graphics.config.target_fps, 60);
        assert_eq!(graphics.current_profile, "Balanced");
    }

    #[test]
    fn test_adaptive_quality() {
        let mut graphics = GraphicsIntegration::default_manager();
        graphics.config.target_fps = 60;
        
        // Simulate multiple frames
        for _ in 0..15 {
            let context = graphics.begin_frame();
            graphics.end_frame(context);
        }
        
        // Should have frame history
        assert!(graphics.frame_history.len() >= 10);
    }
}
</content>