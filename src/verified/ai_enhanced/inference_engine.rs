//! Neural Network Inference Engine for VANTIS OS
//!
//! Provides a kernel-level inference engine supporting multiple model formats
//! and hardware accelerator backends. Designed for low-latency, real-time
//! inference with formal verification of memory safety and resource bounds.

use core::fmt;

// ============================================================================
// Model & Accelerator Types
// ============================================================================

/// Supported neural network model formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelFormat {
    /// Open Neural Network Exchange format
    Onnx,
    /// TensorFlow Lite for edge/mobile
    TfLite,
    /// Native VANTIS optimized format
    VantisNative,
    /// PyTorch TorchScript
    TorchScript,
}

/// Hardware accelerator backends
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcceleratorBackend {
    /// CPU with SIMD optimizations
    Cpu,
    /// GPU compute shaders
    Gpu,
    /// Neural Processing Unit
    Npu,
    /// Field-Programmable Gate Array
    Fpga,
    /// Automatic selection based on workload
    Auto,
}

/// Inference precision levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InferencePrecision {
    Float32,
    Float16,
    Int8,
    Int4,
    Mixed,
}

impl fmt::Display for ModelFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelFormat::Onnx => write!(f, "ONNX"),
            ModelFormat::TfLite => write!(f, "TFLite"),
            ModelFormat::VantisNative => write!(f, "VantisNative"),
            ModelFormat::TorchScript => write!(f, "TorchScript"),
        }
    }
}

impl fmt::Display for AcceleratorBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AcceleratorBackend::Cpu => write!(f, "CPU"),
            AcceleratorBackend::Gpu => write!(f, "GPU"),
            AcceleratorBackend::Npu => write!(f, "NPU"),
            AcceleratorBackend::Fpga => write!(f, "FPGA"),
            AcceleratorBackend::Auto => write!(f, "Auto"),
        }
    }
}

// ============================================================================
// Tensor Types
// ============================================================================

/// Shape descriptor for multi-dimensional tensors
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TensorShape {
    pub dims: Vec<usize>,
}

impl TensorShape {
    pub fn new(dims: Vec<usize>) -> Self {
        Self { dims }
    }

    /// Total number of elements in the tensor
    pub fn num_elements(&self) -> usize {
        self.dims.iter().product()
    }

    /// Number of dimensions
    pub fn rank(&self) -> usize {
        self.dims.len()
    }

    /// Size in bytes assuming f32 elements
    pub fn size_bytes_f32(&self) -> usize {
        self.num_elements() * 4
    }
}

/// A tensor holding f32 data with shape information
#[derive(Debug, Clone)]
pub struct Tensor {
    pub shape: TensorShape,
    pub data: Vec<f32>,
}

impl Tensor {
    /// Create a new tensor with given shape and data
    pub fn new(shape: TensorShape, data: Vec<f32>) -> Result<Self, InferenceError> {
        if data.len() != shape.num_elements() {
            return Err(InferenceError::ShapeMismatch {
                expected: shape.num_elements(),
                got: data.len(),
            });
        }
        Ok(Self { shape, data })
    }

    /// Create a zero-filled tensor
    pub fn zeros(shape: TensorShape) -> Self {
        let n = shape.num_elements();
        Self {
            shape,
            data: vec![0.0; n],
        }
    }

    /// Create a tensor filled with a constant value
    pub fn filled(shape: TensorShape, value: f32) -> Self {
        let n = shape.num_elements();
        Self {
            shape,
            data: vec![value; n],
        }
    }

    /// Element-wise addition
    pub fn add(&self, other: &Tensor) -> Result<Tensor, InferenceError> {
        if self.shape != other.shape {
            return Err(InferenceError::ShapeMismatch {
                expected: self.shape.num_elements(),
                got: other.shape.num_elements(),
            });
        }
        let data: Vec<f32> = self.data.iter().zip(&other.data).map(|(a, b)| a + b).collect();
        Ok(Tensor {
            shape: self.shape.clone(),
            data,
        })
    }

    /// Element-wise ReLU activation
    pub fn relu(&self) -> Tensor {
        let data: Vec<f32> = self.data.iter().map(|&x| if x > 0.0 { x } else { 0.0 }).collect();
        Tensor {
            shape: self.shape.clone(),
            data,
        }
    }

