// ARM64 Display Drivers for VantisOS v0.6.0
// Mobile display support

pub mod mipi_dsi;
pub mod touchscreen;
pub mod gpu;

pub use mipi_dsi::{MipiDsiController, DisplayTiming, ColorFormat, DisplayManager, DisplayStats};
pub use touchscreen::{TouchscreenController, TouchEvent, TouchGesture, TouchscreenStats};
pub use gpu::{GpuController, GpuType, GpuStats};