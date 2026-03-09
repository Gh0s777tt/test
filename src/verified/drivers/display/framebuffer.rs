// Framebuffer Management - VantisOS
//
// This module implements framebuffer management for
// graphics mode displays.

use alloc::vec::Vec;
use alloc::sync::Arc;
use spin::Mutex;

/// Framebuffer information
#[derive(Debug, Clone)]
pub struct FramebufferInfo {
    pub address: usize,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bits_per_pixel: u8,
    pub red_mask: u8,
    pub green_mask: u8,
    pub blue_mask: u8,
}

impl FramebufferInfo {
    /// Create a new framebuffer info
    pub fn new(
        address: usize,
        width: u32,
        height: u32,
        pitch: u32,
        bits_per_pixel: u8,
    ) -> Self {
        Self {
            address,
            width,
            height,
            pitch,
            bits_per_pixel,
            red_mask: 0xFF,
            green_mask: 0xFF,
            blue_mask: 0xFF,
        }
    }
    
    /// Get framebuffer size
    pub fn get_size(&self) -> usize {
        (self.height as usize) * (self.pitch as usize)
    }
    
    /// Get bytes per pixel
    pub fn get_bytes_per_pixel(&self) -> usize {
        (self.bits_per_pixel as usize + 7) / 8
    }
}

/// Framebuffer
pub struct Framebuffer {
    info: FramebufferInfo,
    buffer: *mut u8,
}

impl Framebuffer {
    /// Create a new framebuffer
    pub fn new(info: FramebufferInfo) -> Self {
        Self {
            buffer: info.address as *mut u8,
            info,
        }
    }
    
    /// Get framebuffer info
    pub fn get_info(&self) -> &FramebufferInfo {
        &self.info
    }
    
    /// Get framebuffer buffer
    pub fn get_buffer(&self) -> *mut u8 {
        self.buffer
    }
    
    /// Put pixel
    pub fn put_pixel(&self, x: u32, y: u32, color: u32) {
        if x < self.info.width && y < self.info.height {
            let offset = (y as usize * self.info.pitch as usize + x as usize * self.info.get_bytes_per_pixel());
            unsafe {
                let pixel = self.buffer.add(offset);
                
                match self.info.bits_per_pixel {
                    32 => {
                        *(pixel as *mut u32) = color;
                    }
                    24 => {
                        *(pixel.add(0)) = (color & 0xFF) as u8;
                        *(pixel.add(1)) = ((color >> 8) & 0xFF) as u8;
                        *(pixel.add(2)) = ((color >> 16) & 0xFF) as u8;
                    }
                    16 => {
                        *(pixel as *mut u16) = color as u16;
                    }
                    _ => {}
                }
            }
        }
    }
    
    /// Get pixel
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<u32> {
        if x < self.info.width && y < self.info.height {
            let offset = (y as usize * self.info.pitch as usize + x as usize * self.info.get_bytes_per_pixel());
            unsafe {
                let pixel = self.buffer.add(offset);
                
                match self.info.bits_per_pixel {
                    32 => Some(*(pixel as *mut u32)),
                    24 => Some(
                        (*pixel.add(0) as u32) |
                        ((*pixel.add(1) as u32) << 8) |
                        ((*pixel.add(2) as u32) << 16)
                    ),
                    16 => Some(*(pixel as *mut u16) as u32),
                    _ => None,
                }
            }
        } else {
            None
        }
    }
    
    /// Fill rectangle
    pub fn fill_rect(&self, x: u32, y: u32, width: u32, height: u32, color: u32) {
        for py in y..y + height {
            for px in x..x + width {
                self.put_pixel(px, py, color);
            }
        }
    }
    
    /// Copy rectangle
    pub fn copy_rect(&self, src_x: u32, src_y: u32, dst_x: u32, dst_y: u32, width: u32, height: u32) {
        for py in 0..height {
            for px in 0..width {
                if let Some(color) = self.get_pixel(src_x + px, src_y + py) {
                    self.put_pixel(dst_x + px, dst_y + py, color);
                }
            }
        }
    }
    
    /// Clear framebuffer
    pub fn clear(&self, color: u32) {
        self.fill_rect(0, 0, self.info.width, self.info.height, color);
    }
    
    /// Blit from buffer
    pub fn blit(&self, x: u32, y: u32, width: u32, height: u32, data: &[u8]) {
        let bytes_per_pixel = self.info.get_bytes_per_pixel();
        
        for py in 0..height {
            for px in 0..width {
                let src_offset = (py as usize * width as usize + px as usize) * bytes_per_pixel;
                let dst_offset = ((y + py) as usize * self.info.pitch as usize + (x + px) as usize * bytes_per_pixel);
                
                unsafe {
                    for i in 0..bytes_per_pixel {
                        if src_offset + i < data.len() {
                            *self.buffer.add(dst_offset + i) = data[src_offset + i];
                        }
                    }
                }
            }
        }
    }
}

/// Framebuffer manager
pub struct FramebufferManager {
    framebuffer: Option<Arc<Mutex<Framebuffer>>>,
}

impl FramebufferManager {
    /// Create a new framebuffer manager
    pub fn new() -> Self {
        Self {
            framebuffer: None,
        }
    }
    
    /// Set framebuffer
    pub fn set_framebuffer(&mut self, framebuffer: Framebuffer) {
        self.framebuffer = Some(Arc::new(Mutex::new(framebuffer)));
    }
    
    /// Get framebuffer
    pub fn get_framebuffer(&self) -> Option<Arc<Mutex<Framebuffer>>> {
        self.framebuffer.clone()
    }
}

/// Initialize framebuffer management
pub fn init() {
    // TODO: Initialize framebuffer management
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_framebuffer_info() {
        let info = FramebufferInfo::new(0xE0000000, 1024, 768, 4096, 32);
        
        assert_eq!(info.width, 1024);
        assert_eq!(info.height, 768);
        assert_eq!(info.get_bytes_per_pixel(), 4);
    }
    
    #[test]
    fn test_framebuffer_creation() {
        let info = FramebufferInfo::new(0xE0000000, 1024, 768, 4096, 32);
        let framebuffer = Framebuffer::new(info);
        
        assert_eq!(framebuffer.get_info().width, 1024);
    }
}