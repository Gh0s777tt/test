// VESA VBE Graphics Driver - VantisOS
//
// This module implements VESA VBE (VESA BIOS Extensions)
// for graphics mode support on VGA-compatible displays.

use alloc::vec::Vec;

/// VESA VBE controller information
#[repr(C, packed)]
pub struct VesaVbeControllerInfo {
    pub signature: [u8; 4],      // "VESA"
    pub version: u16,             // VBE version
    pub oem_string_ptr: u32,      // OEM string pointer
    pub capabilities: u32,        // Capabilities
    pub video_mode_ptr: u32,      // Video mode pointer
    pub total_memory: u16,        // Total memory in 64KB blocks
    pub oem_software_rev: u16,    // OEM software revision
    pub oem_vendor_name_ptr: u32,  // OEM vendor name pointer
    pub oem_product_name_ptr: u32, // OEM product name pointer
    pub oem_product_rev_ptr: u32,  // OEM product revision pointer
    pub reserved: [u8; 222],      // Reserved
    pub oem_data: [u8; 256],      // OEM data
}

/// VESA VBE mode information
#[repr(C, packed)]
pub struct VesaVbeModeInfo {
    pub mode_attributes: u16,     // Mode attributes
    pub win_a_attributes: u8,     // Window A attributes
    pub win_b_attributes: u8,     // Window B attributes
    pub win_granularity: u16,     // Window granularity
    pub win_size: u16,            // Window size
    pub win_a_segment: u16,       // Window A segment
    pub win_b_segment: u16,       // Window B segment
    pub win_func_ptr: u32,        // Window function pointer
    pub bytes_per_scanline: u16,  // Bytes per scanline
    pub x_resolution: u16,        // X resolution
    pub y_resolution: u16,        // Y resolution
    pub x_char_size: u8,          // X character size
    pub y_char_size: u8,          // Y character size
    pub number_of_planes: u8,     // Number of planes
    pub bits_per_pixel: u8,       // Bits per pixel
    pub number_of_banks: u8,      // Number of banks
    pub memory_model: u8,         // Memory model
    pub bank_size: u8,            // Bank size
    pub number_of_image_pages: u8, // Number of image pages
    pub reserved0: u8,            // Reserved
    pub red_mask_size: u8,        // Red mask size
    pub red_field_position: u8,   // Red field position
    pub green_mask_size: u8,      // Green mask size
    pub green_field_position: u8, // Green field position
    pub blue_mask_size: u8,       // Blue mask size
    pub blue_field_position: u8,  // Blue field position
    pub rsvd_mask_size: u8,       // Reserved mask size
    pub rsvd_field_position: u8,  // Reserved field position
    pub direct_color_mode_info: u8, // Direct color mode info
    pub phys_base_ptr: u32,       // Physical base pointer
    pub reserved1: [u8; 212],     // Reserved
}

/// VESA VBE color
#[derive(Debug, Clone, Copy)]
pub struct VesaColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl VesaColor {
    /// Create a new VESA color
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
    
    /// Convert to RGB888
    pub fn to_rgb888(&self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }
    
    /// Convert to RGB565
    pub fn to_rgb565(&self) -> u16 {
        let r = (self.red as u16 >> 3) & 0x1F;
        let g = (self.green as u16 >> 2) & 0x3F;
        let b = (self.blue as u16 >> 3) & 0x1F;
        (r << 11) | (g << 5) | b
    }
}

/// VESA VBE mode
#[derive(Debug, Clone)]
pub struct VesaMode {
    pub mode_number: u16,
    pub width: u16,
    pub height: u16,
    pub bits_per_pixel: u8,
    pub framebuffer: *mut u8,
}

impl VesaMode {
    /// Create a new VESA mode
    pub fn new(mode_number: u16, width: u16, height: u16, bits_per_pixel: u8) -> Self {
        Self {
            mode_number,
            width,
            height,
            bits_per_pixel,
            framebuffer: 0 as *mut u8,
        }
    }
    
    /// Get framebuffer size
    pub fn get_framebuffer_size(&self) -> usize {
        (self.width as usize) * (self.height as usize) * (self.bits_per_pixel as usize / 8)
    }
}

