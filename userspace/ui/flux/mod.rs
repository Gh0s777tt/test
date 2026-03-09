pub mod renderer;
pub mod scene;
pub mod vector;
pub mod input;
pub mod theme;
pub mod animation;

// Re-exports for convenience
pub use renderer::{Renderer, RendererConfig, GraphicsBackend, Color, Vertex2D, Vertex3D};
pub use input::{InputManager, InputEvent, InputState, GestureRecognizer, GestureEvent};
pub use theme::{ThemeManager, Theme, ColorPalette, ComponentStyle, ThemeColor};
pub use animation::{AnimationManager, Animation, Transition, EasingFunction};
