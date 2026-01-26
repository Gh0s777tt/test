use crate::horizon::framebuffer::Framebuffer;

pub fn draw(fb: &mut Framebuffer) {
    for y in 0..30 {
        for x in 0..fb.width {
            fb.put_pixel(x, y, 0xFF3B4252);
        }
    }
}