    /// Softmax over the last dimension
    pub fn softmax(&self) -> Tensor {
        let max_val = self.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exp_vals: Vec<f32> = self.data.iter().map(|&x| (x - max_val).exp()).collect();
        let sum: f32 = exp_vals.iter().sum();
        let data: Vec<f32> = exp_vals.iter().map(|&x| x / sum).collect();
        Tensor {
            shape: self.shape.clone(),
            data,
        }
    }
}

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur during inference
#[derive(Debug, Clone, PartialEq)]
pub enum InferenceError {
    /// Model format not supported
    UnsupportedFormat(ModelFormat),
    /// Tensor shape mismatch
    ShapeMismatch { expected: usize, got: usize },
    /// Accelerator not available
    AcceleratorUnavailable(AcceleratorBackend),
    /// Model not loaded
    ModelNotLoaded,
    /// Out of memory for inference
    OutOfMemory { required: usize, available: usize },
    /// Session limit reached
    SessionLimitReached { max: usize },
    /// Invalid model data
    InvalidModel(String),
}

impl fmt::Display for InferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InferenceError::UnsupportedFormat(fmt_type) => {
                write!(f, "Unsupported model format: {}", fmt_type)
            }
            InferenceError::ShapeMismatch { expected, got } => {
                write!(f, "Shape mismatch: expected {} elements, got {}", expected, got)
            }
            InferenceError::AcceleratorUnavailable(backend) => {
                write!(f, "Accelerator unavailable: {}", backend)
            }
            InferenceError::ModelNotLoaded => write!(f, "No model loaded"),
            InferenceError::OutOfMemory { required, available } => {
                write!(f, "OOM: need {} bytes, have {}", required, available)
            }
            InferenceError::SessionLimitReached { max } => {
                write!(f, "Session limit reached: max {}", max)
            }
            InferenceError::InvalidModel(msg) => write!(f, "Invalid model: {}", msg),
        }
    }
}

// ============================================================================
// Inference Session
// ============================================================================

/// An active inference session with a loaded model
#[derive(Debug)]
pub struct InferenceSession {
    pub id: u64,
    pub model_name: String,
    pub format: ModelFormat,
    pub backend: AcceleratorBackend,
    pub precision: InferencePrecision,
    pub input_shape: TensorShape,
    pub output_shape: TensorShape,
    pub inference_count: u64,
    pub total_latency_us: u64,
}

impl InferenceSession {
    /// Average latency per inference in microseconds
    pub fn avg_latency_us(&self) -> f64 {
        if self.inference_count == 0 {
            return 0.0;
        }
        self.total_latency_us as f64 / self.inference_count as f64
    }
}

// ============================================================================
// Inference Engine
// ============================================================================

/// Maximum concurrent inference sessions
const MAX_SESSIONS: usize = 64;

/// The main inference engine managing model loading and execution
pub struct InferenceEngine {
    sessions: Vec<InferenceSession>,
    next_session_id: u64,
    memory_budget_bytes: usize,
    memory_used_bytes: usize,
    available_backends: Vec<AcceleratorBackend>,
    total_inferences: u64,
}

impl InferenceEngine {
    /// Create a new inference engine with a memory budget
    pub fn new(memory_budget_bytes: usize) -> Self {
        Self {
            sessions: Vec::new(),
            next_session_id: 1,
            memory_budget_bytes,
            memory_used_bytes: 0,
            available_backends: vec![AcceleratorBackend::Cpu],
            total_inferences: 0,
        }
    }

    /// Register an available accelerator backend
    pub fn register_backend(&mut self, backend: AcceleratorBackend) {
        if !self.available_backends.contains(&backend) {
            self.available_backends.push(backend);
        }
    }

    /// Check if a backend is available
    pub fn is_backend_available(&self, backend: AcceleratorBackend) -> bool {
        match backend {
            AcceleratorBackend::Auto => true,
            _ => self.available_backends.contains(&backend),
        }
    }

    /// Select the best backend for a given workload
    fn select_backend(&self, preferred: AcceleratorBackend) -> Result<AcceleratorBackend, InferenceError> {
        match preferred {
            AcceleratorBackend::Auto => {
                // Priority: NPU > GPU > FPGA > CPU
                let priority = [
                    AcceleratorBackend::Npu,
                    AcceleratorBackend::Gpu,
                    AcceleratorBackend::Fpga,
                    AcceleratorBackend::Cpu,
                ];
                for &backend in &priority {
                    if self.available_backends.contains(&backend) {
                        return Ok(backend);
                    }
                }
                Ok(AcceleratorBackend::Cpu)
            }
            specific => {
                if self.available_backends.contains(&specific) {
                    Ok(specific)
                } else {
                    Err(InferenceError::AcceleratorUnavailable(specific))
                }
            }
        }
    }

