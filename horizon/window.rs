pub struct Window {
    pub x: i32,
    pub y: i32,
    pub w: usize,
    pub h: usize,
    pub title: &'static str,
}

impl Window {
    pub fn draw(&self, fb: &mut crate::horizon::framebuffer::Framebuffer) {
        for dy in 0..self.h {
            for dx in 0..self.w {
                fb.put_pixel(
                    (self.x as usize) + dx,
                    (self.y as usize) + dy,
                    0xFF2E3440,
                );
            }
        }
    }
}