/// VESA VBE driver
pub struct VesaVbeDriver {
    current_mode: Option<VesaMode>,
    available_modes: Vec<VesaMode>,
}

impl VesaVbeDriver {
    /// Create a new VESA VBE driver
    pub fn new() -> Self {
        Self {
            current_mode: None,
            available_modes: Vec::new(),
        }
    }
    
    /// Initialize the driver
    pub fn init(&mut self) -> Result<(), ()> {
        // TODO: Initialize VESA VBE driver
        // This is a placeholder for actual hardware initialization
        Ok(())
    }
    
    /// Get available modes
    pub fn get_available_modes(&self) -> &[VesaMode] {
        &self.available_modes
    }
    
    /// Set video mode
    pub fn set_mode(&mut self, mode_number: u16) -> Result<(), ()> {
        // TODO: Set video mode
        Err(())
    }
    
    /// Get current mode
    pub fn get_current_mode(&self) -> Option<&VesaMode> {
        self.current_mode.as_ref()
    }
    
    /// Put pixel
    pub fn put_pixel(&self, x: u16, y: u16, color: VesaColor) {
        if let Some(mode) = &self.current_mode {
            if x < mode.width && y < mode.height {
                let offset = (y as usize * mode.width as usize + x as usize) * (mode.bits_per_pixel as usize / 8);
                unsafe {
                    let framebuffer = mode.framebuffer.add(offset);
                    
                    match mode.bits_per_pixel {
                        32 => {
                            let color_value = color.to_rgb888();
                            *(framebuffer as *mut u32) = color_value;
                        }
                        24 => {
                            *(framebuffer.add(0)) = color.blue;
                            *(framebuffer.add(1)) = color.green;
                            *(framebuffer.add(2)) = color.red;
                        }
                        16 => {
                            let color_value = color.to_rgb565();
                            *(framebuffer as *mut u16) = color_value;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    
    /// Get pixel
    pub fn get_pixel(&self, x: u16, y: u16) -> Option<VesaColor> {
        if let Some(mode) = &self.current_mode {
            if x < mode.width && y < mode.height {
                let offset = (y as usize * mode.width as usize + x as usize) * (mode.bits_per_pixel as usize / 8);
                unsafe {
                    let framebuffer = mode.framebuffer.add(offset);
                    
                    match mode.bits_per_pixel {
                        32 => {
                            let color_value = *(framebuffer as *mut u32);
                            Some(VesaColor {
                                red: ((color_value >> 16) & 0xFF) as u8,
                                green: ((color_value >> 8) & 0xFF) as u8,
                                blue: (color_value & 0xFF) as u8,
                            })
                        }
                        24 => {
                            Some(VesaColor {
                                red: *framebuffer.add(2),
                                green: *framebuffer.add(1),
                                blue: *framebuffer.add(0),
                            })
                        }
                        16 => {
                            let color_value = *(framebuffer as *mut u16);
                            Some(VesaColor {
                                red: ((color_value >> 11) & 0x1F) as u8 * 8,
                                green: ((color_value >> 5) & 0x3F) as u8 * 4,
                                blue: (color_value & 0x1F) as u8 * 8,
                            })
                        }
                        _ => None,
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Fill rectangle
    pub fn fill_rect(&self, x: u16, y: u16, width: u16, height: u16, color: VesaColor) {
        for py in y..y + height {
            for px in x..x + width {
                self.put_pixel(px, py, color);
            }
        }
    }
}

/// Initialize VESA VBE driver
pub fn init() {
    // TODO: Initialize VESA VBE driver
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vesa_color() {
        let color = VesaColor::new(255, 128, 64);
        assert_eq!(color.to_rgb888(), 0xFF8040);
    }
    
    #[test]
    fn test_vesa_mode() {
        let mode = VesaMode::new(0x118, 1024, 768, 32);
        assert_eq!(mode.width, 1024);
        assert_eq!(mode.height, 768);
        assert_eq!(mode.bits_per_pixel, 32);
    }
    
    #[test]
    fn test_vesa_driver_creation() {
        let driver = VesaVbeDriver::new();
        assert!(driver.get_current_mode().is_none());
    }
}