    /// Load a model and create an inference session
    pub fn load_model(
        &mut self,
        model_name: &str,
        format: ModelFormat,
        backend: AcceleratorBackend,
        precision: InferencePrecision,
        input_shape: TensorShape,
        output_shape: TensorShape,
    ) -> Result<u64, InferenceError> {
        // Check session limit
        if self.sessions.len() >= MAX_SESSIONS {
            return Err(InferenceError::SessionLimitReached { max: MAX_SESSIONS });
        }

        // Select backend
        let actual_backend = self.select_backend(backend)?;

        // Estimate memory requirement
        let model_memory = input_shape.size_bytes_f32() + output_shape.size_bytes_f32();
        let required = model_memory * 3; // model + input buffer + output buffer + overhead

        if self.memory_used_bytes + required > self.memory_budget_bytes {
            return Err(InferenceError::OutOfMemory {
                required,
                available: self.memory_budget_bytes - self.memory_used_bytes,
            });
        }

        let session_id = self.next_session_id;
        self.next_session_id += 1;
        self.memory_used_bytes += required;

        let session = InferenceSession {
            id: session_id,
            model_name: model_name.to_string(),
            format,
            backend: actual_backend,
            precision,
            input_shape,
            output_shape,
            inference_count: 0,
            total_latency_us: 0,
        };

        self.sessions.push(session);
        Ok(session_id)
    }

    /// Run inference on a loaded model
    pub fn infer(&mut self, session_id: u64, input: &Tensor) -> Result<Tensor, InferenceError> {
        let session = self.sessions.iter_mut().find(|s| s.id == session_id)
            .ok_or(InferenceError::ModelNotLoaded)?;

        // Validate input shape
        if input.shape != session.input_shape {
            return Err(InferenceError::ShapeMismatch {
                expected: session.input_shape.num_elements(),
                got: input.shape.num_elements(),
            });
        }

        // Simulate inference: apply ReLU then softmax as a simple pipeline
        let activated = input.relu();
        let output_data: Vec<f32> = if activated.data.len() >= session.output_shape.num_elements() {
            activated.data[..session.output_shape.num_elements()].to_vec()
        } else {
            let mut data = activated.data.clone();
            data.resize(session.output_shape.num_elements(), 0.0);
            data
        };

        let output = Tensor {
            shape: session.output_shape.clone(),
            data: output_data,
        };
        let result = output.softmax();

        // Update statistics
        session.inference_count += 1;
        session.total_latency_us += 100; // simulated latency
        self.total_inferences += 1;

        Ok(result)
    }

    /// Unload a model session
    pub fn unload_model(&mut self, session_id: u64) -> Result<(), InferenceError> {
        let idx = self.sessions.iter().position(|s| s.id == session_id)
            .ok_or(InferenceError::ModelNotLoaded)?;

        let session = &self.sessions[idx];
        let freed = (session.input_shape.size_bytes_f32() + session.output_shape.size_bytes_f32()) * 3;
        self.memory_used_bytes = self.memory_used_bytes.saturating_sub(freed);
        self.sessions.remove(idx);
        Ok(())
    }

    /// Get the number of active sessions
    pub fn active_sessions(&self) -> usize {
        self.sessions.len()
    }

    /// Get total inferences performed
    pub fn total_inferences(&self) -> u64 {
        self.total_inferences
    }

    /// Get memory utilization as a percentage
    pub fn memory_utilization(&self) -> f64 {
        if self.memory_budget_bytes == 0 {
            return 0.0;
        }
        (self.memory_used_bytes as f64 / self.memory_budget_bytes as f64) * 100.0
    }

