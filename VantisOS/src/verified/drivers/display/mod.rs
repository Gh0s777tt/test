// Display Drivers Module - VantisOS
//
// This module provides display drivers for VantisOS, including:
// - VGA text mode driver
// - VESA VBE graphics driver
// - Framebuffer management
// - Graphics primitives

pub mod vga_text;
pub mod vesa_vbe;
pub mod framebuffer;
pub mod graphics;

pub use vga_text::{VgaTextWriter, VgaColor, VgaColorEntry};
pub use vesa_vbe::{VesaVbeDriver, VesaMode, VesaColor};
pub use framebuffer::{Framebuffer, FramebufferInfo, FramebufferManager};
pub use graphics::{GraphicsContext, Point, Size, Rect, Color};

/// Display drivers initialization
pub fn init() {
    vga_text::init();
    vesa_vbe::init();
    framebuffer::init();
    graphics::init();
}