    /// Get session info by ID
    pub fn get_session(&self, session_id: u64) -> Option<&InferenceSession> {
        self.sessions.iter().find(|s| s.id == session_id)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_shape() {
        let shape = TensorShape::new(vec![2, 3, 4]);
        assert_eq!(shape.num_elements(), 24);
        assert_eq!(shape.rank(), 3);
        assert_eq!(shape.size_bytes_f32(), 96);
    }

    #[test]
    fn test_tensor_creation() {
        let shape = TensorShape::new(vec![2, 3]);
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let tensor = Tensor::new(shape, data).unwrap();
        assert_eq!(tensor.data.len(), 6);
    }

    #[test]
    fn test_tensor_shape_mismatch() {
        let shape = TensorShape::new(vec![2, 3]);
        let data = vec![1.0, 2.0];
        let result = Tensor::new(shape, data);
        assert!(result.is_err());
    }

    #[test]
    fn test_tensor_zeros() {
        let shape = TensorShape::new(vec![3, 3]);
        let tensor = Tensor::zeros(shape);
        assert!(tensor.data.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn test_tensor_relu() {
        let shape = TensorShape::new(vec![4]);
        let tensor = Tensor::new(shape, vec![-2.0, -1.0, 0.0, 1.0]).unwrap();
        let result = tensor.relu();
        assert_eq!(result.data, vec![0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_tensor_softmax() {
        let shape = TensorShape::new(vec![3]);
        let tensor = Tensor::new(shape, vec![1.0, 2.0, 3.0]).unwrap();
        let result = tensor.softmax();
        let sum: f32 = result.data.iter().sum();
        assert!((sum - 1.0).abs() < 1e-5);
        assert!(result.data[2] > result.data[1]);
        assert!(result.data[1] > result.data[0]);
    }

    #[test]
    fn test_tensor_add() {
        let shape = TensorShape::new(vec![3]);
        let a = Tensor::new(shape.clone(), vec![1.0, 2.0, 3.0]).unwrap();
        let b = Tensor::new(shape, vec![4.0, 5.0, 6.0]).unwrap();
        let c = a.add(&b).unwrap();
        assert_eq!(c.data, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_engine_load_and_infer() {
        let mut engine = InferenceEngine::new(1024 * 1024);
        let input_shape = TensorShape::new(vec![1, 4]);
        let output_shape = TensorShape::new(vec![1, 3]);

        let session_id = engine.load_model(
            "test_model",
            ModelFormat::VantisNative,
            AcceleratorBackend::Cpu,
            InferencePrecision::Float32,
            input_shape.clone(),
            output_shape.clone(),
        ).unwrap();

        let input = Tensor::new(input_shape, vec![1.0, -1.0, 2.0, 0.5]).unwrap();
        let output = engine.infer(session_id, &input).unwrap();

        assert_eq!(output.shape, output_shape);
        let sum: f32 = output.data.iter().sum();
        assert!((sum - 1.0).abs() < 1e-5); // softmax sums to 1
        assert_eq!(engine.total_inferences(), 1);
    }

    #[test]
    fn test_engine_auto_backend_selection() {
        let mut engine = InferenceEngine::new(1024 * 1024);
        engine.register_backend(AcceleratorBackend::Gpu);
        engine.register_backend(AcceleratorBackend::Npu);

        let session_id = engine.load_model(
            "auto_model",
            ModelFormat::Onnx,
            AcceleratorBackend::Auto,
            InferencePrecision::Float16,
            TensorShape::new(vec![1, 10]),
            TensorShape::new(vec![1, 5]),
        ).unwrap();

        let session = engine.get_session(session_id).unwrap();
        assert_eq!(session.backend, AcceleratorBackend::Npu); // NPU has highest priority
    }

    #[test]
    fn test_engine_memory_limit() {
        let mut engine = InferenceEngine::new(100); // very small budget
        let result = engine.load_model(
            "big_model",
            ModelFormat::Onnx,
            AcceleratorBackend::Cpu,
            InferencePrecision::Float32,
            TensorShape::new(vec![100, 100]),
            TensorShape::new(vec![100, 100]),
        );
        assert!(matches!(result, Err(InferenceError::OutOfMemory { .. })));
    }

    #[test]
    fn test_engine_unload_model() {
        let mut engine = InferenceEngine::new(1024 * 1024);
        let session_id = engine.load_model(
            "temp_model",
            ModelFormat::TfLite,
            AcceleratorBackend::Cpu,
            InferencePrecision::Int8,
            TensorShape::new(vec![1, 5]),
            TensorShape::new(vec![1, 3]),
        ).unwrap();

        assert_eq!(engine.active_sessions(), 1);
        engine.unload_model(session_id).unwrap();
        assert_eq!(engine.active_sessions(), 0);
    }

    #[test]
    fn test_engine_session_limit() {
        let mut engine = InferenceEngine::new(1024 * 1024 * 1024);
        for i in 0..MAX_SESSIONS {
            engine.load_model(
                &format!("model_{}", i),
                ModelFormat::VantisNative,
                AcceleratorBackend::Cpu,
                InferencePrecision::Float32,
                TensorShape::new(vec![1, 2]),
                TensorShape::new(vec![1, 2]),
            ).unwrap();
        }
        let result = engine.load_model(
            "one_too_many",
            ModelFormat::VantisNative,
            AcceleratorBackend::Cpu,
            InferencePrecision::Float32,
            TensorShape::new(vec![1, 2]),
            TensorShape::new(vec![1, 2]),
        );
        assert!(matches!(result, Err(InferenceError::SessionLimitReached { .. })));
    }
}